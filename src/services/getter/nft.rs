use crate::connection::provider::{
    get_active_mainnet_info, get_active_testnet_info, init_mainnet_provider, init_testnet_provider,
};
use crate::tools::utils::{is_erc1155_nft_contract, is_erc721_nft_contract};
use alloy::{primitives::Address, providers::Provider, sol};
use std::str::FromStr;

// NFT Standard Contract Interface
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

pub async fn get_nft_total_supply_testnet(nft_address: &str) -> String {
    let provider = init_testnet_provider().await;
    let (chain_name, _symbol) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address format: {}", nft_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc721 = IERC721::new(token_addr, provider.clone());

        let call_name = erc721.name();
        let call_symbol = erc721.symbol();
        let call_supply = erc721.totalSupply();

        let (name_res, symbol_res, supply_res) = tokio::join!(
            call_name.call(),
            call_symbol.call(),
            call_supply.call()
        );

        let name = name_res.ok();
        let symbol = symbol_res.ok();
        let totalsupply = supply_res
            .map(|s| s.to_string())
            .unwrap_or_else(|_| "N/A".to_string());

        format!(
            "NFT collection name {:?}, bearing symbol {:?} has a total supply of {} on {}",
            name, symbol, totalsupply, chain_name
        )
    } else {
        format!("The address {}, is a Wallet address", nft_address)
    }
}

pub async fn get_nft_total_supply_mainnet(nft_address: &str) -> String {
    let provider = init_mainnet_provider().await;
    let (chain_name, _symbol) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address format: {}", nft_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let (is_721, is_1155) = tokio::join!(
            is_erc721_nft_contract(&provider, token_addr),
            is_erc1155_nft_contract(&provider, token_addr)
        );

        if is_721 || is_1155 {
            let standard_str = if is_721 { "ERC721" } else { "ERC1155" };
            let erc721 = IERC721::new(token_addr, provider.clone());

            let call_name = erc721.name();
            let call_symbol = erc721.symbol();
            let call_supply = erc721.totalSupply();

            let (name_res, symbol_res, supply_res) = tokio::join!(
                call_name.call(),
                call_symbol.call(),
                call_supply.call()
            );

            let name = name_res.ok();
            let symbol = symbol_res.ok();
            let totalsupply = supply_res
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "N/A".to_string());

            format!(
                "The {} NFT collection name {:?}, bearing symbol {:?} has a total supply of {} on {}",
                standard_str, name, symbol, totalsupply, chain_name
            )
        } else {
            format!("This contract address isn't a standard NFT on {}", chain_name)
        }
    } else {
        format!("The address {}, is a Wallet address", nft_address)
    }
}

pub async fn get_nft_details_testnet(nft_address: &str) -> String {
    let provider = init_testnet_provider().await;
    let (chain_name, _symbol) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address format: {}", nft_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let (is_721, is_1155) = tokio::join!(
            is_erc721_nft_contract(&provider, token_addr),
            is_erc1155_nft_contract(&provider, token_addr)
        );

        if is_721 || is_1155 {
            let standard_str = if is_721 { "ERC721" } else { "ERC1155" };
            let erc721 = IERC721::new(token_addr, provider.clone());

            let call_name = erc721.name();
            let call_symbol = erc721.symbol();
            let call_supply = erc721.totalSupply();

            let (name_res, symbol_res, supply_res) = tokio::join!(
                call_name.call(),
                call_symbol.call(),
                call_supply.call()
            );

            let name = name_res.ok();
            let symbol = symbol_res.ok();
            let totalsupply = supply_res
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "N/A".to_string());

            format!(
                "The {} NFT symbol {:?} with name {:?} has a total supply of {} on {}",
                standard_str, symbol, name, totalsupply, chain_name
            )
        } else {
            format!("The contract address {} isn't a standard NFT on {}", nft_address, chain_name)
        }
    } else {
        format!("The address {}, is a Wallet address", nft_address)
    }
}

