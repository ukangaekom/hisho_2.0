use std::str::FromStr;
use alloy::{
    network::EthereumWallet,
    primitives::{utils::parse_units, Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use crate::connection::provider::{get_active_mainnet_info, get_active_testnet_info};
use crate::settings::storage;

// Definition of IERC20 Contract Interface (matching getter/token.rs)
sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    contract IERC20 {
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
        function name() returns (string);
        function symbol() external view returns (string memory);
        function totalSupply() external view returns (uint256);
        function decimals() external view returns (uint8);
    }
);

/// Helper to derive a PrivateKeySigner from the encrypted wallet storage using system PIN.
pub fn get_signer_from_pin(pin: &str) -> Result<PrivateKeySigner, String> {
    use ethers::signers::{coins_bip39::English, MnemonicBuilder};

    let mnemonic = storage::view_wallet_with_pin(pin)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.phrase())
        .build()
        .map_err(|e| format!("Failed to derive EVM wallet address: {}", e))?;

    let key_bytes = wallet.signer().to_bytes();
    PrivateKeySigner::from_slice(&key_bytes)
        .map_err(|e| format!("Failed to create alloy PrivateKeySigner: {}", e))
}

/// Helper to create a signed provider for a given RPC URL and PIN.
pub async fn get_signed_provider_for_url(rpc_url: &str, pin: &str) -> Result<impl Provider, String> {
    let signer = get_signer_from_pin(pin)?;
    let wallet = EthereumWallet::new(signer);

    let url = rpc_url
        .parse()
        .map_err(|e| format!("Invalid RPC URL '{}': {}", rpc_url, e))?;

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(url);

    Ok(provider)
}

