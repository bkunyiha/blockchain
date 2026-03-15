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

# Chapter 3.2: Server Setup and Configuration

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**[📚 ← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.2: Server Setup** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Server Setup and Configuration

The web server is the entry point for all HTTP requests. Let's understand how it's structured and configured.

### WebServer Structure

The `WebServer` struct encapsulates the server configuration and the node context:

```rust
pub struct WebServer {
    config: WebServerConfig,
    node: Arc<NodeContext>,
}
```

The server holds:
- **Configuration**: Server settings like host, port, CORS, and rate limiting
- **Node Context**: A shared reference to the blockchain node, allowing handlers to access blockchain data

### Configuration

The `WebServerConfig` struct in `server.rs` defines all configurable aspects of the server:

```rust
#[derive(Debug, Clone)]
pub struct WebServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub enable_rate_limiting: bool,
    pub rate_limit_requests_per_second: u32,
    pub rate_limit_burst_size: u32,
}
```

**Default Configuration:**

The `Default` implementation for `WebServerConfig` provides sensible defaults:

- **Host**: `0.0.0.0` (listens on all interfaces)
- **Port**: `8080` (standard HTTP port)
- **CORS**: Enabled (allows cross-origin requests)
- **Rate Limiting**: Enabled with 10 requests per second and a burst size of 20

These defaults work well for development, but in production, you'll want to configure them based on your specific needs.

### Creating the Application Router

The `create_app()` method in `server.rs` (part of the `WebServer` struct) builds the complete application router by combining routes and middleware:

```rust
pub fn create_app(&self) -> Router {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())
        .with_state(self.node.clone());

    // Add middleware layers
    let mut app = app;

    // CORS middleware
    if self.config.enable_cors {
        app = app.layer(cors::create_cors_layer());
    }

    // Compression middleware
    app = app.layer(CompressionLayer::new());

    // Error handling middleware
    app = app.layer(axum::middleware::from_fn(handle_errors));

    app
}
```

**Understanding the Router Construction:**

The `create_app()` method builds the router in several steps:

1. **Route Merging**: We merge three sets of routes:
   - `create_all_api_routes()` (from `routes/api.rs`): Main API endpoints including admin routes and health checks
   - `create_wallet_only_routes()` (from `routes/api.rs`): Wallet-specific endpoints with authentication
   - `create_web_routes()` (from `routes/web.rs`): Web UI serving and Swagger documentation

2. **State Injection**: `.with_state(self.node.clone())` makes the `NodeContext` available to all handlers through Axum's state extraction. See [State Injection in Axum](Axum.md#state-injection) for detailed technical information on how state injection works, type requirements, and best practices.

3. **Middleware Layers**: Middleware is applied in order using `.layer()`:
   - CORS layer (from `middleware/cors.rs`) if enabled - see [CORS Configuration in Axum](Axum.md#cors-configuration) for detailed CORS setup and production configuration
   - Compression layer (from `tower_http`) - see [Compression in Axum](Axum.md#compression) for compression details and [Compression Middleware in Tower](Tower.md#compression-middleware) for Tower-specific implementation
   - Error handling middleware (`handle_errors()` function in `server.rs`) - see [Error Handling in Axum](Axum.md#error-handling) for error handling patterns and middleware
   
   Each layer wraps the previous one, creating a request processing pipeline. See [Middleware Layers in Axum](Axum.md#middleware-layers) for detailed technical information on middleware execution order, creating custom middleware, and middleware composition. For comprehensive Tower middleware information, see Tower Framework Guide. The server runs on Tokio's async runtime - see Tokio Runtime Guide for details on async operations.

### Starting the Server

The `start_with_shutdown()` method in `server.rs` (part of the `WebServer` struct) starts the server with graceful shutdown support:

```rust
pub async fn start_with_shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
    let app = self.create_app();
    let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));

    tracing::info!("Starting web server on {} with graceful shutdown", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Handle shutdown signal
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        tracing::info!("Shutdown signal received");
    };

    // NOTE: Some middleware (e.g. rate limiting) needs to know the client socket address.
    // Axum provides this via the `ConnectInfo<SocketAddr>` extractor, but only if we start the
    // server with `into_make_service_with_connect_info::<SocketAddr>()`.
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    Ok(())
}
```

**Graceful Shutdown:**

The server listens for `CTRL+C` signals and shuts down gracefully. This means:
- In-flight requests are allowed to complete
- New connections are rejected
- The server exits cleanly

This is crucial for production deployments where you need to update the server without dropping active requests. The graceful shutdown is handled by Axum's `with_graceful_shutdown()` method, which integrates with Tokio's signal handling. For more details on Axum server lifecycle and graceful shutdown, see the Axum Framework Guide.

---

## Navigation

- **← Previous: Introduction & Architecture Overview** - Understanding the structure and design principles
- **Next: Routing System →** - Organizing endpoints and route definitions
- **Web API Index** - Overview and navigation
- **Axum Framework Guide** - Detailed Axum feature explanations
- **Tower Framework Guide** - Middleware framework details

---

<div align="center">

**[📚 ← Previous: Introduction](01-Introduction.md)** | **[Chapter 3.2: Server Setup](02-Server-Setup.md)** | **[Next: Routing →](03-Routing.md)** 📚

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers server setup and configuration. Continue to Routing System to learn how endpoints are organized.*
