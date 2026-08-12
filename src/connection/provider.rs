use alloy::providers::RootProvider;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

use crate::settings::storage;

type ProviderInstance = RootProvider;
type CacheMap = Arc<RwLock<HashMap<String, ProviderInstance>>>;

fn cache() -> &'static CacheMap {
    static CACHE: OnceLock<CacheMap> = OnceLock::new();
    CACHE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Dynamically retrieves or instantiates an `alloy` `RootProvider` for a given RPC URL.
/// Synchronizes connections automatically when the RPC endpoint changes or is requested.
pub async fn init_provider_for_url(rpc_url: &str) -> Result<ProviderInstance, String> {
    {
        let map = cache().read().await;
        if let Some(provider) = map.get(rpc_url) {
            return Ok(provider.clone());
        }
    }

    let url = rpc_url
        .parse()
        .map_err(|e| format!("Invalid RPC URL '{}': {}", rpc_url, e))?;

    let provider = RootProvider::new_http(url);

    let mut map = cache().write().await;
    map.insert(rpc_url.to_string(), provider.clone());
    Ok(provider)
}

/// Chain-agnostic RPC provider loader.
/// Dynamically retrieves the active RPC endpoint from system settings and synchronizes connection state.
pub async fn init_rpc_provider() -> ProviderInstance {
    let settings = storage::load_app_settings()
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            crate::settings::config::ensure_configured()
                .expect("Failed to initialize or load app settings")
        });

    let rpc_url = settings
        .custom_rpc
        .unwrap_or_else(|| settings.default_chain.mainnet.rpc_url.clone());

    init_provider_for_url(&rpc_url)
        .await
        .expect("Failed to initialize active RPC provider")
}

/// Alias for `init_rpc_provider()` to preserve backwards functional structure compatibility.
pub async fn init_blockchain_provider() -> ProviderInstance {
    init_rpc_provider().await
}

/// Backward compatibility alias for legacy service getters.
pub async fn init_mantle_provider() -> ProviderInstance {
    init_rpc_provider().await
}

/// Dynamically initializes and synchronizes the provider for the active chain's Mainnet.
pub async fn init_mainnet_provider() -> ProviderInstance {
    let settings = storage::load_app_settings()
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            crate::settings::config::ensure_configured()
                .expect("Failed to initialize or load app settings")
        });

    let rpc_url = settings
        .custom_rpc
        .unwrap_or_else(|| settings.default_chain.mainnet.rpc_url.clone());

    init_provider_for_url(&rpc_url)
        .await
        .expect("Failed to initialize Mainnet RPC provider")
}

/// Dynamically initializes and synchronizes the provider for the active chain's Testnet.
pub async fn init_testnet_provider() -> ProviderInstance {
    let settings = storage::load_app_settings()
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            crate::settings::config::ensure_configured()
                .expect("Failed to initialize or load app settings")
        });

    let rpc_url = settings.default_chain.testnet.rpc_url;

    init_provider_for_url(&rpc_url)
        .await
        .expect("Failed to initialize Testnet RPC provider")
}

/// Helper to dynamically retrieve active Mainnet chain name and native token symbol.
pub fn get_active_mainnet_info() -> (String, String) {
    if let Ok(Some(settings)) = storage::load_app_settings() {
        (
            settings.default_chain.name,
            settings.default_chain.mainnet.native_token.symbol,
        )
    } else {
        ("Mainnet".to_string(), "ETH".to_string())
    }
}

/// Helper to dynamically retrieve active Testnet label/name and native token symbol.
pub fn get_active_testnet_info() -> (String, String) {
    if let Ok(Some(settings)) = storage::load_app_settings() {
        let label = settings
            .default_chain
            .testnet
            .label
            .clone()
            .unwrap_or_else(|| format!("{} Testnet", settings.default_chain.name));
        (
            label,
            settings.default_chain.testnet.native_token.symbol,
        )
    } else {
        ("Testnet".to_string(), "ETH".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_provider_caching() {
        let rpc_url = "https://rpc.ankr.com/eth";
        let _provider1 = init_provider_for_url(rpc_url).await.unwrap();
        let _provider2 = init_provider_for_url(rpc_url).await.unwrap();

        let map = cache().read().await;
        assert!(map.contains_key(rpc_url));
    }
}
