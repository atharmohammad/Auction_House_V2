use crate::constants::*;
use crate::errors::AuctionHouseV2Errors;
use crate::state::AuctionHouseV2Data;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateInstruction<'info> {
    #[account(init,payer=payer,seeds=[AUCTION_HOUSE.as_ref(),authority.key().as_ref(),treasury_mint.key().as_ref()],bump,space=CreateInstruction::MAX_SIZE)]
    auction_house: Account<'info, AuctionHouseV2Data>,

    authority: UncheckedAccount<'info>,

    treasury_mint: UncheckedAccount<'info>,

    #[account(seeds=[TREASURY.as_bytes(),auction_house.key().as_ref()],bump)]
    treasury_account: UncheckedAccount<'info>,

    treasury_withdrawal_account: UncheckedAccount<'info>,

    #[account(seeds=[FEE.as_bytes(),auction_house.key().as_ref()],bump)]
    fee_account: UncheckedAccount<'info>,

    fee_withdrawal_account: UncheckedAccount<'info>,

    #[account(mut)]
    payer: Signer<'info>,

    system_program: Program<'info, System>,
}

impl<'info> CreateInstruction<'info> {
    pub const MAX_SIZE: usize = 32 * 7 + 8 + 1 * 3 + 8;
    pub fn create(
        ctx: Context<CreateInstruction>,
        seller_fee_basis_points: u16,
        requires_sign_off: bool,
    ) -> Result<()> {
        if seller_fee_basis_points > 10000 {
            return Err(AuctionHouseV2Errors::InvalidSellerFeeBasisPoints)?;
        }

        let auction_house = &mut ctx.accounts.auction_house;

        auction_house.bump = *ctx
            .bumps
            .get("auction_house")
            .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

        auction_house.fee_account_bump = *ctx
            .bumps
            .get("auction_house_fee_account")
            .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

        auction_house.treasury_bump = *ctx
            .bumps
            .get("auction_house_treasury_account")
            .ok_or(AuctionHouseV2Errors::BumpSeedNotInHashMap)?;

        auction_house.fee_account = ctx.accounts.fee_account.key();
        auction_house.fee_withdrawal_account = ctx.accounts.fee_withdrawal_account.key();
        auction_house.treasury_withdrawal_account = ctx.accounts.treasury_withdrawal_account.key();
        auction_house.treasury_mint = ctx.accounts.treasury_mint.key();
        auction_house.seller_fee_basis_points = seller_fee_basis_points;
        auction_house.authority = ctx.accounts.authority.key();
        auction_house.requires_sign_off = requires_sign_off;

        Ok(())
    }
}
