#[cfg(test)]
mod tests {
    use gary::wallet::load_wallet;
    use solana_sdk::{signature::Keypair, signer::Signer};
    use std::{env, io::Write};
    use tempfile::NamedTempFile;

    // Helper to create a temp file with a valid keypair
    fn create_valid_keypair_file() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        let keypair = Keypair::new();
        let keypair_bytes = keypair.to_bytes();
        let json = serde_json::to_string(&keypair_bytes.to_vec()).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_load_wallet_from_env_var_success() {
        // Create a valid keypair file
        let temp_file = create_valid_keypair_file();
        let path = temp_file.path().to_str().unwrap().to_string();

        // Set env var
        unsafe { env::set_var("GARY_WALLET_PATH", &path) };

        // Test loading
        let result = load_wallet();
        assert!(result.is_ok(), "Expected Ok, got {:?}", result.err());
        let signer = result.unwrap();
        assert_eq!(
            signer.pubkey().to_string().len(),
            44,
            "Invalid pubkey length"
        ); // Solana pubkeys are 44 chars
    }

    #[test]
    fn test_load_wallet_missing_file() {
        // Set env to a nonexistent file
        unsafe { env::set_var("GARY_WALLET_PATH", "/tmp/nonexistent.json") };

        let result = load_wallet();
        assert!(result.is_err(), "Expected Err, got {:?}", result.is_ok());
        let err = result.err().unwrap();
        assert!(
            err.to_string().contains("Failed to read keypair file"),
            "Unexpected error: {}",
            err
        );
    }

    #[test]
    fn test_load_wallet_invalid_json() {
        // Create a temp file with invalid JSON
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "not json").unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        unsafe { env::set_var("GARY_WALLET_PATH", &path) };

        let result = load_wallet();
        assert!(result.is_err(), "Expected Err, got {:?}", result.is_ok());
        let err = result.err().unwrap();
        assert!(
            err.to_string().contains("Failed to parse keypair JSON"),
            "Unexpected error: {}",
            err
        );
    }

    #[test]
    fn test_load_wallet_invalid_keypair_bytes() {
        // Create a temp file with invalid bytes
        let mut temp_file = NamedTempFile::new().unwrap();
        let invalid_bytes = vec![1, 2, 3]; // Too short for a keypair
        let json = serde_json::to_string(&invalid_bytes).unwrap();
        writeln!(temp_file, "{}", json).unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        unsafe { env::set_var("GARY_WALLET_PATH", &path) };

        let result = load_wallet();
        assert!(result.is_err(), "Expected Err, got {:?}", result.err());
        let err = result.err().unwrap();
        assert!(
            err.to_string()
                .contains("Failed to create keypair from bytes"),
            "Unexpected error: {}",
            err
        );
    }
}
