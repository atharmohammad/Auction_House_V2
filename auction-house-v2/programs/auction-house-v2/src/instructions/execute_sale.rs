use crate::utils::{close, get_fee_payer, hash_metadata};
use crate::MetadataArgs;
use crate::{
    constants::*, errors::AuctionHouseV2Errors, state::AuctionHouseV2Data, utils::cmp_bytes,
};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::{prelude::*, solana_program::system_instruction::transfer};
use anchor_spl::token::Mint;
use mpl_bubblegum::instructions::TransferCpiBuilder;

#[derive(Accounts)]
#[instruction(buyer_price:u64)]
pub struct ExecuteSaleInstruction<'info> {
    #[account(seeds=[AUCTION_HOUSE.as_ref(),auction_house_authority.key().as_ref(),treasury_mint.key().as_ref()],bump=auction_house.bump)]
    pub auction_house: Box<Account<'info, AuctionHouseV2Data>>,

    /// CHECK: verified in auction_house seeds constraints
    pub auction_house_authority: AccountInfo<'info>,

    pub treasury_mint: Box<Account<'info, Mint>>,

    /// CHECK: Account seeds checked in constraints
    #[account(mut,seeds=[TREASURY.as_bytes(),auction_house.key().as_ref()],bump)]
    pub treasury_account: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub tree_config: UncheckedAccount<'info>,

    /// CHECK: verified in seller_trade_state seeds constraints
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    /// CHECK: mutated in downstream program
    #[account(mut)]
    pub merke_tree: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(
        mut,
        seeds=[
            TRADE_STATE.as_ref(),
            seller.key().as_ref(),
            auction_house.key().as_ref(),
            asset_id.key().as_ref(),
            buyer_price.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub seller_trade_state: UncheckedAccount<'info>,

    /// CHECK: verified in buyer_trade_state seeds constraints
    #[account(mut)]
    pub buyer: AccountInfo<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(mut,seeds=[ESCROW.as_ref(),auction_house.key().as_ref(),buyer.key().as_ref()],bump)]
    pub buyer_escrow: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(mut,seeds=[
            TRADE_STATE.as_ref(),
            buyer.key().as_ref(),
            auction_house.key().as_ref(),
            asset_id.key().as_ref(),
            buyer_price.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub buyer_trade_state: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    pub asset_id: UncheckedAccount<'info>,

    /// CHECK: Account seeds checked in constraints
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump=auction_house.fee_account_bump)]
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
    /* Remaining Accounts
       - ...Creator Accounts
       - ...Cnft proofs in the remaining accounts
    */
}

