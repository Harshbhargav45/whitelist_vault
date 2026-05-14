use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
    anchor_spl::token_interface::spl_token_2022,
    anchor_spl::token_2022::spl_token_2022::extension::ExtensionType,
    solana_sdk::{
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction,
        rent::Rent,
    },
};

#[test]
fn test_vault_flow() {
    let program_id = whitelist_vault::id();
    let payer = Keypair::new();
    let mint = Keypair::new();
    let user = Keypair::new();
    let admin = Keypair::new();
    
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/whitelist_vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&admin.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

    // Initialize Mint with Transfer Hook
    let mint_len = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&[
        ExtensionType::TransferHook,
    ]).unwrap();
    let rent = Rent::default().minimum_balance(mint_len);
    

}
