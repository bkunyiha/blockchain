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
   - [Tower Framework Guide](Tower.md) - Middleware framework
   - [Serde Framework Guide](Serde.md) - Serialization framework
   - [Utoipa Framework Guide](Utoipa.md) - OpenAPI framework
   - [Tracing Framework Guide](Tracing.md) - Structured logging ← *You are here*
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

# Tracing Framework Guide

**Part I: Core Blockchain Implementation** | **Technical Reference: Tracing Structured Logging Framework**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Tracing Framework Guide** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

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

> **📘 See the full implementation**: This guide explains Tracing concepts. To see how Tracing is used in our complete web API architecture, see the [Middleware Layer](05-Middleware.md) chapter for logging middleware, the [Server Setup](02-Server-Setup.md) chapter for logging configuration, and the [Error Handling](07-Error-Handling.md) chapter for error logging.

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

Tracing provides five logging levels, ordered from most to least severe:

### ERROR

Errors represent failures that prevent an operation from completing:

```rust
use tracing::error;

error!("Failed to create transaction: {}", e);
error!("Database connection failed: {}", err);
```

**When to use:**
- Operation failures
- Unrecoverable errors
- Critical system issues

**Example from our codebase:**

```rust
// In server.rs - Error handling middleware
tracing::error!(
    "[handle_errors]: Error response ({}): {}",
    parts.status,
    body_str
);
```

### WARN

Warnings indicate potentially problematic situations that don't prevent operation:

```rust
use tracing::warn;

warn!("Rate limit approaching: {} requests remaining", remaining);
warn!("Deprecated API endpoint used: {}", path);
```

**When to use:**
- Deprecated features
- Performance concerns
- Recoverable issues

### INFO

Info logs record normal operation events:

```rust
use tracing::info;

info!("Transaction {} submitted successfully", txid);
info!("Starting web server on {} with graceful shutdown", addr);
```

**When to use:**
- Normal operation events
- Important state changes
- Request/response logging

**Examples from our codebase:**

```rust
// In server.rs - Server startup
tracing::info!("Starting web server on {} with graceful shutdown", addr);

// In handlers/transaction.rs - Transaction submission
info!("Transaction {} submitted successfully", txid);

// In server.rs - Shutdown signal
tracing::info!("Shutdown signal received");
```

### DEBUG

Debug logs provide detailed diagnostic information:

```rust
use tracing::debug;

debug!("Processing block with hash: {}", hash);
debug!("Wallet balance: {} satoshis", balance);
```

**When to use:**
- Detailed diagnostic information
- Development debugging
- Step-by-step operation tracking

**Example from our codebase:**

```rust
// In primitives/transaction.rs
use tracing::debug;

debug!("Transaction inputs: {:?}", inputs);
debug!("Transaction outputs: {:?}", outputs);
```

### TRACE

Trace logs provide the most detailed diagnostic information:

```rust
use tracing::trace;

trace!("Entering function with parameters: {:?}", params);
trace!("Internal state: {:?}", state);
```

**When to use:**
- Very detailed diagnostics
- Internal implementation details
- Performance profiling

---

## Structured Logging

Tracing supports structured logging with key-value pairs, making logs easier to parse and query.

### Basic Structured Logging

Instead of formatting strings, use key-value pairs:

```rust
use tracing::info;

// Traditional logging (less structured)
info!("Transaction {} submitted from {} to {}", txid, from, to);

// Structured logging (better)
info!(
    txid = %txid,
    from = %from,
    to = %to,
    "Transaction submitted"
);
```

**Benefits:**
- **Queryable**: Log aggregation tools can filter by fields
- **Parseable**: Structured data is easier to parse
- **Extensible**: Easy to add more fields

### Field Syntax

Tracing supports different field syntax:

```rust
use tracing::info;

let txid = "abc123";
let amount = 1000;
let address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";

// Display formatting (%)
info!(txid = %txid, "Transaction ID");

// Debug formatting (?)
info!(address = ?address, "Wallet address");

// Both
info!(
    txid = %txid,
    amount,
    address = ?address,
    "Transaction details"
);
```

