# Blockchain Implementation

A production-grade blockchain implementation following Bitcoin Core's architecture, written in Rust with modern UI clients.

> 📖 **For detailed blockchain documentation, see [Bitcoin Implementation README](bitcoin/README.md)**

---

## Project Structure

This workspace contains five main components:

| Component | Description | Documentation |
|-----------|-------------|---------------|
| **`bitcoin/`** | Core blockchain implementation with P2P networking, consensus, and web API | [bitcoin/README.md](bitcoin/README.md) |
| **`bitcoin-api/`** | Shared typed HTTP client library for consuming the blockchain API | See below |
| **`bitcoin-desktop-ui/`** | Admin UI built with Iced (blockchain management, mining, etc.) | - |
| **`bitcoin-wallet-ui/`** | Wallet UI built with Iced (wallet operations, transactions) | - |
| **`bitcoin-web-ui/`** | Modern React-based web admin interface (blockchain management, wallet operations, mining) | [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md) |

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

### Accessing the Web UI

The blockchain node includes a modern React-based web interface:

1. **Build the web UI** (first time only):
   ```bash
   cd bitcoin-web-ui
   npm install
   npm run build
   ```

2. **Start the blockchain server**:
   ```bash
   cargo run --release -p bitcoin
   ```

3. **Access the web UI**:
   - Production: `http://localhost:8080` (served by Rust server)
   - Development: `http://localhost:3000` (Vite dev server, requires running `npm run dev` in `bitcoin-web-ui/`)

4. **Configure API Key** (development mode):
   - Click "Configure API" in the navbar
   - Enter API key: `admin-secret` (default) or your `BITCOIN_API_ADMIN_KEY` value
   - Base URL: `http://127.0.0.1:8080` (default)

For detailed web UI documentation, see [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md).

---

## API Clients and Role-Based Access

The blockchain node exposes a RESTful API that can be consumed by UI clients or other applications. Two Iced UIs are provided as reference implementations.

### Architecture

```
┌─────────────────────┐     ┌─────────────────────┐     ┌─────────────────────┐
│ bitcoin-desktop-ui  │     │ bitcoin-wallet-ui   │     │   bitcoin-web-ui    │
│   (Admin UI)        │     │   (Wallet UI)       │     │   (Web Admin UI)   │
│   (Iced/Rust)      │     │   (Iced/Rust)       │     │   (React/TS)       │
└──────────┬──────────┘     └──────────┬──────────┘     └──────────┬──────────┘
           │                            │                            │
           └────────────┬───────────────┴────────────────────────────┘
                        │
           ┌────────────▼───────────────┐
           │     bitcoin-api            │
           │  (Shared Client Library)   │
           │  (Rust HTTP Client)        │
           └────────────┬───────────────┘
                        │
           ┌────────────▼───────────────┐
           │   bitcoin (Node API)       │
           │   http://localhost:8080   │
           │   REST API + Web UI        │
           └────────────────────────────┘
```

**Note**: The `bitcoin-web-ui` uses Axios directly (not `bitcoin-api`) and communicates with the Rust server's REST API endpoints.

### Client Feature Flags

The `bitcoin-api` crate uses feature flags to control which client surfaces are compiled:

- **`client`**: Enables HTTP client support (reqwest + tokio)
- **`wallet`**: Enables `WalletClient` APIs (create wallet, send transaction)
- **`admin`**: Enables `AdminClient` APIs (blockchain, mining, etc.)
- **`ws`**: Reserved for future websocket client support

**Default features**: `client`, `wallet`, `admin`

### UI Dependencies

- **`bitcoin-desktop-ui`**: Requires `bitcoin-api` with features `client, wallet, admin` (Iced/Rust)
- **`bitcoin-wallet-ui`**: Requires `bitcoin-api` with features `client, wallet` (Iced/Rust)
- **`bitcoin-web-ui`**: Uses Axios directly, no Rust dependencies (React/TypeScript)

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

#### Web UI (bitcoin-web-ui)

The web UI is a React application that provides a browser-based interface:

**Features:**
- Dashboard with real-time blockchain statistics
- Blockchain management (view blocks, search by hash)
- Wallet operations (create, view info, check balance, send transactions)
- Transaction management (mempool, transaction history)
- Mining controls (view info, generate blocks)
- Health monitoring

**Access:**
- After building (`npm run build`), the web UI is served automatically by the Rust server at `http://localhost:8080`
- For development, run `npm run dev` in `bitcoin-web-ui/` to access at `http://localhost:3000`

**API Configuration:**
- Configure API key via the UI's "Configure API" button in the navbar
- Default admin key: `admin-secret` (or `BITCOIN_API_ADMIN_KEY` env var)
- API key is stored in browser localStorage

See [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md) for detailed setup instructions.

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
- **[Web UI Documentation](bitcoin-web-ui/README.md)** - React web interface setup and usage
- **[Web UI Technical Documentation](book-draft/bitcoin-web-ui/Web-UI.md)** - Detailed technical documentation for the web UI architecture

