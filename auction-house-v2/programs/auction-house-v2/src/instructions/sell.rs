use crate::constants::*;
use crate::errors::AuctionHouseV2Errors;
use crate::state::{AuctionHouseV2Data, TradeState};
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
pub struct SellInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),auction_house_authority.key().as_ref(),treasury_mint.key().as_ref()],bump)]
    auction_house: Account<'info, AuctionHouseV2Data>,

    /// CHECK: Verified in auction house seeds
    auction_house_authority: UncheckedAccount<'info>,

    treasury_mint: Account<'info, Mint>,

    /// CHECK: Verified in CPI
    tree_config: UncheckedAccount<'info>,

    #[account(mut)]
    owner: Signer<'info>,

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
            owner.key().as_ref(),
            auction_house.key().as_ref(),
            asset_id.key().as_ref(),
        ],
        bump
    )]
    seller_trade_state: Account<'info, TradeState>,

    /// CHECK: Verified in CPI
    asset_id: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
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

pub fn sell<'b, 'a>(
    ctx: Context<'_, '_, 'b, 'a, SellInstruction<'a>>,
    seller_price: u64,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.to_account_info().clone();
    let asset_id = ctx.accounts.asset_id.to_account_info().clone();
    let merkle_tree = ctx.accounts.merke_tree.to_account_info().clone();
    let owner = ctx.accounts.owner.to_account_info().clone();
    let previous_leaf_delegate = ctx
        .accounts
        .previous_leaf_delegate
        .to_account_info()
        .clone();
    let seller_trade_state = ctx.accounts.seller_trade_state.to_account_info().clone();
    let program_as_signer = ctx.accounts.program_as_signer.to_account_info().clone();
    let compression_program = ctx.accounts.compression_program.to_account_info().clone();
    let system_program = ctx.accounts.system_program.to_account_info().clone();
    let tree_config = ctx.accounts.tree_config.clone();
    let log_wrapper = ctx.accounts.log_wrapper.to_account_info().clone();
    let bubblegum_program = ctx.accounts.bubblegum_program.to_account_info().clone();
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

    if seller_trade_state.data_is_empty() {
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
            &seller_trade_state,
            &system_program,
            &owner,
            TRADE_STATE_SIZE,
            &seller_trade_state_seeds,
        )?;
    }

    let seller_trade_state_info = TradeState {
        auction_house: auction_house.key(),
        owner: owner.key(),
        amount: seller_price,
        asset_id: asset_id.key(),
        bump: *seller_trade_state_bump,
    };
    seller_trade_state_info.try_serialize(&mut *seller_trade_state.try_borrow_mut_data()?)?;

    Ok(())
}
