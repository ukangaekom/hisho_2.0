use crate::connection::provider::{
    get_active_mainnet_info, get_active_testnet_info, init_mainnet_provider, init_testnet_provider,
};
use alloy::{
    primitives::utils::format_units, primitives::Address, providers::Provider, sol,
};
use std::str::FromStr;

// Definition of Contracts
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

pub async fn get_token_total_supply_testnet(token_address: &str) -> String {
    let provider = init_testnet_provider().await;
    let (chain_name, _native_symbol) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token address format: {}", token_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc20 = IERC20::new(token_addr, provider.clone());

        let call_name = erc20.name();
        let call_symbol = erc20.symbol();
        let call_supply = erc20.totalSupply();
        let call_decimals = erc20.decimals();

        let (name_res, symbol_res, supply_res, decimals_res) = tokio::join!(
            call_name.call(),
            call_symbol.call(),
            call_supply.call(),
            call_decimals.call()
        );

        match (name_res, symbol_res, supply_res) {
            (Ok(name), Ok(symbol), Ok(totalsupply)) => {
                let decimals = decimals_res.unwrap_or(18);
                let formatted_supply =
                    format_units(totalsupply, decimals).unwrap_or_else(|_| totalsupply.to_string());

                format!(
                    "The {} token name: {:?}, symbol: {:?}, with a total supply of {}",
                    chain_name, name, symbol, formatted_supply
                )
            }
            _ => format!(
                "The token address {:?} doesn't exist on {}, check if it's an NFT or wallet. If it doesn't exist, it is likely on another chain.",
                token_addr, chain_name
            ),
        }
    } else {
        format!("{} is a wallet address", token_addr)
    }
}

pub async fn get_token_total_supply_mainnet(token_address: &str) -> String {
    let provider = init_mainnet_provider().await;
    let (chain_name, _native_symbol) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token address format: {}", token_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc20 = IERC20::new(token_addr, provider.clone());

        let call_name = erc20.name();
        let call_symbol = erc20.symbol();
        let call_supply = erc20.totalSupply();
        let call_decimals = erc20.decimals();

        let (name_res, symbol_res, supply_res, decimals_res) = tokio::join!(
            call_name.call(),
            call_symbol.call(),
            call_supply.call(),
            call_decimals.call()
        );

        match (name_res, symbol_res, supply_res) {
            (Ok(name), Ok(symbol), Ok(totalsupply)) => {
                let decimals = decimals_res.unwrap_or(18);
                let formatted_supply =
                    format_units(totalsupply, decimals).unwrap_or_else(|_| totalsupply.to_string());

                format!(
                    "The {} token name: {:?}, symbol: {:?}, with a total supply of {}",
                    chain_name, name, symbol, formatted_supply
                )
            }
            _ => format!(
                "The token address {:?} doesn't exist on {}, check if it's an NFT or wallet. If it doesn't exist, it is likely on another chain.",
                token_addr, chain_name
            ),
        }
    } else {
        format!("{} is a wallet address", token_addr)
    }
}

pub async fn get_token_details_testnet(token_address: &str) -> String {
    let provider = init_testnet_provider().await;
    let (chain_name, _native_symbol) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token address format: {}", token_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc20 = IERC20::new(token_addr, provider.clone());

        let call_name = erc20.name();
        let call_symbol = erc20.symbol();
        let call_supply = erc20.totalSupply();
        let call_decimals = erc20.decimals();

        let (name_res, symbol_res, supply_res, decimals_res) = tokio::join!(
            call_name.call(),
            call_symbol.call(),
            call_supply.call(),
            call_decimals.call()
        );

        match (decimals_res, symbol_res, name_res, supply_res) {
            (Ok(decimals), Ok(symbol), Ok(name), Ok(totalsupply)) => {
                let formatted_supply =
                    format_units(totalsupply, decimals).unwrap_or_else(|_| totalsupply.to_string());
                format!(
                    "The {} token symbol {:?} with name {:?} having decimals {} and total supply of {}",
                    chain_name, symbol, name, decimals, formatted_supply
                )
            }
            _ => format!(
                "The token address {:?} doesn't exist as a fungible token on {}, check if it's an NFT. If it doesn't exist, it is likely on another chain.",
                token_addr, chain_name
            ),
        }
    } else {
        format!("{} is a wallet address", token_addr)
    }
}

