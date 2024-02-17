use crate::utils::{check_if_ata_valid, close, get_fee_payer, hash_metadata};
use crate::MetadataArgs;
use crate::{
    constants::*, errors::AuctionHouseV2Errors, state::AuctionHouseV2Data, utils::cmp_bytes,
};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::{prelude::*, solana_program::system_instruction::transfer};
use anchor_spl::token::{Mint, Token};
use mpl_bubblegum::instructions::TransferCpiBuilder;
use spl_associated_token_account::instruction::create_associated_token_account;

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

    /// CHECK: verified in the logic
    #[account(mut)]
    pub seller_receipt_account: AccountInfo<'info>,

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

    pub token_program: Program<'info, Token>,

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
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account.to_account_info();
    let auction_house_authority = &ctx.accounts.auction_house_authority;
    let merkle_tree_info = &ctx.accounts.merke_tree.to_account_info();
    let seller_info = ctx.accounts.seller.to_account_info();
    let buyer_info = ctx.accounts.buyer.to_account_info();
    let treasury_account = &ctx.accounts.treasury_account.to_account_info();
    let treasury_mint = &ctx.accounts.treasury_mint;
    let seller_trade_state_info = ctx.accounts.seller_trade_state.to_account_info();
    let buyer_trade_state_info = ctx.accounts.buyer_trade_state.to_account_info();
    let buyer_escrow = ctx.accounts.buyer_escrow.to_account_info();
    let program_as_signer_info = &ctx.accounts.program_as_signer.to_account_info();
    let compression_program_info = &ctx.accounts.compression_program.to_account_info();
    let system_program_info = ctx.accounts.system_program.to_account_info();
    let tree_config_info = &ctx.accounts.tree_config.to_account_info();
    let log_wrapper_info = &ctx.accounts.log_wrapper.to_account_info();
    let auction_house_info = &ctx.accounts.auction_house.to_account_info();
    let bubblegum_program_info = &ctx.accounts.bubblegum_program.to_account_info();
    let seller_receipt_info = &ctx.accounts.seller_receipt_account.to_account_info();
    let token_program = &ctx.accounts.token_program;
    let remaining_accounts = &ctx.remaining_accounts;

    let hashed_metadata = hash_metadata(&metadata)?;
    if !cmp_bytes(&data_hash, &hashed_metadata, 32) {
        return Err(AuctionHouseV2Errors::MetadataHashMismatch.into());
    }

    if buyer_trade_state_info.data_is_empty() || (buyer_trade_state_info.try_borrow_data()?[0] == 0)
    {
        return Err(AuctionHouseV2Errors::InvalidBuyerTradeState.into());
    }

    if seller_trade_state_info.data_is_empty()
        || (seller_trade_state_info.try_borrow_data()?[0] == 0)
    {
        return Err(AuctionHouseV2Errors::BothPartiesNeedToAgreeToSale.into());
    }

    // assert buyer and seller trade state configs
    if buyer_escrow.lamports() < buyer_price {
        return Err(AuctionHouseV2Errors::NotEnoughFunds.into());
    }

    let treasury_mint_key = treasury_mint.key();
    let auction_house_authority_key = auction_house_authority.key();
    let is_native = treasury_mint_key == spl_token::native_mint::id();

    let auction_house_seeds = [
        AUCTION_HOUSE.as_ref(),
        auction_house_authority_key.as_ref(),
        treasury_mint_key.as_ref(),
        &[auction_house.bump],
    ];

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

    // Use this fee payer for creating token accounts in non native auction house
    let (fee_payer, fee_payer_seeds) = get_fee_payer(
        auction_house.clone(),
        auction_house_fee_account.clone(),
        &auction_house_fee_payer_seeds,
        auction_house_authority.clone(),
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

    let remaining_accounts_iter = &mut remaining_accounts.iter();

    // pay auction house fees
    let auction_house_fees = buyer_price
        .checked_mul(auction_house.seller_fee_basis_points.into())
        .unwrap()
        .checked_div(10000)
        .unwrap();

    if is_native {
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
    } else {
        let pay_to_auction_house_instruction = spl_token::instruction::transfer(
            token_program.key,
            buyer_escrow.key,
            auction_house_fee_account.key,
            &auction_house_key,
            &[],
            auction_house_fees,
        )?;
        let pay_to_auction_house_accounts = [
            token_program.to_account_info(),
            buyer_escrow.clone(),
            auction_house.to_account_info(),
            auction_house_fee_account.clone(),
        ];
        invoke_signed(
            &pay_to_auction_house_instruction,
            &pay_to_auction_house_accounts,
            &[&auction_house_seeds],
        )?;
    }

    // pay creator royalties
    let mut remaining_buyer_funds = buyer_price.checked_sub(auction_house_fees).unwrap();
    let creator_royalties = buyer_price
        .checked_mul(royalty_basis_points.into())
        .unwrap()
        .checked_div(10000)
        .unwrap();

    if !metadata.creators.is_empty() {
        for creator in metadata.creators.iter() {
            let share = creator_royalties
                .checked_mul(creator.share.into())
                .unwrap()
                .checked_div(100)
                .unwrap();
            remaining_buyer_funds = remaining_buyer_funds.checked_sub(share).unwrap();

            let creator_info = next_account_info(remaining_accounts_iter)?;
            if is_native {
                if share > 0 {
                    let pay_to_creator_instruction =
                        transfer(buyer_escrow.key, &creator.address, share);
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
            } else {
                let creator_token_account = next_account_info(remaining_accounts_iter)?;
                if share > 0 {
                    if creator_token_account.data_is_empty() {
                        // create current creator token account if not initialised
                        let create_ata_instruction = create_associated_token_account(
                            fee_payer.key,
                            creator_info.key,
                            &treasury_mint_key,
                            token_program.key,
                        );
                        let fee_payer_seeds = [fee_payer_seeds];
                        let fee_signer_seeds: &[&[&[u8]]] = if fee_payer_seeds.is_empty() {
                            &[]
                        } else {
                            &fee_payer_seeds
                        };
                        invoke_signed(
                            &create_ata_instruction,
                            &[
                                fee_payer.clone(),
                                creator_info.clone(),
                                treasury_mint.to_account_info(),
                                token_program.to_account_info(),
                            ],
                            fee_signer_seeds,
                        )?;
                    }
                    check_if_ata_valid(
                        creator_token_account,
                        &creator_info.key(),
                        &treasury_mint_key,
                    )?;

                    // transfer royalty tokens to creator token account
                    let pay_to_creator_instruction = spl_token::instruction::transfer(
                        token_program.key,
                        buyer_escrow.key,
                        creator_token_account.key,
                        &auction_house_key,
                        &[],
                        share,
                    )?;
                    let pay_to_creator_accounts = [
                        token_program.to_account_info(),
                        buyer_escrow.clone(),
                        auction_house.to_account_info(),
                        creator_token_account.clone(),
                    ];
                    invoke_signed(
                        &pay_to_creator_instruction,
                        &pay_to_creator_accounts,
                        &[&auction_house_seeds],
                    )?;
                }
            }
        }
    }

    // transfer funds to seller
    if is_native {
        let pay_to_seller_instruction =
            transfer(buyer_escrow.key, seller_info.key, remaining_buyer_funds);
        let pay_to_seller_accounts = [
            buyer_escrow,
            seller_info.clone(),
            system_program_info.clone(),
        ];
        invoke_signed(
            &pay_to_seller_instruction,
            &pay_to_seller_accounts,
            &[&buyer_escrow_signer_seeds],
        )?;
    } else {
        // create seller receipt ATA
        if seller_receipt_info.data_is_empty() {
            let create_ata_instruction = create_associated_token_account(
                fee_payer.key,
                seller_info.key,
                &treasury_mint_key,
                token_program.key,
            );
            let fee_payer_seeds = [fee_payer_seeds];
            let fee_signer_seeds: &[&[&[u8]]] = if fee_payer_seeds.is_empty() {
                &[]
            } else {
                &fee_payer_seeds
            };
            invoke_signed(
                &create_ata_instruction,
                &[
                    fee_payer.clone(),
                    seller_info.clone(),
                    treasury_mint.to_account_info(),
                    token_program.to_account_info(),
                ],
                fee_signer_seeds,
            )?;
        }

        let loaded_seller_token_account =
            check_if_ata_valid(&seller_receipt_info, seller_info.key, &treasury_mint_key)?;

        // check if seller token account have a delegate
        if loaded_seller_token_account.delegate.is_some() {
            return Err(AuctionHouseV2Errors::SellerTokenAccountCannotHaveDelegate.into());
        }

        let pay_to_seller_instruction = spl_token::instruction::transfer(
            token_program.key,
            buyer_escrow.key,
            seller_receipt_info.key,
            &auction_house_key,
            &[],
            remaining_buyer_funds,
        )?;
        let pay_to_seller_accounts = [
            token_program.to_account_info(),
            buyer_escrow.clone(),
            auction_house.to_account_info(),
            seller_receipt_info.clone(),
        ];
        invoke_signed(
            &pay_to_seller_instruction,
            &pay_to_seller_accounts,
            &[&auction_house_seeds],
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

    for info in remaining_accounts_iter {
        transfer_nft_to_buyer_builder.add_remaining_account(info, false, false);
    }
    transfer_nft_to_buyer_builder.invoke_signed(&[&program_as_signer_seeds])?;

    // close trade states
    close(seller_trade_state_info, seller_info)?;
    close(buyer_trade_state_info, buyer_info)?;

    Ok(())
}
