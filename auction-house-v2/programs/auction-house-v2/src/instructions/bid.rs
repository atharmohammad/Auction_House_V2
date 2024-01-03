use crate::state::{AuctionHouseV2Data, TradeState};
use crate::ID as PROGRAM_ID;
use crate::{constants::*, errors::AuctionHouseV2Errors};
use anchor_lang::prelude::*;
use anchor_lang::{
    accounts::{account::Account, program::Program, unchecked_account::UncheckedAccount},
    system_program::System,
};
use anchor_spl::token::Mint;
use mpl_utils::create_or_allocate_account_raw;

#[derive(Accounts)]
#[instruction(buyer_price:u64)]
pub struct BidInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),auction_house_authority.key().as_ref(),treasury_mint.key().as_ref()],bump)]
    auction_house: Account<'info, AuctionHouseV2Data>,

    /// CHECK: Verified in the auction house seeds contraints
    auction_house_authority: UncheckedAccount<'info>,

    treasury_mint: Account<'info, Mint>,

    #[account(mut)]
    bidder: Signer<'info>,

    /// CHECK: Verified in CPI
    asset_id: UncheckedAccount<'info>,

    #[account(mut,seeds=[ESCROW.as_ref(),auction_house.key().as_ref(),bidder.key().as_ref()],bump)]
    buyer_escrow: UncheckedAccount<'info>,

    #[account(mut,seeds=[TRADE_STATE.as_ref(),auction_house.key().as_ref(),bidder.key().as_ref()],bump)]
    buyer_trade_state: Account<'info, TradeState>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    auction_house_fee_account: UncheckedAccount<'info>,

    system_program: Program<'info, System>,
}

pub fn bid(ctx: Context<BidInstruction>, buyer_price: u64) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.to_account_info().clone();
    let auction_house_authority = ctx
        .accounts
        .auction_house_authority
        .to_account_info()
        .clone();
    let treasury_mint = ctx.accounts.treasury_mint.to_account_info().clone();
    let bidder = ctx.accounts.bidder.to_account_info().clone();
    let asset_id = ctx.accounts.asset_id.to_account_info().clone();
    let buyer_escrow = ctx.accounts.buyer_escrow.to_account_info().clone();
    let buyer_trade_state = ctx.accounts.buyer_trade_state.to_account_info().clone();
    let auction_house_fee_account = ctx
        .accounts
        .auction_house_fee_account
        .to_account_info()
        .clone();
    let system_program = ctx.accounts.system_program.to_account_info().clone();
    let buyer_trade_state_bump = ctx
        .bumps
        .get("buyer_trade_state")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    if bidder.lamports() < buyer_price {
        return Err(AuctionHouseV2Errors::NotEnoughFunds)?;
    }

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
            1,
            &signer_seeds,
        )?;
    }
    // transfer money to escrow
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
    let buyer_trade_state_info = TradeState {
        auction_house: auction_house.key(),
        owner: bidder.key(),
        amount: buyer_price,
        asset_id: asset_id.key(),
        bump: *buyer_trade_state_bump,
    };
    buyer_trade_state_info.try_serialize(&mut *buyer_trade_state.try_borrow_mut_data()?)?;
    Ok(())
}
