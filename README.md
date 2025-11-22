# Blockchain Implementation

A production-grade blockchain implementation following Bitcoin Core's architecture, written in Rust with modern UI clients.

> 📖 **For detailed blockchain documentation, see [Bitcoin Implementation README](bitcoin/README.md)**

---

## Project Structure

This workspace contains four main components:

| Component | Description | Documentation |
|-----------|-------------|---------------|
| **`bitcoin/`** | Core blockchain implementation with P2P networking, consensus, and web API | [bitcoin/README.md](bitcoin/README.md) |
| **`bitcoin-api/`** | Shared typed HTTP client library for consuming the blockchain API | See below |
| **`bitcoin-desktop-ui/`** | Admin UI built with Iced (blockchain management, mining, etc.) | - |
| **`bitcoin-wallet-ui/`** | Wallet UI built with Iced (wallet operations, transactions) | - |

---

## Quick Start

### Prerequisites

- Rust 1.70+ (`rustup install stable`)
- Cargo (comes with Rust)

### Building the Workspace

```bash
# Build all workspace members
cargo build --release

# Build specific component
cargo build --release -p bitcoin
cargo build --release -p bitcoin-desktop-ui
cargo build --release -p bitcoin-wallet-ui
```

### Running the Blockchain Node

See the [Bitcoin Implementation README](bitcoin/README.md) for detailed instructions on:
- Creating wallets
- Starting nodes (mining and web server modes)
- Running P2P networks
- Accessing the web API

---

## API Clients and Role-Based Access

The blockchain node exposes a RESTful API that can be consumed by UI clients or other applications. Two Iced UIs are provided as reference implementations.

### Architecture

```
┌─────────────────────┐     ┌─────────────────────┐
│ bitcoin-desktop-ui  │     │ bitcoin-wallet-ui   │
│   (Admin UI)        │     │   (Wallet UI)       │
└──────────┬──────────┘     └──────────┬──────────┘
           │                            │
           └────────────┬───────────────┘
                        │
           ┌────────────▼───────────────┐
           │     bitcoin-api            │
           │  (Shared Client Library)   │
           └────────────┬───────────────┘
                        │
           ┌────────────▼───────────────┐
           │   bitcoin (Node API)       │
           │   http://localhost:8080   │
           └────────────────────────────┘
```

### Client Feature Flags

The `bitcoin-api` crate uses feature flags to control which client surfaces are compiled:

- **`client`**: Enables HTTP client support (reqwest + tokio)
- **`wallet`**: Enables `WalletClient` APIs (create wallet, send transaction)
- **`admin`**: Enables `AdminClient` APIs (blockchain, mining, etc.)
- **`ws`**: Reserved for future websocket client support

**Default features**: `client`, `wallet`, `admin`

### UI Dependencies

- **`bitcoin-desktop-ui`**: Requires `bitcoin-api` with features `client, wallet, admin`
- **`bitcoin-wallet-ui`**: Requires `bitcoin-api` with features `client, wallet`

### Server Authentication

The web server enforces role-based access using an `X-API-Key` header:

| Role | Endpoints | Environment Variable | Default Value |
|------|-----------|---------------------|---------------|
| **Wallet** | `/api/wallet/*` | `BITCOIN_API_WALLET_KEY` | `wallet-secret` |
| **Admin** | `/api/admin/*` (also has wallet access) | `BITCOIN_API_ADMIN_KEY` | `admin-secret` |

Configure keys via environment variables before starting the node:

```bash
export BITCOIN_API_WALLET_KEY=your-wallet-key-here
export BITCOIN_API_ADMIN_KEY=your-admin-key-here
```

### Client Usage Examples

#### Admin Client (bitcoin-desktop-ui)

```rust
use bitcoin_api::{AdminClient, ApiConfig};

let admin = AdminClient::new(ApiConfig {
    base_url: "http://127.0.0.1:8080".into(),
    api_key: Some("your-admin-key".into()),
})?;

// Admin operations
let blockchain_info = admin.get_blockchain_info().await?;
admin.start_mining().await?;
```

#### Wallet Client (bitcoin-wallet-ui)

```rust
use bitcoin_api::{WalletClient, ApiConfig};

let wallet = WalletClient::new(ApiConfig {
    base_url: "http://127.0.0.1:8080".into(),
    api_key: Some("your-wallet-key".into()),
})?;

// Wallet operations
let addresses = wallet.list_addresses().await?;
let balance = wallet.get_balance(&address).await?;
wallet.send_transaction(&tx_request).await?;
```

---

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific component
cargo test -p bitcoin
cargo test -p bitcoin-api
```

### Workspace Commands

```bash
# Format all code
cargo fmt --all

# Lint all code
cargo clippy --all -- -D warnings

# Check all components
cargo check --all
```

---

## Documentation

- **[Bitcoin Implementation](bitcoin/README.md)** - Core blockchain documentation, architecture, API reference
- **[Consensus Documentation](bitcoin/CONSENSUS_DOCUMENTATION.md)** - Detailed consensus algorithm documentation
- **[Package Documentation](bitcoin/PACKAGE_DOCUMENTATION.md)** - Package structure and organization
- **[Transaction Documentation](bitcoin/docs/Transaction.md)** - Transaction system details

