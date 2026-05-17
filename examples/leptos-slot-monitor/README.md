# leptos-slot-monitor

Browser app showing [`spume`](../..) in a [Leptos](https://leptos.dev) CSR setting.

- Streams the current Solana **devnet** slot via `WasmPubsubClient::slot_subscribe`.
- Fetches the node software version once via `WasmClient::get_version`.

## Run

```bash
cargo install trunk    # first time only
trunk serve --open
```

Trunk builds the wasm bundle and serves it at `http://127.0.0.1:8080`.