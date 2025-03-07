use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, pubkey::Pubkey,
    signature::Signature, signers::Signers, transaction::Transaction,
};
use tokio::time::{Duration, timeout};

use crate::{BoxedStdError, utils::env_util::is_main};

/// Sends and confirms a transaction with retry logic and configurable options.
///
/// # Arguments
/// * `rpc_client` - The RPC client for Solana Devnet/Mainnet.
/// * `instructions` - List of instructions to include in the transaction.
/// * `payer` - The payer pubkey.
/// * `signing_keypairs` - The keypairs signing the transaction.
/// * `commitment` - Commitment level for confirmation (default: confirmed).
/// * `max_retries` - Maximum number of retries on failure (default: 3).
/// * `timeout_secs` - Timeout in seconds for the entire operation (default: 30).
///
/// # Returns
/// * `Result<Signature, BoxedStdError>` - The transaction signature on success.
pub async fn send_and_confirm_transaction<T: Signers>(
    rpc_client: &RpcClient,
    instructions: &[Instruction],
    payer: &Pubkey,
    signing_keypairs: &T,
    commitment: Option<CommitmentConfig>,
    max_retries: Option<usize>,
    timeout_secs: Option<u64>,
) -> Result<Signature, BoxedStdError> {
    let commitment = commitment.unwrap_or(CommitmentConfig::confirmed());
    let max_retries = max_retries.unwrap_or(3);
    let timeout_duration = Duration::from_secs(timeout_secs.unwrap_or(30));

    let mut last_error = String::default();
    for attempt in 1..=max_retries {
        match timeout(timeout_duration, async {
            // Fetch recent blockhash
            let recent_blockhash = rpc_client
                .get_latest_blockhash()
                .await
                .map_err(|e| format!("Failed to get latest blockhash: {e}"))?;

            // Build and sign transaction
            let transaction = Transaction::new_signed_with_payer(
                instructions,
                Some(payer),
                signing_keypairs,
                recent_blockhash,
            );

            // Send and confirm
            rpc_client
                .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment)
                .await
                .map_err(|e| format!("Failed to send/confirm transaction: {e}"))
        })
        .await
        {
            Ok(Ok(signature)) => {
                let cluster = if is_main() { "" } else { "?cluster=devnet" };
                println!(
                    "Transaction confirmed (attempt {attempt}): https://explorer.solana.com/tx/{signature}{cluster}"
                );

                return Ok(signature);
            }
            Ok(Err(e)) => {
                last_error = e;
                println!("Attempt {attempt}/{max_retries} failed: {last_error}. Retrying...");
                tokio::time::sleep(Duration::from_secs(2)).await; // Backoff
            }
            Err(_) => {
                last_error = "Operation timed out".to_string();
                println!(
                    "Attempt {attempt}/{max_retries} timed out after {}s. Retrying...",
                    timeout_duration.as_secs()
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    Err(last_error.into())
}
