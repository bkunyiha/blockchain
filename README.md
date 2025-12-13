# Blockchain Implementation

A blockchain implementation following Bitcoin Core's architecture, written in Rust with modern UI clients.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Deployment Options](#deployment-options)
- [Project Structure](#project-structure)
- [API Clients and Authentication](#api-clients-and-authentication)
- [Development](#development)
- [Documentation](#documentation)

---

## Quick Start

### Prerequisites

- **Rust 1.70+** (`rustup install stable`)
- **Cargo** (comes with Rust)
- **Docker & Docker Compose** (for containerized deployment - optional)

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

## Deployment Options

This project supports two deployment methods. All deployment files are organized in the `ci/` directory.

### Docker Compose (Recommended for Development)

**Location**: [`ci/docker-compose/`](ci/docker-compose/)

Ideal for local development, single-host deployments, and quick testing.

**Features:**
- **Multi-instance scaling**: Run multiple miners and webservers with automatic configuration
- **Sequential startup**: Nodes wait for previous nodes to be ready before starting
- **Isolated data**: Each instance maintains its own blockchain data directory
- **Automatic port assignment**: Unique ports for each instance (miners: 2001+, webservers: 8080+/2101+)
- **Health checks**: Built-in health monitoring for reliable startup

**Quick Start:**
```bash
cd ci/docker-compose/configs

# Default: 1 miner + 1 webserver
docker compose up -d

# Scale to multiple instances (all ports accessible)
./docker-compose.scale.sh 3 2  # 3 miners, 2 webservers

# Incremental scaling
./scale-up.sh miner 2
./scale-down.sh webserver 3
```

**Documentation:**
- **Quick Start**: [`ci/docker-compose/README.md`](ci/docker-compose/README.md) - Quick reference guide
- **Complete Guide**: [`book-draft/ci/docker-compose/01-Introduction.md`](book-draft/ci/docker-compose/01-Introduction.md) - Comprehensive documentation (12 sections)

### Kubernetes (Recommended for Production)

**Location**: [`ci/kubernetes/`](ci/kubernetes/)

Ideal for production deployments, multi-node clusters, and automatic scaling.

**Features:**
- **Native autoscaling**: HPA (Horizontal Pod Autoscaler) for automatic scaling
- **Service discovery**: DNS-based service discovery
- **Rolling updates**: Zero-downtime deployments
- **Self-healing**: Automatic pod restart on failure
- **Resource management**: CPU/Memory limits and requests
- **Multi-node support**: Distribute across cluster nodes

**Quick Start:**
```bash
cd ci/kubernetes/manifests
kubectl apply -f .
```

**Documentation:**
- **Quick Start**: [`ci/kubernetes/README.md`](ci/kubernetes/README.md) - Quick reference guide
- **Complete Guide**: [`book-draft/ci/kubernetes/README.md`](book-draft/ci/kubernetes/README.md) - Comprehensive documentation (7 sections)


---

## Project Structure

This workspace contains five main components:

| Component | Description | Documentation |
|-----------|-------------|---------------|
| **`bitcoin/`** | Core blockchain implementation with P2P networking, consensus, and web API | [bitcoin/README.md](bitcoin/README.md) |
| **`bitcoin-api/`** | Shared typed HTTP client library for consuming the blockchain API | See API Clients section |
| **`bitcoin-desktop-ui/`** | Admin UI built with Iced (blockchain management, mining, etc.) | - |
| **`bitcoin-wallet-ui/`** | Wallet UI built with Iced (wallet operations, transactions) | - |
| **`bitcoin-web-ui/`** | Modern React-based web admin interface | [bitcoin-web-ui/README.md](bitcoin-web-ui/README.md) |

---

## API Clients and Authentication

The blockchain node exposes a RESTful API that can be consumed by UI clients or other applications.

### Architecture

```
┌─────────────────────┐     ┌─────────────────────┐     ┌────────────────────┐
│ bitcoin-desktop-ui  │     │ bitcoin-wallet-ui   │     │   bitcoin-web-ui   │
│   (Admin UI)        │     │   (Wallet UI)       │     │   (Web Admin UI)   │
│   (Iced/Rust)       │     │   (Iced/Rust)       │     │   (React/TS)       │
└──────────┬──────────┘     └──────────┬──────────┘     └──────────┬─────────┘
           │                           │                           │
           └────────────┬──────────────┴───-───────────────────────┘
                        │
           ┌────────────▼───────────────┐
           │     bitcoin-api            │
           │  (Shared Client Library)   │
           │  (Rust HTTP Client)        │
           └────────────┬───────────────┘
                        │
           ┌────────────▼───────────────┐
           │   bitcoin (Blockchain Node)│
           │   http://localhost:8080    │
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

### Complete Book Documentation

For comprehensive technical documentation covering all aspects of the blockchain implementation, see the **[Complete Book Documentation](book-draft/README.md)**.

**Book Structure:**

**Part I: Core Blockchain Implementation** (Chapters 1-7)
- [Chapter 1: Introduction & Overview](book-draft/01-Introduction.md) - Book introduction, project structure, technical stack
- [Chapter 2: Introduction to Bitcoin & Blockchain](book-draft/bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
- [Chapter 2.1: Cryptography](book-draft/bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
- [Chapter 2.2: Transaction System](book-draft/bitcoin-blockchain/primitives/02-Transaction-ID-Format.md) - Transaction ID format guide
- [Chapter 2.3: Blockchain State Management](book-draft/bitcoin-blockchain/chain/README.md) - Chain state and UTXO management
- [Chapter 2.4: Network Layer](book-draft/bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
- [Chapter 2.5: Node Orchestration](book-draft/bitcoin-blockchain/node/README.md) - Node context and coordination
- [Chapter 2.6: Primitives](book-draft/bitcoin-blockchain/primitives/README.md) - Core data structures
- [Chapter 2.7: Storage Layer](book-draft/bitcoin-blockchain/store/README.md) - Persistent storage implementation
- [Chapter 2.8: Utilities](book-draft/bitcoin-blockchain/util/README.md) - Utility functions and helpers
- [Chapter 2.9: Wallet System](book-draft/bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
- [Chapter 3: Web API Architecture](book-draft/bitcoin-blockchain/web/README.md) - REST API implementation (10 sections)
  - [Introduction & Architecture Overview](book-draft/bitcoin-blockchain/web/01-Introduction.md)
  - [Server Setup and Configuration](book-draft/bitcoin-blockchain/web/02-Server-Setup.md)
  - [Routing System](book-draft/bitcoin-blockchain/web/03-Routing.md)
  - [Request Handlers](book-draft/bitcoin-blockchain/web/04-Handlers.md)
  - [Middleware Layer](book-draft/bitcoin-blockchain/web/05-Middleware.md)
  - [Data Models](book-draft/bitcoin-blockchain/web/06-Data-Models.md)
  - [Error Handling](book-draft/bitcoin-blockchain/web/07-Error-Handling.md)
  - [OpenAPI Documentation](book-draft/bitcoin-blockchain/web/08-OpenAPI.md)
  - [Security Architecture](book-draft/bitcoin-blockchain/web/09-Security.md)
  - [Best Practices and Patterns](book-draft/bitcoin-blockchain/web/10-Best-Practices.md)
  - [Axum Framework Guide](book-draft/bitcoin-blockchain/web/Axum.md)
  - [Tower Framework Guide](book-draft/bitcoin-blockchain/web/Tower.md)
  - [Serde Framework Guide](book-draft/bitcoin-blockchain/web/Serde.md)
  - [Utoipa Framework Guide](book-draft/bitcoin-blockchain/web/Utoipa.md)
  - [Tracing Framework Guide](book-draft/bitcoin-blockchain/web/Tracing.md)
  - [Tokio Runtime Guide](book-draft/bitcoin-blockchain/Tokio.md)
  - [Rust Language Guide](book-draft/rust/README.md) - Comprehensive guide to Rust language features
- [Chapter 4: Desktop Admin Interface](book-draft/bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
- [Chapter 5: Wallet User Interface](book-draft/bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
- [Chapter 6: Embedded Database & Persistence](book-draft/bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
- [Chapter 7: Web Admin Interface](book-draft/bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

**Part II: Deployment & Operations** (Chapters 8-9)
- [Chapter 8: Docker Compose Deployment](book-draft/ci/docker-compose/01-Introduction.md) - Complete Docker Compose guide (12 sections)
- [Chapter 9: Kubernetes Deployment](book-draft/ci/kubernetes/README.md) - Kubernetes production guide (7 sections)

**Part III: Language Reference** (Chapter 10)
- [Chapter 10: Rust Language Guide](book-draft/rust/README.md) - Comprehensive Rust language reference (17 sections)

The book provides detailed technical explanations, code walkthroughs, architecture diagrams, and deployment guides suitable for developers, DevOps engineers, and technical professionals.

---

## License


## Contributing

