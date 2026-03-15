<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. **Chapter 3: Web API Architecture** ← *You are here*
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 3.3: Routing System

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**[📚 ← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.3: Routing System** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Routing System

The routing system organizes endpoints into logical groups and applies appropriate middleware. Let's explore how routes are structured.

### Route Organization

Routes are organized into three main categories:

1. **Public API Routes** (`/api/v1/*`): Main API endpoints
2. **Admin Routes** (`/api/admin/*`): Admin-only endpoints with authentication
3. **Wallet Routes** (`/api/wallet/*`): Wallet operations with authentication
4. **Web Routes** (`/`): Web UI and documentation

### API Route Structure

The `create_api_routes()` function in `routes/api.rs` defines the main API endpoints:

```rust
pub fn create_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        // Blockchain endpoints
        .route("/blockchain", get(blockchain::get_blockchain_info))
        .route("/blockchain/blocks", get(blockchain::get_blocks))
        .route("/blockchain/blocks/latest", get(blockchain::get_latest_blocks))
        .route("/blockchain/blocks/{hash}", get(blockchain::get_block_by_hash))
        
        // Wallet endpoints
        .route("/wallet", post(wallet::create_wallet))
        .route("/wallet/addresses", get(wallet::get_addresses))
        .route("/wallet/{address}", get(wallet::get_wallet_info))
        .route("/wallet/{address}/balance", get(wallet::get_balance))
        
        // Transaction endpoints
        .route("/transactions", post(transaction::send_transaction))
        .route("/transactions", get(transaction::get_transactions))
        .route("/transactions/mempool", get(transaction::get_mempool))
        .route("/transactions/mempool/{txid}", get(transaction::get_mempool_transaction))
        .route("/transactions/address/{address}", get(transaction::get_address_transactions))
        
        // Mining endpoints
        .route("/mining/info", get(mining::get_mining_info))
        .route("/mining/generatetoaddress", post(mining::generate_to_address))
}
```

**Route Patterns:**

The `create_api_routes()` function in `routes/api.rs` defines routes using handler functions from the `handlers/` directory:

- **GET routes**: Retrieve data using handlers like `blockchain::get_blockchain_info`, `blockchain::get_blocks`, `transaction::get_transactions`
- **POST routes**: Create resources using handlers like `wallet::create_wallet`, `transaction::send_transaction`, `mining::generate_to_address`
- **Path parameters**: `{hash}`, `{address}`, `{txid}` extract values from the URL and are passed to handlers via Axum's `Path` extractor - see [Routing in Axum](Axum.md#routing) for detailed routing patterns and [Request Extractors in Axum](Axum.md#request-extractors) for path parameter extraction
- **Query parameters**: Handled through Axum's `Query` extractor in handlers - see [Request Extractors in Axum](Axum.md#request-extractors) for query parameter extraction details

### Admin Routes

The `create_admin_api_routes()` function in `routes/api.rs` wraps the API routes with authentication middleware:

```rust
pub fn create_admin_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .nest("/api/admin", create_api_routes())
        .nest("/api/admin", create_monitor_api_routes())
        .layer(axum::middleware::from_fn(require_admin))
}
```

**Nesting Routes:**

The `.nest()` method (from Axum's Router) allows us to prefix routes. Here, all API routes from `create_api_routes()` and health check routes from `create_monitor_api_routes()` are nested under `/api/admin`, and the `require_admin` middleware (from `middleware/auth.rs`) ensures only authenticated admin users can access them. See [Routing in Axum](Axum.md#routing) for detailed technical information on route nesting, merging, and organizing complex route hierarchies.

### Wallet-Only Routes

The `create_wallet_only_routes()` function in `routes/api.rs` provides a subset of endpoints for wallet operations:

```rust
pub fn create_wallet_only_routes() -> Router<Arc<NodeContext>> {
    let wallet_only = Router::new()
        .route("/wallet", post(wallet::create_wallet))
        .route("/transactions", post(transaction::send_transaction));

    Router::new()
        .nest("/api/wallet", wallet_only)
        .layer(axum::middleware::from_fn(require_wallet))
}
```

**Why Separate Wallet Routes?**

Wallet routes use a different authentication key (`BITCOIN_API_WALLET_KEY`) than admin routes. This allows us to:
- Give wallet applications limited access (create wallets, send transactions)
- Keep admin operations separate and more secure
- Support different use cases (user wallets vs. administrative tools)

### Health Check Routes

The `create_monitor_api_routes()` function in `routes/api.rs` defines health check routes that are public and don't require authentication:

```rust
pub fn create_monitor_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
}
```

**Route Functions:**

- `health::health_check` (from `handlers/health.rs`): Comprehensive health check
- `health::liveness` (from `handlers/health.rs`): Simple liveness probe
- `health::readiness` (from `handlers/health.rs`): Readiness probe

These endpoints are essential for:
- **Container orchestration**: Kubernetes and Docker use these for health checks
- **Monitoring**: External monitoring systems can check service health
- **Load balancers**: Can route traffic based on readiness status

---

## Navigation

- **← Previous: Server Setup** - Server initialization and configuration
- **Next: Request Handlers →** - Processing requests and implementing business logic
- **Web API Index** - Overview and navigation
- **Axum Framework Guide** - Detailed Axum feature explanations
- **Tower Framework Guide** - Middleware framework details

---

<div align="center">

**[📚 ← Previous: Server Setup](02-Server-Setup.md)** | **[Chapter 3.3: Routing System](03-Routing.md)** | **[Next: Handlers →](04-Handlers.md)** 📚

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers the routing system. Continue to Request Handlers to learn how requests are processed.*
