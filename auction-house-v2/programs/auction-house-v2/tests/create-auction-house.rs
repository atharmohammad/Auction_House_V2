use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use anchor_spl::token::spl_token::native_mint;
use auction_house_v2::{
    accounts::CreateInstruction as CreateAuctionHouseAccounts,
    instruction::CreateAh as CreateAuctionHouseInstruction, ID,
};
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::BanksClientError;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use utils::{auction_house_program_test, AUCTION_HOUSE};

pub mod utils;

#[tokio::test]
async fn create_native_auction_house() -> Result<(), BanksClientError> {
    let ctx = auction_house_program_test().start_with_context().await;
    let payer = ctx.payer;
    let payer_pubkey = payer.pubkey();
    let mut client = ctx.banks_client;
    let treasury_mint = native_mint::id();
    let auction_house_seeds = [
        AUCTION_HOUSE.as_ref(),
        payer_pubkey.as_ref(),
        treasury_mint.as_ref(),
    ];
    let (auction_house, _bump) = Pubkey::find_program_address(&auction_house_seeds, &ID);
    let treasury_account = Keypair::new();
    let fee_account = Keypair::new();
    let create_auction_house_accounts = CreateAuctionHouseAccounts {
        auction_house,
        authority: payer_pubkey,
        treasury_mint,
        treasury_withdrawal_account: fee_account.pubkey(),
        treasury_account: treasury_account.pubkey(),
        fee_account: fee_account.pubkey(),
        fee_withdrawal_account: fee_account.pubkey(),
        payer: payer.pubkey(),
        system_program: system_program::ID,
    }
    .to_account_metas(None);

    let create_auction_house_data = CreateAuctionHouseInstruction {
        seller_fee_basis_points: 500,
        requires_sign_off: false,
    }
    .data();

    let create_auction_house_instruction = Instruction {
        program_id: ID,
        accounts: create_auction_house_accounts,
        data: create_auction_house_data,
    };
    let signed_tx = Transaction::new_signed_with_payer(
        &[create_auction_house_instruction],
        Some(&payer_pubkey),
        &[&payer],
        ctx.last_blockhash,
    );
    let signature = client.process_transaction(signed_tx).await;
    signature
}