**Formatting Options:**
- `field = value`: Uses `Debug` formatting
- `field = %value`: Uses `Display` formatting (for strings, numbers)
- `field = ?value`: Uses `Debug` formatting (for structs, enums)
- `field`: Shorthand for `field = field` (uses `Debug`)

### How Structured Logging Conversion Works

Understanding how Tracing converts structured key-value pairs helps you use it effectively. Here's the mechanism:

#### 1. Procedural Macro Expansion

Tracing macros (`info!`, `error!`, `debug!`, etc.) are **procedural macros** that expand at compile time. When you write:

```rust
info!(
    txid = %txid,
    from = %from,
    to = %to,
    "Transaction submitted"
);
```

The macro expands this into code that:
1. Creates a structured event with named fields
2. Formats each field according to its operator (`%`, `?`, or none)
3. Passes the event to the tracing subscriber

#### 2. Field Value Conversion

The conversion happens through Rust's trait system:

**Display Formatting (`%`):**
```rust
txid = %txid
```

**What happens:**
- The `%` operator tells Tracing to use the `Display` trait
- At compile time, the macro generates: `txid: Value::from(format!("{}", txid))`
- The value is converted to a string using `Display::fmt()`
- This is efficient for strings, numbers, and types that implement `Display`

**Debug Formatting (`?`):**
```rust
address = ?address
```

**What happens:**
- The `?` operator tells Tracing to use the `Debug` trait
- At compile time, the macro generates: `address: Value::from(format!("{:?}", address))`
- The value is converted to a string using `Debug::fmt()`
- This is useful for structs, enums, and complex types

**Default Formatting (no operator):**
```rust
amount
```

**What happens:**
- No operator defaults to `Debug` formatting
- Equivalent to `amount = ?amount`
- The macro generates: `amount: Value::from(format!("{:?}", amount))`

#### 3. Value Type System

Tracing uses a `Value` type that can represent different data types:

```rust
// Internal representation (simplified)
enum Value {
    Debug(String),      // From Debug formatting
    Display(String),    // From Display formatting
    I64(i64),           // For integers (when possible)
    U64(u64),           // For unsigned integers
    Bool(bool),         // For booleans
    // ... other types
}
```

When you write `txid = %txid`, Tracing:
1. Evaluates `txid` (gets the actual value)
2. Calls `Display::fmt(&txid, &mut formatter)` to format it
3. Stores the formatted string in a `Value::Display(String)`
4. Associates it with the field name `"txid"`

#### 4. Event Creation

The macro creates an `Event` struct containing:

```rust
// Simplified internal structure
struct Event {
    level: Level,           // INFO, ERROR, etc.
    message: String,        // "Transaction submitted"
    fields: Vec<Field>,     // [(txid, Value), (from, Value), (to, Value)]
    metadata: Metadata,     // Module, file, line number
}
```

#### 5. Subscriber Processing

The `tracing-subscriber` processes the event:

**Human-Readable Format (fmt layer):**
```
2024-01-15T10:30:00.123456Z  INFO blockchain::web::handlers::transaction: Transaction submitted txid=abc123 from=1A1zP... to=1BvBM...
```

**JSON Format (json layer):**
```json
{
  "timestamp": "2024-01-15T10:30:00.123456Z",
  "level": "INFO",
  "target": "blockchain::web::handlers::transaction",
  "message": "Transaction submitted",
  "fields": {
    "txid": "abc123",
    "from": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
    "to": "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2"
  }
}
```

#### 6. Complete Example Breakdown

When you write:

```rust
let txid = "abc123";
let from = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
let to = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";

info!(
    txid = %txid,
    from = %from,
    to = %to,
    "Transaction submitted"
);
```

