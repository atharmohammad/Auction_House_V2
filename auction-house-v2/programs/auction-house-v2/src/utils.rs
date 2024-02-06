use anchor_lang::prelude::*;
use anchor_lang::solana_program::{keccak, program::invoke_signed, program_pack::Pack};
use anchor_lang::{solana_program::program_memory::sol_memcmp, system_program};
use anchor_spl::token::spl_token::instruction::initialize_account3;
use anchor_spl::token_interface::spl_token_2022::cmp_pubkeys;
use mpl_utils::create_or_allocate_account_raw;

use crate::constants::TRADE_STATE;
use crate::errors::AuctionHouseV2Errors;
use crate::{AuctionHouseV2Data, MetadataArgs, ID};

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

pub fn create_program_associated_token_account<'info>(
    account: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    wallet: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    account_seeds: &[&[u8]],
) -> Result<()> {
    create_or_allocate_account_raw(
        token_program.key(),
        &account,
        &system_program,
        &payer,
        spl_token::state::Account::LEN,
        account_seeds,
    )?;
    let initialize_token_account_instruction = initialize_account3(
        &token_program.key(),
        &account.key(),
        &mint.key(),
        &wallet.key(),
    )?;
    invoke_signed(
        &initialize_token_account_instruction,
        &[
            token_program,
            account.clone(),
            system_program.clone(),
            mint,
            wallet,
        ],
        &[account_seeds],
    )?;
    Ok(())
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

/// Computes the hash of the metadata.
///
/// The hash is computed as the keccak256 hash of the metadata bytes, which is
/// then hashed with the `seller_fee_basis_points`.
pub fn hash_metadata(metadata: &MetadataArgs) -> Result<[u8; 32]> {
    let hash = keccak::hashv(&[metadata.try_to_vec()?.as_slice()]);
    // Calculate new data hash.
    Ok(keccak::hashv(&[
        &hash.to_bytes(),
        &metadata.seller_fee_basis_points.to_le_bytes(),
    ])
    .to_bytes())
}

pub fn get_fee_payer<'a, 'b>(
    auction_house: Box<Account<AuctionHouseV2Data>>,
    auction_house_fee_account: AccountInfo<'a>,
    fee_account_seeds: &'b [&'b [u8]],
    authority: AccountInfo<'a>,
    seller: AccountInfo<'a>,
    buyer: AccountInfo<'a>,
) -> Result<(AccountInfo<'a>, &'b [&'b [u8]])> {
    let payer: AccountInfo<'a>;
    let mut seeds: &[&[u8]] = &[];
    if authority.is_signer {
        payer = auction_house_fee_account;
        seeds = fee_account_seeds;
    } else {
        if auction_house.requires_sign_off {
            return Err(AuctionHouseV2Errors::RequireAuctionHouseSignOff.into());
        }
        if seller.is_signer {
            payer = seller;
        } else if buyer.is_signer {
            payer = buyer;
        } else {
            return Err(AuctionHouseV2Errors::PayerNotProvided.into());
        }
    }
    Ok((payer, seeds))
}
