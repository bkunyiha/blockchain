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
   - [03: Routing](03-Routing.md) - Route definitions
   - [04: Handlers](04-Handlers.md) - Request handlers
   - [05: Middleware](05-Middleware.md) - Middleware layer
   - [06: Data Models](06-Data-Models.md) - Request/response models
   - [07: Error Handling](07-Error-Handling.md) - Error management
   - [08: OpenAPI](08-OpenAPI.md) - API documentation
   - [09: Security](09-Security.md) - Security architecture
   - [10: Best Practices](10-Best-Practices.md) - Design patterns
   - [Axum Framework Guide](Axum.md) - Framework reference
   - [Tower Framework Guide](Tower.md) - Middleware framework ← *You are here*
   - [Serde Framework Guide](Serde.md) - Serialization framework
   - [Utoipa Framework Guide](Utoipa.md) - OpenAPI framework
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

# Tower Framework Guide

**Part I: Core Blockchain Implementation** | **Technical Reference: Tower Middleware Framework**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Tower Framework Guide** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Overview

This guide provides detailed explanations of the Tower middleware framework and how it's used in our blockchain web API. Tower is a modular library of reusable components for building robust network services and clients. It provides a foundation for middleware, service composition, and request/response handling that Axum builds upon.

Tower's design philosophy centers around composability and reusability. Middleware components can be combined in various ways to create powerful request processing pipelines. In our blockchain API, we use Tower (via `tower_http`) for CORS, compression, logging, and static file serving.

> **📘 See the full implementation**: This guide explains Tower concepts. To see how these features are used together in our complete web API architecture, see the [Web API Architecture chapter](01-Introduction.md) and the [Middleware Layer](05-Middleware.md) chapter.

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

The `Layer` trait in Tower allows you to wrap services:

```rust
pub trait Layer<S> {
    type Service: Service<Request>;
    
    fn layer(&self, inner: S) -> Self::Service;
}
```

**Example from our implementation:**

In `middleware/rate_limit.rs`, we use Tower's `Layer` trait:

```rust
use tower::Layer;

pub fn create_rate_limit_layer(
    _config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    // Returns a no-op layer (placeholder for future implementation)
    tower::layer::util::Identity::new()
}
```

The `Identity` layer is a Tower utility that passes requests through unchanged—useful for placeholders or conditional middleware.

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

### CORS Preflight Requests

CORS layers automatically handle preflight OPTIONS requests:
- Browser sends OPTIONS request before actual request
- CORS layer responds with appropriate headers
- Browser then sends the actual request if allowed

