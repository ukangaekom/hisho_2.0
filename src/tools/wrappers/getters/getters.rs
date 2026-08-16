use crate::services::getter::{
    market::{get_marketcap, get_price},
    nft::{
        get_nft_balance_mainnet, get_nft_balance_testnet, get_nft_details_mainnet,
        get_nft_details_testnet, get_nft_total_supply_mainnet, get_nft_total_supply_testnet,
    },
    token::{
        get_token_balance_mainnet, get_token_balance_testnet, get_token_details_mainnet,
        get_token_details_testnet, get_token_total_supply_mainnet, get_token_total_supply_testnet,
    },
    wallet::{get_native_balance, get_system_wallet, get_system_wallet_with_pin},
};
use futures::future::join_all;

// ============================= PRICES ===================================

pub async fn get_price_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_price(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_marketcap_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_marketcap(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

// ==================== TOKENS ===================================

pub async fn get_token_details_mainnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_token_details_mainnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_token_details_testnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_token_details_testnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_token_balance_mainnet_wrapper(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: get_token_balance_mainnet <token_address> <wallet_address...>".to_string();
    }
    let token = args[0];
    let futures = args
        .iter()
        .skip(1)
        .map(|&wallet| get_token_balance_mainnet(token, wallet));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_token_balance_testnet_wrapper(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: get_token_balance_testnet <token_address> <wallet_address...>".to_string();
    }
    let token = args[0];
    let futures = args
        .iter()
        .skip(1)
        .map(|&wallet| get_token_balance_testnet(token, wallet));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_token_total_supply_mainnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_token_total_supply_mainnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_token_total_supply_testnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_token_total_supply_testnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_native_balance_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_native_balance(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_system_wallet_wrapper(_args: &[&str]) -> String {
    get_system_wallet().await
}

pub async fn get_system_wallet_with_pin_wrapper(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: get_system_wallet_with_pin <pin>".to_string();
    }
    get_system_wallet_with_pin(args[0])
}

// ==================== NFT ==========================================

pub async fn get_nft_balance_mainnet_wrapper(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: get_nft_balance_mainnet <nft_address> <wallet_address...>".to_string();
    }
    let token = args[0];
    let futures = args
        .iter()
        .skip(1)
        .map(|&wallet| get_nft_balance_mainnet(token, wallet));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_nft_balance_testnet_wrapper(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: get_nft_balance_testnet <nft_address> <wallet_address...>".to_string();
    }
    let token = args[0];
    let futures = args
        .iter()
        .skip(1)
        .map(|&wallet| get_nft_balance_testnet(token, wallet));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_nft_details_mainnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_nft_details_mainnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_nft_details_testnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_nft_details_testnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_nft_total_supply_mainnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_nft_total_supply_mainnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

pub async fn get_nft_total_supply_testnet_wrapper(args: &[&str]) -> String {
    let futures = args.iter().map(|&token| get_nft_total_supply_testnet(token));
    let results: Vec<String> = join_all(futures).await;
    results.join(", ")
}

// ==================== CHAIN SETTINGS ==========================================

pub async fn switch_chain_wrapper(args: &[&str]) -> String {
    if args.is_empty() {
        return "Please specify the target blockchain or testnet to switch to.".to_string();
    }
    let query = args.join(" ");
    crate::settings::chain::switch_chain(&query)
}