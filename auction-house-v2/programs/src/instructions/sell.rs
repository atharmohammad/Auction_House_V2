use crate::constants::*;
use crate::errors::AuctionHouseV2Errors;
use crate::state::AuctionHouseV2Data;
use crate::ID as PROGRAM_ID;
use anchor_lang::prelude::*;
use anchor_lang::{
    accounts::{account::Account, program::Program, unchecked_account::UncheckedAccount},
    system_program::System,
};
use anchor_spl::token::Mint;
use mpl_bubblegum::instructions::DelegateCpiBuilder;
use mpl_utils::create_or_allocate_account_raw;
#[derive(Accounts)]
#[instruction(seller_price:u64)]
pub struct SellInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),auction_house_authority.key().as_ref(),treasury_mint.key().as_ref()],bump=auction_house.bump)]
    pub auction_house: Account<'info, AuctionHouseV2Data>,

    /// CHECK: Verified in auction house seeds
    pub auction_house_authority: UncheckedAccount<'info>,

    pub treasury_mint: Account<'info, Mint>,

    /// CHECK: Verified in CPI
    pub tree_config: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: mutated in downstream program
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub previous_leaf_delegate: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(
        mut,
        seeds=[
            TRADE_STATE.as_ref(),
            owner.key().as_ref(),
            auction_house.key().as_ref(),
            asset_id.key().as_ref(),
            seller_price.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub seller_trade_state: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub asset_id: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[PROGRAM.as_bytes(), SIGNER.as_bytes()], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub bubblegum_program: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub compression_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Verified in CPI
    pub log_wrapper: UncheckedAccount<'info>,
    // Cnft proofs in the remaining accounts
}

pub fn sell<'b, 'a>(
    ctx: Context<'_, '_, 'b, 'a, SellInstruction<'a>>,
    seller_price: u64,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
) -> Result<()> {
    let auction_house = &ctx.accounts.auction_house.to_account_info();
    let asset_id = &ctx.accounts.asset_id.to_account_info();
    let merkle_tree = &ctx.accounts.merkle_tree.to_account_info();
    let owner = &ctx.accounts.owner.to_account_info();
    let previous_leaf_delegate = &ctx.accounts.previous_leaf_delegate.to_account_info();
    let seller_trade_state_info = &ctx.accounts.seller_trade_state.to_account_info();
    let program_as_signer = &ctx.accounts.program_as_signer.to_account_info();
    let compression_program = &ctx.accounts.compression_program.to_account_info();
    let system_program = &ctx.accounts.system_program.to_account_info();
    let tree_config = &ctx.accounts.tree_config;
    let log_wrapper = &ctx.accounts.log_wrapper.to_account_info();
    let bubblegum_program = &ctx.accounts.bubblegum_program.to_account_info();
    let remaining_accounts = ctx.remaining_accounts;
    let seller_trade_state_bump = ctx
        .bumps
        .get("seller_trade_state")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    let mut builder = DelegateCpiBuilder::new(&bubblegum_program);
    builder
        .leaf_owner(&owner)
        .tree_config(&tree_config)
        .previous_leaf_delegate(&previous_leaf_delegate)
        .new_leaf_delegate(&program_as_signer)
        .merkle_tree(&merkle_tree)
        .log_wrapper(&log_wrapper)
        .compression_program(&compression_program)
        .system_program(&system_program)
        .root(root)
        .data_hash(data_hash)
        .creator_hash(creator_hash)
        .nonce(nonce)
        .index(index);
    for info in remaining_accounts.iter() {
        builder.add_remaining_account(info, false, false);
    }
    builder.invoke()?;

    if seller_trade_state_info.data_is_empty() {
        let seller_trade_state_seeds = [
            TRADE_STATE.as_ref(),
            owner.key.as_ref(),
            auction_house.key.as_ref(),
            asset_id.key.as_ref(),
            &seller_price.to_le_bytes(),
            &[*seller_trade_state_bump],
        ];
        create_or_allocate_account_raw(
            PROGRAM_ID,
            &seller_trade_state_info,
            &system_program,
            &owner,
            TRADE_STATE_SIZE,
            &seller_trade_state_seeds,
        )?;
    }

    let data = &mut seller_trade_state_info.data.borrow_mut();
    data[0] = *seller_trade_state_bump;

    Ok(())
}