pub async fn get_token_details_mainnet(token_address: &str) -> String {
    let provider = init_mainnet_provider().await;
    let (chain_name, _native_symbol) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token address format: {}", token_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc20 = IERC20::new(token_addr, provider.clone());

        let call_name = erc20.name();
        let call_symbol = erc20.symbol();
        let call_supply = erc20.totalSupply();
        let call_decimals = erc20.decimals();

        let (name_res, symbol_res, supply_res, decimals_res) = tokio::join!(
            call_name.call(),
            call_symbol.call(),
            call_supply.call(),
            call_decimals.call()
        );

        match (decimals_res, symbol_res, name_res, supply_res) {
            (Ok(decimals), Ok(symbol), Ok(name), Ok(totalsupply)) => {
                let formatted_supply =
                    format_units(totalsupply, decimals).unwrap_or_else(|_| totalsupply.to_string());
                format!(
                    "The {} token symbol {:?} with name {:?} having decimals {} and total supply of {}",
                    chain_name, symbol, name, decimals, formatted_supply
                )
            }
            _ => format!(
                "The token address {:?} doesn't exist as a fungible token on {}, check if it's an NFT. If it doesn't exist, it is likely on another chain.",
                token_addr, chain_name
            ),
        }
    } else {
        format!("{} is a wallet address", token_addr)
    }
}

pub async fn get_token_balance_testnet(token_address: &str, wallet_address: &str) -> String {
    let provider = init_testnet_provider().await;
    let (chain_name, _native_symbol) = get_active_testnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token address format: {}", token_address);
    };
    let Ok(wallet_addr) = Address::from_str(wallet_address) else {
        return format!("Invalid wallet address format: {}", wallet_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc20 = IERC20::new(token_addr, provider.clone());

        let call_decimals = erc20.decimals();
        let call_symbol = erc20.symbol();
        let call_balance = erc20.balanceOf(wallet_addr);

        let (decimals_res, symbol_res, balance_res) = tokio::join!(
            call_decimals.call(),
            call_symbol.call(),
            call_balance.call()
        );

        match (decimals_res, symbol_res, balance_res) {
            (Ok(decimals), Ok(symbol), Ok(balance)) => {
                let token_balance =
                    format_units(balance, decimals).unwrap_or_else(|_| balance.to_string());
                format!(
                    "The wallet {:?} on {} has a token balance of {} {}",
                    wallet_addr, chain_name, token_balance, symbol
                )
            }
            _ => format!(
                "The token address {:?} does not exist as a Fungible Token on {}, check if it's an NFT or contract on another chain.",
                token_addr, chain_name
            ),
        }
    } else {
        format!("{} is a wallet address", token_addr)
    }
}

pub async fn get_token_balance_mainnet(token_address: &str, wallet_address: &str) -> String {
    let provider = init_mainnet_provider().await;
    let (chain_name, _native_symbol) = get_active_mainnet_info();

    let Ok(token_addr) = Address::from_str(token_address) else {
        return format!("Invalid token address format: {}", token_address);
    };
    let Ok(wallet_addr) = Address::from_str(wallet_address) else {
        return format!("Invalid wallet address format: {}", wallet_address);
    };

    let wallet = provider.get_code_at(token_addr).await;

    if matches!(wallet, Ok(ref code) if !code.is_empty()) {
        let erc20 = IERC20::new(token_addr, provider.clone());

        let call_decimals = erc20.decimals();
        let call_symbol = erc20.symbol();
        let call_balance = erc20.balanceOf(wallet_addr);

        let (decimals_res, symbol_res, balance_res) = tokio::join!(
            call_decimals.call(),
            call_symbol.call(),
            call_balance.call()
        );

        match (decimals_res, symbol_res, balance_res) {
            (Ok(decimals), Ok(symbol), Ok(balance)) => {
                let token_balance =
                    format_units(balance, decimals).unwrap_or_else(|_| balance.to_string());
                format!(
                    "The wallet {:?} on {} has a token balance of {} {}",
                    wallet_addr, chain_name, token_balance, symbol
                )
            }
            _ => format!(
                "The token address {:?} does not exist as a Fungible Token on {}",
                token_addr, chain_name
            ),
        }
    } else {
        format!("{} is a wallet address", token_addr)
    }
}
