#[cfg(test)]
mod tests {
    // https://everlastingsong.github.io/nebula/
    // devToken specification
    // | Token Mint Address                            | Decimals | Dev Token |
    // |-----------------------------------------------|---------:|-----------|
    // | brjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k6 |        6 | devUSDC   |
    // | 8UekPGwePSmQ3ttuYGPU1szyFfjZR4N53rymSFwpLPm6  |        6 | devUSDTH  |
    // | Jd4M8bfJG3sAkd82RsGWyEXoaBXQP7njFzBwEaCTuDa9  |        9 | devSAMO   |
    // | fn8YB1p4NsoZeS5XJBZ18LTfEy5NFPwN46wapZcBQr66  |        9 | devTMACA  |

    use std::str::FromStr;

    use gary::{utils::addr_util, wallet};
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::{pubkey::Pubkey, signer::Signer};

    #[tokio::test]
    async fn test_create_ata_devusdc() {
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let wallet = wallet::load_wallet().unwrap();

        // Create ATA for devUSDC
        let dev_usdc_mint =
            Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k").unwrap();
        let token_program_id =
            Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let result = addr_util::create_associated_token_address(
            &rpc,
            &wallet,
            &wallet.pubkey(),
            &dev_usdc_mint,
            &token_program_id,
        )
        .await;
        assert!(result.is_ok());
        let dev_usdc_ata = result.unwrap();
        println!("ATA for devUSDC: {dev_usdc_ata}");
    }
}
