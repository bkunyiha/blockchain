<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="primitives/README.md">Chapter 7: Primitives</a>
8. <a href="util/README.md">Chapter 8: Utilities</a>
9. <a href="crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="store/README.md">Chapter 20: Storage Layer</a>
21. <a href="net/README.md">Chapter 21: Network Layer</a>
22. <a href="node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="web/README.md">Chapter 24: Web API Architecture</a>
25. <a href="../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../Glossary.md">Glossary</a>
49. <a href="../Bibliography.md">Bibliography</a>
50. <a href="../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](web/README.md)** | **[← Back to Main Book](../README.md)**

</div>

---

# Tokio Runtime Guide

**Part I: Foundations & Core Implementation** | **Technical Reference: Tokio Async Runtime**

<div align="center">

**[← Back to Web API Index](web/README.md)** | **[Tokio Runtime Guide](Tokio.md)** | **[Web API Architecture Index →](web/README.md)**

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

> **See the full implementation:**: This guide explains Tokio concepts with examples from our codebase. To see how Tokio integrates with our web API, see the Server Setup chapter. For network operations, see the blockchain node implementation.

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

**Desktop Applications** (`bitcoin-desktop-ui-iced/src/runtime.rs`):
- Global runtime for HTTP client operations
- Bridging Iced UI framework with async operations

**Desktop Wallet Application** (`bitcoin-wallet-ui-iced/src/runtime.rs`):
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

Tokio manages:
- **Polling Futures**: Checking if async operations are ready
- **Task Scheduling**: Running and switching between tasks
- **I/O Event Loop**: Monitoring I/O and waking waiting tasks
- **Thread Pool**: Distributing work efficiently

**Async/Await in Practice:**

```rust
async fn process_block(block: Block) -> Result<()> {
    // .await yields control to Tokio, which polls other tasks
    let result = validate_block(&block).await?;
    Ok(result)
}
```

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

In our desktop applications (`bitcoin-desktop-ui-iced/src/runtime.rs` and `bitcoin-wallet-ui-iced/src/runtime.rs`), we create a runtime manually:

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

### Task Spawning Patterns in Our Project

**Network message sending** (`bitcoin/src/net/net_processing.rs`):
```rust
nodes_to_add.iter().for_each(|node| {
    tokio::spawn(async move {
        send_known_nodes(&node, all_nodes.clone()).await;
    });
});
```

**Background transaction processing** (`bitcoin/src/node/context.rs`):
```rust
tokio::spawn(async move {
    let _ = context.submit_transaction_for_mining(&addr, tx).await;
});
// Return immediately, processing happens in background
```

**Connection handling** (`bitcoin/src/node/server.rs`):
```rust
tokio::spawn(async move {
    if let Err(e) = net_processing::process_stream(blockchain, stream).await {
        error!("Serve error: {}", e);
    }
});
```

**Spawned tasks:**
- Execute independently on Tokio's thread pool
- Enable non-blocking operations (API returns while background work continues)
- Support concurrent execution (multiple connections, messages, transactions)
- Require error handling within the task

---

## Concurrent Operations with select!

`tokio::select!` allows waiting on multiple futures simultaneously, executing the first one that completes.

### select! Usage

In our network server (`bitcoin/src/node/server.rs`), handle shutdown and connections:

```rust
loop {
    tokio::select! {
        _ = shutdown.recv() => {
            info!("Network server shutdown signal received");
            break;
        }
        accept_res = listener.accept() => {
            if let Ok((stream, _peer)) = accept_res {
                // Handle connection
            }
        }
    }
}
```

### Multiple Server Coordination

In `bitcoin/src/main.rs`, coordinate multiple servers:

```rust
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        info!("Ctrl-C received, initiating shutdown...");
        let _ = shutdown_tx.send(());
    }
    _ = &mut web_handle => { info!("Web server task finished"); }
    _ = &mut network_handle => { info!("Network server task finished"); }
}
```

