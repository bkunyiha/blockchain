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

# Tracing Framework Guide

**Part I: Foundations & Core Implementation** | **Technical Reference: Tracing Structured Logging Framework**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Tracing Framework Guide** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)** 

</div>

---

## Overview

This guide provides detailed explanations of the Tracing framework and how it's used throughout our blockchain web API for structured logging and diagnostics. Tracing is a powerful framework for instrumenting Rust programs to collect structured, event-based diagnostic information. It's the foundation for logging, debugging, and monitoring in our blockchain node and web API.

In our blockchain API, Tracing is used extensively for:
- **Request/Response Logging**: Tracking HTTP requests and responses through Tower's TraceLayer
- **Error Logging**: Recording errors with full context and stack traces
- **Performance Monitoring**: Measuring request latency and operation duration
- **Debugging**: Detailed diagnostic information for development
- **Production Monitoring**: Structured logs for production observability

> **See the full implementation**: This guide explains Tracing concepts. To see how Tracing is used in our complete web API architecture, see the Middleware Layer chapter for logging middleware, the Server Setup chapter for logging configuration, and the Error Handling chapter for error logging.

---

## Table of Contents

1. [What is Tracing?](#what-is-tracing) - Understanding Tracing's role
2. [Logging Levels](#logging-levels) - ERROR, WARN, INFO, DEBUG, TRACE
3. [Structured Logging](#structured-logging) - Key-value pairs and context
4. [Spans](#spans) - Tracking operation context and duration
5. [Integration with Tower](#integration-with-tower) - TraceLayer for HTTP logging
6. [Logging Configuration](#logging-configuration) - Setting up tracing-subscriber
7. [Examples from Our Project](#examples-from-our-project) - Real-world usage patterns
8. [Best Practices](#best-practices) - Effective logging strategies
9. [Performance Considerations](#performance-considerations) - Logging overhead

---

## What is Tracing?

Tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information. Unlike traditional logging libraries, Tracing provides:

- **Structured Logging**: Key-value pairs instead of formatted strings
- **Spans**: Contextual information that persists across function calls
- **Event Correlation**: Link related events through span relationships
- **Zero-Cost Abstractions**: Compile-time filtering removes logging code when disabled
- **Async-Aware**: Works seamlessly with async/await code

### Tracing's Design Philosophy

- **Structured**: Logs contain structured data, not just strings
- **Contextual**: Spans provide context that persists across operations
- **Performant**: Zero-cost when disabled, efficient when enabled
- **Flexible**: Works with multiple subscribers (fmt, json, opentelemetry, etc.)
- **Async-Friendly**: Designed for async Rust applications

### Tracing in Our API

In our blockchain API, Tracing is used primarily for:

- **HTTP Request Logging**: Tower's TraceLayer logs all HTTP requests/responses
- **Error Tracking**: Errors are logged with full context
- **Server Lifecycle**: Server startup, shutdown, and configuration events
- **Handler Instrumentation**: Key operations in request handlers
- **Performance Monitoring**: Request latency and operation duration

Tracing integrates seamlessly with Tower HTTP's `TraceLayer` for automatic HTTP request/response logging.

---

## Logging Levels

Tracing provides five levels (most to least severe): ERROR, WARN, INFO, DEBUG, TRACE.

### Project Configuration

In our blockchain API:

- **ERROR**: Operation failures, unrecoverable errors - example: failed transactions
- **WARN**: Recoverable issues, performance concerns
- **INFO**: Normal operations, important state changes
- **DEBUG**: Detailed diagnostics (disabled in production by default)
- **TRACE**: Very detailed diagnostics (disabled in production by default)

### Examples from Our Codebase

```rust
// server.rs - Error handling middleware
tracing::error!(
    "[handle_errors]: Error response ({}): {}",
    parts.status,
    body_str
);

// server.rs - Server startup
tracing::info!("Starting web server on {} with graceful shutdown", addr);

// handlers/transaction.rs - Transaction submission
info!("Transaction {} submitted successfully", txid);

// server.rs - Shutdown signal
tracing::info!("Shutdown signal received");
```

---

## Structured Logging

Tracing supports structured logging with key-value pairs for queryable, parseable logs.

### Field Syntax

```rust
use tracing::info;

info!(
    txid = %txid,           // Display formatting (strings, numbers)
    from = %from,           // Use % for numbers/strings
    amount,                 // Debug formatting (default)
    address = ?address,     // Use ? for structs/enums
    "Transaction submitted"
);
```

**Formatting Options:**
- `field = %value`: Display trait (faster for strings/numbers)
- `field = ?value`: Debug trait (for complex types)
- `field`: Shorthand - defaults to Debug

### Structured Logging in Our Project

Tracing macros expand at compile time, creating structured events with named fields. Field formatting uses:

- **`%` operator**: Display trait (strings, numbers) - faster
- **`?` operator**: Debug trait (structs, enums)
- **No operator**: Defaults to Debug formatting

Fields are only evaluated if the log level is enabled (zero-cost when disabled).

> **See it in action**: Check out the [Examples from Our Project](Tracing.md#examples-from-our-project) section to see how structured logging is used throughout our handlers and middleware.

### Example from Our Project

**In `handlers/transaction.rs`:**

```rust
use tracing::{error, info};

pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    let txid = node
        .btc_transaction(
            &request.from_address,
            &request.to_address,
            request.amount
        )
        .await?;

    info!(
        txid = %txid,
        from = %request.from_address,
        to = %request.to_address,
        amount = request.amount,
        "Transaction submitted successfully"
    );
    Ok(Json(ApiResponse::success(response)))
}
```

---

### Example Log Output

Structured logging output in text format:
```text
2024-01-15T10:30:45Z INFO txid=a1b2c3d4 from=1A1zP amount=1000
```

JSON format:
```json
{"ts":"2024-01-15T10:30:45Z","level":"INFO","txid":"a1b2c3","amount":1000}
```

**Key benefits:**
- Field names preserved for filtering
- Queryable by `txid`, `from`, `amount`
- Easy to parse programmatically

---

## Spans

Spans represent periods of time during which a program was executing in a particular context. They provide contextual information that persists across function calls.

### Creating Spans

Use the `#[instrument]` attribute to automatically create spans for functions:

```rust
use tracing::instrument;

#[instrument]
async fn process_transaction(txid: String) {
    // Automatic span with function name and parameters
}
```

### Span Fields

Include fields in spans for context:

```rust
#[instrument(fields(txid = %txid, amount = amount))]
async fn send_transaction(txid: String, amount: i32) {
    // All logs here include txid and amount
}
```

### Example from Our Project

```rust
#[instrument]
pub async fn start_server(
    node: Arc<NodeContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    Ok(())
}
```

Benefits: Automatic context, duration tracking, nested call hierarchy

---

## Integration with Tower

Tracing integrates seamlessly with Tower HTTP's `TraceLayer` for automatic HTTP request/response logging.

### TraceLayer Overview

`TraceLayer` automatically creates spans for HTTP requests and logs request/response information:

```rust
use tower_http::trace::TraceLayer;

let layer = TraceLayer::new_for_http();
```

**What TraceLayer Does:**

1. **Creates Request Span**: Automatically creates a span for each HTTP request
2. **Logs Request**: Logs incoming request method, URI, headers
3. **Logs Response**: Logs response status, headers, latency
4. **Logs Errors**: Logs errors with full context

### Implementation in Our Project

**In `middleware/logging.rs`:**

```rust
use tower_http::trace::TraceLayer;

/// Create logging middleware for the web server
pub fn create_logging_layer() -> impl tower::Layer<axum::Router> + Clone {
    TraceLayer::new_for_http()
}
```

**In `server.rs` - Applied to Router:**

```rust
// TraceLayer is applied through Tower's middleware system
// It automatically logs all HTTP requests and responses
```

### Custom TraceLayer Configuration

Customize TraceLayer with callbacks:

```rust
TraceLayer::new_for_http()
    .make_span_with(|req: &Request| {
        tracing::info_span!(
            "http_request",
            method = %req.method(),
            uri = %req.uri()
        )
    })
    .on_response(
        |res: &Response, latency: Duration, _: &Span| {
            tracing::info!(
                status = %res.status(),
                latency = ?latency
            );
        }
    )
```

**Options:** `make_span_with`, `on_request`, `on_response`, `on_failure`, `on_eos`

> **See it in action**: Check out the [Logging Middleware section](Tower.md#logging-middleware) in the Tower Framework Guide for more details on TraceLayer configuration and usage.

---

## Logging Configuration

Tracing uses `tracing-subscriber` to configure log output format and filtering.

### Project Configuration

**In `main.rs`:**

```rust
fn initialize_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .init();
}
```

**Configuration:**
- **Default Level**: INFO (filters out DEBUG and TRACE in production)
- **Environment Variable**: `RUST_LOG` environment variable can override
- **Format**: Human-readable format with compile-time and runtime filtering

**Setting Log Levels via Environment Variable:**

```bash
# Global level
RUST_LOG=info ./blockchain

# Specific modules
RUST_LOG=blockchain::web=debug,blockchain::node=info ./blockchain

# Trace level for debugging
RUST_LOG=blockchain::web=trace ./blockchain
```

---

## Examples from Our Project

Let's examine real examples of Tracing usage throughout our blockchain API.

### Server Lifecycle Logging

**In `server.rs`:**

```rust
/// Start the web server with graceful shutdown
pub async fn start_with_shutdown(
    &self,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = self.create_app();
    let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));

    // Log server startup with graceful shutdown
    tracing::info!("Starting web server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

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

**What Gets Logged:**
- Server startup with address
- Shutdown signal reception
- All HTTP requests/responses (via TraceLayer)

### Error Logging in Middleware

```rust
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;
    let is_error = response.status().is_server_error()
        || response.status().is_client_error();
    if is_error {
        tracing::error!(
            status = %response.status(),
            "Error response"
        );
    }
    Ok(response)
}
```

### Handler Logging

```rust
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    let txid = node.btc_transaction(/* ... */).await?;
    tracing::info!(txid = %txid, "Transaction submitted");
    Ok(Json(ApiResponse::success(response)))
}
```

### Instrumented Functions

Use `#[instrument]` for automatic span creation, duration tracking, and nested context.

---

## Best Practices for Our Project

1. **Use Structured Logging**: Include relevant context (txid, addresses, amounts)
2. **Avoid Sensitive Data**: Never log passwords, API keys, or private keys
3. **Include Error Context**: Log errors with full context for debugging
4. **Use Spans**: `#[instrument]` attribute for operation tracking
5. **Environment-Based Configuration**: Use `RUST_LOG` for log level control

---

## Performance Considerations

- **Zero-Cost When Disabled**: Compile-time filtering removes logging code when not needed
- **Fields Only Evaluated if Enabled**: Structured logging fields evaluated only when log level allows
- **Production Configuration**: Set `RUST_LOG=info` for production; DEBUG/TRACE disabled by default
- **Minimal Overhead**: Spans and structured logging are lightweight

---

## Summary

Tracing provides powerful structured logging for our blockchain API:

- **Structured Logging**: Key-value pairs for queryable logs
- **Spans**: Contextual information that persists across operations
- **Integration**: Seamless integration with Tower HTTP's TraceLayer
- **Performance**: Zero-cost when disabled, efficient when enabled
- **Flexibility**: Configurable via environment variables

Tracing's design allows us to instrument our code for debugging and monitoring while maintaining performance in production.

---

## Additional Resources

- **[Tracing Documentation](https://docs.rs/tracing/)**: Official tracing crate documentation
- **[Tracing Subscriber Documentation](https://docs.rs/tracing-subscriber/)**: Tracing subscriber for configuring log output
- **[Tracing Book](https://tokio.rs/tokio/topics/tracing)**: Comprehensive guide to using tracing
- **Tower Framework Guide**: How Tower's TraceLayer integrates with Tracing
- **Middleware Layer**: How logging middleware is implemented
- **Server Setup**: How logging is configured in our server
- **Error Handling**: How errors are logged
- **Axum Framework Guide**: How Axum handlers use Tracing
- **Tokio Runtime Guide**: Async runtime that Tracing works with
- **Rust Language Guide**: Rust language features used throughout

---

<div align="center">

**[← Web API Index](README.md)** | **Tracing Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **Axum** | **Tower** | **Serde** | **Utoipa** | **Tokio** 

</div>

---

*This guide provides detailed explanations of Tracing framework features used in our blockchain API. For implementation details, see the Middleware Layer chapter.*
