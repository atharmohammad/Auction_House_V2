use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionHouseV2Errors {
    #[msg("Bump Seed Not In HashMap")]
    BumpSeedNotInHashMap,

    #[msg("Invalid Seller Fee Basis Points")]
    InvalidSellerFeeBasisPoints,
}
