use anchor_lang::prelude::*;

declare_id!("AHV2WqTNWDjBqY2zv9eUAMHneicckWf5NZxnPJvYxrkA");

#[program]
pub mod auction_house_v2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
