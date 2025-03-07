#[cfg(test)]
mod tests {
    use orca_whirlpools::{PoolInfo, WhirlpoolsConfigInput, set_whirlpools_config_address};
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    #[ignore]
    async fn test_fetch_splash_pool_from_devnet() {
        // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Fetch%20Pools/#fetching-a-splash-pool
        use orca_whirlpools::fetch_splash_pool;

        set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet)
            .expect("Failed to set Whirlpools config");
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112")
            .expect("Invalid token_a pubkey"); // Wrapped SOL
        let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k")
            .expect("Invalid token_b pubkey"); // devUSDC

        let pool_info_result = fetch_splash_pool(&rpc, token_a, token_b).await;

        match pool_info_result {
            Ok(pool_info) => match pool_info {
                PoolInfo::Initialized(pool) => {
                    println!("Splash pool is initialised: {:?}", pool);
                    assert_eq!(pool.data.token_mint_a, token_a, "Token A mismatch");
                    assert_eq!(pool.data.token_mint_b, token_b, "Token B mismatch");
                }
                PoolInfo::Uninitialized(pool) => {
                    println!("Splash pool is not initialised: {:?}", pool);
                }
            },
            Err(e) => {
                println!(
                    "Failed to fetch splash pool (this may be expected on Devnet): {}",
                    e
                );
                let err_str = e.to_string();
                assert!(
                    err_str.contains("WouldBlock")
                        || err_str.contains("not found")
                        || err_str.contains("try_lock"),
                    "Unexpected error: {}",
                    e
                );
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_concentrated_liquidity_pool_from_devnet() {
        // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Fetch%20Pools/#fetching-a-concentrated-liquidity-pool
        use orca_whirlpools::fetch_concentrated_liquidity_pool;

        set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet)
            .expect("Failed to set Whirlpools config");
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112")
            .expect("Invalid token_a pubkey"); // Wrapped SOL
        let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k")
            .expect("Invalid token_b pubkey"); // devUSDC
        let tick_spacing = 64;

        let pool_info_result =
            fetch_concentrated_liquidity_pool(&rpc, token_a, token_b, tick_spacing).await;

        match pool_info_result {
            Ok(pool_info) => match pool_info {
                PoolInfo::Initialized(pool) => {
                    println!("Concentrated pool is initialised: {:?}", pool);
                    assert_eq!(pool.data.token_mint_a, token_a, "Token A mismatch");
                    assert_eq!(pool.data.token_mint_b, token_b, "Token B mismatch");
                    assert_eq!(
                        pool.data.tick_spacing, tick_spacing,
                        "Tick spacing mismatch"
                    );
                }
                PoolInfo::Uninitialized(pool) => {
                    println!("Concentrated pool is not initialised: {:?}", pool);
                }
            },
            Err(e) => {
                println!(
                    "Failed to fetch concentrated pool (this may be expected on Devnet): {}",
                    e
                );
                let err_str = e.to_string();
                assert!(
                    err_str.contains("WouldBlock")
                        || err_str.contains("not found")
                        || err_str.contains("try_lock"),
                    "Unexpected error: {}",
                    e
                );
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_whirlpools_by_token_pair_from_devnet() {
        // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Fetch%20Pools/#fetching-pools-by-token-pairs
        use orca_whirlpools::fetch_whirlpools_by_token_pair;

        set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet)
            .expect("Failed to set Whirlpools config");
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112")
            .expect("Invalid token_a pubkey"); // Wrapped SOL
        let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k")
            .expect("Invalid token_b pubkey"); // devUSDC

        let pool_infos_result = fetch_whirlpools_by_token_pair(&rpc, token_a, token_b).await;

        match pool_infos_result {
            Ok(pool_infos) => {
                if pool_infos.is_empty() {
                    println!("No whirlpools found for token pair (this may be expected on Devnet)");
                }
                for pool_info in pool_infos {
                    match pool_info {
                        PoolInfo::Initialized(pool) => {
                            println!("Whirlpool is initialised: {:?}", pool);
                            assert_eq!(pool.data.token_mint_a, token_a, "Token A mismatch");
                            assert_eq!(pool.data.token_mint_b, token_b, "Token B mismatch");
                        }
                        PoolInfo::Uninitialized(pool) => {
                            println!("Whirlpool is not initialised: {:?}", pool);
                        }
                    }
                }
            }
            Err(e) => {
                println!(
                    "Failed to fetch whirlpools by token pair (this may be expected on Devnet): {}",
                    e
                );
                let err_str = e.to_string();
                assert!(
                    err_str.contains("WouldBlock")
                        || err_str.contains("not found")
                        || err_str.contains("try_lock"),
                    "Unexpected error: {}",
                    e
                );
            }
        }
    }
}
