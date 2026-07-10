use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::transports::BoxTransport;
use tokio::sync::OnceCell;

static MANTLE_PROVIDER: OnceCell<RootProvider<BoxTransport>> = OnceCell::const_new();

pub async fn init_mantle_provider() -> &'static RootProvider<BoxTransport> {
    MANTLE_PROVIDER
        .get_or_init(|| async {
            let rpc_url =
                crate::settings::rpc::get_or_set_endpoint().expect("Failed to get RPC Endpoint");

            ProviderBuilder::new()
                .on_builtin(rpc_url.as_str())
                .await
                .expect("RPC init failed")
        })
        .await
}
