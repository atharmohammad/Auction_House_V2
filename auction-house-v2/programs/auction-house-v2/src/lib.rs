declare_id!("AHV2WqTNWDjBqY2zv9eUAMHneicckWf5NZxnPJvYxrkA");

use anchor_lang::prelude::*;
mod constants;
mod errors;
mod instructions;
mod state;
mod utils;
pub use instructions::*;
use mpl_bubblegum::instructions::DelegateInstructionArgs;

#[program]
pub mod auction_house_v2 {

    use super::*;
    pub fn create(
        ctx: Context<CreateInstruction>,
        seller_fee_basis_points: u16,
        requires_sign_off: bool,
    ) -> Result<()> {
        CreateInstruction::create(ctx, seller_fee_basis_points, requires_sign_off)
    }

    pub fn list<'b, 'a>(
        ctx: Context<'_, '_, 'b, 'a, SellInstruction<'a>>,
        args: DelegateInstructionArgs,
        proof_len: u64,
    ) -> Result<()> {
        sell(ctx, args, proof_len)
    }
}
