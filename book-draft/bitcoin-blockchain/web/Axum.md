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
# Axum Framework Guide

**Part I: Foundations & Core Implementation** | **Technical Reference: Axum Framework**

<div align="center">

**[← Web Index](README.md)** | **Axum Framework Guide** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

</div>

---

## Overview

This guide provides detailed explanations of Axum framework features used in our blockchain web API. Axum is a modern, ergonomic web framework built on top of Tokio and Tower, designed to leverage Rust's type system for building fast, reliable web services.

Throughout this guide, we'll explore the specific Axum features used in our implementation, with code examples from our blockchain API to illustrate each concept.

> **See the full implementation:**: This guide explains the Axum concepts. To see how these features are used together in our complete web API architecture, see the Web API Architecture chapter, which covers server setup, routing, handlers, middleware, and more.

---

## Table of Contents

1. [State Injection](#state-injection) - Share data across handlers
2. [Middleware Layers](#middleware-layers) - Cross-cutting concerns
   - [Tower Layers and Services](#tower-layers-and-services) - Tower foundation for middleware
3. [Request Extractors](#request-extractors) - Extract data from requests
4. [Routing](#routing) - Organize endpoints
5. [Async/Await](#asyncawait) - Handle concurrency
6. [Error Handling](#error-handling) - Manage errors gracefully
7. [CORS Configuration](#cors-configuration) - Enable cross-origin requests
8. [Compression](#compression) - Reduce response sizes
9. [Response Types](#response-types) - Return various response formats
   - [Understanding IntoResponse](#understanding-intoresponse) - The trait that enables flexible handler return types

> **Tip:**: Throughout this guide, you'll find links to specific sections in the Web API Architecture chapter where these Axum features are actually implemented. Use these links to see real-world examples and understand how theory translates to practice.

---

## State Injection

State injection allows you to share data across all handlers in your application. In Axum, state is injected using `.with_state()` and extracted in handlers using the `State` extractor.

### How State Injection Works

State injection makes data available to all handlers without global variables, with compile-time type safety.

```rust
// Create router with state
let app = Router::new()
    .merge(routes)
    .with_state(self.node.clone());

// Extract in handlers
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    let height = node.get_blockchain_height().await?;
    Ok(Json(ApiResponse::success(response)))
}
```

> **See it in action:**: Check out the [Blockchain Handlers section](04-Handlers.md#blockchain-handlers) in the Web API Architecture chapter to see how state extraction is used in real handlers like `get_blockchain_info()` and `get_block_by_hash()`.

**Key Concepts:**
1. Type must match between `.with_state()` and `State()` extractor
2. Use `Arc<T>` for shared ownership without cloning
3. State is typically immutable; use interior mutability if needed

**Benefits:** No globals, compile-time type safety, easy testing, explicit architecture

> **Implementation example:**: See how we inject `NodeContext` as state in the [Server Setup and Configuration section](02-Server-Setup.md#server-setup-and-configuration), and how handlers extract it in the [Request Handlers section](04-Handlers.md#request-handlers).

State type must implement `Clone` (use `Arc<T>` for shared ownership).

---

## Middleware Layers

Middleware in Axum provides cross-cutting concerns like authentication, logging, CORS, and error handling. Middleware is applied using `.layer()` and processes requests in order.

### Middleware Execution Order

Middleware layers wrap each other in order. Layer order matters:

```rust
let mut app = Router::new().with_state(self.node.clone());

// 1. CORS (outermost - handles preflight first)
app = app.layer(cors_layer());

// 2. Compression
app = app.layer(CompressionLayer::new());

// 3. Error handling (innermost - catches handler errors)
app = app.layer(axum::middleware::from_fn(handle_errors));
```

Request flow: CORS → Compression → Error Handler → Route Handler → Response

> **See the full implementation:**: The exact middleware order used in our blockchain API is shown in the [Creating the Application Router section](02-Server-Setup.md#creating-the-application-router) of the Web API Architecture chapter.

### Creating Custom Middleware

Middleware follows a specific signature:

```rust
async fn my_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Pre-processing
    let response = next.run(request).await;
    // Post-processing
    Ok(response)
}

// Apply with from_fn
.layer(axum::middleware::from_fn(my_middleware))
```

> **Implementation example:**: See the complete `handle_errors()` middleware implementation in the [Error Handling Middleware section](05-Middleware.md#error-handling-middleware), and authentication middleware in the [Authentication Middleware section](05-Middleware.md#authentication-middleware).

### Tower Layers and Services

Axum is built on top of Tower, which provides the foundation for middleware composition. Understanding Tower's concepts helps you build more effective middleware.

#### What is Tower?

Tower is a library of modular components for building robust clients and servers. It provides:
- **Services**: Async functions that take requests and return responses
- **Layers**: Middleware that wraps services to add functionality
- **Composability**: Middleware can be stacked and combined in any order

#### Axum's Relationship to Tower

- **Axum Routers are Tower Services**: Every router is a service that Axum routes requests through
- **Axum Middleware uses Tower Layers**: Middleware is implemented as Tower layers
- **Seamless Integration**: Tower's design enables Axum's flexible middleware system

#### Pre-built Middleware from tower-http

Tower HTTP provides HTTP-specific middleware components we use in our API:

```rust
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;

.layer(CorsLayer::new().allow_origin(Any))
.layer(CompressionLayer::new())
.layer(TraceLayer::new_for_http())
.nest_service("/assets", ServeDir::new("/path/to/assets"))
```

**Key Tower HTTP Components:**
- **CorsLayer**: Cross-origin resource sharing (handled by Tower)
- **CompressionLayer**: Automatic response compression (gzip, brotli, deflate)
- **TraceLayer**: HTTP request/response logging integrated with Tracing
- **ServeDir**: Static file serving with appropriate content types

#### Layer Composition Order

Layers wrap each other, so order matters. Request flows through layers in reverse order:

```rust
// Application order
app = app.layer(cors_layer());              // 1st (outermost)
app = app.layer(CompressionLayer::new());   // 2nd
app = app.layer(error_handling_layer());    // 3rd (innermost)

// Request flow (left → right)
Request → CORS → Compression → Error Handler → Router → Handler
Response ← CORS ← Compression ← Error Handler ← Handler
```

#### ServiceBuilder for Fluent Composition

Tower provides `ServiceBuilder` for composing multiple layers:

```rust
use tower::ServiceBuilder;

let layer = ServiceBuilder::new()
    .layer(CorsLayer::new())
    .layer(CompressionLayer::new())
    .layer(TraceLayer::new_for_http())
    .into_inner();

app = app.layer(layer);
```

---

## Request Extractors

Extractors pull data from HTTP requests and make it available to handlers. Axum provides many built-in extractors and allows you to create custom ones.

### Common Extractors

#### State Extractor

Extracts application state:

```rust
pub async fn handler(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<T>>, StatusCode> {
    // Use node here
}
```

#### Path Extractor

```rust
// Route: "/blockchain/blocks/{hash}"
pub async fn get_block_by_hash(
    State(node): State<Arc<NodeContext>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    let block = node.get_block_by_hash(&hash).await?;
    Ok(Json(ApiResponse::success(block)))
}
```

> **See it in action:**: The `get_block_by_hash()` handler is fully implemented in the [Blockchain Handlers section](04-Handlers.md#blockchain-handlers). Also see how we use type-safe path extraction with `WalletAddress` in the [Wallet Handlers section](04-Handlers.md#wallet-handlers).

**Type-Safe Path Extraction:**

Axum deserializes path parameters into custom types:

```rust
pub async fn get_balance(
    State(node): State<Arc<NodeContext>>,
    Path(address): Path<WalletAddress>,  // Custom type
) -> Result<Json<ApiResponse<BalanceResponse>>, StatusCode> {
    let balance = node.get_balance(&address).await?;
    Ok(Json(ApiResponse::success(balance)))
}
```

#### Query Extractor

```rust
#[derive(serde::Deserialize)]
struct BlockQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

pub async fn get_blocks(
    State(node): State<Arc<NodeContext>>,
    Query(params): Query<BlockQuery>,
) -> Result<Json<ApiResponse<Vec<BlockResponse>>>, StatusCode> {
    let page = params.page.unwrap_or(0);
    Ok(Json(ApiResponse::success(blocks)))
}
```

#### Json Extractor

```rust
pub async fn create_wallet(
    State(_node): State<Arc<NodeContext>>,
    Json(request): Json<CreateWalletRequest>,
) -> Result<Json<ApiResponse<WalletResponse>>, StatusCode> {
    // request is deserialized
    Ok(Json(ApiResponse::success(response)))
}
```

> **Implementation example:**: See `create_wallet()` in the [Wallet Handlers section](04-Handlers.md#wallet-handlers) and `send_transaction()` in the [Transaction Handlers section](04-Handlers.md#transaction-handlers) for real-world JSON extraction examples.

### Multiple Extractors

Combine multiple extractors in one handler:

```rust
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
    // Path(id): Path<String>, Query(params): Query<...>
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    // All extractors available
    Ok(Json(ApiResponse::success(response)))
}
```

> **Implementation example:**: The `send_transaction()` handler uses both `State` and `Json` extractors. See the complete implementation in the [Transaction Handlers section](04-Handlers.md#transaction-handlers).

Extractors are processed in order; `State` listed first by convention. Failures (invalid JSON, etc.) automatically return appropriate error responses.

---

## Routing

Axum's routing system allows you to organize endpoints logically and apply middleware selectively.

### Basic Routes

```rust
use axum::{Router, routing::{get, post}};

Router::new()
    .route("/blockchain", get(get_blockchain_info))
    .route("/wallet", post(create_wallet))
```

### Route Methods

- `get(handler)` - GET requests
- `post(handler)` - POST requests
- `put(handler)` - PUT requests
- `delete(handler)` - DELETE requests
- `patch(handler)` - PATCH requests

### Path Parameters

Use `{name}` in paths to capture parameters:

```rust
.route("/blockchain/blocks/{hash}", get(get_block_by_hash))
.route("/wallet/{address}/balance", get(get_balance))
```

### Nesting Routes

Use `.nest()` to prefix route groups:

```rust
Router::new()
    .nest("/api/admin", create_api_routes())
    .nest("/api/admin", create_monitor_api_routes())
    .layer(axum::middleware::from_fn(require_admin))
```

Results in routes like `/api/admin/blockchain`, `/api/admin/health`

> **See it in action:**: Our complete routing organization, including nested admin routes and wallet routes, is explained in the [Routing System section](03-Routing.md#routing-system) of the Web API Architecture chapter.

### Merging Routes

Use `.merge()` to combine routers without prefix:

```rust
Router::new()
    .merge(create_all_api_routes())
    .merge(create_wallet_only_routes())
    .merge(create_web_routes())
```

> **Implementation example:**: See how we merge multiple route groups in the [Creating the Application Router section](02-Server-Setup.md#creating-the-application-router), and explore the full route structure in the [API Route Structure section](03-Routing.md#api-route-structure).

### Route-Specific Middleware

Apply middleware to specific routes:

```rust
Router::new()
    .route("/public", get(public_handler))
    .route("/admin", get(admin_handler))
    // Only applies to routes above
    .layer(axum::middleware::from_fn(require_admin))
```

### Fallback Routes

Handle 404s with a fallback:

```rust
Router::new()
    .route("/api/*", get(api_handler))
    .fallback(fallback_handler)  // Handles all unmatched routes
```

---

## Async/Await

Axum is built on Tokio and is async-first. All handlers are async functions.

### Async Handler Pattern

```rust
pub async fn handler_name(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<T>>, StatusCode> {
    let result = node.get_blockchain_height().await?;
    let block = node.blockchain().get_last_block().await?;
    Ok(Json(ApiResponse::success(response)))
}
```

> **Implementation example:**: All our handlers follow this async pattern. See the [Handler Pattern section](04-Handlers.md#handler-pattern) and explore specific implementations in [Blockchain Handlers](04-Handlers.md#blockchain-handlers), [Wallet Handlers](04-Handlers.md#wallet-handlers), and [Transaction Handlers](04-Handlers.md#transaction-handlers).

**Why Async:**
- Handle many requests simultaneously without blocking
- Non-blocking I/O for efficiency
- Thousands of concurrent connections per thread

**Best Practices:**
1. Use `.await` for I/O operations
2. Use `?` for error handling
3. Avoid blocking; use `tokio::task::spawn_blocking` if needed

### Concurrent Operations

```rust
use futures::future::join_all;

pub async fn get_multiple_blocks(
    State(node): State<Arc<NodeContext>>,
    Path(hashes): Path<Vec<String>>,
) -> Result<Json<ApiResponse<Vec<BlockResponse>>>, StatusCode> {
    let futures = hashes.iter().map(|h| node.get_block_by_hash(h));
    let blocks = join_all(futures).await;
    Ok(Json(ApiResponse::success(blocks)))
}
```

---

## Error Handling

Axum provides flexible error handling through return types and middleware.

### Handler Error Types

Handlers can return errors in several ways:

```rust
// Return StatusCode directly
pub async fn handler() -> Result<Json<ApiResponse<T>>, StatusCode> {
    if some_condition {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(Json(ApiResponse::success(data)))
}

// Return custom error type (must implement IntoResponse)
pub async fn handler() -> Result<Json<ApiResponse<T>>, MyError> {
    // ...
}
```

### Error Conversion

Use `.map_err()` to convert errors:

```rust
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    let height = node.get_blockchain_height().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  // Convert error

    // ...
}
```

### Error Middleware

Error middleware catches and formats errors:

```rust
// In server.rs
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;

    // Check if response indicates an error
    let is_error = response.status().is_server_error()
        || response.status().is_client_error();
    if is_error {
        // Log error
        tracing::error!("Error: {}", response.status());

        // Format internal server errors
        if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
            let error_response = ErrorResponse {
                error: "Internal Server Error".to_string(),
                message: "An unexpected error occurred".to_string(),
                status_code: 500,
                timestamp: chrono::Utc::now(),
            };

            return Ok(Json(ApiResponse::<()>::error(
                serde_json::to_string(&error_response).unwrap()
            )).into_response());
        }
    }

    Ok(response)
}
```

> **Implementation example:**: See the full `handle_errors()` implementation and error handling patterns in the [Error Handling Middleware section](05-Middleware.md#error-handling-middleware) and [Error Handling section](07-Error-Handling.md#error-handling) of the Web API Architecture chapter.

### Common Error Patterns

**Not Found:**

```rust
match block {
    Some(block) => Ok(Json(ApiResponse::success(response))),
    None => Err(StatusCode::NOT_FOUND),
}
```

**Validation Error:**

```rust
if !request.validate().is_ok() {
    return Err(StatusCode::BAD_REQUEST);
}
```

**Internal Server Error:**

```rust
let result = operation().await
    .map_err(|e| {
        error!("Operation failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
```

> **Implementation example:**: These patterns are used throughout our handlers. See the [Common Error Patterns section](07-Error-Handling.md#common-error-patterns) for more examples and the [Error Flow section](07-Error-Handling.md#error-flow) for understanding how errors propagate.

---

## CORS Configuration

CORS (Cross-Origin Resource Sharing) allows web browsers to make requests from different origins.

### Basic CORS Setup

```rust
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)                    // Allow all origins (dev only)
        .allow_methods(Any)                   // Allow all HTTP methods
        .allow_headers(Any)                   // Allow all headers
        .expose_headers(Any)                  // Expose all response headers
        // Cache preflight for 24h
        .max_age(std::time::Duration::from_secs(86400))
}
```

### Production CORS

Restrict to specific origins:

```rust
pub fn create_cors_layer_with_origins(origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400));

    // Add specific origins
    for origin in origins {
        if let Ok(parsed_origin) = origin.parse::<axum::http::HeaderValue>() {
            cors = cors.allow_origin(parsed_origin);
        }
    }

    cors
}
```

> **Implementation example:**: See how we configure CORS for development and production in the [CORS Middleware section](05-Middleware.md#cors-middleware) of the Web API Architecture chapter.

**CORS Options:** `allow_origin`, `allow_methods`, `allow_headers`, `expose_headers`, `max_age`

Browsers send OPTIONS preflight requests; CORS middleware handles automatically.

---

## Compression

Compression reduces response sizes, improving performance.

### Compression Setup

```rust
use tower_http::compression::CompressionLayer;

// In server.rs
app = app.layer(CompressionLayer::new());
```

> **Where it's used**: Compression is configured in our server setup. See the [Creating the Application Router section](02-Server-Setup.md#creating-the-application-router) to see how compression middleware is applied alongside other middleware layers.

Automatically compresses when client supports it (gzip, deflate, brotli). Transparent to handlers.

---

## Response Types

Axum handlers can return various response types. The flexibility to return different types from handlers is made possible by the `IntoResponse` trait, which is central to Axum's response handling system.

### Understanding IntoResponse

`IntoResponse` is a trait that allows any type to be converted into an HTTP response. This trait is what makes Axum's handler return types so flexible—you can return `Json`, `StatusCode`, tuples, custom types, and more, as long as they implement `IntoResponse`.

#### What is IntoResponse?

`IntoResponse` is defined as:

```rust
pub trait IntoResponse {
    fn into_response(self) -> Response;
}
```

Any type that implements this trait can be returned from an Axum handler. Axum automatically calls `.into_response()` on the return value to convert it into an HTTP response.

#### Why is IntoResponse Used?

Provides type flexibility, composability (tuples), type safety, and clean API without boilerplate.

#### Built-in IntoResponse Implementations

Common types implementing `IntoResponse`:
- `StatusCode`: Simple status responses
- `Json<T>`: JSON responses with Content-Type
- `(StatusCode, Json<T>)`: Status + body
- `(HeaderMap, Json<T>)`: Headers + body
- `Result<T, E>`: Error handling
- `&str`, `String`: Text responses

#### How IntoResponse Works

Axum calls `.into_response()` on handler return values. `Json` wrapper:
1. Serializes to JSON
2. Sets `Content-Type: application/json`
3. Sets status code
4. Creates response body

#### Custom IntoResponse Implementation

Implement `IntoResponse` for custom types:

```rust
impl IntoResponse for CustomResponse {
    fn into_response(self) -> Response {
        (self.status, body).into_response()
    }
}
```

#### Common Patterns Using IntoResponse

**Error Handling:** `Err(StatusCode::NOT_FOUND)` returns error response

**Conditional Responses:** Return different status codes based on condition

**Response with Headers:** Use tuples to combine status, headers, and body

> **See it in action:**: Our handlers use `IntoResponse` extensively. See the [Request Handlers section](04-Handlers.md#request-handlers) for examples of handlers returning `Json<ApiResponse<T>>` and `Result<Json<ApiResponse<T>>, StatusCode>`, both of which rely on `IntoResponse` for conversion.

### Json Response

```rust
use axum::response::Json;

pub async fn handler() -> Result<Json<ApiResponse<T>>, StatusCode> {
    Ok(Json(ApiResponse::success(data)))
}
```

> **Implementation example:**: See how we use `Json` responses throughout our handlers in the [Request Handlers section](04-Handlers.md#request-handlers), and explore our response models in the [Data Models section](06-Data-Models.md#data-models).

#### Example: Converting WalletResponse to HTTP Response

Conversion chain: `WalletResponse` → `ApiResponse<WalletResponse>` → `Json(ApiResponse)` → `Result` → `IntoResponse` → HTTP response

The conversion happens automatically without manual response construction. The type system ensures correctness at compile time.

> **See the full implementation:**: The `create_wallet()` handler in [Wallet Handlers](04-Handlers.md#wallet-handlers) demonstrates this exact pattern. For more details on serialization, see the Serde Framework Guide.

### Status Code Only

```rust
pub async fn handler() -> Result<(), StatusCode> {
    if error {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(())
}
```

### Custom Responses

Implement `IntoResponse` for custom response types to encapsulate response construction logic.

### Response Headers

Use tuples to combine headers with body: `(headers, Json(data)).into_response()`

### Streaming Responses

Use `Body::from_stream()` for large responses.

---

## Summary

Axum provides a powerful, type-safe foundation for building web APIs:

- **State Injection**: Share data across handlers safely - see [State Injection in our API](02-Server-Setup.md#creating-the-application-router)
- **Middleware**: Handle cross-cutting concerns elegantly - see [Middleware Layer](05-Middleware.md#middleware-layer)
- **Extractors**: Pull data from requests with type safety - see [Request Handlers](04-Handlers.md#request-handlers)
- **Routing**: Organize endpoints logically - see [Routing System](03-Routing.md#routing-system)
- **Async/Await**: Handle concurrency efficiently - see [Handler Pattern](04-Handlers.md#handler-pattern)
- **Error Handling**: Provide clear error responses - see [Error Handling](07-Error-Handling.md#error-handling)
- **CORS**: Enable cross-origin requests securely - see [CORS Middleware](05-Middleware.md#cors-middleware)
- **Compression**: Improve performance automatically - see [Creating the Application Router](02-Server-Setup.md#creating-the-application-router)

These features work together to create a robust, maintainable web API. For more details on how we use these features in our blockchain API, see the Web API Architecture chapter, which provides:

- Complete server setup and configuration
- Detailed handler implementations
- Middleware architecture
- Security patterns
- Best practices and patterns

---

## Additional Resources

### Internal Documentation

- **Web API Architecture Index**: Overview and navigation for the web API section
- **Web API Architecture Overview**: Chapter overview and navigation
- **Introduction & Architecture**: Architecture overview and design principles
- **Server Setup**: Server initialization and configuration
- **Routing**: Route definitions and organization
- **Handlers**: Request handler implementations
- **Middleware**: Middleware layer implementation (includes Tower HTTP components)

### External Resources

- **[Axum Documentation](https://docs.rs/axum/)**: Official Axum crate documentation
- **[Tower Documentation](https://docs.rs/tower/)**: Tower service and layer traits
- **[Tower HTTP Documentation](https://docs.rs/tower-http/)**: HTTP-specific middleware (CORS, compression, tracing)
- **[Tokio Documentation](https://docs.rs/tokio/)**: Async runtime used by Axum

---

<div align="center">

**[← Web API Index](README.md)** | **Axum Framework Guide** (includes Tower) | **[Introduction & Architecture Overview →](01-Introduction.md)** | **Serde** | **Utoipa** | **Tracing** | **Tokio**

</div>
