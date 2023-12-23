use crate::constants::*;
use crate::state::AuctionHouseV2Data;
use crate::utils::invoke_with_remaining_accounts;
use anchor_lang::prelude::*;
use anchor_lang::{
    accounts::{account::Account, program::Program, unchecked_account::UncheckedAccount},
    system_program::System,
};
use mpl_bubblegum::instructions::{DelegateCpiAccounts, DelegateInstructionArgs};
#[derive(Accounts)]
#[instruction(buyer_price:u64)]
pub struct SellInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),authority.key().as_ref(),treasury_mint.key().as_ref()],bump)]
    auction_house: Account<'info, AuctionHouseV2Data>,

    authority: UncheckedAccount<'info>,

    treasury_mint: UncheckedAccount<'info>,

    tree_config: UncheckedAccount<'info>,

    #[account(mut)]
    owner: UncheckedAccount<'info>,

    merke_tree: UncheckedAccount<'info>,

    previous_leaf_delegate: UncheckedAccount<'info>,

    #[account(
        seeds=[
            TRADE_STATE.as_ref(),
            owner.key().as_ref(),
            auction_house.key().as_ref(),
            asset_id.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            &buyer_price.to_le_bytes()
        ],
        bump
    )]
    seller_trade_state: UncheckedAccount<'info>,

    asset_id: UncheckedAccount<'info>,

    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    auction_house_fee_account: UncheckedAccount<'info>,

    #[account(seeds=[PROGRAM.as_bytes(), SIGNER.as_bytes()], bump)]
    program_as_signer: UncheckedAccount<'info>,

    compression_program: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    log_wrapper: UncheckedAccount<'info>,
    // Cnft proofs in the remaining accounts
}
pub fn sell<'b, 'a>(
    ctx: Context<'_, '_, 'b, 'a, SellInstruction<'a>>,
    args: DelegateInstructionArgs,
    proof_len: u64,
) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.clone();
    let authority = ctx.accounts.authority.clone();
    let asset_id = ctx.accounts.asset_id.clone();
    let merkle_tree = ctx.accounts.merke_tree.clone();
    let owner = ctx.accounts.owner.clone();
    let previous_leaf_delegate = ctx.accounts.previous_leaf_delegate.clone();
    let auction_house_fee_account = ctx.accounts.auction_house_fee_account.clone();
    let program_as_signer = ctx.accounts.program_as_signer.clone();
    let compression_program = ctx.accounts.compression_program.clone();
    let system_program = ctx.accounts.system_program.clone();
    let tree_config = ctx.accounts.tree_config.clone();
    let log_wrapper = ctx.accounts.log_wrapper.clone();
    let mut remaining_accounts = ctx.remaining_accounts;
    // check who is signer and pay for trade according to it
    //assert seller trade state
    let mut remaining_account_meta: Vec<(&AccountInfo<'_>, bool, bool)> = Vec::new();

    // Populate the vector based on the dynamic proof_len
    for info in remaining_accounts.iter().take(proof_len as usize) {
        remaining_account_meta.push((info, false, false));
    }
    let delegate_accounts = DelegateCpiAccounts {
        tree_config: &tree_config,
        leaf_owner: &owner.to_account_info(),
        previous_leaf_delegate: &previous_leaf_delegate.to_account_info(),
        new_leaf_delegate: &program_as_signer.to_account_info(),
        merkle_tree: &merkle_tree.to_account_info(),
        log_wrapper: &log_wrapper.to_account_info(),
        compression_program: &compression_program.to_account_info(),
        system_program: &system_program.to_account_info(),
    };

    invoke_with_remaining_accounts(
        &delegate_accounts,
        &args,
        &[&[&[]]],
        &remaining_account_meta,
    )?;

    Ok(())
}
