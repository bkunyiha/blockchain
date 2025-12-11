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
- **[Transaction System](../bitcoin-blockchain/02-Transaction-System.md)** - Module structure in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - API module organization

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Modules and Visibility** | **[← Previous](12-Concurrency.md)** | **[Next →](14-Iterators-Closures.md)** 📚

</div>

---


*This chapter covers modules and code organization. Continue to [Iterators and Closures](14-Iterators-Closures.md) to learn functional programming patterns.*