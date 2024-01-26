use anchor_lang::prelude::*;

#[account]
pub struct AuctionHouseV2Data {
    pub authority: Pubkey,
    pub treasury_mint: Pubkey,
    pub seller_fee_basis_points: u16,
    pub requires_sign_off: bool,
    pub treasury_account: Pubkey,
    pub treasury_withdrawal_account: Pubkey,
    pub fee_account: Pubkey,
    pub fee_withdrawal_account: Pubkey,
    pub bump: u8,
    pub treasury_bump: u8,
    pub fee_account_bump: u8,
}