**Compile-time expansion (simplified):**
```rust
// The macro expands to something like:
tracing::Event::dispatch(
    &tracing::Metadata::new(
        "Transaction submitted",
        "blockchain::web::handlers",
        tracing::Level::INFO,
        // ... other metadata
    ),
    &tracing::valueset! {
        txid = %txid,  // Calls Display::fmt
        from = %from,  // Calls Display::fmt
        to = %to,      // Calls Display::fmt
    }
);
```

**Runtime execution:**
1. `txid` is evaluated (gets `"abc123"`)
2. `Display::fmt(&txid, &mut formatter)` is called → `"abc123"`
3. Field `"txid"` with value `"abc123"` is stored
4. Same process for `from` and `to`
5. Event is dispatched to all active subscribers
6. Subscriber formats and outputs the log

#### 7. Performance Considerations

**Lazy Evaluation:**
- Field values are only evaluated if the log level is enabled
- If `INFO` level is disabled, `txid`, `from`, and `to` are never evaluated
- This is zero-cost when logging is disabled

**Formatting Overhead:**
- `%` (Display) is typically faster than `?` (Debug) for simple types
- Complex `Debug` formatting can be expensive
- Use `%` for strings/numbers, `?` only when you need detailed structure

**Example:**
```rust
// Efficient: Display formatting for simple types
info!(txid = %txid, amount = amount, "Transaction");

// More expensive: Debug formatting for complex types
info!(transaction = ?complex_struct, "Processing");
```

#### 8. Why This Design?

**Benefits:**
- **Type Safety**: Compile-time checking of field names and types
- **Performance**: Zero-cost when disabled, efficient when enabled
- **Flexibility**: Multiple formatting options for different use cases
- **Structured Data**: Fields remain separate, not just formatted strings
- **Queryability**: Log aggregation tools can filter/search by field names

**Comparison to Traditional Logging:**
```rust
// Traditional: Everything is a string
log::info!("Transaction {} submitted from {} to {}", txid, from, to);
// Output: "Transaction abc123 submitted from 1A1zP... to 1BvBM..."
// Can't filter by txid, from, or to - must parse the string

// Structured: Fields are separate
info!(txid = %txid, from = %from, to = %to, "Transaction submitted");
// Output: Structured fields that can be queried
// Can filter: WHERE txid = "abc123" OR from = "1A1zP..."
```

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
        .btc_transaction(&request.from_address, &request.to_address, request.amount)
        .await
        .map_err(|e| {
            error!(
                error = %e,
                from = %request.from_address,
                to = %request.to_address,
                amount = request.amount,
                "Failed to create transaction"
            );
            StatusCode::BAD_REQUEST
        })?;

    info!(
        txid = %txid,
        from = %request.from_address,
        to = %request.to_address,
        amount = request.amount,
        "Transaction submitted successfully"
    );

    // ... rest of handler
}
```

This structured logging provides:
- **Queryable fields**: Filter logs by `txid`, `from`, `to`, `amount`
- **Context**: All relevant information in one log entry
- **Consistency**: Same structure across all transaction logs

---

### Example Log Output

Here's what the actual log output looks like for structured logging:

**Code:**

```rust
let txid = "a1b2c3d4e5f6";
let from_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
let amount = 1000;

info!(
    txid = %txid,
    from = %from_address,
    amount = amount,
    "Transaction submitted"
);
```

**Human-Readable Output (fmt layer - default):**

```text
2024-01-15T10:30:45.123456Z  INFO blockchain::web::handlers::transaction: Transaction submitted txid=a1b2c3d4e5f6 from=1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa amount=1000
```

**Detailed Human-Readable Output (with target and span info):**

```text
2024-01-15T10:30:45.123456Z  INFO blockchain::web::handlers::transaction
    at bitcoin/src/web/handlers/transaction.rs:43
    in send_transaction
Transaction submitted
    txid: "a1b2c3d4e5f6"
    from: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
    amount: 1000