/// Transfers ERC-20 tokens on Testnet.
pub async fn transfer_token_testnet(
    token_address: &str,
    to_address: &str,
    amount: &str,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings.default_chain.testnet.rpc_url;
    let (chain_name, _) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token contract address: {}", token_address);
    };
    let Ok(to_addr) = Address::from_str(to_address) else {
        return format!("Invalid recipient address: {}", to_address);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc20 = IERC20::new(token_addr, &provider);

    let decimals = match erc20.decimals().call().await {
        Ok(d) => d,
        Err(_) => 18,
    };

    let parsed_amount = match parse_units(amount, decimals) {
        Ok(a) => a.into(),
        Err(_) => match U256::from_str(amount) {
            Ok(u) => u,
            Err(_) => return format!("Invalid token transfer amount: {}", amount),
        },
    };

    let tx_builder = erc20.transfer(to_addr, parsed_amount);
    match tx_builder.send().await {
        Ok(pending_tx) => {
            let tx_hash = *pending_tx.tx_hash();
            match pending_tx.get_receipt().await {
                Ok(receipt) => {
                    let block_num = receipt.block_number.unwrap_or_default();
                    let block_hash = receipt.block_hash.unwrap_or_default();
                    let gas_used = receipt.gas_used;
                    let status_str = if receipt.status() { "SUCCESS" } else { "FAILED" };
                    format!(
                        "Successfully executed token transfer of {} to recipient {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        amount, to_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted token transfer of {} to recipient {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        amount, to_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast token transfer transaction on {}: {}", chain_name, e),
    }
}

/// Transfers ERC-20 tokens on Mainnet.
pub async fn transfer_token_mainnet(
    token_address: &str,
    to_address: &str,
    amount: &str,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings
        .custom_rpc
        .unwrap_or_else(|| settings.default_chain.mainnet.rpc_url.clone());
    let (chain_name, _) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token contract address: {}", token_address);
    };
    let Ok(to_addr) = Address::from_str(to_address) else {
        return format!("Invalid recipient address: {}", to_address);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc20 = IERC20::new(token_addr, &provider);

    let decimals = match erc20.decimals().call().await {
        Ok(d) => d,
        Err(_) => 18,
    };

    let parsed_amount = match parse_units(amount, decimals) {
        Ok(a) => a.into(),
        Err(_) => match U256::from_str(amount) {
            Ok(u) => u,
            Err(_) => return format!("Invalid token transfer amount: {}", amount),
        },
    };

    let tx_builder = erc20.transfer(to_addr, parsed_amount);
    match tx_builder.send().await {
        Ok(pending_tx) => {
            let tx_hash = *pending_tx.tx_hash();
            match pending_tx.get_receipt().await {
                Ok(receipt) => {
                    let block_num = receipt.block_number.unwrap_or_default();
                    let block_hash = receipt.block_hash.unwrap_or_default();
                    let gas_used = receipt.gas_used;
                    let status_str = if receipt.status() { "SUCCESS" } else { "FAILED" };
                    format!(
                        "Successfully executed token transfer of {} to recipient {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        amount, to_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted token transfer of {} to recipient {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        amount, to_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast token transfer transaction on {}: {}", chain_name, e),
    }
}

/// Approves ERC-20 tokens for a spender on Testnet.
pub async fn approve_token_testnet(
    token_address: &str,
    spender_address: &str,
    amount: &str,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings.default_chain.testnet.rpc_url;
    let (chain_name, _) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token contract address: {}", token_address);
    };
    let Ok(spender_addr) = Address::from_str(spender_address) else {
        return format!("Invalid spender address: {}", spender_address);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc20 = IERC20::new(token_addr, &provider);

    let decimals = match erc20.decimals().call().await {
        Ok(d) => d,
        Err(_) => 18,
    };

    let parsed_amount = match parse_units(amount, decimals) {
        Ok(a) => a.into(),
        Err(_) => match U256::from_str(amount) {
            Ok(u) => u,
            Err(_) => return format!("Invalid token approval amount: {}", amount),
        },
    };

    let tx_builder = erc20.approve(spender_addr, parsed_amount);
    match tx_builder.send().await {
        Ok(pending_tx) => {
            let tx_hash = *pending_tx.tx_hash();
            match pending_tx.get_receipt().await {
                Ok(receipt) => {
                    let block_num = receipt.block_number.unwrap_or_default();
                    let block_hash = receipt.block_hash.unwrap_or_default();
                    let gas_used = receipt.gas_used;
                    let status_str = if receipt.status() { "SUCCESS" } else { "FAILED" };
                    format!(
                        "Successfully executed token approval of {} for spender {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        amount, spender_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted token approval of {} for spender {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        amount, spender_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast token approval transaction on {}: {}", chain_name, e),
    }
}

/// Approves ERC-20 tokens for a spender on Mainnet.
pub async fn approve_token_mainnet(
    token_address: &str,
    spender_address: &str,
    amount: &str,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings
        .custom_rpc
        .unwrap_or_else(|| settings.default_chain.mainnet.rpc_url.clone());
    let (chain_name, _) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token contract address: {}", token_address);
    };
    let Ok(spender_addr) = Address::from_str(spender_address) else {
        return format!("Invalid spender address: {}", spender_address);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc20 = IERC20::new(token_addr, &provider);

    let decimals = match erc20.decimals().call().await {
        Ok(d) => d,
        Err(_) => 18,
    };

    let parsed_amount = match parse_units(amount, decimals) {
        Ok(a) => a.into(),
        Err(_) => match U256::from_str(amount) {
            Ok(u) => u,
            Err(_) => return format!("Invalid token approval amount: {}", amount),
        },
    };

    let tx_builder = erc20.approve(spender_addr, parsed_amount);
    match tx_builder.send().await {
        Ok(pending_tx) => {
            let tx_hash = *pending_tx.tx_hash();
            match pending_tx.get_receipt().await {
                Ok(receipt) => {
                    let block_num = receipt.block_number.unwrap_or_default();
                    let block_hash = receipt.block_hash.unwrap_or_default();
                    let gas_used = receipt.gas_used;
                    let status_str = if receipt.status() { "SUCCESS" } else { "FAILED" };
                    format!(
                        "Successfully executed token approval of {} for spender {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        amount, spender_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted token approval of {} for spender {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        amount, spender_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast token approval transaction on {}: {}", chain_name, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_setter_invalid_addresses() {
        let res = transfer_token_testnet("invalid_token", "invalid_to", "10", "1234").await;
        assert!(res.contains("Invalid token contract address"));

        let valid_addr = "0x0000000000000000000000000000000000000001";
        let res2 = transfer_token_testnet(valid_addr, "invalid_to", "10", "1234").await;
        assert!(res2.contains("Invalid recipient address"));
    }
}