pub fn execute_sale<'a>(
    ctx: Context<'_, '_, '_, 'a, ExecuteSaleInstruction<'a>>,
    buyer_price: u64,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    royalty_basis_points: u16,
    metadata: MetadataArgs,
) -> Result<()> {
    let auction_house = ctx.accounts.auction_house.clone();
    let auction_house_fee_account = ctx
        .accounts
        .auction_house_fee_account
        .to_account_info()
        .clone();
    let auction_house_authority = ctx.accounts.auction_house_authority.clone();
    let merkle_tree_info = ctx.accounts.merke_tree.to_account_info().clone();
    let seller_info = ctx.accounts.seller.to_account_info().clone();
    let buyer_info = ctx.accounts.buyer.to_account_info().clone();
    let treasury_account = ctx.accounts.treasury_account.to_account_info().clone();
    let seller_trade_state_info = ctx.accounts.seller_trade_state.to_account_info().clone();
    let buyer_trade_state_info = ctx.accounts.buyer_trade_state.to_account_info().clone();
    let buyer_escrow = ctx.accounts.buyer_escrow.to_account_info().clone();
    let program_as_signer_info = ctx.accounts.program_as_signer.to_account_info().clone();
    let compression_program_info = ctx.accounts.compression_program.to_account_info().clone();
    let system_program_info = ctx.accounts.system_program.to_account_info().clone();
    let tree_config_info = ctx.accounts.tree_config.to_account_info().clone();
    let log_wrapper_info = ctx.accounts.log_wrapper.to_account_info().clone();
    let auction_house_info = ctx.accounts.auction_house.to_account_info().clone();
    let bubblegum_program_info = ctx.accounts.bubblegum_program.to_account_info().clone();
    let remaining_accounts = ctx.remaining_accounts;

    let hashed_metadata = hash_metadata(&metadata)?;
    if !cmp_bytes(&data_hash, &hashed_metadata, 32) {
        return Err(AuctionHouseV2Errors::MetadataHashMismatch.into());
    }

    if buyer_trade_state_info.data_is_empty() {
        return Err(AuctionHouseV2Errors::InvalidBuyerTradeState.into());
    }

    if seller_trade_state_info.data_is_empty() {
        return Err(AuctionHouseV2Errors::BothPartiesNeedToAgreeToSale.into());
    }

    // assert buyer and seller trade state configs
    if buyer_escrow.lamports() < buyer_price {
        return Err(AuctionHouseV2Errors::NotEnoughFunds.into());
    }

    let auction_house_fee_account_bump = ctx
        .bumps
        .get("auction_house_fee_account")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    let auction_house_key = auction_house.key();
    let auction_house_fee_payer_seeds = [
        FEE.as_bytes(),
        auction_house_key.as_ref(),
        &[*auction_house_fee_account_bump],
    ];

    //TODO: Use this fee payer for creating token accounts in non native auction house
    let (_fee_payer, _fee_payer_seeds) = get_fee_payer(
        auction_house.clone(),
        auction_house_fee_account.clone(),
        &auction_house_fee_payer_seeds,
        auction_house_authority,
        seller_info.clone(),
        buyer_info.clone(),
    )?;

    let buyer_escrow_bump = ctx
        .bumps
        .get("buyer_escrow")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    let buyer_escrow_signer_seeds = [
        ESCROW.as_bytes(),
        auction_house_info.key.as_ref(),
        buyer_info.key.as_ref(),
        &[*buyer_escrow_bump],
    ];

    let program_as_signer_bump = ctx
        .bumps
        .get("program_as_signer")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    let program_as_signer_seeds = [
        PROGRAM.as_bytes(),
        SIGNER.as_bytes(),
        &[*program_as_signer_bump],
    ];

    let creators = metadata.creators;
    let (creators_path, proof_path) = remaining_accounts.split_at(creators.len());

    // pay auction house fees
    let auction_house_fees = buyer_price
        .checked_mul(auction_house.seller_fee_basis_points.into())
        .unwrap()
        .checked_div(10000)
        .unwrap();

    let pay_to_auction_house_instruction =
        transfer(buyer_escrow.key, treasury_account.key, auction_house_fees);

    let pay_to_auction_house_accounts = [
        buyer_escrow.clone(),
        treasury_account.clone(),
        system_program_info.clone(),
    ];
    invoke_signed(
        &pay_to_auction_house_instruction,
        &pay_to_auction_house_accounts,
        &[&buyer_escrow_signer_seeds],
    )?;

    // pay creator royalties
    let mut remaining_buyer_funds = buyer_price.checked_sub(auction_house_fees).unwrap();
    let creator_royalties = buyer_price
        .checked_mul(royalty_basis_points.into())
        .unwrap()
        .checked_div(10000)
        .unwrap();
    let creator_iter = &mut creators_path.iter();
    for creator in creators.iter() {
        let share = creator_royalties
            .checked_mul(creator.share.into())
            .unwrap()
            .checked_div(100)
            .unwrap();
        remaining_buyer_funds = remaining_buyer_funds.checked_sub(share).unwrap();
        let creator_info = next_account_info(creator_iter)?;
        let pay_to_creator_instruction = transfer(buyer_escrow.key, &creator.address, share);
        let pay_to_creator_accounts = [
            buyer_escrow.clone(),
            creator_info.clone(),
            system_program_info.clone(),
        ];
        invoke_signed(
            &pay_to_creator_instruction,
            &pay_to_creator_accounts,
            &[&buyer_escrow_signer_seeds],
        )?;
    }

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

    for info in proof_path {
        transfer_nft_to_buyer_builder.add_remaining_account(info, false, false);
    }
    transfer_nft_to_buyer_builder.invoke_signed(&[&program_as_signer_seeds])?;

    // transfer funds to seller
    let transfer_to_seller_instruction =
        transfer(buyer_escrow.key, seller_info.key, remaining_buyer_funds);
    let transfer_to_seller_accounts = [buyer_escrow, seller_info.clone(), system_program_info];
    invoke_signed(
        &transfer_to_seller_instruction,
        &transfer_to_seller_accounts,
        &[&buyer_escrow_signer_seeds],
    )?;

    // close trade states
    close(seller_trade_state_info, seller_info)?;
    close(buyer_trade_state_info, buyer_info)?;

    Ok(())
}
