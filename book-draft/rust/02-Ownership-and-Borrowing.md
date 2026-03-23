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