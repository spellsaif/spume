#![cfg_attr(docsrs, feature(doc_cfg))]

use crate::provider::HttpProvider;
#[cfg(feature = "pubsub")]
use crate::pubsub_provider::PubsubProvider;

mod methods;
pub mod provider;

#[cfg(feature = "pubsub")]
mod pubsub_methods;
#[cfg(feature = "pubsub")]
#[cfg_attr(docsrs, doc(cfg(feature = "pubsub")))]
pub mod pubsub_provider;

pub struct WasmClient {
    provider: HttpProvider,
}

impl WasmClient {
    /// Construct a client.
    ///
    /// - `url` — the JSON-RPC HTTP endpoint (e.g. `https://api.mainnet-beta.solana.com`).
    pub fn new(url: impl ToString) -> Self {
        let provider = HttpProvider::new(url);
        Self { provider }
    }

    /// The endpoint URL this client was constructed with.
    pub fn url(&self) -> &str {
        &self.provider.url
    }
}

#[cfg(feature = "pubsub")]
#[cfg_attr(docsrs, doc(cfg(feature = "pubsub")))]
pub struct WasmPubsubClient {
    provider: PubsubProvider,
}

#[cfg(feature = "pubsub")]
impl WasmPubsubClient {
    /// Construct a PubSub client by opening a WebSocket connection.
    ///
    /// - `url` — the JSON-RPC PubSub WebSocket endpoint
    ///   (e.g. `wss://api.mainnet-beta.solana.com`).
    pub fn connect(
        url: impl ToString,
    ) -> Result<Self, Box<solana_rpc_client_types::request::RpcError>> {
        let provider = PubsubProvider::connect(url)?;
        Ok(Self { provider })
    }

    /// The endpoint URL this client was constructed with.
    pub fn url(&self) -> &str {
        self.provider.url()
    }
}
