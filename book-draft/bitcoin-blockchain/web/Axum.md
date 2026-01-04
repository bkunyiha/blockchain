<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../wallet/README.md) - Wallet implementation and key management
15. **Chapter 3: Web API Architecture** ← *You are here*
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../../ci/kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
# Axum Framework Guide

**Part I: Core Blockchain Implementation** | **Technical Reference: Axum Framework**

<div align="center">

**📚 [← Web Index](README.md)** | **Axum Framework Guide** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Overview

This guide provides detailed explanations of Axum framework features used in our blockchain web API. Axum is a modern, ergonomic web framework built on top of Tokio and Tower, designed to leverage Rust's type system for building fast, reliable web services.

Throughout this guide, we'll explore the specific Axum features used in our implementation, with code examples from our blockchain API to illustrate each concept.

> **📘 See the full implementation**: This guide explains the Axum concepts. To see how these features are used together in our complete web API architecture, see the [Web API Architecture chapter](01-Introduction.md), which covers server setup, routing, handlers, middleware, and more.

---

## Table of Contents

1. [State Injection](#state-injection) - Share data across handlers
2. [Middleware Layers](#middleware-layers) - Cross-cutting concerns
3. [Request Extractors](#request-extractors) - Extract data from requests
4. [Routing](#routing) - Organize endpoints
5. [Async/Await](#asyncawait) - Handle concurrency
6. [Error Handling](#error-handling) - Manage errors gracefully
7. [CORS Configuration](#cors-configuration) - Enable cross-origin requests
8. [Compression](#compression) - Reduce response sizes
9. [Response Types](#response-types) - Return various response formats
   - [Understanding IntoResponse](#understanding-intoresponse) - The trait that enables flexible handler return types

> **💡 Tip**: Throughout this guide, you'll find links to specific sections in the [Web API Architecture chapter](01-Introduction.md) where these Axum features are actually implemented. Use these links to see real-world examples and understand how theory translates to practice.

---

## State Injection

State injection allows you to share data across all handlers in your application. In Axum, state is injected using `.with_state()` and extracted in handlers using the `State` extractor.

### How State Injection Works

State injection makes data available to all handlers without global variables. The state is type-safe and checked at compile time.

**Example from our implementation:**

```rust
// In server.rs - Creating the router with state
pub fn create_app(&self) -> Router {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())
        .with_state(self.node.clone());  // ← State injection
    
    // ... middleware layers
    app
}
```

**Extracting State in Handlers:**

```rust
// In handlers/blockchain.rs
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,  // ← State extraction
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    // Now we can use `node` to access blockchain data
    let height = node.get_blockchain_height().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // ... rest of handler
}
```

> **See it in action**: Check out the [Blockchain Handlers section](01-Introduction.md#blockchain-handlers) in the Web API Architecture chapter to see how state extraction is used in real handlers like `get_blockchain_info()` and `get_block_by_hash()`.

### Key Concepts

1. **Type Safety**: The state type must match exactly between `.with_state()` and `State()` extractor
2. **Shared Ownership**: We use `Arc<T>` to share state across handlers without cloning
3. **Immutable Access**: State is typically immutable; if you need mutable access, use interior mutability patterns

### Why Use State?

- **No Global Variables**: Avoids the pitfalls of global state
- **Type Safety**: Compiler ensures state is correctly typed
- **Testability**: Easy to inject mock state for testing
- **Clean Architecture**: State flows explicitly through your application

> **Implementation example**: See how we inject `NodeContext` as state in the [Server Setup and Configuration section](01-Introduction.md#server-setup-and-configuration), and how handlers extract it in the [Request Handlers section](01-Introduction.md#request-handlers).

### State Type Requirements

The state type must implement `Clone` (or be wrapped in `Arc<T>` which implements `Clone`):

```rust
// Our NodeContext is wrapped in Arc for shared ownership
.with_state(Arc::new(node_context))

// Or clone the Arc
.with_state(self.node.clone())  // Arc<NodeContext> implements Clone
```

---

## Middleware Layers

Middleware in Axum provides cross-cutting concerns like authentication, logging, CORS, and error handling. Middleware is applied using `.layer()` and processes requests in order.

### Middleware Execution Order

Middleware layers wrap each other, creating a request processing pipeline. The order matters:

```rust
// In server.rs - Middleware layers applied in order
pub fn create_app(&self) -> Router {
    let mut app = Router::new()
        .merge(create_all_api_routes())
        .with_state(self.node.clone());

    // 1. CORS middleware (outermost - handles preflight requests first)
    if self.config.enable_cors {
        app = app.layer(cors::create_cors_layer());
    }

    // 2. Compression middleware (compresses responses)
    app = app.layer(CompressionLayer::new());

    // 3. Error handling middleware (innermost - catches errors from handlers)
    app = app.layer(axum::middleware::from_fn(handle_errors));

    app
}
```

**Request Flow:**

```
Request → CORS Layer → Compression Layer → Error Handler → Route Handler → Response
```

> **See the implementation**: The exact middleware order used in our blockchain API is shown in the [Creating the Application Router section](01-Introduction.md#creating-the-application-router) of the Web API Architecture chapter.

### Creating Custom Middleware

Middleware functions follow a specific signature:

```rust
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};

async fn my_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Pre-processing: Modify request, log, authenticate, etc.
    tracing::info!("Request: {:?}", request.uri());
    
    // Call next middleware/handler
    let response = next.run(request).await;
    
    // Post-processing: Modify response, log, etc.
    tracing::info!("Response: {:?}", response.status());
    
    Ok(response)
}

// Apply middleware
.router.layer(axum::middleware::from_fn(my_middleware))
```

### Middleware from Functions

Axum provides `axum::middleware::from_fn()` to convert async functions into middleware:

```rust
// Our error handling middleware
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;
    
    // Process response...
    Ok(response)
}

// Apply it
.layer(axum::middleware::from_fn(handle_errors))
```

> **Real implementation**: See the complete `handle_errors()` middleware implementation in the [Error Handling Middleware section](01-Introduction.md#error-handling-middleware), and authentication middleware in the [Authentication Middleware section](01-Introduction.md#authentication-middleware).

### Tower Layers

Axum uses Tower's layer system. Many middleware come from `tower-http`:

```rust
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;

// Compression layer
.layer(CompressionLayer::new())

// CORS layer
.layer(CorsLayer::new().allow_origin(Any))
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

Extracts path parameters:

```rust
// Route: "/blockchain/blocks/{hash}"
pub async fn get_block_by_hash(
    State(node): State<Arc<NodeContext>>,
    Path(hash): Path<String>,  // ← Extracts {hash} from URL
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    let block = node.get_block_by_hash(&hash).await?;
    // ...
}
```

> **See it in action**: The `get_block_by_hash()` handler is fully implemented in the [Blockchain Handlers section](01-Introduction.md#blockchain-handlers). Also see how we use type-safe path extraction with `WalletAddress` in the [Wallet Handlers section](01-Introduction.md#wallet-handlers).

**Type-Safe Path Extraction:**

Axum can deserialize path parameters into custom types:

```rust
// Route: "/wallet/{address}"
pub async fn get_balance(
    State(node): State<Arc<NodeContext>>,
    Path(address): Path<WalletAddress>,  // ← Custom type!
) -> Result<Json<ApiResponse<BalanceResponse>>, StatusCode> {
    // `address` is already a WalletAddress, not a String
    let balance = node.get_balance(&address).await?;
    // ...
}
```

#### Query Extractor

Extracts query parameters:

```rust
use axum::extract::Query;

#[derive(serde::Deserialize)]
struct BlockQuery {
    page: Option<u32>,
    limit: Option<u32>,
    hash: Option<String>,
}

pub async fn get_blocks(
    State(node): State<Arc<NodeContext>>,
    Query(params): Query<BlockQuery>,  // ← Extracts ?page=1&limit=10
) -> Result<Json<ApiResponse<Vec<BlockResponse>>>, StatusCode> {
    let page = params.page.unwrap_or(0);
    let limit = params.limit.unwrap_or(20);
    // ...
}
```

#### Json Extractor

Extracts JSON request body:

```rust
use axum::extract::Json;

pub async fn create_wallet(
    State(_node): State<Arc<NodeContext>>,
    Json(request): Json<CreateWalletRequest>,  // ← Extracts JSON body
) -> Result<Json<ApiResponse<WalletResponse>>, StatusCode> {
    // `request` is already deserialized into CreateWalletRequest
    // ...
}
```

> **Implementation examples**: See `create_wallet()` in the [Wallet Handlers section](01-Introduction.md#wallet-handlers) and `send_transaction()` in the [Transaction Handlers section](01-Introduction.md#transaction-handlers) for real-world JSON extraction examples.

### Multiple Extractors

You can use multiple extractors in a single handler:

```rust
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,           // State
    Json(request): Json<SendTransactionRequest>,    // JSON body
    // Could also add: Path(id): Path<String>,      // Path param
    // Could also add: Query(params): Query<...>,    // Query params
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    // All extractors are available
}
```

> **Real example**: The `send_transaction()` handler uses both `State` and `Json` extractors. See the complete implementation in the [Transaction Handlers section](01-Introduction.md#transaction-handlers).

### Extractor Order

Extractors are processed in order, but the order doesn't matter for most cases. However, `State` is typically listed first by convention.

### Extractor Failures

If an extractor fails (e.g., invalid JSON), Axum automatically returns an appropriate error response (400 Bad Request, etc.). You can customize this behavior.

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

`.nest()` prefixes a group of routes:

```rust
// In routes/api.rs
pub fn create_admin_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .nest("/api/admin", create_api_routes())      // All routes under /api/admin
        .nest("/api/admin", create_monitor_api_routes())  // Health routes also under /api/admin
        .layer(axum::middleware::from_fn(require_admin))
}
```

**Result:** Routes like `/api/admin/blockchain`, `/api/admin/health`, etc.

> **See the routing structure**: Our complete routing organization, including nested admin routes and wallet routes, is explained in the [Routing System section](01-Introduction.md#routing-system) of the Web API Architecture chapter.

### Merging Routes

`.merge()` combines routers without a prefix:

```rust
// In server.rs
Router::new()
    .merge(create_all_api_routes())      // /api/v1/*, /api/admin/*, /health/*
    .merge(create_wallet_only_routes())  // /api/wallet/*
    .merge(create_web_routes())          // /* (web UI)
```

> **Complete routing example**: See how we merge multiple route groups in the [Creating the Application Router section](01-Introduction.md#creating-the-application-router), and explore the full route structure in the [API Route Structure section](01-Introduction.md#api-route-structure).

### Route-Specific Middleware

Apply middleware to specific routes:

```rust
Router::new()
    .route("/public", get(public_handler))
    .route("/admin", get(admin_handler))
    .layer(axum::middleware::from_fn(require_admin))  // Only applies to routes above
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
    // Async operations use .await
    let result = node.get_blockchain_height().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // More async operations
    let block = node.blockchain().get_last_block().await?;
    
    Ok(Json(ApiResponse::success(response)))
}
```

> **Handler examples**: All our handlers follow this async pattern. See the [Handler Pattern section](01-Introduction.md#handler-pattern) and explore specific implementations in [Blockchain Handlers](01-Introduction.md#blockchain-handlers), [Wallet Handlers](01-Introduction.md#wallet-handlers), and [Transaction Handlers](01-Introduction.md#transaction-handlers).

### Why Async?

- **Concurrency**: Handle many requests simultaneously without blocking
- **Efficiency**: Non-blocking I/O allows the server to process other requests while waiting
- **Scalability**: One thread can handle thousands of concurrent connections

### Async Best Practices

1. **Use `.await` for I/O operations**: Database queries, network requests, file operations
2. **Error handling**: Use `?` operator or `.map_err()` to convert errors
3. **Avoid blocking**: Don't use blocking operations in async handlers (use `tokio::task::spawn_blocking` if needed)

### Example: Concurrent Operations

```rust
use futures::future::join_all;

pub async fn get_multiple_blocks(
    State(node): State<Arc<NodeContext>>,
    Path(hashes): Path<Vec<String>>,
) -> Result<Json<ApiResponse<Vec<BlockResponse>>>, StatusCode> {
    // Fetch blocks concurrently
    let futures = hashes.iter().map(|hash| {
        node.get_block_by_hash(hash)
    });
    
    let blocks = join_all(futures).await;
    // ...
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
    if response.status().is_server_error() || response.status().is_client_error() {
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

> **Complete error handling**: See the full `handle_errors()` implementation and error handling patterns in the [Error Handling Middleware section](01-Introduction.md#error-handling-middleware) and [Error Handling section](01-Introduction.md#error-handling) of the Web API Architecture chapter.

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

> **Error handling examples**: These patterns are used throughout our handlers. See the [Common Error Patterns section](01-Introduction.md#common-error-patterns) for more examples and the [Error Flow section](01-Introduction.md#error-flow) for understanding how errors propagate.

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
        .max_age(std::time::Duration::from_secs(86400))  // Cache preflight for 24h
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

> **CORS implementation**: See how we configure CORS for development and production in the [CORS Middleware section](01-Introduction.md#cors-middleware) of the Web API Architecture chapter.

### CORS Options

- **`allow_origin`**: Which origins can make requests
- **`allow_methods`**: Which HTTP methods are allowed (GET, POST, etc.)
- **`allow_headers`**: Which headers can be sent
- **`expose_headers`**: Which headers JavaScript can read
- **`max_age`**: How long to cache preflight requests

### Preflight Requests

Browsers send OPTIONS requests (preflight) for certain cross-origin requests. CORS middleware handles these automatically.

---

## Compression

Compression reduces response sizes, improving performance.

### Compression Setup

```rust
use tower_http::compression::CompressionLayer;

// In server.rs
app = app.layer(CompressionLayer::new());
```

> **Where it's used**: Compression is configured in our server setup. See the [Creating the Application Router section](01-Introduction.md#creating-the-application-router) to see how compression middleware is applied alongside other middleware layers.

### How It Works

- Automatically compresses responses when client supports it (via `Accept-Encoding` header)
- Supports gzip, deflate, and brotli
- Only compresses responses above a certain size threshold
- Transparent to handlers - no code changes needed

### Compression Benefits

- **Reduced Bandwidth**: Smaller responses use less bandwidth
- **Faster Transfers**: Less data to transfer means faster page loads
- **Better Performance**: Especially important for mobile users

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

`IntoResponse` provides several key benefits:

1. **Type Flexibility**: Handlers can return different types (`Json`, `StatusCode`, tuples, custom types) without explicit conversion
2. **Composability**: You can combine status codes, headers, and body types using tuples
3. **Type Safety**: The compiler ensures only valid response types are returned
4. **Clean API**: Handlers remain readable without boilerplate response construction

#### Built-in IntoResponse Implementations

Axum provides `IntoResponse` implementations for many common types:

**Status Codes:**
```rust
pub async fn handler() -> StatusCode {
    StatusCode::OK  // Automatically converted to HTTP response
}
```

**JSON Responses:**
```rust
use axum::response::Json;

pub async fn handler() -> Json<ApiResponse<Data>> {
    Json(ApiResponse::success(data))  // Automatically sets Content-Type: application/json
}
```

**Tuples (Status Code + Body):**
```rust
pub async fn handler() -> (StatusCode, Json<ApiResponse<Data>>) {
    (StatusCode::CREATED, Json(ApiResponse::success(data)))
}
```

**Tuples (Headers + Body):**
```rust
use axum::http::HeaderMap;

pub async fn handler() -> (HeaderMap, Json<ApiResponse<Data>>) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom-Header", HeaderValue::from_static("value"));
    (headers, Json(ApiResponse::success(data)))
}
```

**Tuples (Status Code + Headers + Body):**
```rust
pub async fn handler() -> (StatusCode, HeaderMap, Json<ApiResponse<Data>>) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom-Header", HeaderValue::from_static("value"));
    (StatusCode::CREATED, headers, Json(ApiResponse::success(data)))
}
```

**String/&str:**
```rust
pub async fn handler() -> &'static str {
    "Hello, World!"  // Automatically converted to text/plain response
}
```

**Result Types:**
```rust
pub async fn handler() -> Result<Json<ApiResponse<Data>>, StatusCode> {
    if error {
        Err(StatusCode::NOT_FOUND)  // Error type must also implement IntoResponse
    } else {
        Ok(Json(ApiResponse::success(data)))
    }
}
```

#### How IntoResponse Works

When a handler returns a value, Axum calls `.into_response()` on it:

```rust
// Handler returns Json<ApiResponse<Data>>
pub async fn handler() -> Json<ApiResponse<Data>> {
    Json(ApiResponse::success(data))
}

// Axum internally does:
let response = handler().await;
let http_response = response.into_response();  // Converts to Response
```

The `Json` wrapper implements `IntoResponse`, which:
1. Serializes the inner type to JSON
2. Sets the `Content-Type` header to `application/json`
3. Sets the status code to `200 OK`
4. Creates the HTTP response body

#### Custom IntoResponse Implementation

You can implement `IntoResponse` for your own types:

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

struct CustomResponse {
    message: String,
    status: StatusCode,
}

impl IntoResponse for CustomResponse {
    fn into_response(self) -> Response {
        let body = format!(r#"{{"message": "{}"}}"#, self.message);
        (
            self.status,
            [("Content-Type", "application/json")],
            body,
        )
            .into_response()
    }
}

// Now you can return CustomResponse from handlers
pub async fn handler() -> CustomResponse {
    CustomResponse {
        message: "Success".to_string(),
        status: StatusCode::OK,
    }
}
```

#### Common Patterns Using IntoResponse

**Error Handling:**
```rust
pub async fn handler() -> Result<Json<ApiResponse<Data>>, StatusCode> {
    // StatusCode implements IntoResponse, so it can be used as error type
    if not_found {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(Json(ApiResponse::success(data)))
}
```

**Conditional Responses:**
```rust
pub async fn handler() -> impl IntoResponse {
    if condition {
        (StatusCode::CREATED, Json(data))
    } else {
        (StatusCode::OK, Json(data))
    }
}
```

**Response with Headers:**
```rust
use axum::http::{HeaderMap, HeaderValue};

pub async fn handler() -> (StatusCode, HeaderMap, Json<ApiResponse<Data>>) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Total-Count", HeaderValue::from_static("100"));
    
    (StatusCode::OK, headers, Json(ApiResponse::success(data)))
}
```

> **See it in action**: Our handlers use `IntoResponse` extensively. See the [Request Handlers section](01-Introduction.md#request-handlers) for examples of handlers returning `Json<ApiResponse<T>>` and `Result<Json<ApiResponse<T>>, StatusCode>`, both of which rely on `IntoResponse` for conversion.

### Json Response

```rust
use axum::response::Json;

pub async fn handler() -> Result<Json<ApiResponse<T>>, StatusCode> {
    Ok(Json(ApiResponse::success(data)))
}
```

> **Response examples**: See how we use `Json` responses throughout our handlers in the [Request Handlers section](01-Introduction.md#request-handlers), and explore our response models in the [Data Models section](01-Introduction.md#data-models).

#### Example: Converting WalletResponse to HTTP Response

Let's trace how a `WalletResponse` struct is converted to an HTTP `Response` using the `IntoResponse` trait. This demonstrates the complete conversion chain:

**Step 1: Create the WalletResponse struct**

```rust
// In handlers/wallet.rs
let response = WalletResponse {
    address: address.as_string(),
    public_key: HEXLOWER.encode(wallet.get_public_key()),
    created_at: chrono::Utc::now(),
};
```

`WalletResponse` is a regular Rust struct that implements `Serialize` (via Serde's `#[derive(Serialize)]`):

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletResponse {
    pub address: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
}
```

**Step 2: Wrap in ApiResponse**

```rust
ApiResponse::success(response)
```

This creates an `ApiResponse<WalletResponse>`:

```rust
ApiResponse {
    success: true,
    data: Some(WalletResponse { ... }),
    error: None,
    timestamp: Utc::now(),
}
```

`ApiResponse<T>` also implements `Serialize` (via derive macro), so it can be serialized to JSON.

**Step 3: Wrap in Json**

```rust
Json(ApiResponse::success(response))
```

This creates a `Json<ApiResponse<WalletResponse>>`. The `Json` wrapper type is provided by Axum and implements `IntoResponse`.

**Step 4: Wrap in Result**

```rust
Ok(Json(ApiResponse::success(response)))
```

This creates a `Result<Json<ApiResponse<WalletResponse>>, StatusCode>`. The `Result` type also implements `IntoResponse` when both `Ok` and `Err` variants implement `IntoResponse` (which they do: `Json` implements it, and `StatusCode` implements it).

**Step 5: Axum calls into_response()**

When the handler returns, Axum automatically calls `.into_response()` on the return value:

```rust
// Axum internally does something like:
let handler_result = handler().await;
let http_response = handler_result.into_response();
```

**Step 6: Json's IntoResponse implementation**

The `Json<T>` type's `IntoResponse` implementation:

1. **Serializes the inner type**: Calls `serde_json::to_vec()` on `ApiResponse<WalletResponse>` to convert it to JSON bytes
2. **Sets Content-Type header**: Sets `Content-Type: application/json`
3. **Sets status code**: Sets status to `200 OK` (or uses the status from `Result` if present)
4. **Creates Response body**: Wraps the JSON bytes in an HTTP response body

**The Complete Conversion Chain:**

```
WalletResponse (struct)
    ↓ (wrapped in ApiResponse::success)
ApiResponse<WalletResponse> (implements Serialize)
    ↓ (wrapped in Json)
Json<ApiResponse<WalletResponse>> (implements IntoResponse)
    ↓ (wrapped in Ok)
Result<Json<ApiResponse<WalletResponse>>, StatusCode> (implements IntoResponse)
    ↓ (Axum calls .into_response())
Response (HTTP response with JSON body)
```

**Why This Works:**

1. **Serde Serialization**: Both `WalletResponse` and `ApiResponse<T>` implement `Serialize`, allowing them to be converted to JSON bytes
2. **Json Wrapper**: Axum's `Json<T>` implements `IntoResponse` and handles the HTTP response construction
3. **Type Safety**: The compiler ensures `WalletResponse` can be serialized (checked at compile time)
4. **Automatic Conversion**: Axum automatically calls `.into_response()` on handler return values

**Final HTTP Response:**

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 156

{
  "success": true,
  "data": {
    "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
    "public_key": "02a1b2c3...",
    "created_at": "2024-01-15T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

This conversion happens automatically—you never need to manually construct HTTP responses. The type system ensures everything is correct at compile time.

> **See the actual implementation**: The `create_wallet()` handler in [Wallet Handlers](04-Handlers.md#wallet-handlers) demonstrates this exact pattern. For more details on serialization, see the [Serde Framework Guide](Serde.md).

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

As explained in [Understanding IntoResponse](#understanding-intoresponse), you can implement `IntoResponse` for your own types to create custom response formats:

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use axum::response::Json;

struct MyResponse {
    message: String,
}

impl IntoResponse for MyResponse {
    fn into_response(self) -> Response {
        // Build custom response using tuple syntax
        // This leverages the built-in IntoResponse for tuples
        (StatusCode::OK, Json(self)).into_response()
    }
}

// Now handlers can return MyResponse directly
pub async fn handler() -> MyResponse {
    MyResponse {
        message: "Custom response".to_string(),
    }
}
```

This pattern is useful when you need consistent response formatting across multiple handlers or want to encapsulate response construction logic.

### Response Headers

Add headers to responses:

```rust
use axum::response::Response;
use axum::http::{HeaderMap, HeaderValue};

pub async fn handler() -> Response {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom-Header", HeaderValue::from_static("value"));
    
    (headers, Json(data)).into_response()
}
```

### Streaming Responses

For large responses, use streaming:

```rust
use axum::body::Body;
use axum::response::Response;

pub async fn handler() -> Response {
    let stream = /* create stream */;
    Response::builder()
        .body(Body::from_stream(stream))
        .unwrap()
}
```

---

## Summary

Axum provides a powerful, type-safe foundation for building web APIs:

- **State Injection**: Share data across handlers safely - see [State Injection in our API](01-Introduction.md#creating-the-application-router)
- **Middleware**: Handle cross-cutting concerns elegantly - see [Middleware Layer](01-Introduction.md#middleware-layer)
- **Extractors**: Pull data from requests with type safety - see [Request Handlers](01-Introduction.md#request-handlers)
- **Routing**: Organize endpoints logically - see [Routing System](01-Introduction.md#routing-system)
- **Async/Await**: Handle concurrency efficiently - see [Handler Pattern](01-Introduction.md#handler-pattern)
- **Error Handling**: Provide clear error responses - see [Error Handling](01-Introduction.md#error-handling)
- **CORS**: Enable cross-origin requests securely - see [CORS Middleware](01-Introduction.md#cors-middleware)
- **Compression**: Improve performance automatically - see [Creating the Application Router](01-Introduction.md#creating-the-application-router)

These features work together to create a robust, maintainable web API. For more details on how we use these features in our blockchain API, see the [Web API Architecture](01-Introduction.md) chapter, which provides:

- Complete server setup and configuration
- Detailed handler implementations
- Middleware architecture
- Security patterns
- Best practices and patterns

---

## Additional Resources

### Internal Documentation

- **[Web API Architecture Index](README.md)**: Overview and navigation for the web API section
- **[Web API Architecture Overview](01-Introduction.md)**: Chapter overview and navigation
- **[Introduction & Architecture](01-Introduction.md)**: Architecture overview and design principles
- **[Server Setup](02-Server-Setup.md)**: Server initialization and configuration
- **[Routing](03-Routing.md)**: Route definitions and organization
- **[Handlers](04-Handlers.md)**: Request handler implementations
- **[Middleware](05-Middleware.md)**: Middleware layer implementation

### External Resources

- **[Axum Documentation](https://docs.rs/axum/)**: Official Axum crate documentation
- **[Tower Documentation](https://docs.rs/tower/)**: Tower middleware and service traits
- **[Tokio Documentation](https://docs.rs/tokio/)**: Async runtime used by Axum

---

<div align="center">

**📚 [← Web API Index](README.md)** | **Axum Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **[Tower](Tower.md)** | **[Serde](Serde.md)** | **[Utoipa](Utoipa.md)** | **[Tracing](Tracing.md)** | **[Tokio](../Tokio.md)** 📚

</div>