pub async fn get_nft_details_mainnet(nft_address: &str) -> String {
    let provider = init_mainnet_provider().await;
    let (chain_name, _symbol) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address format: {}", nft_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let (is_721, is_1155) = tokio::join!(
            is_erc721_nft_contract(&provider, token_addr),
            is_erc1155_nft_contract(&provider, token_addr)
        );

        if is_721 || is_1155 {
            let standard_str = if is_721 { "ERC721" } else { "ERC1155" };
            let erc721 = IERC721::new(token_addr, provider.clone());

            let call_name = erc721.name();
            let call_symbol = erc721.symbol();
            let call_supply = erc721.totalSupply();

            let (name_res, symbol_res, supply_res) = tokio::join!(
                call_name.call(),
                call_symbol.call(),
                call_supply.call()
            );

            let name = name_res.ok();
            let symbol = symbol_res.ok();
            let totalsupply = supply_res
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "N/A".to_string());

            format!(
                "The {} NFT symbol {:?} with name {:?} has a total supply of {} on {}",
                standard_str, symbol, name, totalsupply, chain_name
            )
        } else {
            format!("The contract address {} isn't a standard NFT contract on {}", nft_address, chain_name)
        }
    } else {
        format!("The address {}, is a Wallet address", nft_address)
    }
}

pub async fn get_nft_balance_testnet(nft_address: &str, wallet_address: &str) -> String {
    let provider = init_testnet_provider().await;
    let (chain_name, _symbol) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address format: {}", nft_address);
    };
    let Ok(wallet_addr) = Address::from_str(wallet_address) else {
        return format!("Invalid wallet address format: {}", wallet_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let verify_wallet = provider.get_code_at(wallet_addr).await;

        if matches!(verify_wallet, Ok(ref code) if code.is_empty()) {
            let (is_721, is_1155) = tokio::join!(
                is_erc721_nft_contract(&provider, token_addr),
                is_erc1155_nft_contract(&provider, token_addr)
            );

            if is_721 || is_1155 {
                let standard_str = if is_721 { "ERC721" } else { "ERC1155" };
                let erc721 = IERC721::new(token_addr, provider.clone());

                let call_name = erc721.name();
                let call_symbol = erc721.symbol();
                let call_balance = erc721.balanceOf(wallet_addr);

                let (name_res, symbol_res, balance_res) = tokio::join!(
                    call_name.call(),
                    call_symbol.call(),
                    call_balance.call()
                );

                let name = name_res.ok();
                let symbol = symbol_res.ok();
                let balance = balance_res
                    .map(|b| b.to_string())
                    .unwrap_or_else(|_| "0".to_string());

                format!(
                    "The wallet {:?} has an {} NFT balance of {} in {:?} ({:?}) on {}",
                    wallet_addr, standard_str, balance, name, symbol, chain_name
                )
            } else {
                format!("The address {} isn't a standard NFT on {}", nft_address, chain_name)
            }
        } else {
            format!("The address {}, is not a wallet address", wallet_address)
        }
    } else {
        format!("The address {}, is a Wallet address", nft_address)
    }
}

pub async fn get_nft_balance_mainnet(nft_address: &str, wallet_address: &str) -> String {
    let provider = init_mainnet_provider().await;
    let (chain_name, _symbol) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(nft_address) else {
        return format!("Invalid NFT contract address format: {}", nft_address);
    };
    let Ok(wallet_addr) = Address::from_str(wallet_address) else {
        return format!("Invalid wallet address format: {}", wallet_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let verify_wallet = provider.get_code_at(wallet_addr).await;

        if matches!(verify_wallet, Ok(ref code) if code.is_empty()) {
            let (is_721, is_1155) = tokio::join!(
                is_erc721_nft_contract(&provider, token_addr),
                is_erc1155_nft_contract(&provider, token_addr)
            );

            if is_721 || is_1155 {
                let standard_str = if is_721 { "ERC721" } else { "ERC1155" };
                let erc721 = IERC721::new(token_addr, provider.clone());

                let call_name = erc721.name();
                let call_symbol = erc721.symbol();
                let call_balance = erc721.balanceOf(wallet_addr);

                let (name_res, symbol_res, balance_res) = tokio::join!(
                    call_name.call(),
                    call_symbol.call(),
                    call_balance.call()
                );

                let name = name_res.ok();
                let symbol = symbol_res.ok();
                let balance = balance_res
                    .map(|b| b.to_string())
                    .unwrap_or_else(|_| "0".to_string());

                format!(
                    "The wallet {:?} has an {} NFT balance of {} in {:?} ({:?}) on {}",
                    wallet_addr, standard_str, balance, name, symbol, chain_name
                )
            } else {
                format!("The address {} isn't a standard NFT on {}", nft_address, chain_name)
            }
        } else {
            format!("The address {}, is not a wallet address", wallet_address)
        }
    } else {
        format!("The address {}, is a Wallet address", nft_address)
    }
}