use std::{collections::HashMap, fs::read_to_string, path::Path, str::FromStr};

use serde::Deserialize;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};

use super::networks::Network;

#[cfg(not(test))]
const TOKEN_FOLDER: &str = "./src/config/";

#[cfg(test)]
const TOKEN_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/config/");

const MAINNET_TOKENS_JSON: &str = "mainnet_tokens.json";
const DEVNET_TOKENS_JSON: &str = "devnet_tokens.json";

#[derive(Clone, Debug, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub mint_address: String,
    pub decimals: u8,
}

impl Token {
    pub fn as_pubkey(&self) -> Result<Pubkey, ParsePubkeyError> {
        Pubkey::from_str(&self.mint_address)
    }
}

#[derive(Debug)]
pub struct Tokens {
    store: HashMap<Network, HashMap<String, Token>>,
}

impl Tokens {
    pub fn load() -> Self {
        let mut store = HashMap::new();
        store.insert(Network::Mainnet, load_tokens(MAINNET_TOKENS_JSON));
        store.insert(Network::Devnet, load_tokens(DEVNET_TOKENS_JSON));

        Self { store }
    }

    pub fn get(&self, network: &Network, symbol: &str) -> Option<&Token> {
        if let Some(tokens) = self.store.get(network) {
            tokens.get(symbol)
        } else {
            None
        }
    }
}

fn load_tokens(filename: &str) -> HashMap<String, Token> {
    let filepath = Path::new(TOKEN_FOLDER).join(filename);
    let data = read_to_string(filepath).expect("Unable to read file");
    let tokens: Vec<Token> = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut token_map = HashMap::new();
    for token in tokens {
        token_map.insert(token.symbol.clone(), token);
    }
    token_map
}