> **See it in action**: Check out the [CORS Middleware section](05-Middleware.md#cors-middleware) in the Middleware Layer chapter for more details on how CORS is configured and used.

---

## Compression Middleware

Compression middleware automatically compresses response bodies, reducing bandwidth and improving performance. We use Tower HTTP's `CompressionLayer`.

### Implementation in `server.rs`

The compression layer is added in the `create_app()` method in `server.rs`:

```rust
use tower_http::compression::CompressionLayer;

pub fn create_app(&self) -> Router {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())
        .with_state(self.node.clone());

    // Add compression middleware
    app = app.layer(CompressionLayer::new());
    
    // ... other middleware
    app
}
```

**How Compression Works:**

1. **Request Processing**: Handler generates response
2. **Compression Check**: CompressionLayer checks if response should be compressed
3. **Content-Type Filtering**: Only compresses certain content types (text, JSON, etc.)
4. **Compression**: Uses gzip or brotli based on client support
5. **Headers**: Sets `Content-Encoding` header appropriately

### Compression Benefits

- **Reduced Bandwidth**: Smaller responses use less network bandwidth
- **Faster Transfers**: Less data to transfer means faster response times
- **Automatic**: Works transparently without handler changes
- **Content-Aware**: Only compresses compressible content types

### Compression Algorithms

`CompressionLayer` supports multiple algorithms:
- **gzip**: Widely supported, good compression ratio
- **brotli**: Better compression, newer browsers
- **deflate**: Legacy support

The layer automatically negotiates with the client via `Accept-Encoding` header.

> **See it in action**: Check out the [Server Setup](02-Server-Setup.md#creating-the-application-router) chapter to see how compression is configured alongside other middleware.

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

`TraceLayer::new_for_http()` provides sensible defaults, but you can customize:

```rust
TraceLayer::new_for_http()
    .make_span_with(|request: &Request| {
        tracing::info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
        )
    })
    .on_request(|request: &Request, _span: &Span| {
        tracing::info!("Started processing request: {} {}", request.method(), request.uri())
    })
    .on_response(|response: &Response, latency: Duration, _span: &Span| {
        tracing::info!(
            "Finished processing request: {} (took {:?})",
            response.status(),
            latency
        )
    })
```

**Customization Options:**

- **`make_span_with()`**: Customize span creation
- **`on_request()`**: Callback when request starts
- **`on_response()`**: Callback when response is ready
- **`on_failure()`**: Callback for errors

### Logging Levels

TraceLayer respects Rust's logging levels:
- **ERROR**: Errors and failures
- **WARN**: Warnings
- **INFO**: Normal request/response logging
- **DEBUG**: Detailed debugging information
- **TRACE**: Very detailed tracing

> **See it in action**: Check out the [Middleware Layer](05-Middleware.md) chapter for more details on logging middleware.

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

### ServeDir Features

- **Content-Type Detection**: Automatically sets correct `Content-Type` headers
- **Range Requests**: Supports HTTP range requests for partial content
- **Caching Headers**: Can set cache control headers
- **Fallback**: Can serve a fallback file (useful for SPAs)

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

The order in which layers are applied affects how requests are processed:

```rust
// In server.rs - create_app() method
let mut app = Router::new()
    .merge(create_all_api_routes())
    .with_state(self.node.clone());

// CORS layer (outermost - handles preflight requests first)
if self.config.enable_cors {
    app = app.layer(cors::create_cors_layer());
}

// Compression layer (compresses responses)
app = app.layer(CompressionLayer::new());

// Error handling layer (catches and formats errors)
app = app.layer(axum::middleware::from_fn(handle_errors));
```

**Request Processing Order:**

1. **CORS Layer**: Handles preflight OPTIONS requests, adds CORS headers
2. **Compression Layer**: Compresses response bodies
3. **Error Handling Layer**: Catches errors and formats responses
4. **Router**: Routes requests to handlers
5. **Handler**: Processes the request

**Response Processing Order (reversed):**

1. **Handler**: Generates response
2. **Error Handling Layer**: Formats errors if any
3. **Compression Layer**: Compresses response body
4. **CORS Layer**: Adds CORS headers
5. **Client**: Receives response

### Composing Custom Layers

You can create custom layers that combine multiple Tower components:

```rust
use tower::Layer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub fn create_production_middleware() -> impl Layer<Router> + Clone {
    // Combine multiple layers
    tower::layer::util::Stack::new(
        TraceLayer::new_for_http(),
        CompressionLayer::new(),
    )
}
```

---

## Custom Middleware

While we use pre-built Tower HTTP middleware, you can create custom middleware using Tower's `Layer` trait.

### Creating a Custom Layer

Here's how you would create a custom middleware layer:

```rust
use tower::{Layer, Service};
use std::future::Future;
use std::pin::Pin;

pub struct CustomMiddlewareLayer;

impl<S> Layer<S> for CustomMiddlewareLayer
where
    S: Service<Request>,
{
    type Service = CustomMiddlewareService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CustomMiddlewareService { inner }
    }
}

pub struct CustomMiddlewareService<S> {
    inner: S,
}

impl<S, Request> Service<Request> for CustomMiddlewareService<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Pre-processing
        println!("Before request: {:?}", request);
        
        // Call inner service
        let future = self.inner.call(request);
        
        // Post-processing
        Box::pin(async move {
            let response = future.await?;
            println!("After response: {:?}", response);
            Ok(response)
        })
    }
}
```

### Rate Limiting Example

In `middleware/rate_limit.rs`, we have a placeholder for rate limiting:

```rust
pub fn create_rate_limit_layer(
    _config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    // Placeholder - returns no-op layer
    tower::layer::util::Identity::new()
}
```

When implemented, this would use Tower's rate limiting capabilities to:
- Track requests per IP or API key
- Enforce rate limits
- Return `429 Too Many Requests` when limits are exceeded

---

## Tower Utilities

Tower provides several utility layers and services:

### Identity Layer

The `Identity` layer passes requests through unchanged:

```rust
use tower::layer::util::Identity;

let layer = Identity::new();
```

We use this as a placeholder for unimplemented middleware.

### Stack Layer

The `Stack` layer combines multiple layers:

```rust
use tower::layer::util::Stack;

let combined = Stack::new(layer1, layer2);
```

### ServiceBuilder

`ServiceBuilder` provides a fluent API for composing layers:

```rust
use tower::ServiceBuilder;

let service = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .service(router);
```

---

## Best Practices

### 1. Layer Order

Order layers from outermost to innermost based on when you want them to process requests:
- CORS should be outermost (handles preflight first)
- Compression should be before error handling (compress before formatting)
- Error handling should be innermost (catches handler errors)

### 2. Conditional Middleware

Use conditional logic to enable/disable middleware:

```rust
let mut app = Router::new()/* ... */;

if self.config.enable_cors {
    app = app.layer(cors::create_cors_layer());
}
```

### 3. Reusable Layer Functions

Create functions that return layers for reusability:

```rust
pub fn create_cors_layer() -> CorsLayer { /* ... */ }
pub fn create_logging_layer() -> impl Layer<Router> + Clone { /* ... */ }
```

### 4. Type Annotations

Use explicit type annotations for clarity:

```rust
pub fn create_rate_limit_layer(
    _config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    tower::layer::util::Identity::new()
}
```

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
- **[Axum Framework Guide](Axum.md)**: How Axum uses Tower
- **[Serde Framework Guide](Serde.md)**: Serialization framework used alongside Tower
- **[Utoipa Framework Guide](Utoipa.md)**: OpenAPI framework that integrates with Tower middleware
- **[Tokio Runtime Guide](../Tokio.md)**: Async runtime that Tower runs on
- **[Rust Language Guide](../../rust/README.md)**: Rust language features used throughout
- **[Middleware Layer](05-Middleware.md)**: How we use Tower middleware in our API
- **[Server Setup](02-Server-Setup.md)**: How middleware is configured in our server

---

<div align="center">

**📚 [← Web API Index](README.md)** | **Tower Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **[Axum](Axum.md)** | **[Serde](Serde.md)** | **[Utoipa](Utoipa.md)** | **[Tracing](Tracing.md)** | **[Tokio](../Tokio.md)** 📚

</div>

---

*This guide provides detailed explanations of Tower framework features used in our blockchain API. For implementation details, see the [Middleware Layer](05-Middleware.md) chapter.*
