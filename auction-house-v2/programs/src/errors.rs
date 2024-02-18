use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionHouseV2Errors {
    #[msg("Bump Seed Not In HashMap")]
    BumpSeedNotInHashMap,

    #[msg("Account not initialized")]
    AccountNotInitialized,

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

    #[msg("Buyer trade state is invalid")]
    InvalidBuyerTradeState,

    #[msg("Both parties need to agree on a price")]
    BothPartiesNeedToAgreeToSale,

    #[msg("Seller trade is invalid")]
    InvalidSellerTradeState,

    #[msg("Provided keys don't match")]
    PublicKeyMismatch,

    #[msg("Payer not provided")]
    PayerNotProvided,

    #[msg("Require auction house to sign off")]
    RequireAuctionHouseSignOff,

    #[msg("Seller token account cannot have delegate")]
    SellerTokenAccountCannotHaveDelegate,
}
