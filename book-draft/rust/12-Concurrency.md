# Concurrency: Thread Safety and Shared State

Concurrency is fundamental to our blockchain—we handle multiple network connections, process transactions concurrently, and serve HTTP requests in parallel. Rust's type system ensures thread safety through two marker traits: `Send` and `Sync`. These traits don't have methods—they're markers that indicate whether types can be safely used across thread boundaries.

The compiler uses these traits to prevent data races at compile time. If we try to share non-`Send` data between threads, the compiler will reject our code. This compile-time checking prevents entire classes of concurrency bugs. In this chapter, we'll explore how Rust ensures thread safety and how we use locks and message passing for safe concurrent programming in our blockchain.

### Send: Transferring Data Between Threads

The `Send` trait indicates that a type can be safely transferred between threads. When we spawn a task or send data to another thread, that data must implement `Send`. Most types in Rust are `Send` automatically, but types containing non-`Send` data (like `Rc`) are not.

**Example from `bitcoin/src/chain/chainstate.rs`:**

```rust
async fn read<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(BlockchainFileSystem) -> Fut + Send,  // ← Must be Send
    Fut: Future<Output = Result<T>> + Send,  // ← Must be Send
    T: Send + 'static,  // ← Must be Send
{
    // ...
}
```

This generic function has trait bounds requiring `Send` on several types. The function `F` must be `Send`, meaning it can be transferred to another thread (which happens when we spawn async tasks). The future `Fut` must also be `Send`, and the return type `T` must be `Send`.

These bounds ensure thread safety:
- **Owned Data**: Types that own their data are typically `Send` because moving ownership between threads is safe
- **No Non-Send Types**: Types containing `Rc` (single-threaded reference counting) or raw pointers are not `Send` because they're not thread-safe
- **Automatic for Most Types**: Most Rust types implement `Send` automatically. Only types with interior mutability or thread-local data are not `Send`

The compiler checks these bounds at compile time. If we try to use a non-`Send` type in a context requiring `Send`, we'll get a clear error message explaining why the type cannot be sent between threads.

### Sync: Sharing References Between Threads

While `Send` allows transferring ownership between threads, `Sync` allows sharing references (`&T`) between threads. A type `T` is `Sync` if `&T` is `Send`—meaning we can share immutable references across threads safely.

```rust
// Arc<T> is Send + Sync if T is Send + Sync
let shared: Arc<NodeContext> = Arc::new(node);
// Can share &Arc<NodeContext> across threads safely
```

**Understanding Sync:**

The `Sync` trait indicates that sharing immutable references to a type between threads is safe:
- **Shared References**: If `T: Sync`, then `&T` can be shared between threads. Multiple threads can read the same data simultaneously.
- **Interior Mutability**: Types like `Mutex<T>` and `RwLock<T>` are `Sync` even when `T` is not, because they provide thread-safe interior mutability
- **Most Types Are Sync**: Like `Send`, most Rust types implement `Sync` automatically. Only types with interior mutability that's not thread-safe (like `Rc`) are not `Sync`

In our blockchain, `Arc<NodeContext>` is both `Send` and `Sync`, allowing us to share the node context across threads safely. Handlers can hold references to the shared context without data races, enabling safe concurrent request processing.

### Mutex and RwLock: Thread-Safe Interior Mutability

Sometimes we need to mutate data that's shared between threads. Rust's ownership system prevents mutable sharing, but `Mutex` and `RwLock` provide "interior mutability"—the ability to mutate data through an immutable reference, with thread-safety guarantees.

**Example from `bitcoin/src/node/peers.rs`:**

```rust
use std::sync::RwLock;

pub struct Nodes {
    inner: RwLock<HashSet<Node>>,  // ← Thread-safe interior mutability
}

impl Nodes {
    pub fn add_node(&self, addr: SocketAddr) -> Result<()> {
        let mut inner = self
            .inner
            .write()  // ← Acquires write lock
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        inner.insert(Node::new(addr));
        Ok(())
    }
}
```

The `Nodes` struct wraps a `HashSet<Node>` in an `RwLock`. The `RwLock` provides thread-safe access—multiple threads can read simultaneously, or one thread can write exclusively. When we call `write()`, we acquire an exclusive lock, preventing other threads from reading or writing until we release it.

