declare_id!("AHV2WqTNWDjBqY2zv9eUAMHneicckWf5NZxnPJvYxrkA");

use anchor_lang::prelude::*;
mod constants;
mod errors;
mod instructions;
use instructions::*;
mod state;
pub use state::*;
mod utils;
use mpl_bubblegum::types::MetadataArgs;
#[program]
pub mod auction_house_v2 {
    use super::*;
    pub fn create(
        ctx: Context<CreateInstruction>,
        seller_fee_basis_points: u16,
        requires_sign_off: bool,
    ) -> Result<()> {
        instructions::create(ctx, seller_fee_basis_points, requires_sign_off)
    }

    pub fn sell<'b, 'a>(
        ctx: Context<'_, '_, 'b, 'a, SellInstruction<'a>>,
        seller_price: u64,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
    ) -> Result<()> {
        instructions::sell(
            ctx,
            seller_price,
            root,
            data_hash,
            creator_hash,
            nonce,
            index,
        )
    }

    pub fn bid(ctx: Context<BidInstruction>, buyer_price: u64) -> Result<()> {
        instructions::bid(ctx, buyer_price)
    }

    pub fn execute_sale<'a>(
        ctx: Context<'_, '_, '_, 'a, ExecuteSaleInstruction<'a>>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,
        royalty_basis_points: u16,
        metadata: MetadataArgs,
    ) -> Result<()> {
        instructions::execute_sale(
            ctx,
            root,
            data_hash,
            creator_hash,
            nonce,
            index,
            royalty_basis_points,
            metadata,
        )
    }
}
