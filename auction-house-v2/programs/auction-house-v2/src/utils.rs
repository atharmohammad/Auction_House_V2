use anchor_lang::solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
};
use anchor_lang::AnchorSerialize;
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_bubblegum::{
    instructions::{DelegateCpiAccounts, DelegateInstructionArgs},
    ID as MPL_BUBBLEGUM_ID,
};

#[derive(BorshSerialize, BorshDeserialize)]
struct DelegateInstructionData {
    discriminator: [u8; 8],
}

impl DelegateInstructionData {
    fn new() -> Self {
        Self {
            discriminator: [90, 147, 75, 178, 85, 88, 4, 137],
        }
    }
}

pub fn invoke_with_remaining_accounts<'b, 'a>(
    delegate_accounts: &DelegateCpiAccounts<'a, 'b>,
    args: &DelegateInstructionArgs,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &Vec<(&AccountInfo<'a>, bool, bool)>,
) -> Result<(), ProgramError> {
    let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.tree_config.key,
        false,
    ));
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.leaf_owner.key,
        true,
    ));
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.previous_leaf_delegate.key,
        false,
    ));
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.new_leaf_delegate.key,
        false,
    ));
    accounts.push(AccountMeta::new(*delegate_accounts.merkle_tree.key, false));
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.log_wrapper.key,
        false,
    ));
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.compression_program.key,
        false,
    ));
    accounts.push(AccountMeta::new_readonly(
        *delegate_accounts.system_program.key,
        false,
    ));
    remaining_accounts.iter().for_each(|remaining_account| {
        accounts.push(AccountMeta {
            pubkey: *remaining_account.0.key,
            is_signer: remaining_account.1,
            is_writable: remaining_account.2,
        })
    });
    let mut data = DelegateInstructionData::new().try_to_vec().unwrap();
    let mut args = args.try_to_vec().unwrap();
    data.append(&mut args);

    let instruction = Instruction {
        program_id: MPL_BUBBLEGUM_ID,
        accounts,
        data,
    };
    let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
    account_infos.push(delegate_accounts.compression_program.clone());
    account_infos.push(delegate_accounts.tree_config.clone());
    account_infos.push(delegate_accounts.leaf_owner.clone());
    account_infos.push(delegate_accounts.previous_leaf_delegate.clone());
    account_infos.push(delegate_accounts.new_leaf_delegate.clone());
    account_infos.push(delegate_accounts.merkle_tree.clone());
    account_infos.push(delegate_accounts.log_wrapper.clone());
    account_infos.push(delegate_accounts.compression_program.clone());
    account_infos.push(delegate_accounts.system_program.clone());
    remaining_accounts
        .iter()
        .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
        invoke(&instruction, &account_infos)
    } else {
        invoke_signed(&instruction, &account_infos, signers_seeds)
    }
}
