// Modified from: https://github.com/orca-so/whirlpools/tree/main/examples/rust-sdk/whirlpool_repositioning_bot
// Original: https://github.com/orca-so/whirlpools

use solana_sdk::{signature::Keypair, signer::Signer};
use std::{env, fs, path::PathBuf};

use crate::BoxedStdError;

// Loads a Solana keypair from a file, defaulting to ~/.config/solana/id.json or an env var.
// Returns an impl Signer for use in transactions.
pub fn load_wallet() -> Result<impl Signer, BoxedStdError> {
    // Get keypair path from env var WALLET_PATH, fallback to default
    let keypair_path = env::var("WALLET_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| get_solana_keypair_path());
    println!("Solana keypair path: {:?}", keypair_path);

    // Read the keypair file
    let wallet_string = fs::read_to_string(&keypair_path)
        .map_err(|e| format!("Failed to read keypair file at {:?}: {}", keypair_path, e))?;

    // Parse JSON into bytes
    let keypair_bytes: Vec<u8> = serde_json::from_str(&wallet_string)
        .map_err(|e| format!("Failed to parse keypair JSON: {}", e))?;

    // Convert bytes to Keypair
    let wallet = Keypair::from_bytes(&keypair_bytes)
        .map_err(|e| format!("Failed to create keypair from bytes: {}", e))?;
    println!("Valid keypair found. Public key: {}", wallet.pubkey());

    Ok(wallet)
}

// Constructs the Solana keypair path.
// If GARY_WALLET_PATH is set, it uses that.
// Otherwise, it defaults to ~/.config/solana/id.json.
fn get_solana_keypair_path() -> PathBuf {
    if let Ok(path) = env::var("GARY_WALLET_PATH") {
        PathBuf::from(path)
    } else {
        let home_dir = env::var("HOME").expect("Could not find HOME directory");
        PathBuf::from(home_dir).join(".config/solana/id.json")
    }
}
