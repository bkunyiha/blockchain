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

# Chapter 15.2: Server Setup and Configuration

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.2: Server Setup** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)** 

</div>

---

## Server Setup and Configuration

The web server is the entry point for all HTTP requests. Let's explore how we structure and configure it.

### WebServer Structure

We encapsulate the server configuration and node context in the `WebServer` struct:

```rust
pub struct WebServer {
    config: WebServerConfig,
    node: Arc<NodeContext>,
}
```

The server holds two key components:
- **Configuration**: Server settings like host, port, CORS, and rate limiting
- **Node Context**: A shared reference to the blockchain node, allowing handlers to access blockchain data

### Configuration

We define all configurable aspects of the server in the `WebServerConfig` struct in `server.rs`:

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

**Default Configuration**

We provide sensible defaults through the `Default` implementation for `WebServerConfig`:

- **Host**: `0.0.0.0` (listens on all interfaces)
- **Port**: `8080` (standard HTTP port)
- **CORS**: Enabled (allows cross-origin requests)
- **Rate Limiting**: Enabled with 10 requests per second and a burst size of 20

These defaults work well for development. In production, we configure them based on specific needs.

### Creating the Application Router

We build the complete application router by combining routes and middleware in the `create_app()` method (part of the `WebServer` struct in `server.rs`):

```rust
pub fn create_app(
    &self,
) -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())
        .with_state(self.node.clone());

    // Add basic middleware
    let mut app = app;

    // Add rate limiting middleware
    if self.config.enable_rate_limiting {
        let rl_config = RateLimitConfig::default();
        if let Some(manager) = build_rate_limiter_manager(&rl_config)? {
            app = app.layer(from_fn_with_state(
                manager,
                axum_rate_limiter::limiter::middleware,
            ));
        }
    }

    // Add CORS middleware
    if self.config.enable_cors {
        app = app.layer(cors::create_cors_layer());
    }

    // Add compression middleware
    app = app.layer(CompressionLayer::new());

    // Add error handling middleware
    app = app.layer(axum::middleware::from_fn(handle_errors));

    Ok(app)
}
```

**Understanding Router Construction**

We build the router in several steps:

1. **Route Merging**: We merge three sets of routes:
   - `create_all_api_routes()` (from `routes/api.rs`): Main API endpoints including admin routes and health checks
   - `create_wallet_only_routes()` (from `routes/api.rs`): Wallet-specific endpoints with authentication
   - `create_web_routes()` (from `routes/web.rs`): Web UI serving and Swagger documentation

2. **State Injection**: We use `.with_state(self.node.clone())` to make the `NodeContext` available to all handlers through Axum's state extraction. See [State Injection in Axum](Axum.md#state-injection) for detailed technical information on how state injection works, type requirements, and best practices.

3. **Middleware Layers**: We apply middleware in order using `.layer()`:
   - CORS layer (from `middleware/cors.rs`) if enabled—see [CORS Configuration in Axum](Axum.md#cors-configuration) for detailed CORS setup and production configuration
   - Compression layer (from `tower_http`)—see [Compression in Axum](Axum.md#compression) for compression details and [Compression Middleware in Tower](Tower.md#compression-middleware) for Tower-specific implementation
   - Error handling middleware (`handle_errors()` function in `server.rs`)—see [Error Handling in Axum](Axum.md#error-handling) for error handling patterns and middleware

   Each layer wraps the previous one, creating a request processing pipeline. See [Middleware Layers in Axum](Axum.md#middleware-layers) for detailed technical information on middleware execution order, creating custom middleware, and middleware composition. For comprehensive Tower middleware information, see Tower Framework Guide. The server runs on Tokio's async runtime—see Tokio Runtime Guide for details on async operations.

### Starting the Server

We start the server with graceful shutdown support in the `start_with_shutdown()` method (part of the `WebServer` struct in `server.rs`):

```rust
pub async fn start_with_shutdown(
    &self,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = self.create_app()?;

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

    // `axum_rate_limiter` relies on `ConnectInfo<SocketAddr>` to determine the
    // client IP.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await?;

    Ok(())
}
```

**Graceful Shutdown**

The server listens for `CTRL+C` signals and shuts down gracefully. This means:
- In-flight requests are allowed to complete
- New connections are rejected
- The server exits cleanly

This is crucial for production deployments where we need to update the server without dropping active requests. Axum's `with_graceful_shutdown()` method handles graceful shutdown, integrating with Tokio's signal handling. For more details on Axum server lifecycle and graceful shutdown, see the Axum Framework Guide.

---

## Navigation

- **← Previous: Introduction & Architecture Overview** - Understanding the structure and design principles
- **Next: Routing System →** - Organizing endpoints and route definitions
- **Web API Index** - Overview and navigation
- **Axum Framework Guide** - Detailed Axum feature explanations
- **Tower Framework Guide** - Middleware framework details

---

<div align="center">

**[← Previous: Introduction](01-Introduction.md)** | **[Chapter 15.2: Server Setup](02-Server-Setup.md)** | **[Next: Routing →](03-Routing.md)** 

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers server setup and configuration. Continue to Routing System to learn how endpoints are organized.*
