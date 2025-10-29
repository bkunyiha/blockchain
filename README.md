#
## API clients and role-based access

Two Iced UIs consume the `bitcoin` web API via a shared crate `bitcoin-api`:

- `bitcoin-desktop-ui`: Admin UI. Depends on `bitcoin-api` with features `client, wallet, admin`.
- `bitcoin-wallet-ui`: Wallet UI. Depends on `bitcoin-api` with features `client, wallet`.

### Server auth

The web server enforces roles using an `X-API-Key` header:

- Wallet-only endpoints (nested under `/api/wallet`) require a key matching `BITCOIN_API_WALLET_KEY` (default `wallet-secret`).
- Admin endpoints (nested under `/api/admin`) require a key matching `BITCOIN_API_ADMIN_KEY` (default `admin-secret`). Admin keys can also access wallet endpoints.

Configure keys via environment variables before starting the node:

```
export BITCOIN_API_WALLET_KEY=some-wallet-key
export BITCOIN_API_ADMIN_KEY=some-admin-key
```

### Client usage (example)

```
// bitcoin-desktop-ui
let admin = bitcoin_api::AdminClient::new(bitcoin_api::ApiConfig {
    base_url: "http://127.0.0.1:8080".into(),
    api_key: Some("some-admin-key".into()),
})?;

// bitcoin-wallet-ui
let wallet = bitcoin_api::WalletClient::new(bitcoin_api::ApiConfig {
    base_url: "http://127.0.0.1:8080".into(),
    api_key: Some("some-wallet-key".into()),
})?;
```

