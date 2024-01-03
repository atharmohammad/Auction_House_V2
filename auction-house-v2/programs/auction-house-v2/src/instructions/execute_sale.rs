use crate::{
    constants::*,
    state::{AuctionHouseV2Data, BuyerTradeState, SellerTradeState},
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct ExecuteSaleInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),auction_house_authority.key().as_ref(),treasury_mint.key().as_ref()],bump=auction_house.bump)]
    auction_house: Account<'info, AuctionHouseV2Data>,

    auction_house_authority: AccountInfo<'info>,

    treasury_mint: Account<'info, Mint>,

    /// CHECK: Verified in CPI
    tree_config: UncheckedAccount<'info>,

    #[account(mut)]
    seller: AccountInfo<'info>,

    /// CHECK: mutated in downstream program
    #[account(mut)]
    merke_tree: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    previous_leaf_delegate: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(
        mut,
        seeds=[
            TRADE_STATE.as_ref(),
            seller.key().as_ref(),
            auction_house.key().as_ref(),
            asset_id.key().as_ref(),
        ],
        bump=seller_trade_state.bump
    )]
    seller_trade_state: Account<'info, SellerTradeState>,

    #[account(mut)]
    buyer: AccountInfo<'info>,

    #[account(mut,seeds=[ESCROW.as_ref(),auction_house.key().as_ref(),buyer.key().as_ref()],bump)]
    buyer_escrow: UncheckedAccount<'info>,

    #[account(mut,seeds=[TRADE_STATE.as_ref(),buyer.key().as_ref(),auction_house.key().as_ref(),asset_id.key().as_ref()],bump=buyer_trade_state.bump)]
    buyer_trade_state: Account<'info, BuyerTradeState>,

    /// CHECK: Verified in CPI
    asset_id: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump=auction_house.fee_account_bump)]
    auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[PROGRAM.as_bytes(), SIGNER.as_bytes()], bump)]
    program_as_signer: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    bubblegum_program: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    compression_program: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    /// CHECK: Verified in CPI
    log_wrapper: UncheckedAccount<'info>,
    // Cnft proofs in the remaining accounts
}
