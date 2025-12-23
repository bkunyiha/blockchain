<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../README.md)
2. [Chapter 2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)
3. **Chapter 3: Web API Architecture** ← *You are here*
   - [Web API Index](README.md) - Overview and navigation
   - [01: Introduction](01-Introduction.md) - Architecture overview
   - [02: Server Setup](02-Server-Setup.md) - Server configuration
   - [03: Routing](03-Routing.md) - Route definitions ← *You are here*
   - [04: Handlers](04-Handlers.md) - Request handlers
   - [05: Middleware](05-Middleware.md) - Middleware layer
   - [06: Data Models](06-Data-Models.md) - Request/response models
   - [07: Error Handling](07-Error-Handling.md) - Error management
   - [08: OpenAPI](09-OpenAPI.md) - API documentation
   - [09: Security](10-Security.md) - Security architecture
   - [10: Best Practices](11-Best-Practices.md) - Design patterns
   - [Axum Framework Guide](Axum.md) - Framework reference
4. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
5. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md)
6. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md)
7. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

8. [Chapter 8: Docker Compose Deployment](../../ci/docker-compose/01-Introduction.md)
9. [Chapter 9: Kubernetes Deployment](../../ci/kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 3.3: Routing System

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.3: Routing System** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

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

- **[← Previous: Server Setup](02-Server-Setup.md)** - Server initialization and configuration
- **[Next: Request Handlers →](04-Handlers.md)** - Processing requests and implementing business logic
- **[Web API Index](README.md)** - Overview and navigation
- **[Axum Framework Guide](Axum.md)** - Detailed Axum feature explanations
- **[Tower Framework Guide](Tower.md)** - Middleware framework details

---

<div align="center">

**📚 [← Previous: Server Setup](02-Server-Setup.md)** | **Chapter 3.3: Routing System** | **[Next: Handlers →](04-Handlers.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers the routing system. Continue to [Request Handlers](04-Handlers.md) to learn how requests are processed.*
