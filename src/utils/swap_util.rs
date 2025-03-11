use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
};
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;

use crate::{
    BoxedStdError,
    config::tokens::Token,
    constants::addresses::{
        ASSOCIATED_TOKEN_PROGRAM_ID, ORCA_DEVTOKEN_ADMIN, ORCA_DEVTOKEN_DISTRIBUTOR_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
    },
    utils::{addr_util::create_associated_token_address, tx_utils::send_and_confirm_transaction},
};

// https://github.com/everlastingsong/tour-de-whirlpool/blob/main/src/EN/convert_sol_to_dev_token.ts
pub async fn swap_sol_to_orca_dev_token(
    rpc_client: &RpcClient,
    wallet: &Keypair,
    token: &Token,
) -> Result<(), BoxedStdError> {
    let token_symbol = &token.symbol;
    let token_mint_address = token.as_pubkey()?;
    let token_program_id = Pubkey::from_str(TOKEN_PROGRAM_ID)?;
    let associated_token_program_id = Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM_ID)?;
    let devtoken_distributor_program_id = Pubkey::from_str(ORCA_DEVTOKEN_DISTRIBUTOR_PROGRAM_ID)?;
    let devtoken_admin = Pubkey::from_str(ORCA_DEVTOKEN_ADMIN)?;
    let (pda, _) = Pubkey::find_program_address(
        &[token_mint_address.as_ref()],
        &devtoken_distributor_program_id,
    );

    let user = wallet.pubkey();
    let vault = get_associated_token_address(&pda, &token_mint_address);
    let user_vault = get_associated_token_address(&user, &token_mint_address);

    // Ensure user_vault (ATA) exists
    if rpc_client.get_account(&user_vault).await.is_err() {
        let _ = create_associated_token_address(
            rpc_client,
            wallet,
            &user,
            &token_mint_address,
            &token_program_id,
        )
        .await?;
        println!("Created {token_symbol} ATA: {user_vault}");
    }

    // Build the swap instruction
    let instruction = Instruction {
        program_id: devtoken_distributor_program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new_readonly(token_mint_address, false),
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

    // Send and confirm the transaction
    let signature = send_and_confirm_transaction(
        rpc_client,
        &[instruction],
        &user,
        &[wallet],
        None,
        None,
        None,
    )
    .await?;
    println!("Swapped SOL to {token_symbol}, signature: {}", signature);

    // Check balance (optional for logging)
    let balance = rpc_client
        .get_token_account_balance(&user_vault)
        .await
        .map_err(|e| format!("Failed to get token balance: {}", e))?;
    println!(
        "{token_symbol} balance: {}",
        balance.ui_amount.unwrap_or(0.0)
    );

    Ok(())
}
