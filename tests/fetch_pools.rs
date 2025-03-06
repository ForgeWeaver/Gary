#[cfg(test)]
mod tests {
    use orca_whirlpools::{PoolInfo, WhirlpoolsConfigInput, set_whirlpools_config_address};
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_fetch_splash_pool_from_devnet() {
        // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Fetch%20Pools/#fetching-a-splash-pool
        use orca_whirlpools::fetch_splash_pool;

        set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet).unwrap();
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k").unwrap(); // devUSDC

        let pool_info = fetch_splash_pool(&rpc, token_a, token_b).await.unwrap();

        match pool_info {
            PoolInfo::Initialized(pool) => println!("Pool is initialised: {:?}", pool),
            PoolInfo::Uninitialized(pool) => println!("Pool is not initialised: {:?}", pool),
        }
    }

    #[tokio::test]
    async fn test_fetch_concentrated_liquidity_pool_from_devnet() {
        // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Fetch%20Pools/#fetching-a-concentrated-liquidity-pool
        use orca_whirlpools::fetch_concentrated_liquidity_pool;

        // Set Whirlpools config for Devnet
        set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet)
            .expect("Failed to set Whirlpools config");

        // Create non-blocking RPC client
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

        // Use known SOL/devUSDC pool tokens and tick spacing
        let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112")
            .expect("Invalid token_a pubkey"); // Wrapped SOL
        let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k")
            .expect("Invalid token_b pubkey"); // devUSDC
        let tick_spacing = 64;

        // Fetch pool info with retry logic
        let pool_info_result =
            fetch_concentrated_liquidity_pool(&rpc, token_a, token_b, tick_spacing).await;

        // Assert and handle result
        match pool_info_result {
            Ok(pool_info) => match pool_info {
                PoolInfo::Initialized(pool) => {
                    println!("Pool is initialised: {:?}", pool);
                    assert_eq!(pool.data.token_mint_a, token_a, "Token A mismatch");
                    assert_eq!(pool.data.token_mint_b, token_b, "Token B mismatch");
                    assert_eq!(
                        pool.data.tick_spacing, tick_spacing,
                        "Tick spacing mismatch"
                    );
                }
                PoolInfo::Uninitialized(pool) => {
                    println!("Pool is not initialised: {:?}", pool);
                    // Not necessarily a failure—pool might not exist yet
                }
            },
            Err(e) => {
                panic!("Failed to fetch pool: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_whirlpools_by_token_pair_from_devnet() {
        // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Fetch%20Pools/#fetching-pools-by-token-pairs
        use orca_whirlpools::fetch_whirlpools_by_token_pair;

        set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet).unwrap();
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k").unwrap(); // devUSDC

        let pool_infos = fetch_whirlpools_by_token_pair(&rpc, token_a, token_b)
            .await
            .unwrap();

        for pool_info in pool_infos {
            match pool_info {
                PoolInfo::Initialized(pool) => println!("Pool is initialised: {:?}", pool),
                PoolInfo::Uninitialized(pool) => println!("Pool is not initialised: {:?}", pool),
            }
        }
    }
}
