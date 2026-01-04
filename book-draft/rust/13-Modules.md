<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../ci/kubernetes/README.md) - Kubernetes production guide
22. **Chapter 10: Rust Language Guide** ← *You are here*

</details>

</div>

---
# Modules: Code Organization and Visibility

As our blockchain implementation grows, organizing code becomes crucial. Rust's module system provides a way to structure code hierarchically and control what's visible to other parts of the program. Modules help us group related functionality, manage namespaces, and control the public API of our code.

The module system also controls visibility—what code can access what. By default, items in Rust are private to their module, and we must explicitly mark items as `pub` to make them accessible from outside the module. This encapsulation helps maintain clean interfaces and prevents accidental coupling between modules. In this chapter, we'll explore how we organize our blockchain code using Rust's module system.

### Module Declaration: Organizing Code Hierarchically

Modules in Rust can be declared in two ways: inline with `mod` blocks, or in separate files. Our project uses separate files for better organization, with `mod.rs` files serving as module entry points.

**Example from `bitcoin/src/wallet/mod.rs`:**

```rust
// Declare and defines a module for the wallet layer
pub mod wallet_impl;
pub mod wallet_service;

// Re-export the modules
pub use wallet_impl::{
    ADDRESS_CHECK_SUM_LEN, Wallet, WalletAddress, convert_address, get_pub_key_hash, hash_pub_key,
};

// Re-export wallet service
pub use wallet_service::{DEFAULT_WALLETS_FILE, WalletService};
```

This module file demonstrates several module system features. We declare sub-modules with `pub mod`, making them part of the public API. The `pub use` statements re-export items from sub-modules, allowing external code to import them directly from the `wallet` module rather than needing to know the internal structure.

The module system provides several organizational benefits:
- **Hierarchical Organization**: Modules can contain sub-modules, creating a tree structure that mirrors our domain concepts
- **Namespace Management**: Each module creates its own namespace, preventing name conflicts
- **Re-exports**: `pub use` allows us to create convenient public APIs while keeping internal organization flexible

This pattern appears throughout our codebase. We organize code by domain (wallet, transaction, blockchain) and use modules to create clean boundaries between components. The `pub use` statements create a convenient public API while allowing us to reorganize internals without breaking external code.

### Visibility: Controlling Access

Rust's visibility system controls what code can access what. By default, everything is private to its module, and we must explicitly mark items as `pub` to make them accessible. This default privacy encourages encapsulation and helps maintain clean interfaces.

```rust
pub struct NodeContext {  // ← Public struct, can be used outside module
    blockchain: BlockchainService,  // ← Private field, only accessible within module
}

impl NodeContext {
    pub fn new(blockchain: BlockchainService) -> NodeContext {  // ← Public function
        NodeContext { blockchain }
    }
    
    fn internal_helper(&self) {  // ← Private function, only usable within module
        // ...
    }
}
```

This example shows how visibility works in practice. The `NodeContext` struct is public, so external code can create and use it. However, the `blockchain` field is private, so external code cannot access it directly—they must use the public methods we provide. This encapsulation ensures we can change internal representation without breaking external code.

Rust provides several visibility levels:
- **`pub`**: Public, accessible from anywhere that can see the module
- **No modifier**: Private, only accessible within the same module
- **`pub(crate)`**: Public within the crate, but not exported to external crates
- **`pub(super)`**: Public to the parent module, useful for nested module hierarchies

In our blockchain, we use private fields extensively to hide implementation details. The public API consists of methods that provide controlled access to internal state, ensuring we can evolve the implementation without breaking external code.

## Summary

Modules provide hierarchical code organization and control visibility. By default, items are private to their module, and we must explicitly mark items as `pub` to make them accessible. This encapsulation helps maintain clean interfaces and prevents accidental coupling.

In our blockchain, modules organize code by domain (wallet, transaction, blockchain) and use visibility to create clean public APIs. The `pub use` statements create convenient public interfaces while allowing flexible internal organization.

In the next chapter, we'll explore iterators and closures, which enable functional programming patterns in Rust.

---

## Navigation

- **[← Previous: Concurrency](12-Concurrency.md)** - Thread safety and shared state
- **[Next: Iterators and Closures →](14-Iterators-Closures.md)** - Functional programming patterns
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Traits](04-Traits.md)** - Trait organization with modules
- **[Testing](16-Testing.md)** - Test organization
- **[Best Practices](17-Best-Practices.md)** - Module organization patterns

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Module structure in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - API module organization

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Modules and Visibility** | **[← Previous](12-Concurrency.md)** | **[Next →](14-Iterators-Closures.md)** 📚

</div>

---


*This chapter covers modules and code organization. Continue to [Iterators and Closures](14-Iterators-Closures.md) to learn functional programming patterns.*