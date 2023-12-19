use crate::constants::{FEE, PREFIX, PROGRAM, SIGNER};
use crate::state::AuctionHouseV2Data;
use anchor_lang::prelude::*;
use anchor_lang::{
    accounts::{
        account::Account, program::Program, signer::Signer, unchecked_account::UncheckedAccount,
    },
    system_program::System,
};

#[derive(Accounts)]
pub struct SellInstruction<'info> {
    #[account(seeds=[PREFIX.as_ref(),auction_house.creator.key().as_ref(),auction_house.treasury_mint.key().as_ref()],bump)]
    auction_house: Account<'info, AuctionHouseV2Data>,

    owner: Signer<'info>,

    #[account(
        seeds=[
            PREFIX.as_ref(),
            owner.key().as_ref(),
            auction_house.key().as_ref(), 
            asset_id.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            &u64::MAX.to_le_bytes()
        ],
        bump
    )]
    seller_trade_state: UncheckedAccount<'info>,

    asset_id: UncheckedAccount<'info>,

    #[account(seeds=[auction_house.key().as_ref(),FEE.as_bytes()],bump)]
    auction_house_fee_account: UncheckedAccount<'info>,

    #[account(seeds=[PROGRAM.as_bytes(), SIGNER.as_bytes()], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    compression_program: UncheckedAccount<'info>,

    system_program: Program<'info, System>,
    // Cnft proofs in the remaining accounts
}

impl <'info> SellInstruction<'info> {
    pub fn sell(&mut self)->Result<()>{
        Ok(())
    }
}