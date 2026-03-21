<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. <a href="../node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 15.3: Routing System

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.3: Routing System** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

</div>

---

## Routing System

The routing system organizes endpoints into logical groups and applies appropriate middleware. We explore how routes are structured below.

### Route Organization

We organize routes into four main categories:

1. **Public API Routes** (`/api/v1/*`): Main API endpoints
2. **Admin Routes** (`/api/admin/*`): Admin-only endpoints with authentication
3. **Wallet Routes** (`/api/wallet/*`): Wallet operations with authentication
4. **Web Routes** (`/`): Web UI and documentation

### API Route Structure

We define the main API endpoints in the `create_api_routes()` function in `routes/api.rs`:

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
        .route(
            "/transactions/mempool/{txid}",
            get(transaction::get_mempool_transaction)
        )
        .route(
            "/transactions/address/{address}",
            get(transaction::get_address_transactions)
        )

        // Mining endpoints
        .route("/mining/info", get(mining::get_mining_info))
        .route("/mining/generatetoaddress", post(mining::generate_to_address))
}
```

**Route Patterns**

We define routes in the `create_api_routes()` function using handler functions from the `handlers/` directory:

- **GET routes**: Retrieve data using handlers like `blockchain::get_blockchain_info`, `blockchain::get_blocks`, `transaction::get_transactions`
- **POST routes**: Create resources using handlers like `wallet::create_wallet`, `transaction::send_transaction`, `mining::generate_to_address`
- **Path parameters**: `{hash}`, `{address}`, `{txid}` extract values from the URL and are passed to handlers via Axum's `Path` extractor—see [Routing in Axum](Axum.md#routing) for detailed routing patterns and [Request Extractors in Axum](Axum.md#request-extractors) for path parameter extraction
- **Query parameters**: We handle these through Axum's `Query` extractor in handlers—see [Request Extractors in Axum](Axum.md#request-extractors) for query parameter extraction details

### Admin Routes

We wrap the API routes with authentication middleware in the `create_admin_api_routes()` function in `routes/api.rs`:

```rust
pub fn create_admin_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .nest("/api/admin", create_api_routes())
        .nest("/api/admin", create_monitor_api_routes())
        .layer(axum::middleware::from_fn(require_admin))
}
```

**Nesting Routes**

Axum's `.nest()` method allows us to prefix routes. Here, we nest all API routes from `create_api_routes()` and health check routes from `create_monitor_api_routes()` under `/api/admin`, and the `require_admin` middleware (from `middleware/auth.rs`) ensures only authenticated admin users can access them. See [Routing in Axum](Axum.md#routing) for detailed technical information on route nesting, merging, and organizing complex route hierarchies.

### Wallet-Only Routes

We provide a subset of endpoints for wallet operations in the `create_wallet_only_routes()` function in `routes/api.rs`:

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

Wallet routes use a different authentication key (`BITCOIN_API_WALLET_KEY`) than admin routes. This approach allows us to:
- Give wallet applications limited access (create wallets, send transactions)
- Keep admin operations separate and more secure
- Support different use cases (user wallets vs. administrative tools)

### Health Check Routes

We define health check routes that are public and don't require authentication in the `create_monitor_api_routes()` function in `routes/api.rs`:

```rust
pub fn create_monitor_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
}
```

**Route Functions**

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

**[← Previous: Server Setup](02-Server-Setup.md)** | **[Chapter 15.3: Routing System](03-Routing.md)** | **[Next: Handlers →](04-Handlers.md)**

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers the routing system. Continue to Request Handlers to learn how requests are processed.*
