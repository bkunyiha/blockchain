<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../bitcoin-blockchain/README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../bitcoin-blockchain/util/README.md">Chapter 7: Utilities</a>
8. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../bitcoin-blockchain/chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../bitcoin-blockchain/store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../bitcoin-blockchain/net/README.md">Chapter 12: Network Layer</a>
13. <a href="../bitcoin-blockchain/node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../bitcoin-blockchain/web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
# Modules: Code Organization and Visibility

# Modules: Code Organization and Visibility

Modules organize code hierarchically and control visibility. For comprehensive treatment, see [The Rust Book: Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-modules-and-paths.html).

## Module Organization

From `bitcoin/src/wallet/mod.rs`:

```rust
pub mod wallet_impl;
pub mod wallet_service;

pub use wallet_impl::{
    Wallet, WalletAddress, convert_address, hash_pub_key,
};

pub use wallet_service::{WalletService};
```

Declare sub-modules with `pub mod`. Use `pub use` to re-export items, creating convenient public APIs while keeping internal structure flexible.

## Visibility Control

By default, items are private to their module. Explicitly mark as `pub` to expose:

```rust
pub struct NodeContext {
    // Private field - only accessible within module
    blockchain: BlockchainService,
}

impl NodeContext {
    pub fn new(blockchain: BlockchainService) -> Self {
        NodeContext { blockchain }
    }

    fn internal_helper(&self) { /* private */ }
}
```

Visibility levels:
- **`pub`**: Public, accessible from anywhere
- **(no modifier)**: Private, within same module only
- **`pub(crate)`**: Public within crate only
- **`pub(super)`**: Public to parent module

## Benefits

- **Hierarchical organization** mirrors domain concepts
- **Namespace management** prevents name conflicts
- **Encapsulation** enables changing internals without breaking APIs
- **Re-exports** decouple internal structure from public interface

## Summary

Modules organize code by domain (wallet, transaction, blockchain). Default privacy enforces encapsulation. `pub use` creates convenient public APIs while keeping internal organization flexible.

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

**[← Rust Guide Index](README.md)** | **Modules and Visibility** | **[← Previous](12-Concurrency.md)** | **[Next →](14-Iterators-Closures.md)**

</div>

---


*This chapter covers modules and code organization. Continue to [Iterators and Closures](14-Iterators-Closures.md) to learn functional programming patterns.*