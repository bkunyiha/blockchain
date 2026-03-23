<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../bitcoin-blockchain/README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../bitcoin-blockchain/util/README.md">Chapter 8: Utilities</a>
9. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../bitcoin-blockchain/chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../bitcoin-blockchain/chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../bitcoin-blockchain/chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../bitcoin-blockchain/chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../bitcoin-blockchain/chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../bitcoin-blockchain/chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../bitcoin-blockchain/chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../bitcoin-blockchain/chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../bitcoin-blockchain/store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../bitcoin-blockchain/net/README.md">Chapter 21: Network Layer</a>
22. <a href="../bitcoin-blockchain/node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="../bitcoin-blockchain/web/README.md">Chapter 24: Web API Architecture</a>
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

47. <a href="README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../Glossary.md">Glossary</a>
49. <a href="../Bibliography.md">Bibliography</a>
50. <a href="../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
# Smart Pointers: Shared Ownership

So far, we've seen how ownership ensures each value has a single owner. But what happens when we need multiple owners? What if we need to share data between threads, or between different parts of our application? Rust's smart pointers provide solutions for these scenarios while maintaining memory safety.

Smart pointers are types that manage memory automatically, providing additional functionality beyond simple references. The most common smart pointers in Rust are `Arc` (Atomically Reference Counted) and `Rc` (Reference Counted), which enable shared ownership through reference counting. In this chapter, we'll explore when and how to use smart pointers in our blockchain implementation.

### Arc: Thread-Safe Shared Ownership

When we need to share data across multiple threads, simple ownership transfer isn't sufficient—we need a way for multiple owners to coexist. `Arc` (Atomically Reference Counted) provides thread-safe shared ownership through atomic reference counting.

**Example from `bitcoin/src/chain/chainstate.rs`:**

**Example from `bitcoin/src/chain/chainstate.rs`:**

```rust
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

#[derive(Debug)]
pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl Clone for BlockchainService {
    fn clone(&self) -> Self {
        BlockchainService(self.0.clone())  // ← Clones Arc, not data
    }
}
```

The `BlockchainService` wraps the blockchain data in `Arc<TokioRwLock<BlockchainFileSystem>>`. The `Arc` provides shared ownership—multiple `BlockchainService` instances can share the same underlying blockchain data. When we clone a `BlockchainService`, we're cloning the `Arc`, which increments a reference counter. The actual blockchain data isn't copied, making cloning efficient.

`Arc` provides several important features:
- **Thread Safety**: `Arc` uses atomic operations for reference counting, making it safe to share across threads. Multiple threads can hold `Arc` references simultaneously.
- **Reference Counting**: `Arc` tracks how many references exist. When the last reference is dropped, the data is automatically freed.
- **Immutable by Default**: The data inside `Arc` is immutable. To modify it, we wrap it in `Mutex` or `RwLock`, as we do with `TokioRwLock` in our blockchain service.

This combination of `Arc` and `RwLock` gives us both shared ownership and controlled mutability, enabling safe concurrent access to our blockchain state.

### Arc in Practice: Sharing Node Context

In our web server, we need to share the `NodeContext` across multiple HTTP handlers. Each handler runs potentially on different threads, and they all need access to the same blockchain node state. `Arc` makes this sharing safe and efficient.

**Example from `bitcoin/src/web/server.rs`:**

```rust
pub struct WebServer {
    config: WebServerConfig,
    node: Arc<NodeContext>,  // ← Shared across handlers
}

impl WebServer {
    pub fn create_app(&self) -> Router {
        Router::new()
            .with_state(self.node.clone())  // ← Clones Arc, not NodeContext
    }
}
```

When we create the Axum router, we call `self.node.clone()`. This doesn't clone the entire `NodeContext`—it clones the `Arc`, which is just incrementing a reference counter. The actual `NodeContext` data is shared between the original and the cloned `Arc`.

This design provides several benefits:
- **Shared State**: All handlers access the same `NodeContext` instance, ensuring consistency across requests
- **Thread Safety**: `Arc` is thread-safe, allowing handlers to run concurrently on different threads
- **Efficiency**: Cloning an `Arc` is a cheap atomic operation—much faster than cloning the entire `NodeContext` structure

Without `Arc`, we'd need to either clone the entire context for each handler (expensive) or use unsafe code to share references (dangerous). `Arc` gives us safe, efficient shared ownership.

### Rc: Single-Threaded Reference Counting

Rust also provides `Rc` (Reference Counted), which is similar to `Arc` but not thread-safe. `Rc` uses non-atomic reference counting, making it slightly faster but only usable within a single thread.

```rust
// Conceptual example - not thread-safe
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let shared1 = Rc::clone(&data);
let shared2 = Rc::clone(&data);
// All three references point to same data
```

**Choosing Between Rc and Arc:**

The choice between `Rc` and `Arc` depends on whether we need thread safety:
- **`Rc`**: Single-threaded, slightly faster due to non-atomic operations. Use when data stays within one thread.
- **`Arc`**: Thread-safe, uses atomic operations for reference counting. Use when data needs to be shared across threads.
- **Our Project**: We use `Arc` throughout because our blockchain handles concurrent requests and operations across multiple threads.

The performance difference between `Rc` and `Arc` is typically negligible, so when in doubt, `Arc` is the safer choice. In our blockchain, where concurrency is fundamental to the design, `Arc` is the appropriate choice.

## Summary

Smart pointers enable shared ownership when single ownership isn't sufficient. `Arc` provides thread-safe shared ownership through atomic reference counting, while `Rc` provides single-threaded shared ownership. Both enable multiple owners of data while maintaining memory safety.

In our blockchain, `Arc` enables safe sharing of node context across HTTP handlers and concurrent access to blockchain state. Combined with locks like `RwLock`, we get both shared ownership and controlled mutability, enabling safe concurrent programming.

In the next chapter, we'll explore pattern matching, which works seamlessly with the data structures we've learned about to enable exhaustive case handling.

---

## Navigation

- **[← Previous: Lifetimes](07-Lifetimes.md)** - Managing reference validity
- **[Next: Pattern Matching →](09-Pattern-Matching.md)** - Exhaustive case handling
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Ownership and Borrowing](02-Ownership-and-Borrowing.md)** - Single ownership model
- **[Concurrency](12-Concurrency.md)** - Thread-safe shared state
- **[Async/Await](11-Async-Await.md)** - Sharing state across async tasks

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Shared state in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Sharing node context
- **[Tokio Runtime Guide](../bitcoin-blockchain/Tokio.md)** - Async state management

---

<div align="center">

**[← Rust Guide Index](README.md)** | **Smart Pointers** | **[← Previous](07-Lifetimes.md)** | **[Next →](09-Pattern-Matching.md)**

</div>

---


*This chapter covers smart pointers and shared ownership. Continue to [Pattern Matching](09-Pattern-Matching.md) to learn exhaustive case handling.*