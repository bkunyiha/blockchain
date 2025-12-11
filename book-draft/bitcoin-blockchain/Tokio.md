<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../README.md)
2. [Chapter 2: Transaction System](02-Transaction-System.md)
3. **Chapter 3: Web API Architecture**
   - [Web API Index](web/README.md) - Overview and navigation
   - [01: Introduction](web/01-Introduction.md) - Architecture overview
   - [02: Server Setup](web/02-Server-Setup.md) - Server configuration
   - [03: Routing](web/03-Routing.md) - Route definitions
   - [04: Handlers](web/04-Handlers.md) - Request handlers
   - [05: Middleware](web/05-Middleware.md) - Middleware layer
   - [06: Data Models](web/06-Data-Models.md) - Request/response models
   - [07: Error Handling](web/07-Error-Handling.md) - Error management
   - [08: OpenAPI](web/08-OpenAPI.md) - API documentation
   - [09: Security](web/09-Security.md) - Security architecture
   - [10: Best Practices](web/10-Best-Practices.md) - Design patterns
   - [Axum Framework Guide](web/Axum.md) - Framework reference
   - [Tower Framework Guide](web/Tower.md) - Middleware framework
   - [Serde Framework Guide](web/Serde.md) - Serialization framework
   - [Utoipa Framework Guide](web/Utoipa.md) - OpenAPI framework
   - [Tokio Runtime Guide](Tokio.md) - Async runtime framework ← *You are here*
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

**[← Back to Web API Index](web/README.md)** | **[← Back to Main Book](../README.md)**

</div>

---

# Tokio Runtime Guide

**Part I: Core Blockchain Implementation** | **Technical Reference: Tokio Async Runtime**

<div align="center">

**[← Back to Web API Index](web/README.md)** | **Tokio Runtime Guide** | **[Web API Architecture Index →](web/01-Introduction.md)** 📚

</div>

---

## Overview

This guide provides detailed explanations of the Tokio async runtime and how it's used throughout our blockchain implementation. Tokio is a production-ready asynchronous runtime for Rust that provides the foundation for all async operations in our blockchain node, web API, and desktop applications.

In our blockchain project, Tokio powers:
- **Network I/O**: TCP connections, peer communication, message processing
- **Concurrent Operations**: Parallel task execution, background processing
- **Web Server**: HTTP request handling, graceful shutdown
- **Database Operations**: Async database access, concurrent reads/writes
- **Signal Handling**: Graceful shutdown on Ctrl+C
- **Task Coordination**: Task spawning, synchronization, cancellation

> **📘 See the full implementation**: This guide explains Tokio concepts with examples from our codebase. To see how Tokio integrates with our web API, see the [Server Setup](web/02-Server-Setup.md) chapter. For network operations, see the blockchain node implementation.

---

## Table of Contents

