#[cfg(test)]
mod tests {
    // https://everlastingsong.github.io/nebula/

    use std::str::FromStr;

    use gary::{utils::addr_util::create_associated_token_address, wallet};
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
    use spl_associated_token_account::get_associated_token_address;

    const DEVNET_RPC: &str = "https://api.devnet.solana.com";
    const DEV_USDC_MINT: &str = "BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k";
    const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

    // Helper to load wallet and ensure it has SOL
    async fn setup_wallet(rpc: &RpcClient) -> Keypair {
        let wallet = wallet::load_wallet().expect("Failed to load wallet");
        // Ensure wallet has some SOL for transaction fees
        let balance = rpc.get_balance(&wallet.pubkey()).await.unwrap_or(0);
        if balance < 1_000_000 {
            // ~0.001 SOL for fees
            panic!(
                "Wallet {} has insufficient SOL on Devnet. Airdrop some SOL with: `solana airdrop 2 {} --url devnet`",
                wallet.pubkey(),
                wallet.pubkey()
            );
        }
        wallet
    }

    #[tokio::test]
    async fn test_create_ata_devusdc() {
        let rpc = RpcClient::new(DEVNET_RPC.to_string());
        let wallet = setup_wallet(&rpc).await;

        let dev_usdc_mint = Pubkey::from_str(DEV_USDC_MINT).expect("Invalid devUSDC mint");
        let token_program_id =
            Pubkey::from_str(TOKEN_PROGRAM_ID).expect("Invalid token program ID");

        // Calculate expected ATA
        let expected_ata = get_associated_token_address(&wallet.pubkey(), &dev_usdc_mint);

        // Attempt to create ATA
        let result = create_associated_token_address(
            &rpc,
            &wallet,
            &wallet.pubkey(),
            &dev_usdc_mint,
            &token_program_id,
        )
        .await;

        assert!(result.is_ok(), "Failed to create ATA: {:?}", result.err());
        let dev_usdc_ata = result.unwrap();
        println!("ATA for devUSDC: {}", dev_usdc_ata);

        // Verify ATA matches expected address
        assert_eq!(dev_usdc_ata, expected_ata, "ATA address mismatch");

        // Verify ATA exists on chain
        let account = rpc.get_account(&dev_usdc_ata).await;
        assert!(
            account.is_ok(),
            "ATA not found on chain: {:?}",
            account.err()
        );
        let account_data = account.unwrap();
        assert_eq!(
            account_data.owner, token_program_id,
            "ATA has incorrect owner"
        );
    }

    #[tokio::test]
    async fn test_create_ata_devusdc_idempotent() {
        let rpc = RpcClient::new(DEVNET_RPC.to_string());
        let wallet = setup_wallet(&rpc).await;

        let dev_usdc_mint = Pubkey::from_str(DEV_USDC_MINT).expect("Invalid devUSDC mint");
        let token_program_id =
            Pubkey::from_str(TOKEN_PROGRAM_ID).expect("Invalid token program ID");

        // First call: Create or get existing ATA
        let first_result = create_associated_token_address(
            &rpc,
            &wallet,
            &wallet.pubkey(),
            &dev_usdc_mint,
            &token_program_id,
        )
        .await;
        assert!(
            first_result.is_ok(),
            "First ATA creation failed: {:?}",
            first_result.err()
        );
        let first_ata = first_result.unwrap();

        // Second call: Should return existing ATA without error
        let second_result = create_associated_token_address(
            &rpc,
            &wallet,
            &wallet.pubkey(),
            &dev_usdc_mint,
            &token_program_id,
        )
        .await;
        assert!(
            second_result.is_ok(),
            "Second ATA creation failed: {:?}",
            second_result.err()
        );
        let second_ata = second_result.unwrap();

        assert_eq!(first_ata, second_ata, "ATA addresses differ between calls");
        println!("Idempotent ATA for devUSDC: {}", second_ata);

        // Verify ATA still exists
        let account = rpc.get_account(&second_ata).await;
        assert!(
            account.is_ok(),
            "ATA not found after second call: {:?}",
            account.err()
        );
    }
}
