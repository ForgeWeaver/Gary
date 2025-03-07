use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, pubkey::Pubkey,
    signature::Signature, signers::Signers, transaction::Transaction,
};

use crate::{BoxedStdError, utils::env_util::is_main};

pub async fn send_and_confirm_transaction(
    rpc_client: &RpcClient,
    instructions: &[Instruction],
    payer: Option<&Pubkey>,
    signing_keypairs: &impl Signers,
) -> Result<Signature, BoxedStdError> {
    // Build and send transaction
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(|e| format!("Failed to get latest blockhash: {}", e))?;
    let transaction =
        Transaction::new_signed_with_payer(instructions, payer, signing_keypairs, recent_blockhash);

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &transaction,
            CommitmentConfig::confirmed(),
        )
        .await
        .map_err(|e| format!("Failed to send and confirm transaction: {}", e))?;
    let cluster = if is_main() { "" } else { "?cluster=devnet" };
    println!("Scan: https://explorer.solana.com/tx/{signature}{cluster}");

    Ok(signature)
}