1. [What is Tokio?](#what-is-tokio) - Understanding Tokio's role
2. [Async Runtime Fundamentals](#async-runtime-fundamentals) - Core concepts
3. [Runtime Creation and Management](#runtime-creation-and-management) - Creating and managing runtimes
4. [Task Spawning](#task-spawning) - Concurrent task execution
5. [Concurrent Operations with select!](#concurrent-operations-with-select) - Handling multiple futures
6. [Network I/O](#network-io) - TCP listeners and streams
7. [Async Synchronization](#async-synchronization) - Locks and channels
8. [Signal Handling](#signal-handling) - Graceful shutdown
9. [Bridging Sync and Async](#bridging-sync-and-async) - Integrating with synchronous code
10. [Best Practices](#best-practices) - Production patterns

---

## What is Tokio?

Tokio is an asynchronous runtime for Rust that provides:
- **Async I/O**: Non-blocking network and file operations
- **Task Scheduling**: Efficient task execution on multiple threads
- **Timer Support**: Delayed and periodic operations
- **Synchronization Primitives**: Async locks, channels, and barriers
- **Signal Handling**: OS signal processing for graceful shutdown

### Tokio's Role in Our Blockchain

Our blockchain uses Tokio extensively:

**Network Layer** (`bitcoin/src/node/server.rs`):
- TCP listener for peer connections
- Concurrent connection handling
- Message processing in background tasks

**Web API** (`bitcoin/src/web/server.rs`):
- HTTP server built on Tokio
- Async request handlers
- Graceful shutdown handling

**Blockchain Operations** (`bitcoin/src/chain/chainstate.rs`):
- Async database access with `TokioRwLock`
- Concurrent read/write operations
- Background transaction processing

**Desktop Applications** (`bitcoin-desktop-ui/src/runtime.rs`):
- Global runtime for HTTP client operations
- Bridging Iced UI framework with async operations

**Desktop Wallet Application** (`bitcoin-wallet-ui/src/runtime.rs`):
- Global runtime for HTTP client operations
- Bridging Iced UI framework with async operations
- Wallet-specific async operations and API calls

### Why Tokio?

- **Performance**: Efficient task scheduling and I/O multiplexing
- **Scalability**: Handles thousands of concurrent connections
- **Ecosystem**: Widely used, well-maintained, production-ready
- **Integration**: Works seamlessly with Axum, Tower, and other async libraries

---

## Async Runtime Fundamentals

### What is an Async Runtime?

An async runtime is responsible for:
1. **Polling Futures**: Checking if async operations are ready
2. **Scheduling Tasks**: Deciding which tasks to run and when
3. **I/O Event Loop**: Monitoring I/O events and waking waiting tasks
4. **Thread Pool Management**: Distributing work across threads

### Tokio Runtime Components

- **Reactor**: Handles I/O events (network, file system)
- **Scheduler**: Manages task execution and scheduling
- **Timer**: Provides time-based operations (delays, intervals)

### Async/Await Syntax

Rust's `async`/`await` syntax creates futures that Tokio executes:

```rust
// Async function returns a Future
async fn process_block(block: Block) -> Result<()> {
    // .await yields control back to the runtime
    let result = validate_block(&block).await?;
    Ok(result)
}
```

**How it works:**
1. `async fn` creates a state machine (Future)
2. `.await` points yield control back to Tokio
3. Tokio polls the future when I/O completes
4. Future resumes execution from the `.await` point

---

## Runtime Creation and Management

### Creating a Runtime

In our main blockchain node (`bitcoin/src/main.rs`), Tokio runtime is created automatically by the `#[tokio::main]` attribute:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Runtime is created automatically
    // All async code runs on this runtime
}
```

**What `#[tokio::main]` does:**
- Expands to create a `Runtime::new()`
- Calls `block_on()` to run the async main function
- Handles runtime lifecycle automatically

### Manual Runtime Creation

In our desktop applications (`bitcoin-desktop-ui/src/runtime.rs` and `bitcoin-wallet-ui/src/runtime.rs`), we create a runtime manually:

```rust
pub fn init_runtime() {
    // Create a Tokio runtime for async operations
    // This must outlive the application to keep the reactor running
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime");

    // Store the handle globally so it can be accessed from any thread
    TOKIO_HANDLE
        .set(rt.handle().clone())
        .expect("Failed to set Tokio handle");

    // Keep the runtime alive in a background thread
    std::thread::spawn(move || {
        rt.block_on(async {
            // Keep the runtime alive indefinitely
            std::future::pending::<()>().await;
        });
    });
}
```

**Why Manual Creation?**

Desktop applications (both the Admin UI and Wallet UI using Iced) run on their own event loop. We need a separate Tokio runtime for HTTP client operations (`reqwest`). The runtime must:
1. **Outlive the Application**: Stay alive for the entire application lifetime
2. **Be Accessible Globally**: Available from any thread via `OnceLock`
3. **Run in Background**: Execute in a separate thread to not block the UI

### Runtime Handle

The runtime handle (`tokio::runtime::Handle`) allows spawning tasks on an existing runtime:

```rust
static TOKIO_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

// Later, spawn tasks using the handle
let handle = TOKIO_HANDLE.get().expect("Runtime not initialized");
handle.spawn(async {
    // This task runs on the Tokio runtime
    make_http_request().await;
});
```

**Handle Benefits:**
- **Cloneable**: Can be cloned and shared
- **Thread-Safe**: Can be used from any thread
- **Lightweight**: Doesn't own the runtime, just references it

---

## Task Spawning

Task spawning allows concurrent execution of async operations. Tokio provides `tokio::spawn` to create independent tasks.

### Basic Task Spawning

In our network processing (`bitcoin/src/net/net_processing.rs`), we spawn tasks for concurrent message sending:

```rust
// Send known nodes to multiple peers concurrently
nodes_to_add.iter().for_each(|node| {
    let node_addr = *node;
    let all_nodes = all_known_nodes_addresses.clone();
    tokio::spawn(async move {
        send_known_nodes(&node_addr, all_nodes).await;
    });
});
```

**What happens:**
1. `tokio::spawn` creates a new task
2. Task is scheduled on Tokio's thread pool
3. Task executes independently of the caller
4. Caller continues immediately (fire-and-forget)

### Spawning with JoinHandle

Tasks can return values via `JoinHandle`:

```rust
// In bitcoin/src/main.rs
let network_handle = tokio::spawn(async move {
    network_server
        .run_with_shutdown(&socket_addr, connect_nodes_set, net_shutdown_rx)
        .await;
});

// Later, wait for completion
let _ = network_handle.await;
```

**JoinHandle Benefits:**
- **Await Results**: Wait for task completion
- **Cancel Tasks**: Abort tasks if needed
- **Error Handling**: Propagate errors from spawned tasks

### Background Transaction Processing

In our node context (`bitcoin/src/node/context.rs`), we spawn background tasks for transaction processing:

```rust
// Submit transaction for mining and broadcast in background
// This prevents blocking the API response
let context = self.clone();
let addr_copy = *addr_from;
let tx = utxo.clone();
tokio::spawn(async move {
    let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
});

// Return transaction ID immediately
Ok(utxo.get_tx_id_hex())
```

**Why Background Processing?**

- **Non-Blocking API**: API handler returns immediately
- **Concurrent Execution**: Mining and broadcasting happen in parallel
- **Better UX**: Users don't wait for background operations

### Connection Handling

In our network server (`bitcoin/src/node/server.rs`), each connection is handled in a spawned task:

```rust
accept_res = listener.accept() => {
    match accept_res {
        Ok((stream, _peer)) => {
            let blockchain = self.node_context.clone();
            tokio::spawn(async move {
                // Process each connection concurrently
                match stream.into_std() {
                    Ok(std_stream) => {
                        if let Err(e) = net_processing::process_stream(blockchain, std_stream).await {
                            error!("Serve error: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to convert stream: {}", e);
                    }
                }
            });
        }
        Err(e) => {
            error!("accept error: {}", e);
        }
    }
}
```

**Concurrent Connection Handling:**
- Each connection spawns a new task
- Multiple connections processed simultaneously
- Isolated error handling per connection
- No blocking of the accept loop

---

## Concurrent Operations with select!

`tokio::select!` allows waiting on multiple futures simultaneously, executing the first one that completes.

### Basic select! Usage

In our network server (`bitcoin/src/node/server.rs`), we use `select!` to handle shutdown and connections:

```rust
loop {
    tokio::select! {
        _ = shutdown.recv() => {
            info!("Network server shutdown signal received");
            break;
        }
        accept_res = listener.accept() => {
            match accept_res {
                Ok((stream, _peer)) => {
                    // Handle connection
                }
                Err(e) => {
                    error!("accept error: {}", e);
                }
            }
        }
    }
}
```

**How select! Works:**
1. **Polls All Branches**: Checks all futures simultaneously
2. **First Ready Wins**: Executes the branch whose future completes first
3. **Cancels Others**: Other branches are cancelled (if they support cancellation)
4. **Non-Blocking**: Doesn't block waiting for a specific future

### Multiple Server Coordination

In our main function (`bitcoin/src/main.rs`), we coordinate multiple servers with `select!`:

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        info!("Ctrl-C received, initiating shutdown...");
        let _ = shutdown_tx.send(());
    }
    _ = &mut web_handle => {
        info!("Web server task finished");
    }
    _ = &mut network_handle => {
        info!("Network server task finished");
    }
}
```

**Server Coordination:**
- **Shutdown Signal**: Ctrl+C triggers graceful shutdown
- **Server Completion**: Either server finishing triggers cleanup
- **Concurrent Monitoring**: All servers monitored simultaneously
- **Graceful Shutdown**: Broadcast shutdown signal to all servers

### Why Mutable References?

Note the `&mut` in `&mut web_handle`. This is required because:
- `select!` polls futures by mutable reference
- `JoinHandle` must be mutable to poll
- Shadowing creates mutable bindings:

```rust
// Shadow to make mutable
let mut web_handle = web_handle;
tokio::select! {
    _ = &mut web_handle => { /* ... */ }
}
```

---

## Network I/O

Tokio provides async network primitives for efficient I/O operations.

### TCP Listener

In our web server (`bitcoin/src/web/server.rs`), we bind a TCP listener:

```rust
let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));
let listener = tokio::net::TcpListener::bind(addr).await?;
```

**Async Binding:**
- `.await` yields control during bind operation
- Non-blocking: doesn't block the thread
- Returns immediately if binding fails

### TCP Listener in Network Server

In our network server (`bitcoin/src/node/server.rs`), we use `TcpListener` for peer connections:

```rust
use tokio::net::TcpListener;

// Create listener
let listener = TcpListener::bind(socket_addr).await?;

// Accept connections in a loop
loop {
    tokio::select! {
        accept_res = listener.accept() => {
            // Handle new connection
        }
    }
}
```

**Connection Acceptance:**
- `accept()` is async and non-blocking
- Returns `(TcpStream, SocketAddr)` when connection arrives
- Can be used in `select!` for concurrent operations

### Stream Processing

TCP streams are processed asynchronously:

```rust
tokio::spawn(async move {
    match stream.into_std() {
        Ok(std_stream) => {
            // Process stream asynchronously
            net_processing::process_stream(blockchain, std_stream).await
        }
    }
});
```

**Async Stream Processing:**
- Each stream processed in its own task
- Non-blocking I/O operations
- Concurrent stream handling

---

## Async Synchronization

Tokio provides async versions of synchronization primitives that don't block threads.

### TokioRwLock

In our blockchain service (`bitcoin/src/chain/chainstate.rs`), we use `TokioRwLock` for concurrent access:

```rust
use tokio::sync::RwLock as TokioRwLock;

pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl BlockchainService {
    pub async fn initialize(genesis_address: &WalletAddress) -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::create_blockchain(genesis_address).await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
}
```

**Why TokioRwLock?**

- **Async-Aware**: `.await` points yield control instead of blocking
- **Concurrent Reads**: Multiple readers can access simultaneously
- **Exclusive Writes**: Writers get exclusive access
- **Non-Blocking**: Doesn't block threads, yields to runtime

### Read Operations

Reading from the lock:

```rust
async fn read<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(BlockchainFileSystem) -> Fut + Send,
    Fut: Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    let blockchain = self.0.read().await; // ← Async lock acquisition
    f(blockchain.clone()).await
}
```

**Read Lock Behavior:**
- `.await` yields if lock is held by writer
- Multiple readers can hold lock simultaneously
- Lock released when guard is dropped

### Write Operations

Writing to the lock:

```rust
async fn write<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(&mut BlockchainFileSystem) -> Fut + Send,
    Fut: Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    let mut blockchain = self.0.write().await; // ← Exclusive write lock
    f(&mut *blockchain).await
}
```

**Write Lock Behavior:**
- `.await` yields if lock is held by readers or writers
- Exclusive access: only one writer at a time
- Blocks all readers and writers until released

### Broadcast Channels

In our main function (`bitcoin/src/main.rs`), we use broadcast channels for shutdown coordination:

```rust
// Centralized shutdown handling
let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(1);

// Subscribe to shutdown signal
let net_shutdown_rx = shutdown_tx.subscribe();

// Send shutdown signal
let _ = shutdown_tx.send(());
```

**Broadcast Channel Benefits:**
- **One-to-Many**: Single sender, multiple receivers
- **Non-Blocking**: Sends don't block if receivers aren't ready
- **Coordination**: Perfect for shutdown signals

### Receiving Shutdown Signals

Servers receive shutdown signals:

```rust
loop {
    tokio::select! {
        _ = shutdown.recv() => {
            info!("Network server shutdown signal received");
            break;
        }
        // ... other operations
    }
}
```

**Graceful Shutdown Pattern:**
1. Create broadcast channel
2. Subscribe in each server/task
3. Send shutdown signal when needed
4. Tasks check shutdown signal in `select!`
5. Tasks exit gracefully when signal received

---

## Signal Handling

Tokio provides async signal handling for graceful shutdown.

### Ctrl+C Handling

In our web server (`bitcoin/src/web/server.rs`), we handle Ctrl+C:

```rust
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
```

**Signal Handling:**
- `tokio::signal::ctrl_c()` returns a future
- Future completes when Ctrl+C is pressed
- Used with `with_graceful_shutdown` for clean shutdown

### Signal Handling in Main

In our main function (`bitcoin/src/main.rs`), we handle Ctrl+C for multiple servers:

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        info!("Ctrl-C received, initiating shutdown...");
        let _ = shutdown_tx.send(());
    }
    _ = &mut web_handle => {
        info!("Web server task finished");
    }
    _ = &mut network_handle => {
        info!("Network server task finished");
    }
}
```

**Multi-Server Shutdown:**
- Single Ctrl+C handler coordinates all servers
- Broadcasts shutdown signal to all servers
- Servers receive signal and shut down gracefully

---

## Bridging Sync and Async

Sometimes we need to bridge synchronous and asynchronous code.

### block_on

In our desktop runtimes (`bitcoin-desktop-ui/src/runtime.rs` and `bitcoin-wallet-ui/src/runtime.rs`), we use `block_on` to keep the runtime alive:

```rust
std::thread::spawn(move || {
    rt.block_on(async {
        // Keep the runtime alive indefinitely
        std::future::pending::<()>().await;
    });
});
```

**block_on Usage:**
- Runs async code from synchronous context
- Blocks current thread until future completes
- Used here to keep runtime alive in background thread

### Spawning from Sync Context

Our `spawn_on_tokio` function bridges Iced (sync) and Tokio (async):

```rust
pub fn spawn_on_tokio<F>(fut: F) -> impl std::future::Future<Output = F::Output> + Send
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    let handle = TOKIO_HANDLE
        .get()
        .expect("Tokio runtime not initialized")
        .clone();
    async move { handle.spawn(fut).await.unwrap() }
}
```

**Bridging Pattern:**
1. Get runtime handle from global storage
2. Spawn task on Tokio runtime
3. Return future that yields task result
4. Iced can `.await` the returned future

**Why This Pattern?**

- **Iced Executor**: Iced has its own executor for UI tasks (used in both Admin UI and Wallet UI)
- **Tokio Requirement**: HTTP clients (`reqwest`) need Tokio runtime
- **Bridge Function**: `spawn_on_tokio` bridges the two executors

---

## Best Practices

### 1. Prefer Async Over Blocking

Use async I/O operations instead of blocking ones:

```rust
// ✅ Good: Async TCP listener
let listener = tokio::net::TcpListener::bind(addr).await?;

// ❌ Bad: Blocking TCP listener (blocks thread)
let listener = std::net::TcpListener::bind(addr)?;
```

### 2. Spawn Tasks for Independent Work

Spawn tasks for operations that can run independently:

```rust
// ✅ Good: Spawn background task
tokio::spawn(async move {
    process_transaction(tx).await;
});

// ❌ Bad: Blocking the caller
process_transaction(tx).await; // Blocks until complete
```

### 3. Use select! for Concurrent Operations

Use `select!` when waiting on multiple operations:

```rust
// ✅ Good: Concurrent waiting
tokio::select! {
    result = operation1() => { /* ... */ }
    result = operation2() => { /* ... */ }
}

// ❌ Bad: Sequential waiting (slower)
let result1 = operation1().await;
let result2 = operation2().await;
```

### 4. Handle Errors in Spawned Tasks

Always handle errors in spawned tasks:

```rust
// ✅ Good: Error handling
tokio::spawn(async move {
    if let Err(e) = process().await {
        error!("Task error: {}", e);
    }
});

// ❌ Bad: Ignoring errors
tokio::spawn(async move {
    process().await; // Errors are lost
});
```

### 5. Use Async Locks for Shared State

Use `TokioRwLock` instead of `std::sync::RwLock` in async code:

```rust
// ✅ Good: Async lock
use tokio::sync::RwLock as TokioRwLock;
let lock = TokioRwLock::new(data);
let guard = lock.read().await;

// ❌ Bad: Blocking lock (blocks thread)
use std::sync::RwLock;
let lock = RwLock::new(data);
let guard = lock.read().unwrap(); // Blocks thread
```

### 6. Coordinate Shutdown Gracefully

Use broadcast channels for coordinated shutdown:

```rust
// ✅ Good: Broadcast shutdown
let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
// ... spawn tasks with shutdown_rx
shutdown_tx.send(()); // All tasks receive signal

// ❌ Bad: No coordination
// Tasks don't know when to shut down
```

### 7. Keep Runtime Handle Accessible

Store runtime handle globally for access from any thread:

```rust
// ✅ Good: Global handle
static TOKIO_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

// ❌ Bad: Runtime not accessible
// Can't spawn tasks from other threads
```

---

## Summary

Tokio provides the async runtime foundation for our blockchain:

- **Runtime Management**: Creating and managing async runtimes
- **Task Spawning**: Concurrent task execution with `tokio::spawn`
- **Concurrent Operations**: Coordinating multiple futures with `select!`
- **Network I/O**: Async TCP listeners and streams
- **Async Synchronization**: Non-blocking locks and channels
- **Signal Handling**: Graceful shutdown on Ctrl+C
- **Sync/Async Bridging**: Integrating with synchronous code

Tokio's design allows our blockchain to handle thousands of concurrent connections, process transactions in parallel, and coordinate multiple servers efficiently, all while maintaining excellent performance and resource utilization.

---

## Additional Resources

- **[Tokio Documentation](https://tokio.rs/)**: Official Tokio documentation and guides
- **[Tokio API Documentation](https://docs.rs/tokio/)**: Complete API reference
- **[Async Book](https://rust-lang.github.io/async-book/)**: Comprehensive async programming guide
- **[Server Setup](web/02-Server-Setup.md)**: How Tokio is used in our web server
- **[Axum Framework Guide](web/Axum.md)**: How Axum uses Tokio for async operations
- **[Tower Framework Guide](web/Tower.md)**: How Tower middleware runs on Tokio
- **[Rust Language Guide](../rust/README.md)**: Rust language features including async/await

---

<div align="center">

**📚 [← Back to Web API Index](web/README.md)** | **Tokio Runtime Guide** | **[Web API Architecture Index →](web/01-Introduction.md)** | **[Tower](web/Tower.md)** | **[Serde](web/Serde.md)** | **[Utoipa](web/Utoipa.md)** 📚

</div>

---

*This guide provides detailed explanations of Tokio runtime features used in our blockchain implementation. For implementation details, see the [Server Setup](web/02-Server-Setup.md) chapter and the blockchain node implementation.*
