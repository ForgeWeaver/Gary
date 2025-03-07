use crate::BoxedStdError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

pub async fn create_associated_token_address(
    rpc_client: &RpcClient,
    payer: &impl Signer,
    owner: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Result<Pubkey, BoxedStdError> {
    let associated_token_address = get_associated_token_address(owner, token_mint_address);

    // Check if the Associated Token Address already exists
    if rpc_client
        .get_account(&associated_token_address)
        .await
        .is_ok()
    {
        println!("Associated Token Address already exists: {associated_token_address}");

        return Ok(associated_token_address);
    }

    // Create the Associated Token Address instruction
    let create_ata_ix = create_associated_token_account(
        &payer.pubkey(),
        owner,
        token_mint_address,
        token_program_id,
    );

    // Get the latest blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(|e| format!("Failed to get latest blockhash: {e}"))?;

    // Build and sign the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    // Send and confirm the transaction
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &transaction,
            CommitmentConfig::confirmed(),
        )
        .await
        .map_err(|e| format!("Failed to send and confirm transaction: {e}"))?;

    println!(
        "Associated Token Address created: {associated_token_address} (Transaction: {signature})"
    );

    Ok(associated_token_address)
}
