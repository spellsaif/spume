use {
    futures::stream::StreamExt,
    leptos::{mount::mount_to_body, prelude::*, task::spawn_local},
    spume::{WasmClient, WasmPubsubClient},
};

const RPC_URL: &str = "https://api.devnet.solana.com";
const WS_URL: &str = "wss://api.devnet.solana.com";

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let slot: RwSignal<Option<u64>> = RwSignal::new(None);
    let status = RwSignal::new(String::from("connecting…"));
    let version: RwSignal<Option<String>> = RwSignal::new(None);

    // HTTP: fetch node version once.
    spawn_local(async move {
        let result = WasmClient::new(RPC_URL).get_version().await;
        version.set(Some(match result {
            Ok(v) => v.solana_core,
            Err(err) => format!("error: {err}"),
        }));
    });

    // WebSocket: stream the current slot until the connection closes.
    spawn_local(async move {
        let client = match WasmPubsubClient::connect(WS_URL) {
            Ok(client) => client,
            Err(err) => {
                status.set(format!("connect failed: {err}"));
                return;
            }
        };
        let mut sub = match client.slot_subscribe().await {
            Ok(sub) => sub,
            Err(err) => {
                status.set(format!("subscribe failed: {err}"));
                return;
            }
        };
        status.set("subscribed".into());
        while let Some(item) = sub.next().await {
            match item {
                Ok(info) => slot.set(Some(info.slot)),
                Err(err) => status.set(format!("decode error: {err}")),
            }
        }
        status.set("disconnected".into());
    });

    view! {
        <main>
            <h1>"spume × leptos"</h1>
            <div class="card">
                <Row label="endpoint" value=move || WS_URL.to_string() />
                <Row label="status"   value=move || status.get() />
                <Row label="slot"     value=move || slot.get().map(|s| s.to_string()).unwrap_or_else(|| "—".into()) />
                <Row label="version"  value=move || version.get().unwrap_or_else(|| "loading…".into()) />
            </div>
        </main>
    }
}

#[component]
fn Row(label: &'static str, value: impl Fn() -> String + Send + Sync + 'static) -> impl IntoView {
    view! {
        <div class="row">
            <span class="label">{label}</span>
            <span class="value mono">{value}</span>
        </div>
    }
}
