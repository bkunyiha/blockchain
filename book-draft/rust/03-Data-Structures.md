<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../bitcoin-blockchain/README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
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
# Data Structures: Structs and Enums

Structs and enums are Rust's primary mechanisms for modeling domain concepts. For a comprehensive treatment, see [The Rust Book: Structs and Enums](https://doc.rust-lang.org/book/ch05-00-structs.html).

## Structs: Grouping Related Data

Structs group related data with named fields. In our blockchain (`bitcoin/src/primitives/block.rs`):

```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    nonce: i64,
    height: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}
```

Structs compose data hierarchically, make the domain model self-documenting, and allow implementing methods in `impl` blocks.

## Enums: Representing Multiple Possibilities

Enums represent values that are one of several variants. From `bitcoin/src/error.rs`:

```rust
#[derive(Clone, Error, Debug)]
pub enum BtcError {
    #[error("Blockchain not found: {0}")]
    BlockchainNotFoundError(String),
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Not enough funds")]
    NotEnoughFunds,
}
```

Enums can carry data (unit, tuple, or struct variants) and are exhaustively pattern-matched by the compiler, ensuring all cases are handled.

## Network Messages: Enums with Associated Data

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Version { best_height: usize },
    GetBlocks { from: usize },
    Inv { op_type: OpType, items: Vec<String> },
    Block { block: Block },
    Tx { transaction: Transaction },
}
```

Each variant carries different data. Type system ensures we access only valid fields. Exhaustive `match` statements catch bugs at compile time.

## Summary

Structs group data; enums represent one-of-many values. Together they enable safe, type-checked modeling of blockchain domain concepts with exhaustive pattern matching enforcing correctness.

---

## Navigation

- **[← Previous: Ownership and Borrowing](02-Ownership-and-Borrowing.md)** - Rust's memory management system
- **[Next: Traits →](04-Traits.md)** - Polymorphism and code reuse
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Pattern Matching](09-Pattern-Matching.md)** - Working with structs and enums
- **[Derive Macros](10-Derive-Macros.md)** - Automatic trait implementations
- **[Error Handling](05-Error-Handling.md)** - Error enums and Result types

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - See data structures in action
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Data models and serialization

---

<div align="center">

**[← Rust Guide Index](README.md)** | **Structs and Enums** | **[← Previous](02-Ownership-and-Borrowing.md)** | **[Next →](04-Traits.md)** 

</div>

---


*This chapter covers structs and enums. Continue to [Traits](04-Traits.md) to learn how we define shared behavior across types.*