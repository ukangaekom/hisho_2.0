use crate::connection::provider::{get_active_mainnet_info, init_rpc_provider};
use crate::settings::{storage, wallet::get_evm_wallet_address};
use alloy::{primitives::utils::format_units, primitives::Address, providers::Provider};
use inquire::Password;
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

pub async fn get_system_wallet() -> String {
    if !storage::has_wallet() {
        return "No system wallet configured in vault. Please run setup wizard first.".to_string();
    }

    let pin = match Password::new("Enter System PIN to unlock wallet:")
        .without_confirmation()
        .prompt()
    {
        Ok(p) => p,
        Err(_) => return "System PIN prompt was cancelled or invalid.".to_string(),
    };

    get_system_wallet_with_pin(&pin)
}

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