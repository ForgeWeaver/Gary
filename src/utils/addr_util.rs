use crate::{BoxedStdError, utils::tx_utils::send_and_confirm_transaction};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

pub async fn create_associated_token_address(
    rpc_client: &RpcClient,
    payer: &Keypair,
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

    // Send and confirm the transaction
    let signature = send_and_confirm_transaction(
        rpc_client,
        &[create_ata_ix],
        &payer.pubkey(),
        &[payer],
        None,
        None,
        None,
    )
    .await?;

    println!(
        "Associated Token Address created: {associated_token_address} (Transaction: {signature})"
    );

    Ok(associated_token_address)
}
