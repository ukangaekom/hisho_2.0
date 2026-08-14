use std::str::FromStr;
use alloy::{
    primitives::{Address, U256},
    sol,
};
use crate::connection::provider::{get_active_mainnet_info, get_active_testnet_info};
use crate::services::setter::token::get_signed_provider_for_url;
use crate::settings::storage;

// NFT Standard Contract Interface (matching getter/nft.rs)
sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    contract IERC721 {
        function name() returns (string);
        function symbol() external view returns (string memory);
        function tokenURI(uint256 tokenId) external view returns (string memory);
        function totalSupply() external view returns (uint256);
        function balanceOf(address owner) external view returns (uint256 balance);
        function ownerOf(uint256 tokenId) external view returns (address owner);
        function safeTransferFrom(address from, address to, uint256 tokenId) external;
        function safeTransferFrom(address from, address to, uint256 tokenId, bytes calldata data) external;
        function transferFrom(address from, address to, uint256 tokenId) external;
        function approve(address to, uint256 tokenId) external;
        function getApproved(uint256 tokenId) external view returns (address operator);
        function setApprovalForAll(address operator, bool _approved) external;
        function isApprovedForAll(address owner, address operator) external view returns (bool);
    }
);

/// Transfers an NFT (ERC-721) on Testnet.
pub async fn transfer_nft_testnet(
    nft_address: &str,
    from_address: &str,
    to_address: &str,
    token_id: &str,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings.default_chain.testnet.rpc_url;
    let (chain_name, _) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address: {}", nft_address);
    };
    let Ok(from_addr) = Address::from_str(from_address) else {
        return format!("Invalid sender address: {}", from_address);
    };
    let Ok(to_addr) = Address::from_str(to_address) else {
        return format!("Invalid recipient address: {}", to_address);
    };
    let Ok(parsed_token_id) = U256::from_str(token_id) else {
        return format!("Invalid NFT token ID: {}", token_id);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc721 = IERC721::new(token_addr, &provider);

    let tx_builder = erc721.safeTransferFrom_0(from_addr, to_addr, parsed_token_id);
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
                        "Successfully executed NFT transfer (Token ID: {}) from {} to {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        token_id, from_addr, to_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted NFT transfer (Token ID: {}) from {} to {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        token_id, from_addr, to_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast NFT transfer transaction on {}: {}", chain_name, e),
    }
}

/// Transfers an NFT (ERC-721) on Mainnet.
pub async fn transfer_nft_mainnet(
    nft_address: &str,
    from_address: &str,
    to_address: &str,
    token_id: &str,
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

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address: {}", nft_address);
    };
    let Ok(from_addr) = Address::from_str(from_address) else {
        return format!("Invalid sender address: {}", from_address);
    };
    let Ok(to_addr) = Address::from_str(to_address) else {
        return format!("Invalid recipient address: {}", to_address);
    };
    let Ok(parsed_token_id) = U256::from_str(token_id) else {
        return format!("Invalid NFT token ID: {}", token_id);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc721 = IERC721::new(token_addr, &provider);

    let tx_builder = erc721.safeTransferFrom_0(from_addr, to_addr, parsed_token_id);
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
                        "Successfully executed NFT transfer (Token ID: {}) from {} to {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        token_id, from_addr, to_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted NFT transfer (Token ID: {}) from {} to {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        token_id, from_addr, to_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast NFT transfer transaction on {}: {}", chain_name, e),
    }
}

/// Approves an operator for a specific NFT Token ID on Testnet.
pub async fn approve_nft_testnet(
    nft_address: &str,
    to_address: &str,
    token_id: &str,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings.default_chain.testnet.rpc_url;
    let (chain_name, _) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address: {}", nft_address);
    };
    let Ok(to_addr) = Address::from_str(to_address) else {
        return format!("Invalid approved operator address: {}", to_address);
    };
    let Ok(parsed_token_id) = U256::from_str(token_id) else {
        return format!("Invalid NFT token ID: {}", token_id);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc721 = IERC721::new(token_addr, &provider);

    let tx_builder = erc721.approve(to_addr, parsed_token_id);
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
                        "Successfully executed NFT approval for Token ID {} to operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        token_id, to_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted NFT approval for Token ID {} to operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        token_id, to_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast NFT approval transaction on {}: {}", chain_name, e),
    }
}