Rust provides several lock types for different use cases:
- **`Mutex<T>`**: Provides exclusive access. Only one thread can hold the lock at a time, whether reading or writing. Simpler but less concurrent than `RwLock`.
- **`RwLock<T>`**: Allows multiple readers or one writer. More concurrent for read-heavy workloads, but slightly more complex.
- **`TokioRwLock<T>`**: Async version that works with async/await. Used extensively in our project because our codebase is async-first.

In our blockchain, we use `RwLock` for the node collection because reads (checking if a node exists) are more common than writes (adding/removing nodes). The `RwLock` allows multiple concurrent reads while ensuring exclusive access for modifications.

The error handling with `map_err` shows how we convert lock poisoning errors (which occur if a thread panics while holding a lock) into our domain error type, maintaining clean error propagation throughout our codebase.

### Lock Performance Considerations

Understanding lock performance is crucial for writing efficient concurrent code:

**Lock Types and Performance:**
- **`Mutex`**: Simple, fast for low contention. Use when you need exclusive access.
- **`RwLock`**: Better for read-heavy workloads. Multiple readers can access simultaneously.
- **`TokioRwLock`**: Async version, doesn't block the async runtime. Use in async contexts.

**Example: Choosing the Right Lock**
```rust
// Read-heavy: Use RwLock
struct BlockchainState {
    blocks: RwLock<Vec<Block>>,  // Many readers, few writers
}

// Write-heavy: Use Mutex
struct TransactionPool {
    transactions: Mutex<Vec<Transaction>>,  // Frequent writes
}
```

### Concurrency Patterns in Our Blockchain

**Pattern 1: Read-Copy-Update (RCU)**
```rust
// Clone data for reading, update atomically
let snapshot = state.read().await.clone();
// Process snapshot without holding lock
update_state(state, new_data).await;
```

**Pattern 2: Lock-Free with Atomic Operations**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter {
    value: AtomicUsize,
}

impl Counter {
    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }
}
```

**Pattern 3: Message Passing**
```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

// Producer
tx.send(message).await?;

// Consumer
while let Some(msg) = rx.recv().await {
    process(msg).await?;
}
```

### Thread Safety Guarantees

Rust's type system ensures thread safety at compile time:

- **`Send`**: Types that can be transferred between threads
- **`Sync`**: Types that can be shared between threads via references
- **Compiler enforcement**: Violations result in compile-time errors, not runtime crashes

**Common `Send` + `Sync` Types:**
- Primitives: `i32`, `bool`, `String`
- Collections: `Vec<T>` where `T: Send + Sync`
- Smart pointers: `Arc<T>` where `T: Send + Sync`
- Locks: `Mutex<T>`, `RwLock<T>` (even if `T` is not `Send`)

### Concurrency Best Practices

1. **Prefer message passing**: Use channels for communication between threads
2. **Minimize shared state**: Reduce the amount of data shared between threads
3. **Use appropriate locks**: Choose `Mutex` vs `RwLock` based on access patterns
4. **Avoid deadlocks**: Always acquire locks in the same order
5. **Use `Arc` for shared ownership**: Enables safe sharing across threads

In our blockchain, these patterns ensure safe, efficient concurrent processing of transactions, network messages, and HTTP requests, enabling high throughput while maintaining data integrity.

## Summary

Rust's type system ensures thread safety through `Send` and `Sync` traits, preventing data races at compile time. Locks like `Mutex` and `RwLock` provide controlled mutability for shared state, while message passing enables safe communication between threads.

Understanding lock performance characteristics helps us choose the right synchronization primitive. In our blockchain, we use `RwLock` for read-heavy workloads and message passing for thread communication, enabling safe concurrent programming without sacrificing performance.

In the next chapter, we'll explore modules, which help us organize code and control visibility.

---

## Navigation

- **[← Previous: Async/Await](11-Async-Await.md)** - Asynchronous programming
- **[Next: Modules →](13-Modules.md)** - Code organization and visibility
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Ownership and Borrowing](02-Ownership-and-Borrowing.md)** - Foundation of memory safety
- **[Smart Pointers](08-Smart-Pointers.md)** - Shared ownership with Arc
- **[Async/Await](11-Async-Await.md)** - Async concurrency patterns

**Related Guides:**
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Concurrent request handling
- **[Tokio Runtime Guide](../bitcoin-blockchain/Tokio.md)** - Async runtime and synchronization
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Concurrent transaction processing

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Concurrency Primitives** | **[← Previous](11-Async-Await.md)** | **[Next →](13-Modules.md)** 📚

</div>

---


*This chapter covers concurrency primitives and thread safety. Continue to [Modules](13-Modules.md) to learn code organization.*