**select! executes the first future that completes**, enabling concurrent monitoring and graceful shutdown coordination.

---

## Network I/O

Tokio provides async network primitives for efficient I/O operations.

### Network I/O in Our Project

**Web server** (`bitcoin/src/web/server.rs`):
```rust
let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));
let listener = tokio::net::TcpListener::bind(addr).await?;
```

**Network server** (`bitcoin/src/node/server.rs`):
```rust
let listener = TcpListener::bind(socket_addr).await?;
loop {
    tokio::select! {
        accept_res = listener.accept() => {
            if let Ok((stream, _peer)) = accept_res {
                // Handle connection
            }
        }
    }
}
```

**Stream processing**: Each accepted stream spawned as a task for concurrent handling.

Tokio's async I/O:
- Non-blocking network operations
- Efficient connection handling
- Concurrent processing of multiple streams

---

## Async Synchronization

Tokio provides async versions of synchronization primitives that don't block threads.

### TokioRwLock

In our blockchain service (`bitcoin/src/chain/chainstate.rs`), we use `TokioRwLock` for concurrent access:

```rust
use tokio::sync::RwLock as TokioRwLock;

pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl BlockchainService {
    pub async fn initialize(
        genesis_address: &WalletAddress,
    ) -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::create_blockchain(
            genesis_address,
        )
        .await?;
        Ok(BlockchainService(Arc::new(
            TokioRwLock::new(blockchain),
        )))
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

Our desktop applications (Iced Admin UI and Wallet UI) need to bridge the Iced UI executor (sync) with Tokio (async) for HTTP operations.

### Desktop Runtimes Pattern

In `bitcoin-desktop-ui-iced/src/runtime.rs` and `bitcoin-wallet-ui-iced/src/runtime.rs`:

```rust
pub fn init_runtime() {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime");
    TOKIO_HANDLE.set(rt.handle().clone()).expect("Failed to set Tokio handle");

    std::thread::spawn(move || {
        rt.block_on(async {
            std::future::pending::<()>().await; // Keep runtime alive
        });
    });
}

pub fn spawn_on_tokio<F>(
    fut: F,
) -> impl std::future::Future<Output = F::Output> + Send
where F: std::future::Future + Send + 'static, F::Output: Send + 'static,
{
    let handle = TOKIO_HANDLE
        .get()
        .expect("Tokio runtime not initialized")
        .clone();
    async move { handle.spawn(fut).await.unwrap() }
}
```

**Why This Pattern:**
- Iced has its own executor for UI tasks
- HTTP clients (`reqwest`) require Tokio
- `spawn_on_tokio` bridges the two executors

---

## Best Practices for Our Project

1. **Use Async I/O**: Prefer async operations (`tokio::net::TcpListener::bind`) over blocking ones
2. **Spawn Tasks for Background Work**: Non-blocking execution with `tokio::spawn`
3. **Use select! for Coordination**: Handle multiple operations concurrently (shutdown, connections)
4. **Handle Task Errors**: Always handle errors in spawned tasks with proper logging
5. **Use Async Locks**: `TokioRwLock` instead of `std::sync::RwLock` to avoid blocking threads
6. **Coordinate Shutdown**: Broadcast channels for graceful, coordinated shutdown
7. **Global Runtime Handle**: Store handle globally for access from any thread (required for UI frameworks like Iced)

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
- **Server Setup**: How Tokio is used in our web server
- **Axum Framework Guide**: How Axum uses Tokio for async operations
- **Tower Framework Guide**: How Tower middleware runs on Tokio
- **Rust Language Guide**: Rust language features including async/await

---

<div align="center">

**[← Back to Web API Index](web/README.md)** | **[Tokio Runtime Guide](Tokio.md)** | **[Web API Architecture Index →](web/README.md)** | **Tower** | **Serde** | **Utoipa**

</div>

---

*This guide provides detailed explanations of Tokio runtime features used in our blockchain implementation. For implementation details, see the Server Setup chapter and the blockchain node implementation.*
