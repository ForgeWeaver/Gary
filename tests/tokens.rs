#[cfg(test)]
mod tests {
    use gary::config::{networks::Network, tokens::Tokens};

    #[test]
    fn test_load_tokens() {
        let tokens = Tokens::load();

        // Test for known tokens
        assert!(
            tokens.get(&Network::Mainnet, "RAY").is_some(),
            "RAY token not found"
        );
        assert!(
            tokens.get(&Network::Devnet, "devUSDC").is_some(),
            "devUSDC token not found on Devnet"
        );

        // Test for an unknown token
        assert!(
            tokens.get(&Network::Mainnet, "UNKNOWN").is_none(),
            "Unexpected token found"
        );
    }
}
