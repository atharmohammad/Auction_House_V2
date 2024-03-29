use crate::constants::*;
use crate::errors::AuctionHouseV2Errors;
use crate::state::AuctionHouseV2Data;
use crate::utils::{check_if_ata_valid, create_program_associated_token_account};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token::Token;
use spl_associated_token_account::instruction::create_associated_token_account;

#[derive(Accounts)]
pub struct CreateInstruction<'info> {
    #[account(init,payer=payer,seeds=[AUCTION_HOUSE.as_ref(),authority.key().as_ref(),treasury_mint.key().as_ref()],bump,space=MAX_AUCTION_HOUSE_SIZE)]
    pub auction_house: Account<'info, AuctionHouseV2Data>,

    /// CHECK: User can use whatever they want for intialization.
    pub authority: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for intialization.
    pub treasury_mint: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for intialization.
    #[account(mut,seeds=[TREASURY.as_bytes(),auction_house.key().as_ref()],bump)]
    pub treasury_account: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for intialization.
    #[account(mut)]
    pub treasury_withdrawal_account: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for initialization
    pub treasury_withdrawal_owner: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for intialization.
    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    pub fee_account: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for intialization.
    pub fee_withdrawal_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create(
    ctx: Context<CreateInstruction>,
    seller_fee_basis_points: u16,
    requires_sign_off: bool,
) -> Result<()> {
    if seller_fee_basis_points > 10000 {
        return Err(AuctionHouseV2Errors::InvalidSellerFeeBasisPoints)?;
    }

    let auction_house = &mut ctx.accounts.auction_house;
    let treasury_account = &ctx.accounts.treasury_account;
    let treasury_mint = &ctx.accounts.treasury_mint;
    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let treasury_withdrawal_account = &ctx.accounts.treasury_withdrawal_account;
    let treasury_withdrawal_owner = &ctx.accounts.treasury_withdrawal_owner;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let auction_house_key = auction_house.key().clone();

    auction_house.bump = *ctx
        .bumps
        .get("auction_house")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    auction_house.fee_account_bump = *ctx
        .bumps
        .get("fee_account")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    let treasury_bump = *ctx
        .bumps
        .get("treasury_account")
        .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

    auction_house.treasury_bump = treasury_bump;
    auction_house.fee_account = ctx.accounts.fee_account.key();
    auction_house.fee_withdrawal_account = ctx.accounts.fee_withdrawal_account.key();
    auction_house.treasury_withdrawal_account = ctx.accounts.treasury_withdrawal_account.key();
    auction_house.treasury_mint = ctx.accounts.treasury_mint.key();
    auction_house.seller_fee_basis_points = seller_fee_basis_points;
    auction_house.authority = ctx.accounts.authority.key();
    auction_house.requires_sign_off = requires_sign_off;
    auction_house.treasury_account = ctx.accounts.treasury_account.key();

    let is_native = ctx.accounts.treasury_mint.key() == native_mint::id();
    if !is_native {
        // initialize treasury account as token account
        let treasury_seeds = [
            TREASURY.as_bytes(),
            auction_house_key.as_ref(),
            &[treasury_bump],
        ];
        create_program_associated_token_account(
            &treasury_account.to_account_info(),
            &payer.to_account_info(),
            auction_house.to_account_info(),
            treasury_mint.to_account_info(),
            &system_program.to_account_info(),
            token_program.to_account_info(),
            &treasury_seeds,
        )?;

        // Create treasury withdrawal ATA
        if treasury_withdrawal_account.data_is_empty() {
            let create_ata_instruction = create_associated_token_account(
                payer.key,
                treasury_withdrawal_owner.key,
                treasury_mint.key,
                token_program.key,
            );
            invoke(
                &create_ata_instruction,
                &[
                    payer.to_account_info(),
                    treasury_withdrawal_owner.to_account_info(),
                    treasury_withdrawal_account.to_account_info(),
                    treasury_mint.to_account_info(),
                    token_program.to_account_info(),
                    system_program.to_account_info(),
                    associated_token_program.to_account_info(),
                ],
            )?;
            check_if_ata_valid(
                treasury_withdrawal_account,
                treasury_withdrawal_owner.key,
                treasury_mint.key,
            )?;
        }
    }

    Ok(())
}
