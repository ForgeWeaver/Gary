#[cfg(test)]
mod tests {
    use std::time::Duration;

    use gary::{
        config::{
            networks::Network::{self, Devnet},
            tokens::Tokens,
        },
        utils::swap_util::swap_sol_to_orca_dev_token,
        wallet,
    };
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::{signature::Keypair, signer::Signer};
    use spl_associated_token_account::get_associated_token_address;
    use tokio::time::sleep;

    // Helper to ensure wallet has SOL
    async fn setup_wallet(rpc: &RpcClient) -> Keypair {
        let wallet = wallet::load_wallet().expect("Failed to load wallet");
        let balance = rpc.get_balance(&wallet.pubkey()).await.unwrap_or(0);
        if balance < 200_000_000 {
            // 0.2 SOL in lamports
            panic!(
                "Wallet {} has insufficient SOL (< 0.2 SOL). Airdrop with: `solana airdrop 2 {} --url devnet`",
                wallet.pubkey(),
                wallet.pubkey()
            );
        }
        wallet
    }

    #[tokio::test]
    async fn test_swap_sol_to_devusdc() {
        let rpc = Devnet.rpc_client();
        let wallet = setup_wallet(&rpc).await;
        let tokens = Tokens::load();

        let dev_usdc: gary::config::tokens::Token =
            tokens.get(&Network::Devnet, "devUSDC").unwrap().clone();
        let dev_usdc_mint = dev_usdc.clone().as_pubkey().unwrap();
        let user_vault = get_associated_token_address(&wallet.pubkey(), &dev_usdc_mint);
        let initial_balance = rpc
            .get_token_account_balance(&user_vault)
            .await
            .map(|b| b.ui_amount.unwrap_or(0.0))
            .unwrap_or(0.0);
        if initial_balance > 0.0 {
            println!("devUSDC balance: {initial_balance}");

            return;
        }

        let result = swap_sol_to_orca_dev_token(&rpc, &wallet, &dev_usdc).await;
        assert!(result.is_ok(), "Swap failed: {:?}", result.err());

        // Wait for RPC to catch up (Devnet can be slow)
        sleep(Duration::from_secs(5)).await;

        // Retry balance check up to 3 times
        let mut final_balance = 0.0;
        for attempt in 1..=3 {
            final_balance = rpc
                .get_token_account_balance(&user_vault)
                .await
                .map(|b| b.ui_amount.unwrap_or(0.0))
                .unwrap_or(0.0);
            if final_balance > initial_balance {
                break;
            }
            println!(
                "Attempt {}: devUSDC balance still {}, retrying...",
                attempt, final_balance
            );
            sleep(Duration::from_secs(2)).await;
        }

        assert!(
            final_balance > initial_balance,
            "devUSDC balance did not increase: {} -> {}",
            initial_balance,
            final_balance
        );
        println!(
            "devUSDC balance increased from {} to {}",
            initial_balance, final_balance
        );
    }
}
