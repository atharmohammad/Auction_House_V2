use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionHouseV2Errors {
    #[msg("Bump Seed Not In HashMap")]
    BumpSeedNotInHashMap,

    #[msg("Invalid Seller Fee Basis Points")]
    InvalidSellerFeeBasisPoints,

    #[msg("Numeric Overflow")]
    NumericOverflow,

    #[msg("Not Enough Funds")]
    NotEnoughFunds,

    #[msg("Invalid buying or selling order")]
    InvalidBuyingOrSellingOrder,

    #[msg("Invalid buying order price don't match with selling order")]
    InvalidBuyingOrderPrice,

    #[msg("Metadata hash does not match")]
    MetadataHashMismatch,
}