```

**JSON Output (json layer):**

```json
{
  "timestamp": "2024-01-15T10:30:45.123456Z",
  "level": "INFO",
  "fields": {
    "message": "Transaction submitted",
    "txid": "a1b2c3d4e5f6",
    "from": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
    "amount": 1000
  },
  "target": "blockchain::web::handlers::transaction",
  "span": {
    "name": "send_transaction"
  },
  "spans": [
    {
      "name": "send_transaction",
      "txid": "a1b2c3d4e5f6"
    }
  ]
}
```

**Compact JSON Output:**

```json
{
  "ts": "2024-01-15T10:30:45.123456Z",
  "level": "INFO",
  "msg": "Transaction submitted",
  "txid": "a1b2c3d4e5f6",
  "from": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
  "amount": 1000,
  "target": "blockchain::web::handlers::transaction"
}
```

**Key Observations:**

1. **Field Names Preserved**: `txid`, `from`, and `amount` appear as separate fields, not just in the message
2. **Formatting Applied**: 
   - `txid = %txid` → Uses `Display` formatting → `"a1b2c3d4e5f6"`
   - `from = %from_address` → Uses `Display` formatting → `"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"`
   - `amount = amount` → Uses `Debug` formatting (default) → `1000` (as number in JSON, or `1000` in text)
3. **Queryable**: Log aggregation tools can filter by:
   - `txid = "a1b2c3d4e5f6"`
   - `from = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"`
   - `amount = 1000`
4. **Structured**: Fields are separate from the message, making parsing and querying easy

**Comparison with Traditional Logging:**

**Traditional (string formatting):**

```rust
info!("Transaction {} submitted from {} with amount {}", txid, from_address, amount);
```

**Output:**

```text
2024-01-15T10:30:45.123456Z  INFO blockchain::web::handlers::transaction: Transaction a1b2c3d4e5f6 submitted from 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa with amount 1000
```

**Problems:**

- Can't filter by `txid` alone (must parse the entire string)
- Can't query `amount > 500` (must extract and parse)
- Harder to parse programmatically
- All data is in one string

**Structured (Tracing):**

```rust
info!(
    txid = %txid,
    from = %from_address,
    amount = amount,
    "Transaction submitted"
);
```

**Benefits:**

- Can filter: `WHERE txid = "a1b2c3d4e5f6"`
- Can query: `WHERE amount > 500`
- Easy to parse (JSON or structured format)
- Fields are separate and typed

---

## Spans

Spans represent periods of time during which a program was executing in a particular context. They provide contextual information that persists across function calls.

### Creating Spans

Spans can be created explicitly or using the `#[instrument]` attribute:

**Explicit Span:**

```rust
use tracing::{info_span, Instrument};

let span = info_span!("process_transaction", txid = %txid);
let _guard = span.enter();

// All logs within this scope include span context
info!("Processing transaction");
```

**Instrument Attribute:**

```rust
use tracing::instrument;

#[instrument]
async fn process_transaction(txid: String) {
    // This function automatically creates a span
    // All logs include the function name and parameters
    info!("Processing transaction");
}
```

### Span Fields

Spans can include fields that are automatically included in all logs within the span:

```rust
use tracing::instrument;

#[instrument(
    fields(
        txid = %txid,
        from = %from_address,
        to = %to_address,
        amount = amount
    )
)]
async fn send_transaction(txid: String, from_address: String, to_address: String, amount: i32) {
    info!("Starting transaction processing");
    // All logs here include txid, from, to, amount
}
```

### Example from Our Project

**In `node/server.rs`:**

```rust
use tracing::{error, info, instrument};

#[instrument]
pub async fn start_server(node: Arc<NodeContext>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting blockchain node server");
    
    // All logs within this function include the span context
    // Span name: "start_server"
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Server listening on {}", addr);
    
    // ... rest of function
}
```

**Benefits:**
- **Automatic Context**: Function name and parameters automatically included
- **Duration Tracking**: Spans automatically track function execution time
- **Nested Context**: Spans can be nested to show call hierarchy

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

You can customize TraceLayer behavior:

