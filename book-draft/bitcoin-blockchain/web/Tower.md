<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Tower Framework Guide

**Part I: Foundations & Core Implementation** | **Technical Reference: Tower Middleware Framework**

<div align="center">

**[← Chapter 24: Web API Architecture](README.md)** | **Tower Framework Guide** | **[Next: Chapter 25: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

</div>

---

## Overview

This guide provides detailed explanations of the Tower middleware framework and how it's used in our blockchain web API. Tower is a modular library of reusable components for building robust network services and clients. It provides a foundation for middleware, service composition, and request/response handling that Axum builds upon.

Tower's design philosophy centers around composability and reusability. Middleware components can be combined in various ways to create powerful request processing pipelines. In our blockchain API, we use Tower (via `tower_http`) for CORS, compression, logging, and static file serving.

> **See the full implementation:**: This guide explains Tower concepts. To see how these features are used together in our complete web API architecture, see the Web API Architecture chapter and the Middleware Layer chapter.

---

## Table of Contents

1. [What is Tower?](#what-is-tower) - Understanding Tower's role
2. [Tower Services and Layers](#tower-services-and-layers) - Core Tower concepts
3. [Tower HTTP](#tower-http) - HTTP-specific middleware
4. [CORS Middleware](#cors-middleware) - Cross-origin resource sharing
5. [Compression Middleware](#compression-middleware) - Response compression
6. [Logging Middleware](#logging-middleware) - Request/response logging
7. [Static File Serving](#static-file-serving) - Serving static assets
8. [Layer Composition](#layer-composition) - Combining middleware
9. [Custom Middleware](#custom-middleware) - Building your own layers

---

## What is Tower?

Tower is a library of modular and reusable components for building robust clients and servers. It provides abstractions for:

- **Services**: Async functions that take a request and return a response
- **Layers**: Middleware that wraps services to add functionality
- **Utilities**: Common patterns like retries, timeouts, and load balancing

### Tower's Relationship to Axum

Axum is built on top of Tower, which means:
- Axum routers are Tower services
- Axum middleware uses Tower layers
- Tower's composability enables Axum's flexible middleware system

In our blockchain API, we use Tower primarily through `tower_http`, which provides HTTP-specific middleware components.

### Why Tower?

- **Composability**: Middleware can be combined in any order
- **Reusability**: Write middleware once, use it anywhere
- **Type Safety**: Rust's type system ensures correct composition
- **Performance**: Zero-cost abstractions with minimal overhead
- **Ecosystem**: Large collection of pre-built middleware

---

## Tower Services and Layers

### Services

A Tower **Service** is an async function that takes a request and returns a response. In Axum, handlers are services:

```rust
// A Tower Service signature
async fn service(request: Request) -> Result<Response, Error>
```

### Layers

A Tower **Layer** wraps a service to add functionality. Layers are composable and can be stacked:

```rust
// Layer wraps a service
Layer::new(service) -> WrappedService
```

### Layer Trait

The `Layer` trait wraps services to add functionality. Example:

```rust
pub fn create_rate_limit_layer(
    _config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    tower::layer::util::Identity::new()  // Pass-through layer
}
```

---

## Tower HTTP

`tower_http` provides HTTP-specific middleware built on Tower. We use several `tower_http` components in our blockchain API.

### Tower HTTP Components We Use

1. **CORS Layer** (`tower_http::cors::CorsLayer`) - Cross-origin resource sharing
2. **Compression Layer** (`tower_http::compression::CompressionLayer`) - Response compression
3. **Trace Layer** (`tower_http::trace::TraceLayer`) - Request/response logging
4. **ServeDir** (`tower_http::services::ServeDir`) - Static file serving

---

## CORS Middleware

CORS (Cross-Origin Resource Sharing) middleware allows web browsers to make requests from different origins. We use Tower HTTP's `CorsLayer` for this.

### Implementation in `middleware/cors.rs`

The `create_cors_layer()` function in `middleware/cors.rs` creates a CORS layer:

```rust
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400)) // 24 hours
}
```

**How CORS Layer Works:**

1. **`CorsLayer::new()`**: Creates a new CORS layer
2. **`.allow_origin(Any)`**: Allows requests from any origin (development only)
3. **`.allow_methods(Any)`**: Allows all HTTP methods (GET, POST, PUT, DELETE, etc.)
4. **`.allow_headers(Any)`**: Allows all request headers
5. **`.expose_headers(Any)`**: Exposes all response headers to JavaScript
6. **`.max_age(...)`**: Sets how long browsers can cache preflight requests

### Production CORS

For production, we use `create_cors_layer_with_origins()` to restrict origins:

```rust
pub fn create_cors_layer_with_origins(origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400));

    for origin in origins {
        if let Ok(parsed_origin) = origin.parse::<axum::http::HeaderValue>() {
            cors = cors.allow_origin(parsed_origin);
        }
    }

    cors
}
```

**How It Works:**

- Iterates through allowed origins
- Parses each origin as an HTTP header value
- Adds each origin to the CORS configuration
- Returns a configured CORS layer

CORS layers automatically handle preflight OPTIONS requests from browsers.

> **See it in action:**: Check out the [CORS Middleware section](05-Middleware.md#cors-middleware) in the Middleware Layer chapter for more details on how CORS is configured and used.

---

## Compression Middleware

Compression middleware automatically compresses response bodies, reducing bandwidth and improving performance. We use Tower HTTP's `CompressionLayer`.

### Implementation in `server.rs`

The compression layer is added in the `create_app()` method in `server.rs`:

```rust
use tower_http::compression::CompressionLayer;

pub fn create_app(
    &self,
) -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())
        .with_state(self.node.clone());

    // Add compression middleware
    app = app.layer(CompressionLayer::new());

    // ... other middleware
    Ok(app)
}
```

**How Compression Works:**

1. **Request Processing**: Handler generates response
2. **Compression Check**: CompressionLayer checks if response should be compressed
3. **Content-Type Filtering**: Only compresses certain content types (text, JSON, etc.)
4. **Compression**: Uses gzip or brotli based on client support
5. **Headers**: Sets `Content-Encoding` header appropriately

Compression benefits: reduced bandwidth, faster transfers, automatic, content-aware. Supports gzip, brotli, deflate with automatic negotiation via `Accept-Encoding` header.

> **See it in action:**: Check out the [Server Setup](02-Server-Setup.md#creating-the-application-router) chapter to see how compression is configured alongside other middleware.

---

## Logging Middleware

Logging middleware records request and response information for debugging and monitoring. We use Tower HTTP's `TraceLayer`.

### Implementation in `middleware/logging.rs`

The `create_logging_layer()` function in `middleware/logging.rs` creates a logging layer:

```rust
use tower_http::trace::TraceLayer;

pub fn create_logging_layer() -> impl tower::Layer<axum::Router> + Clone {
    TraceLayer::new_for_http()
}
```

**How TraceLayer Works:**

1. **Request Logging**: Logs incoming requests with method, path, headers
2. **Response Logging**: Logs responses with status code, headers, duration
3. **Error Logging**: Logs errors with full context
4. **Integration**: Works with `tracing` crate for structured logging

### TraceLayer Configuration

Customize with callbacks:

```rust
TraceLayer::new_for_http()
    .make_span_with(|req: &Request| /* span */)
    .on_request(|req: &Request, _: &Span| /* log */)
    .on_response(|res: &Response, latency: Duration, _: &Span| /* log */)
```

**Options:** `make_span_with`, `on_request`, `on_response`, `on_failure`

TraceLayer respects logging levels: ERROR, WARN, INFO, DEBUG, TRACE

> **See it in action:**: Check out the Middleware Layer chapter for more details on logging middleware.

---

## Static File Serving

Static file serving allows the web server to serve static assets like HTML, CSS, JavaScript, and images. We use Tower HTTP's `ServeDir` service.

### Implementation in `routes/web.rs`

The `create_web_routes()` function in `routes/web.rs` uses `ServeDir` to serve the React app:

```rust
use tower_http::services::ServeDir;

pub fn create_web_routes() -> Router<Arc<NodeContext>> {
    let react_app_path = /* ... find React app path ... */;

    if let Some(path) = react_app_path {
        let assets_path = format!("{}/assets", path);

        Router::new()
            .nest_service("/assets", ServeDir::new(&assets_path))
            // ... other routes
    }
}
```

**How ServeDir Works:**

1. **Path Resolution**: Maps URL paths to file system paths
2. **File Serving**: Serves files with appropriate content types
3. **Directory Listing**: Can optionally list directory contents
4. **Error Handling**: Returns 404 for missing files

ServeDir features: Content-Type detection, range requests, caching headers, fallback files (useful for SPAs).

### Serving React App Assets

In our implementation, we serve React app assets like this:

```rust
.nest_service("/assets", ServeDir::new(&assets_path))
```

This serves files from the `assets` directory at the `/assets` URL path. The React build process creates hashed filenames for cache busting, and `ServeDir` serves them correctly.

---

## Layer Composition

One of Tower's strengths is the ability to compose multiple layers. Layers are applied in order, with each layer wrapping the previous one.

### Layer Order Matters

Order affects request processing:

```rust
// CORS (outermost)
app = app.layer(cors_layer());
// Compression
app = app.layer(CompressionLayer::new());
// Error handling (innermost)
app = app.layer(axum::middleware::from_fn(handle_errors));
```

Request flow: CORS → Compression → Error Handling → Router → Handler
Response flow: Handler → Error Handling → Compression → CORS → Client

Combine multiple layers with `Stack`:

```rust
tower::layer::util::Stack::new(
    TraceLayer::new_for_http(),
    CompressionLayer::new(),
)
```

---

## Custom Middleware

While we use pre-built Tower HTTP middleware, you can create custom middleware using Tower's `Layer` trait.

### Creating a Custom Layer

Implement `Layer` and `Service` traits:

```rust
impl<S> Layer<S> for CustomMiddlewareLayer {
    type Service = CustomMiddlewareService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        CustomMiddlewareService { inner }
    }
}

impl<S> Service<Request> for CustomMiddlewareService<S> {
    // Pre-processing, call inner service, post-processing
}
```

### Rate Limiting Example

Rate limiting placeholder in `middleware/rate_limit.rs`:

```rust
pub fn create_rate_limit_layer(_config: RateLimitConfig)
    -> impl tower::Layer<axum::Router> + Clone {
    tower::layer::util::Identity::new()
}
```

When implemented: track requests, enforce limits, return 429 when exceeded.

---

## Tower Utilities

- **Identity Layer**: Pass-through layer for placeholders
- **Stack Layer**: Combine multiple layers
- **ServiceBuilder**: Fluent API for layer composition

---

## Best Practices

1. **Layer Order**: CORS outer, compression middle, error handling inner
2. **Conditional Middleware**: Use config flags to enable/disable
3. **Reusable Functions**: Create layer functions for reusability
4. **Type Annotations**: Use explicit type annotations for clarity

---

## Summary

Tower provides the foundation for middleware in our blockchain API:

- **Services**: Async functions that process requests
- **Layers**: Composable middleware components
- **Tower HTTP**: HTTP-specific middleware (CORS, compression, logging, static files)
- **Composition**: Layers can be combined in any order
- **Type Safety**: Rust's type system ensures correct composition

Tower's modular design allows us to build a robust middleware stack that handles CORS, compression, logging, and static file serving efficiently and safely.

---

## Additional Resources

- **[Tower Documentation](https://docs.rs/tower/)**: Official Tower crate documentation
- **[Tower HTTP Documentation](https://docs.rs/tower-http/)**: HTTP-specific middleware components
- **[Tracing Documentation](https://docs.rs/tracing/)**: Official tracing crate documentation for structured logging
- **[Tracing Subscriber Documentation](https://docs.rs/tracing-subscriber/)**: Tracing subscriber for configuring log output format and filtering
- **[Tracing Book](https://tokio.rs/tokio/topics/tracing)**: Comprehensive guide to using tracing in Rust applications
- **Axum Framework Guide**: How Axum uses Tower
- **Serde Framework Guide**: Serialization framework used alongside Tower
- **Utoipa Framework Guide**: OpenAPI framework that integrates with Tower middleware
- **Tokio Runtime Guide**: Async runtime that Tower runs on
- **Rust Language Guide**: Rust language features used throughout
- **Middleware Layer**: How we use Tower middleware in our API
- **Server Setup**: How middleware is configured in our server

---

<div align="center">

**[← Web API Index](README.md)** | **Tower Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **Axum** | **Serde** | **Utoipa** | **Tracing** | **Tokio**

</div>

---

*This guide provides detailed explanations of Tower framework features used in our blockchain API. For implementation details, see the Middleware Layer chapter.*
