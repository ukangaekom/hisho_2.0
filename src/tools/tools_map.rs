use crate::services::getter::protocols::*;
use crate::tools::wrappers::getters::getters::*;
use crate::tools::wrappers::setters::setters::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type AsyncFn = Arc<
    dyn for<'a> Fn(&'a [&'a str]) -> Pin<Box<dyn Future<Output = String> + Send + 'a>>
        + Send
        + Sync,
>;

pub static TOOLS: Lazy<HashMap<&'static str, AsyncFn>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, AsyncFn> = HashMap::new();

    // Market & Native Balance Tools
    m.insert("get_price", Arc::new(|args| Box::pin(get_price_wrapper(args))));
    m.insert("get_marketcap", Arc::new(|args| Box::pin(get_marketcap_wrapper(args))));
    m.insert("get_native_balance", Arc::new(|args| Box::pin(get_native_balance_wrapper(args))));
    m.insert("get_system_wallet", Arc::new(|args| Box::pin(get_system_wallet_wrapper(args))));

    // Mainnet Token & NFT Getters
    m.insert("get_token_details_mainnet", Arc::new(|args| Box::pin(get_token_details_mainnet_wrapper(args))));
    m.insert("get_token_balance_mainnet", Arc::new(|args| Box::pin(get_token_balance_mainnet_wrapper(args))));
    m.insert("get_nft_balance_mainnet", Arc::new(|args| Box::pin(get_nft_balance_mainnet_wrapper(args))));
    m.insert("get_nft_details_mainnet", Arc::new(|args| Box::pin(get_nft_details_mainnet_wrapper(args))));
    m.insert("get_nft_total_supply_mainnet", Arc::new(|args| Box::pin(get_nft_total_supply_mainnet_wrapper(args))));

    // Testnet Token & NFT Getters
    m.insert("get_token_details_testnet", Arc::new(|args| Box::pin(get_token_details_testnet_wrapper(args))));
    m.insert("get_token_balance_testnet", Arc::new(|args| Box::pin(get_token_balance_testnet_wrapper(args))));
    m.insert("get_nft_balance_testnet", Arc::new(|args| Box::pin(get_nft_balance_testnet_wrapper(args))));
    m.insert("get_nft_details_testnet", Arc::new(|args| Box::pin(get_nft_details_testnet_wrapper(args))));
    m.insert("get_nft_total_supply_testnet", Arc::new(|args| Box::pin(get_nft_total_supply_testnet_wrapper(args))));

    // Chain Control Tools
    m.insert("switch_chain", Arc::new(|args| Box::pin(switch_chain_wrapper(args))));

    // Signed Token Setters
    m.insert("transfer_token_mainnet", Arc::new(|args| Box::pin(transfer_token_mainnet_wrapper(args))));
    m.insert("transfer_token_testnet", Arc::new(|args| Box::pin(transfer_token_testnet_wrapper(args))));
    m.insert("approve_token_mainnet", Arc::new(|args| Box::pin(approve_token_mainnet_wrapper(args))));
    m.insert("approve_token_testnet", Arc::new(|args| Box::pin(approve_token_testnet_wrapper(args))));

    // Signed NFT Setters
    m.insert("transfer_nft_mainnet", Arc::new(|args| Box::pin(transfer_nft_mainnet_wrapper(args))));
    m.insert("transfer_nft_testnet", Arc::new(|args| Box::pin(transfer_nft_testnet_wrapper(args))));
    m.insert("approve_nft_mainnet", Arc::new(|args| Box::pin(approve_nft_mainnet_wrapper(args))));
    m.insert("approve_nft_testnet", Arc::new(|args| Box::pin(approve_nft_testnet_wrapper(args))));
    m.insert("set_approval_for_all_nft_mainnet", Arc::new(|args| Box::pin(set_approval_for_all_nft_mainnet_wrapper(args))));
    m.insert("set_approval_for_all_nft_testnet", Arc::new(|args| Box::pin(set_approval_for_all_nft_testnet_wrapper(args))));

    // AAVE Protocol Tools
    m.insert(
        "aave_wrapped_token_get_wrappedethaddress_token",
        Arc::new(|args| Box::pin(lending::aave::aave_wrapped_token_get_wrappedethaddress_token(args[0]))),
    );
    m.insert(
        "aave_wrapped_token_get_weth_address",
        Arc::new(|args| Box::pin(lending::aave::aave_wrapped_token_get_weth_address(args[0]))),
    );
    m.insert(
        "aave_wrapped_token_get_pool_address",
        Arc::new(|args| Box::pin(lending::aave::aave_wrapped_token_get_pool_address(args[0]))),
    );
    m.insert(
        "aave_wrapped_token_get_owner",
        Arc::new(|args| Box::pin(lending::aave::aave_wrapped_token_get_owner(args[0]))),
    );

    m
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_map_registration() {
        assert!(TOOLS.contains_key("switch_chain"));
        assert!(TOOLS.contains_key("get_system_wallet"));
        assert!(TOOLS.contains_key("transfer_token_mainnet"));
        assert!(TOOLS.contains_key("transfer_token_testnet"));
        assert!(TOOLS.contains_key("approve_token_mainnet"));
        assert!(TOOLS.contains_key("approve_token_testnet"));
        assert!(TOOLS.contains_key("transfer_nft_mainnet"));
        assert!(TOOLS.contains_key("transfer_nft_testnet"));
        assert!(TOOLS.contains_key("approve_nft_mainnet"));
        assert!(TOOLS.contains_key("approve_nft_testnet"));
        assert!(TOOLS.contains_key("set_approval_for_all_nft_mainnet"));
        assert!(TOOLS.contains_key("set_approval_for_all_nft_testnet"));
    }
}
