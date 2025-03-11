use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::genesis_config::ClusterType;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Network {
    Mainnet,
    Devnet,
    Testnet,
    Development,
}

impl Network {
    pub fn rpc_client(self) -> RpcClient {
        RpcClient::new(self.rpc_url())
    }

    pub fn rpc_url(self) -> String {
        match self {
            Network::Development => "http://localhost:8899".to_string(),
            _ => format!("https://api.{}.solana.com", String::from(self)),
        }
    }
}

impl From<ClusterType> for Network {
    fn from(value: ClusterType) -> Self {
        match value {
            ClusterType::MainnetBeta => Network::Mainnet,
            ClusterType::Devnet => Network::Devnet,
            ClusterType::Testnet => Network::Testnet,
            ClusterType::Development => Network::Development,
        }
    }
}

impl From<Network> for ClusterType {
    fn from(value: Network) -> Self {
        match value {
            Network::Mainnet => ClusterType::MainnetBeta,
            Network::Devnet => ClusterType::Devnet,
            Network::Testnet => ClusterType::Testnet,
            Network::Development => ClusterType::Development,
        }
    }
}

impl From<Network> for String {
    fn from(value: Network) -> Self {
        match value {
            Network::Mainnet => "mainnet-beta",
            Network::Devnet => "devnet",
            Network::Testnet => "testnet",
            Network::Development => "localhost",
        }
        .to_string()
    }
}