/// Approves an operator for a specific NFT Token ID on Mainnet.
pub async fn approve_nft_mainnet(
    nft_address: &str,
    to_address: &str,
    token_id: &str,
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

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address: {}", nft_address);
    };
    let Ok(to_addr) = Address::from_str(to_address) else {
        return format!("Invalid approved operator address: {}", to_address);
    };
    let Ok(parsed_token_id) = U256::from_str(token_id) else {
        return format!("Invalid NFT token ID: {}", token_id);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc721 = IERC721::new(token_addr, &provider);

    let tx_builder = erc721.approve(to_addr, parsed_token_id);
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
                        "Successfully executed NFT approval for Token ID {} to operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        token_id, to_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    format!(
                        "Broadcasted NFT approval for Token ID {} to operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        token_id, to_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast NFT approval transaction on {}: {}", chain_name, e),
    }
}

/// Sets or revokes operator approval for all NFTs in a collection on Testnet.
pub async fn set_approval_for_all_nft_testnet(
    nft_address: &str,
    operator_address: &str,
    approved: bool,
    pin: &str,
) -> String {
    let settings = match storage::load_app_settings() {
        Ok(Some(s)) => s,
        _ => return "Failed to load system chain settings.".to_string(),
    };
    let rpc_url = settings.default_chain.testnet.rpc_url;
    let (chain_name, _) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address: {}", nft_address);
    };
    let Ok(operator_addr) = Address::from_str(operator_address) else {
        return format!("Invalid operator address: {}", operator_address);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc721 = IERC721::new(token_addr, &provider);

    let tx_builder = erc721.setApprovalForAll(operator_addr, approved);
    match tx_builder.send().await {
        Ok(pending_tx) => {
            let tx_hash = *pending_tx.tx_hash();
            match pending_tx.get_receipt().await {
                Ok(receipt) => {
                    let block_num = receipt.block_number.unwrap_or_default();
                    let block_hash = receipt.block_hash.unwrap_or_default();
                    let gas_used = receipt.gas_used;
                    let status_str = if receipt.status() { "SUCCESS" } else { "FAILED" };
                    let status_action = if approved { "granted" } else { "revoked" };
                    format!(
                        "Successfully executed setApprovalForAll ({}) for operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        status_action, operator_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    let status_action = if approved { "granted" } else { "revoked" };
                    format!(
                        "Broadcasted setApprovalForAll ({}) for operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        status_action, operator_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast setApprovalForAll transaction on {}: {}", chain_name, e),
    }
}

/// Sets or revokes operator approval for all NFTs in a collection on Mainnet.
pub async fn set_approval_for_all_nft_mainnet(
    nft_address: &str,
    operator_address: &str,
    approved: bool,
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

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address: {}", nft_address);
    };
    let Ok(operator_addr) = Address::from_str(operator_address) else {
        return format!("Invalid operator address: {}", operator_address);
    };

    let provider = match get_signed_provider_for_url(&rpc_url, pin).await {
        Ok(p) => p,
        Err(e) => return format!("Wallet authentication/provider error: {}", e),
    };

    let erc721 = IERC721::new(token_addr, &provider);

    let tx_builder = erc721.setApprovalForAll(operator_addr, approved);
    match tx_builder.send().await {
        Ok(pending_tx) => {
            let tx_hash = *pending_tx.tx_hash();
            match pending_tx.get_receipt().await {
                Ok(receipt) => {
                    let block_num = receipt.block_number.unwrap_or_default();
                    let block_hash = receipt.block_hash.unwrap_or_default();
                    let gas_used = receipt.gas_used;
                    let status_str = if receipt.status() { "SUCCESS" } else { "FAILED" };
                    let status_action = if approved { "granted" } else { "revoked" };
                    format!(
                        "Successfully executed setApprovalForAll ({}) for operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Block Number: {}\n- Block Hash: {:#x}\n- Gas Used: {}\n- Status: {}",
                        status_action, operator_addr, chain_name, tx_hash, block_num, block_hash, gas_used, status_str
                    )
                }
                Err(_) => {
                    let status_action = if approved { "granted" } else { "revoked" };
                    format!(
                        "Broadcasted setApprovalForAll ({}) for operator {} on {}.\n[Transaction Proof]\n- Tx Hash: {:#x}\n- Status: PENDING_CONFIRMATION",
                        status_action, operator_addr, chain_name, tx_hash
                    )
                }
            }
        }
        Err(e) => format!("Failed to broadcast setApprovalForAll transaction on {}: {}", chain_name, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nft_setter_invalid_addresses() {
        let res = transfer_nft_testnet("invalid_nft", "invalid_from", "invalid_to", "1", "1234").await;
        assert!(res.contains("Invalid NFT contract address"));

        let valid_addr = "0x0000000000000000000000000000000000000001";
        let res2 = transfer_nft_testnet(valid_addr, "invalid_from", "invalid_to", "1", "1234").await;
        assert!(res2.contains("Invalid sender address"));
    }
}
