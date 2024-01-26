use crate::constants::*;
use crate::errors::AuctionHouseV2Errors;
use crate::state::AuctionHouseV2Data;
use crate::utils::{assert_valid_trade_state, close, cmp_bytes};
use anchor_lang::prelude::*;
use anchor_lang::{
    accounts::{account::Account, program::Program, unchecked_account::UncheckedAccount},
    system_program::System,
};
use anchor_spl::token::Mint;
use mpl_bubblegum::instructions::DelegateCpiBuilder;

#[derive(Accounts)]
#[instruction(price:u64)]
pub struct CancelInstruction<'info> {
    #[account(seeds=[
        AUCTION_HOUSE.as_ref(),
        authority.key().as_ref(),
        treasury_mint.key().as_ref()
        ],
        has_one=authority,
        bump=auction_house.bump)]
    pub auction_house: Account<'info, AuctionHouseV2Data>,

    /// CHECK: Verified in auction house seeds
    pub authority: UncheckedAccount<'info>,

    pub treasury_mint: Account<'info, Mint>,

    /// CHECK: Verified in CPI
    pub asset_id: UncheckedAccount<'info>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    /// CHECK: validated in the main functionality
    #[account(mut)]
    pub trade_state: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CancelRemainingAccounts<'info> {
    /// CHECK: Verified in CPI
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: mutated in downstream program
    #[account(mut)]
    pub merke_tree: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub tree_config: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub bubblegum_program: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[PROGRAM.as_bytes(), SIGNER.as_bytes()], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Verified in CPI
    pub log_wrapper: UncheckedAccount<'info>,
    // Cnft proofs in the remaining accounts
}

pub fn cancel<'a>(
    ctx: Context<'_, '_, '_, 'a, CancelInstruction<'a>>,
    price: u64,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.to_account_info().clone();
    let wallet = ctx.accounts.wallet.to_account_info().clone();
    let trade_state_info = ctx.accounts.trade_state.to_account_info().clone();
    let asset_id = ctx.accounts.asset_id.to_account_info().clone();
    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    if trade_state_info.data_is_empty() {
        return Err(AuctionHouseV2Errors::InvalidSellerTradeState.into());
    }

    let compression_program = next_account_info(remaining_accounts)?;
    if cmp_bytes(
        &compression_program.key.as_ref(),
        COMPRESSION_ID.as_bytes(),
        32,
    ) {
        let merkle_tree = next_account_info(remaining_accounts)?;
        let tree_config = next_account_info(remaining_accounts)?;
        let bubblegum_program = next_account_info(remaining_accounts)?;
        let program_as_signer = next_account_info(remaining_accounts)?;
        let system_program = next_account_info(remaining_accounts)?;
        let log_wrapper = next_account_info(remaining_accounts)?;
        if !assert_valid_trade_state(
            trade_state_info.key,
            wallet.key,
            auction_house.key,
            asset_id.key,
            price.to_be_bytes(),
        ) {
            return Err(AuctionHouseV2Errors::InvalidBuyingOrSellingOrder.into());
        }
        let mut builder = DelegateCpiBuilder::new(&bubblegum_program);
        builder
            .leaf_owner(&wallet)
            .tree_config(&tree_config)
            .previous_leaf_delegate(&program_as_signer)
            .new_leaf_delegate(&wallet)
            .merkle_tree(&merkle_tree)
            .log_wrapper(&log_wrapper)
            .compression_program(&compression_program)
            .system_program(&system_program)
            .root(root)
            .data_hash(data_hash)
            .creator_hash(creator_hash)
            .nonce(nonce)
            .index(index);
        for info in remaining_accounts {
            builder.add_remaining_account(info, false, false);
        }
        builder.invoke()?;
    }
    close(trade_state_info, wallet)?;

    Ok(())
}
