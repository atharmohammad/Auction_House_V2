use crate::state::{AuctionHouseV2Data, BuyerTradeState};
use crate::ID as PROGRAM_ID;
use crate::{constants::*, errors::AuctionHouseV2Errors};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use anchor_lang::{
    accounts::{account::Account, program::Program, unchecked_account::UncheckedAccount},
    system_program::System,
};
use anchor_spl::token::Mint;
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

    /// CHECK: Verified in CPI
    pub asset_id: UncheckedAccount<'info>,

    #[account(mut,seeds=[ESCROW.as_ref(),auction_house.key().as_ref(),bidder.key().as_ref()],bump)]
    pub buyer_escrow: UncheckedAccount<'info>,

    #[account(mut,seeds=[TRADE_STATE.as_ref(),bidder.key().as_ref(),auction_house.key().as_ref(),asset_id.key().as_ref()],bump)]
    pub buyer_trade_state: Account<'info, BuyerTradeState>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn bid(ctx: Context<BidInstruction>, buyer_price: u64) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.to_account_info().clone();
    let _treasury_mint = ctx.accounts.treasury_mint.to_account_info().clone();
    let bidder = ctx.accounts.bidder.to_account_info().clone();
    let asset_id = ctx.accounts.asset_id.to_account_info().clone();
    let buyer_escrow = ctx.accounts.buyer_escrow.to_account_info().clone();
    let buyer_trade_state = ctx.accounts.buyer_trade_state.to_account_info().clone();
    let system_program = ctx.accounts.system_program.to_account_info().clone();
    let rent = ctx.accounts.rent.clone();
    let buyer_trade_state_bump = ctx
        .bumps
        .get("buyer_trade_state")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;
    let buyer_escrow_bump = ctx
        .bumps
        .get("buyer_escrow")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    if buyer_escrow.data_is_empty() {
        let signer_seeds = [
            ESCROW.as_ref(),
            auction_house.key.as_ref(),
            bidder.key.as_ref(),
        ];
        create_or_allocate_account_raw(
            PROGRAM_ID,
            &buyer_escrow,
            &system_program,
            &bidder,
            0,
            &signer_seeds,
        )?;
    }
    let total_escrow_funds = buyer_escrow
        .lamports()
        .checked_sub(rent.minimum_balance(buyer_escrow.data_len()))
        .ok_or(AuctionHouseV2Errors::NumericOverflow)?;

    if total_escrow_funds < buyer_price {
        let required_funds = buyer_price
            .checked_sub(total_escrow_funds)
            .ok_or(AuctionHouseV2Errors::NumericOverflow)?;

        let total_buyer_funds = bidder
            .lamports()
            .checked_sub(rent.minimum_balance(bidder.data_len()))
            .ok_or(AuctionHouseV2Errors::NumericOverflow)?;

        if total_buyer_funds < required_funds {
            return Err(AuctionHouseV2Errors::NotEnoughFunds)?;
        }

        let from = bidder.to_account_info().clone();
        let to = buyer_escrow.to_account_info().clone();
        let transfer_instruction_accounts = [from, to, system_program.to_account_info()];
        let transfer_instruction = transfer(bidder.key, buyer_escrow.key, required_funds);
        invoke(&transfer_instruction, &transfer_instruction_accounts)?;
    }

    if buyer_trade_state.data_is_empty() {
        let signer_seeds = [
            ESCROW.as_ref(),
            auction_house.key.as_ref(),
            bidder.key.as_ref(),
        ];
        create_or_allocate_account_raw(
            PROGRAM_ID,
            &buyer_trade_state,
            &system_program,
            &bidder,
            TRADE_STATE_SIZE,
            &signer_seeds,
        )?;
    }
    let buyer_trade_state_info = BuyerTradeState {
        auction_house: auction_house.key(),
        buyer: bidder.key(),
        amount: buyer_price,
        asset_id: asset_id.key(),
        bump: *buyer_trade_state_bump,
        escrow_bump: *buyer_escrow_bump,
    };
    buyer_trade_state_info.try_serialize(&mut *buyer_trade_state.try_borrow_mut_data()?)?;
    Ok(())
}
