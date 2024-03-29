use crate::state::AuctionHouseV2Data;
use crate::utils::create_program_associated_token_account;
use crate::ID as PROGRAM_ID;
use crate::{constants::*, errors::AuctionHouseV2Errors};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction::transfer;
use anchor_lang::solana_program::{
    program::invoke,
    program_pack::{IsInitialized, Pack},
};
use anchor_lang::{
    accounts::{account::Account, program::Program, unchecked_account::UncheckedAccount},
    system_program::System,
};
use anchor_spl::token::{Mint, Token};
use mpl_utils::create_or_allocate_account_raw;

#[derive(Accounts)]
#[instruction(buyer_price:u64)]
pub struct BidInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),auction_house_authority.key().as_ref(),treasury_mint.key().as_ref()],bump=auction_house.bump)]
    pub auction_house: Account<'info, AuctionHouseV2Data>,

    /// CHECK: Verified in the auction house seeds contraints
    pub auction_house_authority: UncheckedAccount<'info>,

    pub treasury_mint: Account<'info, Mint>,

    #[account(mut)]
    pub bidder: Signer<'info>,

    /// CHECK: Validated in CPI
    #[account(mut)]
    pub payment_account: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub asset_id: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(mut,seeds=[ESCROW.as_ref(),auction_house.key().as_ref(),bidder.key().as_ref()],bump)]
    pub buyer_escrow: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(mut,seeds=[
        TRADE_STATE.as_ref(),
        bidder.key().as_ref(),
        auction_house.key().as_ref(),
        asset_id.key().as_ref(),
        buyer_price.to_le_bytes().as_ref()
        ],bump)]
    pub buyer_trade_state: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn bid(ctx: Context<BidInstruction>, buyer_price: u64) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.to_account_info();
    let treasury_mint = ctx.accounts.treasury_mint.to_account_info();
    let bidder = &ctx.accounts.bidder.to_account_info();
    let asset_id = ctx.accounts.asset_id.to_account_info();
    let buyer_escrow = &ctx.accounts.buyer_escrow.to_account_info();
    let buyer_trade_state_info = ctx.accounts.buyer_trade_state.to_account_info();
    let system_program = &ctx.accounts.system_program.to_account_info();
    let token_program = &ctx.accounts.token_program;
    let payment_account = ctx.accounts.payment_account.to_account_info();
    let rent = &ctx.accounts.rent;
    let buyer_trade_state_bump = ctx
        .bumps
        .get("buyer_trade_state")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;
    let auction_house_key = auction_house.key();
    let token_program_key = token_program.key();
    let bidder_key = bidder.key();
    let buyer_escrow_bump = ctx
        .bumps
        .get("buyer_escrow")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    let is_native = treasury_mint.key() == spl_token::native_mint::ID;

    if is_native {
        let minimum_funds_required = buyer_price
            .checked_add(rent.minimum_balance(buyer_escrow.data_len()))
            .ok_or(AuctionHouseV2Errors::NumericOverflow)?;

        if buyer_escrow.lamports() < minimum_funds_required {
            let required_funds = minimum_funds_required
                .checked_sub(buyer_escrow.lamports())
                .ok_or(AuctionHouseV2Errors::NumericOverflow)?;

            let total_buyer_funds = bidder
                .lamports()
                .checked_sub(rent.minimum_balance(bidder.data_len()))
                .ok_or(AuctionHouseV2Errors::NumericOverflow)?;

            if total_buyer_funds < required_funds {
                return Err(AuctionHouseV2Errors::NotEnoughFunds)?;
            }

            let transfer_instruction_accounts = [
                bidder.to_account_info(),
                buyer_escrow.to_account_info(),
                system_program.to_account_info(),
            ];
            let transfer_instruction = transfer(bidder.key, buyer_escrow.key, required_funds);
            invoke(&transfer_instruction, &transfer_instruction_accounts)?;
        }
    } else {
        // create token account for the escrow
        let escrow_signer_seeds = [
            ESCROW.as_ref(),
            auction_house_key.as_ref(),
            bidder_key.as_ref(),
            &[*buyer_escrow_bump],
        ];
        create_program_associated_token_account(
            buyer_escrow,
            bidder,
            auction_house,
            treasury_mint,
            system_program,
            token_program.to_account_info(),
            &escrow_signer_seeds,
        )?;

        // transfer the required amount to escrow
        let escrow_data = spl_token::state::Account::unpack(&buyer_escrow.data.borrow())?;

        if !escrow_data.is_initialized() {
            return Err(AuctionHouseV2Errors::AccountNotInitialized.into());
        }

        if escrow_data.amount < buyer_price {
            let required_amount = buyer_price
                .checked_sub(escrow_data.amount)
                .ok_or(AuctionHouseV2Errors::NumericOverflow)?;
            let transfer_instruction_accounts = [
                bidder.to_account_info(),
                buyer_escrow.to_account_info(),
                token_program.to_account_info(),
                payment_account.to_account_info(),
                system_program.to_account_info(),
            ];
            let transfer_instruction = spl_token::instruction::transfer(
                &token_program_key,
                payment_account.key,
                buyer_escrow.key,
                &bidder_key,
                &[&bidder_key],
                required_amount,
            )?;
            invoke(&transfer_instruction, &transfer_instruction_accounts)?;
        }
    }

    if buyer_trade_state_info.data_is_empty() {
        let buyer_trade_state_seeds = [
            TRADE_STATE.as_ref(),
            bidder_key.as_ref(),
            auction_house_key.as_ref(),
            asset_id.key.as_ref(),
            &buyer_price.to_le_bytes(),
            &[*buyer_trade_state_bump],
        ];
        create_or_allocate_account_raw(
            PROGRAM_ID,
            &buyer_trade_state_info,
            &system_program,
            &bidder,
            TRADE_STATE_SIZE,
            &buyer_trade_state_seeds,
        )?;
    }
    let data = &mut buyer_trade_state_info.data.borrow_mut();
    data[0] = *buyer_trade_state_bump;

    Ok(())
}
