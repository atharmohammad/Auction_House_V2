use crate::{
    constants::*,
    errors::AuctionHouseV2Errors,
    state::{AuctionHouseV2Data, BuyerTradeState, SellerTradeState},
    utils::assert_trade_states,
};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::{prelude::*, solana_program::system_instruction::transfer};
use anchor_spl::token::Mint;
use mpl_bubblegum::instructions::TransferCpiBuilder;

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

    auction_house_v2_program: AccountInfo<'info>,

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

pub fn execute_sale<'a>(
    ctx: Context<'_, '_, '_, 'a, ExecuteSaleInstruction<'a>>,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.clone();
    let asset_id_info = ctx.accounts.asset_id.to_account_info().clone();
    let merkle_tree_info = ctx.accounts.merke_tree.to_account_info().clone();
    let seller_info = ctx.accounts.seller.to_account_info().clone();
    let buyer_info = ctx.accounts.buyer.to_account_info().clone();
    let previous_leaf_delegate_info = ctx
        .accounts
        .previous_leaf_delegate
        .to_account_info()
        .clone();
    let seller_trade_state = ctx.accounts.seller_trade_state.clone();
    let buyer_trade_state = ctx.accounts.buyer_trade_state.clone();
    let buyer_escrow = ctx.accounts.buyer_escrow.to_account_info().clone();
    let program_as_signer_info = ctx.accounts.program_as_signer.to_account_info().clone();
    let compression_program_info = ctx.accounts.compression_program.to_account_info().clone();
    let system_program_info = ctx.accounts.system_program.to_account_info().clone();
    let tree_config_info = ctx.accounts.tree_config.to_account_info().clone();
    let log_wrapper_info = ctx.accounts.log_wrapper.to_account_info().clone();
    let auction_house_info = ctx.accounts.auction_house.to_account_info().clone();
    let bubblegum_program_info = ctx.accounts.bubblegum_program.to_account_info().clone();
    let remaining_accounts = ctx.remaining_accounts;

    let seller_trade_state_bump = seller_trade_state.bump;
    // assert buyer and seller trade state configs
    assert_trade_states(&seller_trade_state, &buyer_trade_state)?;
    if buyer_escrow.lamports() < buyer_trade_state.amount {
        return Err(AuctionHouseV2Errors::NotEnoughFunds.into());
    }

    let buyer_escrow_signer_seeds = [
        ESCROW.as_bytes(),
        auction_house_info.key.as_ref(),
        buyer_info.key.as_ref(),
    ];
    // todo: pay marketplace fees
    // todo: pay creator royalties

    let buyer_funds_after_tax: u64 = 2;

    // transfer nft to buyer
    let mut transfer_nft_to_buyer_builder = TransferCpiBuilder::new(&bubblegum_program_info);
    transfer_nft_to_buyer_builder
        .leaf_owner(&seller_info, false)
        .leaf_delegate(&program_as_signer_info, true)
        .new_leaf_owner(&buyer_info)
        .tree_config(&tree_config_info)
        .merkle_tree(&merkle_tree_info)
        .log_wrapper(&log_wrapper_info)
        .compression_program(&compression_program_info)
        .system_program(&system_program_info)
        .root(root)
        .data_hash(data_hash)
        .creator_hash(creator_hash)
        .nonce(nonce)
        .index(index);
    for info in remaining_accounts.iter() {
        transfer_nft_to_buyer_builder.add_remaining_account(info, false, false);
    }
    transfer_nft_to_buyer_builder.invoke()?;

    // transfer funds to seller
    let transfer_instruction = transfer(buyer_escrow.key, seller_info.key, buyer_funds_after_tax);
    let transfer_accounts = [buyer_escrow, seller_info, system_program_info];
    invoke_signed(
        &transfer_instruction,
        &transfer_accounts,
        &[&buyer_escrow_signer_seeds],
    )?;

    // todo: close trade states
    Ok(())
}
