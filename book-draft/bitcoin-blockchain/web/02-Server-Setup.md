<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../README.md)
2. [Chapter 2: Transaction System](../02-Transaction-System.md)
3. **Chapter 3: Web API Architecture** ← *You are here*
   - [Web API Index](README.md) - Overview and navigation
   - [01: Introduction](01-Introduction.md) - Architecture overview
   - [02: Server Setup](02-Server-Setup.md) - Server configuration ← *You are here*
   - [03: Routing](03-Routing.md) - Route definitions
   - [04: Handlers](04-Handlers.md) - Request handlers
   - [05: Middleware](05-Middleware.md) - Middleware layer
   - [06: Data Models](06-Data-Models.md) - Request/response models
   - [07: Error Handling](07-Error-Handling.md) - Error management
   - [08: OpenAPI](08-OpenAPI.md) - API documentation
   - [09: Security](09-Security.md) - Security architecture
   - [10: Best Practices](10-Best-Practices.md) - Design patterns
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

# Chapter 3.2: Server Setup and Configuration

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Chapter 2.2: Transaction System](../02-Transaction-System.md)** | **Chapter 3.2: Server Setup** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

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
   
   Each layer wraps the previous one, creating a request processing pipeline. See [Middleware Layers in Axum](Axum.md#middleware-layers) for detailed technical information on middleware execution order, creating custom middleware, and middleware composition. For comprehensive Tower middleware information, see [Tower Framework Guide](Tower.md). The server runs on Tokio's async runtime - see [Tokio Runtime Guide](../Tokio.md) for details on async operations.

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

    axum::serve(listener, app)
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

This is crucial for production deployments where you need to update the server without dropping active requests. The graceful shutdown is handled by Axum's `with_graceful_shutdown()` method, which integrates with Tokio's signal handling. For more details on Axum server lifecycle and graceful shutdown, see the [Axum Framework Guide](Axum.md).

---

## Navigation

- **[← Previous: Introduction & Architecture Overview](01-Introduction.md)** - Understanding the structure and design principles
- **[Next: Routing System →](03-Routing.md)** - Organizing endpoints and route definitions
- **[Web API Index](README.md)** - Overview and navigation
- **[Axum Framework Guide](Axum.md)** - Detailed Axum feature explanations
- **[Tower Framework Guide](Tower.md)** - Middleware framework details

---

<div align="center">

**📚 [← Previous: Introduction](01-Introduction.md)** | **Chapter 3.2: Server Setup** | **[Next: Routing →](03-Routing.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers server setup and configuration. Continue to [Routing System](03-Routing.md) to learn how endpoints are organized.*
