use anchor_lang::AccountDeserialize;
use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use anchor_spl::token::spl_token::native_mint;
use auction_house_v2::{
    accounts::CreateInstruction as CreateAuctionHouseAccounts,
    instruction::Create as CreateAuctionHouseInstruction, AuctionHouseV2Data, ID,
};
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use utils::{auction_house_program_test, AUCTION_HOUSE, FEE, TREASURY};

pub mod utils;

#[tokio::test]
async fn create_native_auction_house() {
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
    let (auction_house, auction_house_bump) =
        Pubkey::find_program_address(&auction_house_seeds, &ID);
    let (treasury_account, treasury_bump) =
        Pubkey::find_program_address(&[TREASURY.as_bytes(), auction_house.as_ref()], &ID);
    let (fee_account, fee_account_bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[FEE.as_bytes(), auction_house.as_ref()], &ID);
    let withdrawal_account = Keypair::new();

    let create_auction_house_accounts = CreateAuctionHouseAccounts {
        auction_house,
        authority: payer_pubkey,
        treasury_mint,
        treasury_withdrawal_account: withdrawal_account.pubkey(),
        treasury_withdrawal_owner: payer_pubkey,
        treasury_account,
        fee_account,
        fee_withdrawal_account: withdrawal_account.pubkey(),
        payer: payer.pubkey(),
        system_program: system_program::ID,
        token_program: spl_token::ID,
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
    let signature = client.process_transaction(signed_tx).await.unwrap();
    println!("Auction house transaction signature: {:?}", signature);

    let auction_house_account = client.get_account(auction_house).await.unwrap().unwrap();
    let auction_house_data =
        AuctionHouseV2Data::try_deserialize(&mut auction_house_account.data.as_ref()).unwrap();
    assert_eq!(auction_house_data.requires_sign_off, false);
    assert_eq!(auction_house_data.seller_fee_basis_points, 500);
    assert_eq!(auction_house_data.authority, payer_pubkey);
    assert_eq!(auction_house_data.fee_account, fee_account);
    assert_eq!(auction_house_data.fee_withdrawal_account, fee_account);
    assert_eq!(auction_house_data.treasury_account, treasury_account);
    assert_eq!(auction_house_data.treasury_withdrawal_account, fee_account);
    assert_eq!(auction_house_data.treasury_mint, treasury_mint);
    assert_eq!(auction_house_data.bump, auction_house_bump);
    assert_eq!(auction_house_data.fee_account_bump, fee_account_bump);
    assert_eq!(auction_house_data.treasury_bump, treasury_bump);
}
