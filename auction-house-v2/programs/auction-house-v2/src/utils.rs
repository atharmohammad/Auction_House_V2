use anchor_lang::{
    accounts::account::Account, error::Error, solana_program::program_memory::sol_memcmp,
};

use crate::{
    errors::AuctionHouseV2Errors,
    state::{BuyerTradeState, SellerTradeState},
};

pub fn assert_trade_states<'a>(
    seller_trade_state: &'a Account<SellerTradeState>,
    buyer_trade_state: &'a Account<BuyerTradeState>,
) -> Result<(), Error> {
    if seller_trade_state.asset_id != buyer_trade_state.asset_id {
        return Err(AuctionHouseV2Errors::InvalidBuyingOrSellingOrder.into());
    }
    if seller_trade_state.amount < buyer_trade_state.amount {
        return Err(AuctionHouseV2Errors::InvalidBuyingOrderPrice.into());
    }
    Ok(())
}

pub fn cmp_bytes(a: &[u8], b: &[u8], size: usize) -> bool {
    sol_memcmp(a, b, size) == 0
}
