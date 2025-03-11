use std::{collections::HashMap, fs::read_to_string};

use serde::Deserialize;

use super::networks::Network;

const MAINNET_TOKENS_JSON: &str = "mainnet_tokens.json";
const DEVNET_TOKENS_JSON: &str = "devnet_tokens.json";

#[derive(Debug, Deserialize)]
pub struct Token {
    symbol: String,
    mint_address: String,
    decimals: u8,
}

#[derive(Debug)]
pub struct Tokens {
    store: HashMap<Network, HashMap<String, Token>>,
}

impl Tokens {
    pub fn new() -> Self {
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

fn load_tokens(filepath: &str) -> HashMap<String, Token> {
    let data = read_to_string(filepath).expect("Unable to read file");
    let tokens: Vec<Token> = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut token_map = HashMap::new();
    for token in tokens {
        token_map.insert(token.symbol.clone(), token);
    }
    token_map
}