```rust
use tower_http::trace::TraceLayer;
use tracing::Span;

TraceLayer::new_for_http()
    .make_span_with(|request: &Request| {
        tracing::info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
        )
    })
    .on_request(|request: &Request, _span: &Span| {
        tracing::info!(
            method = %request.method(),
            uri = %request.uri(),
            "Started processing request"
        );
    })
    .on_response(|response: &Response, latency: Duration, _span: &Span| {
        tracing::info!(
            status = %response.status(),
            latency = ?latency,
            "Finished processing request"
        );
    })
    .on_failure(|error: &Error, latency: Duration, _span: &Span| {
        tracing::error!(
            error = %error,
            latency = ?latency,
            "Request failed"
        );
    })
```

**Customization Options:**

- **`make_span_with()`**: Customize span creation with specific fields
- **`on_request()`**: Callback when request starts
- **`on_response()`**: Callback when response is ready
- **`on_failure()`**: Callback for errors
- **`on_eos()`**: Callback when connection ends

> **See it in action**: Check out the [Logging Middleware section](Tower.md#logging-middleware) in the Tower Framework Guide for more details on TraceLayer configuration and usage.

---

## Logging Configuration

Tracing uses `tracing-subscriber` to configure log output format and filtering.

### Basic Configuration

**In `main.rs`:**

```rust
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    fmt,
};

fn initialize_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .init();
}
```

**What This Does:**

1. **EnvFilter**: Reads log level from `RUST_LOG` environment variable
2. **Default Level**: Sets INFO as default if `RUST_LOG` not set
3. **Format Layer**: Configures output format (human-readable)
4. **Registry**: Combines multiple layers

### Environment Variable Configuration

Set log level via `RUST_LOG` environment variable:

```bash
# Set global log level
RUST_LOG=info ./blockchain

# Set specific module log level
RUST_LOG=blockchain::web=debug,blockchain::node=info ./blockchain

# Set multiple modules
RUST_LOG=blockchain::web::handlers=debug,blockchain::web::server=info ./blockchain

# Enable trace level for specific module
RUST_LOG=blockchain::web::handlers=trace ./blockchain
```

**Log Level Syntax:**

- `module=level`: Set level for specific module
- `module::submodule=level`: Set level for nested module
- `level`: Set global level
- `module1=level1,module2=level2`: Multiple modules

### Example from Our Project

**In `main.rs`:**

```rust
/// Initialize logging with functional configuration
fn initialize_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .init();
}

fn main() {
    initialize_logging();
    
    // Now all tracing macros work
    info!("Application starting");
    error!("Something went wrong");
}
```

**Configuration Details:**

- **Default Level**: INFO (filters out DEBUG and TRACE in production)
- **Environment Override**: `RUST_LOG` environment variable can override
- **Format**: Human-readable format suitable for development
- **Filtering**: Compile-time and runtime filtering for performance

---

## Examples from Our Project

Let's examine real examples of Tracing usage throughout our blockchain API.

### Server Lifecycle Logging

**In `server.rs`:**

```rust
/// Start the web server with graceful shutdown
pub async fn start_with_shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
    let app = self.create_app();
    let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));

    tracing::info!("Starting web server on {} with graceful shutdown", addr);

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

**In `server.rs` - Error Handling Middleware:**

```rust
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;

    if response.status().is_server_error() || response.status().is_client_error() {
        let (parts, body) = response.into_parts();
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();
        let body_str = String::from_utf8_lossy(&body_bytes);
        
        tracing::error!(
            "[handle_errors]: Error response ({}): {}",
            parts.status,
            body_str
        );
        
        // ... error handling ...
    }

    Ok(response)
}
```

**What Gets Logged:**
- HTTP status code
- Error response body
- Full error context

### Handler Logging

**In `handlers/transaction.rs`:**

```rust
use tracing::{error, info};

pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    let txid = node
        .btc_transaction(&request.from_address, &request.to_address, request.amount)
        .await
        .map_err(|e| {
            error!("Failed to create transaction: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    info!("Transaction {} submitted successfully", txid);

    // ... create response ...
}
```

**What Gets Logged:**
- Transaction submission success
- Transaction creation failures
- Transaction ID for correlation

### Instrumented Functions

**In `node/server.rs`:**

```rust
use tracing::{error, info, instrument};

#[instrument]
pub async fn start_server(node: Arc<NodeContext>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting blockchain node server");
    
    // Function automatically creates a span
    // All logs include function name and parameters
    
    // ... server startup logic ...
}
```

**Benefits:**
- Automatic span creation
- Function parameters included in logs
- Duration tracking
- Nested context in call hierarchy

---

## Best Practices

### 1. Use Appropriate Log Levels

- **ERROR**: Only for actual errors that prevent operation
- **WARN**: For recoverable issues or deprecations
- **INFO**: For normal operation events
- **DEBUG**: For detailed diagnostics (disabled in production)
- **TRACE**: For very detailed tracing (disabled in production)

### 2. Use Structured Logging

**Prefer:**
```rust
info!(
    txid = %txid,
    from = %from_address,
    amount = amount,
    "Transaction submitted"
);
```

**Over:**
```rust
info!("Transaction {} submitted from {} with amount {}", txid, from_address, amount);
```

### 3. Include Context

Always include relevant context in error logs:

```rust
error!(
    error = %e,
    txid = %txid,
    from = %from_address,
    to = %to_address,
    "Failed to process transaction"
);
```

### 4. Use Spans for Operations

Use `#[instrument]` for functions that represent operations:

```rust
#[instrument]
async fn process_transaction(txid: String) {
    // Automatic span with function name and parameters
}
```

### 5. Avoid Logging Sensitive Data

Never log:
- Passwords
- API keys
- Private keys
- Personal information

### 6. Use Environment-Based Configuration

Configure log levels via `RUST_LOG` environment variable:

```bash
# Development
RUST_LOG=debug ./blockchain

# Production
RUST_LOG=info ./blockchain
```

---

## Performance Considerations

### Compile-Time Filtering

Tracing uses compile-time filtering to remove logging code when disabled:

```rust
// This code is removed at compile time if DEBUG level is disabled
debug!("Detailed diagnostic: {:?}", data);
```

**Benefits:**
- Zero runtime cost when disabled
- No performance impact in production
- Conditional compilation

### Runtime Filtering

Even when compiled, logs can be filtered at runtime:

```rust
// Set via RUST_LOG environment variable
RUST_LOG=info  // DEBUG and TRACE logs are filtered at runtime
```

### Structured Logging Overhead

Structured logging has minimal overhead:
- Key-value pairs are efficient
- Fields are only evaluated if log level is enabled
- Spans are lightweight

### Production Recommendations

For production:
- Set default level to INFO
- Use structured logging for queryability
- Enable DEBUG/TRACE only for specific modules when needed
- Use log aggregation tools (e.g., ELK, Loki, Datadog)

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
- **[Tower Framework Guide](Tower.md)**: How Tower's TraceLayer integrates with Tracing
- **[Middleware Layer](05-Middleware.md)**: How logging middleware is implemented
- **[Server Setup](02-Server-Setup.md)**: How logging is configured in our server
- **[Error Handling](07-Error-Handling.md)**: How errors are logged
- **[Axum Framework Guide](Axum.md)**: How Axum handlers use Tracing
- **[Tokio Runtime Guide](../Tokio.md)**: Async runtime that Tracing works with
- **[Rust Language Guide](../../rust/README.md)**: Rust language features used throughout

---

<div align="center">

**📚 [← Web API Index](README.md)** | **Tracing Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **[Axum](Axum.md)** | **[Tower](Tower.md)** | **[Serde](Serde.md)** | **[Utoipa](Utoipa.md)** | **[Tokio](../Tokio.md)** 📚

</div>

---

*This guide provides detailed explanations of Tracing framework features used in our blockchain API. For implementation details, see the [Middleware Layer](05-Middleware.md) chapter.*
