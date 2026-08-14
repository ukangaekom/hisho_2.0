use crate::services::setter::{
    nft::{
        approve_nft_mainnet, approve_nft_testnet, set_approval_for_all_nft_mainnet,
        set_approval_for_all_nft_testnet, transfer_nft_mainnet, transfer_nft_testnet,
    },
    token::{
        approve_token_mainnet, approve_token_testnet, transfer_token_mainnet,
        transfer_token_testnet,
    },
};

// ==================== TOKEN SETTERS ====================

pub async fn transfer_token_mainnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: transfer_token_mainnet <token_address> <to_address> <amount> [pin]".to_string();
    }
    let token = args[0];
    let to = args[1];
    let amount = args[2];
    let pin = args.get(3).copied().unwrap_or("");
    transfer_token_mainnet(token, to, amount, pin).await
}

pub async fn transfer_token_testnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: transfer_token_testnet <token_address> <to_address> <amount> [pin]".to_string();
    }
    let token = args[0];
    let to = args[1];
    let amount = args[2];
    let pin = args.get(3).copied().unwrap_or("");
    transfer_token_testnet(token, to, amount, pin).await
}

pub async fn approve_token_mainnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: approve_token_mainnet <token_address> <spender_address> <amount> [pin]".to_string();
    }
    let token = args[0];
    let spender = args[1];
    let amount = args[2];
    let pin = args.get(3).copied().unwrap_or("");
    approve_token_mainnet(token, spender, amount, pin).await
}

pub async fn approve_token_testnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: approve_token_testnet <token_address> <spender_address> <amount> [pin]".to_string();
    }
    let token = args[0];
    let spender = args[1];
    let amount = args[2];
    let pin = args.get(3).copied().unwrap_or("");
    approve_token_testnet(token, spender, amount, pin).await
}

// ==================== NFT SETTERS ====================

pub async fn transfer_nft_mainnet_wrapper(args: &[&str]) -> String {
    if args.len() < 4 {
        return "Usage: transfer_nft_mainnet <nft_address> <from_address> <to_address> <token_id> [pin]".to_string();
    }
    let nft = args[0];
    let from = args[1];
    let to = args[2];
    let token_id = args[3];
    let pin = args.get(4).copied().unwrap_or("");
    transfer_nft_mainnet(nft, from, to, token_id, pin).await
}

pub async fn transfer_nft_testnet_wrapper(args: &[&str]) -> String {
    if args.len() < 4 {
        return "Usage: transfer_nft_testnet <nft_address> <from_address> <to_address> <token_id> [pin]".to_string();
    }
    let nft = args[0];
    let from = args[1];
    let to = args[2];
    let token_id = args[3];
    let pin = args.get(4).copied().unwrap_or("");
    transfer_nft_testnet(nft, from, to, token_id, pin).await
}

pub async fn approve_nft_mainnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: approve_nft_mainnet <nft_address> <to_address> <token_id> [pin]".to_string();
    }
    let nft = args[0];
    let to = args[1];
    let token_id = args[2];
    let pin = args.get(3).copied().unwrap_or("");
    approve_nft_mainnet(nft, to, token_id, pin).await
}

pub async fn approve_nft_testnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: approve_nft_testnet <nft_address> <to_address> <token_id> [pin]".to_string();
    }
    let nft = args[0];
    let to = args[1];
    let token_id = args[2];
    let pin = args.get(3).copied().unwrap_or("");
    approve_nft_testnet(nft, to, token_id, pin).await
}

pub async fn set_approval_for_all_nft_mainnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: set_approval_for_all_nft_mainnet <nft_address> <operator_address> <approved_true_false> [pin]".to_string();
    }
    let nft = args[0];
    let operator = args[1];
    let approved = args[2].eq_ignore_ascii_case("true");
    let pin = args.get(3).copied().unwrap_or("");
    set_approval_for_all_nft_mainnet(nft, operator, approved, pin).await
}

pub async fn set_approval_for_all_nft_testnet_wrapper(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: set_approval_for_all_nft_testnet <nft_address> <operator_address> <approved_true_false> [pin]".to_string();
    }
    let nft = args[0];
    let operator = args[1];
    let approved = args[2].eq_ignore_ascii_case("true");
    let pin = args.get(3).copied().unwrap_or("");
    set_approval_for_all_nft_testnet(nft, operator, approved, pin).await
}
