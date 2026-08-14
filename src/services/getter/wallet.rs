use crate::connection::provider::{get_active_mainnet_info, init_rpc_provider};
use crate::settings::{config::get_public_wallet_address, storage, wallet::get_evm_wallet_address};
use alloy::{primitives::utils::format_units, primitives::Address, providers::Provider};
use std::str::FromStr;

pub async fn get_native_balance(wallet: &str) -> String {
    let provider = init_rpc_provider().await;

    let wallet_addr = match Address::from_str(wallet) {
        Ok(addr) => addr,
        Err(_) => return format!("Invalid wallet address: {}", wallet),
    };

    let balance = match provider.get_balance(wallet_addr).await {
        Ok(b) => b,
        Err(e) => return format!("Failed to fetch balance for {}: {}", wallet, e),
    };

    let formatted_balance = format_units(balance, 18).unwrap_or_else(|_| balance.to_string());
    let (chain_name, symbol) = get_active_mainnet_info();

    format!(
        "The address {} has a {} native token ({}) balance of {}",
        wallet, chain_name, symbol, formatted_balance
    )
}

/// Retrieves the system's public EVM wallet address without requiring a PIN prompt.
pub async fn get_system_wallet() -> String {
    if !storage::has_wallet() {
        return "No system wallet configured in vault. Please run setup wizard first.".to_string();
    }

    let public_addr = get_public_wallet_address();
    if public_addr.starts_with("0x") {
        format!("System EVM Wallet Address: {}", public_addr)
    } else {
        "Public EVM wallet address is configured. You can derive it anytime.".to_string()
    }
}

/// Derives and returns the EVM wallet address using a PIN.
pub fn get_system_wallet_with_pin(pin: &str) -> String {
    if !storage::has_wallet() {
        return "No system wallet configured in vault.".to_string();
    }

    match storage::view_wallet_with_pin(pin) {
        Ok(mnemonic) => match get_evm_wallet_address(&mnemonic) {
            Ok(address) => format!("System EVM Wallet Address: {}", address),
            Err(e) => format!("Error deriving EVM wallet address: {}", e),
        },
        Err(e) => format!("Authentication failed: {}", e),
    }
}