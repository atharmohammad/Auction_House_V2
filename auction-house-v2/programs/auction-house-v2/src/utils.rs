use anchor_lang::prelude::*;
use anchor_lang::{solana_program::program_memory::sol_memcmp, system_program};
use anchor_spl::token_interface::spl_token_2022::cmp_pubkeys;

use crate::constants::TRADE_STATE;
use crate::ID;

pub fn cmp_bytes(a: &[u8], b: &[u8], size: usize) -> bool {
    sol_memcmp(a, b, size) == 0
}

pub fn close<'info>(info: AccountInfo<'info>, sol_destination: AccountInfo<'info>) -> Result<()> {
    // Transfer tokens from the account to the sol_destination.
    let dest_starting_lamports = sol_destination.lamports();
    **sol_destination.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(info.lamports()).unwrap();
    **info.lamports.borrow_mut() = 0;

    info.assign(&system_program::ID);
    info.realloc(0, false).map_err(Into::into)
}

pub fn assert_valid_trade_state(
    trade_state: &Pubkey,
    wallet: &Pubkey,
    auction_house: &Pubkey,
    asset_id: &Pubkey,
    price: [u8; 8],
) -> bool {
    let seeds = [
        TRADE_STATE.as_ref(),
        wallet.as_ref(),
        auction_house.as_ref(),
        asset_id.as_ref(),
        price.as_ref(),
    ];
    let (derived_trade_state_key, _bump) = Pubkey::find_program_address(&seeds, &ID);
    return cmp_pubkeys(&derived_trade_state_key, trade_state);
}
