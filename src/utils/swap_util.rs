use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;

use crate::{BoxedStdError, utils::addr_util::create_associated_token_address};

// https://github.com/everlastingsong/tour-de-whirlpool/blob/main/src/EN/convert_sol_to_dev_token.ts
pub async fn swap_sol_to_devusdc(
    rpc_client: &RpcClient,
    wallet: &Keypair,
) -> Result<(), BoxedStdError> {
    let dev_usdc_mint = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k")?;
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
    let associated_token_program_id =
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?;
    let devtoken_distributor_program_id =
        Pubkey::from_str("Bu2AaWnVoveQT47wP4obpmmZUwK9bN9ah4w6Vaoa93Y9")?;
    let devtoken_admin = Pubkey::from_str("3otH3AHWqkqgSVfKFkrxyDqd2vK6LcaqigHrFEmWcGuo")?;
    let pda = Pubkey::from_str("3pgfe1L6jcq59uy3LZmmeSCk9mwVvHXjn21nSvNr8D6x")?;

    let user = wallet.pubkey();
    let vault = get_associated_token_address(&pda, &dev_usdc_mint);
    let user_vault = get_associated_token_address(&user, &dev_usdc_mint);

    // Ensure user_vault (ATA) exists
    if rpc_client.get_account(&user_vault).await.is_err() {
        let _ = create_associated_token_address(
            rpc_client,
            wallet,
            &user,
            &dev_usdc_mint,
            &token_program_id,
        )
        .await?;
        println!("Created devUSDC ATA: {}", user_vault);
    }

    // Build the swap instruction
    let instruction = Instruction {
        program_id: devtoken_distributor_program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new_readonly(dev_usdc_mint, false),
            solana_sdk::instruction::AccountMeta::new(vault, false),
            solana_sdk::instruction::AccountMeta::new_readonly(pda, false),
            solana_sdk::instruction::AccountMeta::new(user, true),
            solana_sdk::instruction::AccountMeta::new(user_vault, false),
            solana_sdk::instruction::AccountMeta::new(devtoken_admin, false),
            solana_sdk::instruction::AccountMeta::new_readonly(token_program_id, false),
            solana_sdk::instruction::AccountMeta::new_readonly(system_program::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(associated_token_program_id, false),
        ],
        data: vec![0xBF, 0x2C, 0xDF, 0xCF, 0xA4, 0xEC, 0x7E, 0x3D], // Distribute instruction
    };

    // Build and send transaction
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(|e| format!("Failed to get latest blockhash: {}", e))?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&user),
        &[wallet],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &transaction,
            CommitmentConfig::confirmed(),
        )
        .await
        .map_err(|e| format!("Failed to send and confirm transaction: {}", e))?;

    println!("Swapped SOL to devUSDC, signature: {}", signature);
    println!(
        "Scan: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    // Check balance (optional for logging)
    let balance = rpc_client
        .get_token_account_balance(&user_vault)
        .await
        .map_err(|e| format!("Failed to get token balance: {}", e))?;
    println!("devUSDC balance: {}", balance.ui_amount.unwrap_or(0.0));

    Ok(())
}
