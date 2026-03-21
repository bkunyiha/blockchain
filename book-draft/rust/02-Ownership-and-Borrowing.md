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
# Ownership and Borrowing

Ownership is Rust's distinctive feature—a compile-time system that tracks data ownership and ensures memory is freed exactly once, automatically. Three rules govern it:

1. **Each value has one owner**: Only one variable owns each value at any time.
2. **Only one mutable borrow at a time**: Prevents data races.
3. **Values drop when owner goes out of scope**: Automatic cleanup without garbage collection.

For comprehensive treatment, see [The Rust Book: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).

## Ownership in Practice

In our blockchain (`bitcoin/src/primitives/transaction.rs`):

```rust
// From bitcoin/src/primitives/transaction.rs
impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),  // Takes ownership of new Vec
            vout,
        }
    }
    pub fn get_txid(&self) -> &[u8] {  // Returns borrowed ref
        self.txid.as_slice()
    }
}
```

The `new` function receives a borrowed slice but creates a new `Vec<u8>` that the `TXInput` owns.

## Borrowing

**Immutable borrows (`&T`):** Multiple readers, no modifications.

```rust
pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
    hash_pub_key(self.pub_key.as_slice()).eq(pub_key_hash)
}
```

**Mutable borrows (`&mut T`):** One exclusive writer.

```rust
fn set_key(&mut self, key: Vec<u8>) {
    self.pub_key = key;  // Exclusive mutable access
}
```

The borrow checker enforces these rules at compile time, preventing data races without runtime overhead.

## Ownership Transfer (Moves)

When ownership transfers between variables, it's zero-cost—just pointer transfer:

```rust
pub fn new(blockchain: BlockchainService) -> NodeContext {
    NodeContext { blockchain }  // Move, not copy
}
```

In our blockchain, we borrow for reads (efficient), move for isolation (transactional safety).

## Summary

Ownership and borrowing enable memory safety without garbage collection overhead. Borrowed references allow efficient sharing; moves ensure exclusive ownership when needed. Together, they form the foundation of Rust's type-system-enforced safety.

---

## Navigation

- **[← Previous: Introduction](01-Introduction.md)** - Getting started with Rust
- **[Next: Data Structures →](03-Data-Structures.md)** - Structs and Enums for modeling domain concepts
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Smart Pointers](08-Smart-Pointers.md)** - Shared ownership with Arc and Rc
- **[Lifetimes](07-Lifetimes.md)** - Managing reference validity
- **[Concurrency](12-Concurrency.md)** - Thread safety and shared state

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - See ownership in action
- **[Tokio Runtime Guide](../bitcoin-blockchain/Tokio.md)** - Async programming patterns

---

<div align="center">

**[← Rust Guide Index](README.md)** | **Ownership and Borrowing** | **[← Previous: Introduction](01-Introduction.md)** | **[Next: Data Structures →](03-Data-Structures.md)**

</div>

---


*This chapter covers Rust's ownership system. Continue to [Data Structures](03-Data-Structures.md) to learn how we model blockchain data with structs and enums.*