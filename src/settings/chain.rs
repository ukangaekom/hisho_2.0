use serde::{Deserialize, Serialize};

pub const CHAIN_JSON: &str = include_str!("../../chain.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NativeToken {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Mainnet {
    pub chain_id: u64,
    pub rpc_url: String,
    pub native_token: NativeToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Testnet {
    pub label: Option<String>,
    pub chain_id: u64,
    pub rpc_url: String,
    pub native_token: NativeToken,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blockchain {
    pub name: String,
    pub mainnet: Mainnet,
    pub testnet: Testnet,
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let testnet_name = self
            .testnet
            .label
            .as_deref()
            .unwrap_or("Testnet");
        write!(
            f,
            "⚡ {:<22} │ Mainnet ID: {:<6} │ Testnet: {} ({})",
            self.name, self.mainnet.chain_id, testnet_name, self.testnet.chain_id
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub chains: Vec<Blockchain>,
}

impl AppConfig {
    pub fn load_default() -> Result<Self, String> {
        serde_json::from_str(CHAIN_JSON).map_err(|e| format!("Failed to parse chain.json: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_chain_json() {
        let config = AppConfig::load_default().expect("Failed to load chain.json");
        assert!(!config.chains.is_empty(), "Chains list should not be empty");

        let eth = config.chains.iter().find(|c| c.name == "Ethereum");
        assert!(eth.is_some(), "Ethereum should be present in chain.json");
        let eth = eth.unwrap();
        assert_eq!(eth.mainnet.chain_id, 1);
        assert_eq!(eth.testnet.chain_id, 11155111);

        let mantle = config.chains.iter().find(|c| c.name == "Mantle");
        assert!(mantle.is_some(), "Mantle should be present in chain.json");
        let mantle = mantle.unwrap();
        assert_eq!(mantle.mainnet.chain_id, 5000);
        assert_eq!(mantle.testnet.chain_id, 5003);
    }
}

