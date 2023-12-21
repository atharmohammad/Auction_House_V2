declare_id!("AHV2WqTNWDjBqY2zv9eUAMHneicckWf5NZxnPJvYxrkA");

use anchor_lang::prelude::*;
mod constants;
mod errors;
mod instructions;
mod state;
pub use instructions::*;
#[program]
pub mod auction_house_v2 {
    use super::*;

    pub fn create(
        ctx: Context<CreateInstruction>,
        seller_fee_basis_points: u64,
        requires_sign_off: bool,
    ) -> Result<()> {
        CreateInstruction::create(ctx, seller_fee_basis_points, requires_sign_off)
    }

    pub fn sell(ctx: Context<SellInstruction>) -> Result<()> {
        ctx.accounts.sell()
    }
}
