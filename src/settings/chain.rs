use colored::*;
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

        let icon = "⚡".truecolor(255, 215, 0).bold();
        let name_str = format!("{:<20}", self.name).truecolor(0, 255, 170).bold();
        let pipe = "│".truecolor(0, 225, 255);
        let mainnet_label = "Mainnet ID:".truecolor(180, 180, 180);
        let mainnet_id = format!("{:<6}", self.mainnet.chain_id).truecolor(0, 225, 255).bold();
        let testnet_label = "Testnet:".truecolor(180, 180, 180);
        let testnet_info = format!("{} ({})", testnet_name, self.testnet.chain_id)
            .truecolor(255, 200, 0)
            .bold();

        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            icon, name_str, pipe, mainnet_label, mainnet_id, pipe, testnet_label, testnet_info
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