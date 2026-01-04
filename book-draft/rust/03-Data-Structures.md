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
# Data Structures: Structs and Enums

Now that we understand ownership and borrowing, we can explore how Rust allows us to create custom data types. Structs and enums are Rust's primary mechanisms for modeling domain concepts. In this chapter, we'll see how we use these types to represent blockchain data structures.

As we design our blockchain's data structures, we need ways to group related data and represent values that can take multiple forms. Rust provides two fundamental building blocks: structs for grouping data, and enums for representing values that can be one of several variants. Together, they form the foundation of Rust's type system.

### Structs: Grouping Related Data

Structs allow us to create custom types that group related data together. In our blockchain, structs model everything from individual transactions to entire blocks. Let's examine how we use structs to represent blockchain data:

**Example from `bitcoin/src/primitives/block.rs`:**

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

This example demonstrates several important struct concepts. The `BlockHeader` struct groups related block metadata—timestamp, hash, and proof-of-work nonce—into a single type. The `Block` struct then composes a `BlockHeader` with a vector of transactions, showing how structs can contain other structs to build complex data structures.

Structs in Rust provide several key features that make them ideal for modeling blockchain data:
- **Named Fields**: Each field has a name and type, making the data structure self-documenting. When we access `block.header.timestamp`, it's immediately clear what we're reading.
- **Composition**: Structs can contain other structs, allowing us to build hierarchical data models that mirror our domain concepts.
- **Methods**: We implement behavior for structs using `impl` blocks, keeping data and operations together logically.

### Enums: Representing Multiple Possibilities

While structs group related data, enums represent values that can be one of several distinct variants. This makes enums perfect for modeling concepts that have multiple possible states or outcomes. In our blockchain, we use enums extensively for error handling, message types, and operation types.

**Example from `bitcoin/src/error.rs`:**

```rust
#[derive(Clone, Error, Debug)]
pub enum BtcError {
    // Recoverable errors
    #[error("Blockchain not found error: {0}")]
    BlockchainNotFoundError(String),

    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("Not enough funds")]
    NotEnoughFunds,

    // ... many more variants
}
```

Our error enum demonstrates the flexibility of Rust enums. Some variants, like `InvalidTransaction`, carry no additional data—they simply represent that an error occurred. Other variants, like `BlockchainNotFoundError(String)`, carry context about what went wrong. This allows us to provide detailed error information where needed while keeping simple errors concise.

Rust enums support three types of variants:
- **Unit Variants**: Variants that carry no data, like `InvalidTransaction`. These are useful for simple state representation.
- **Tuple Variants**: Variants that carry unnamed data, like `BlockchainNotFoundError(String)`. The data is accessed by position.
- **Struct Variants**: Variants with named fields, similar to structs. These provide the most readable access to variant data.

The compiler ensures we handle all possible variants when we use enums, preventing bugs from unhandled cases. This exhaustive checking is one of Rust's most powerful safety features.

### Enums with Associated Data: Network Messages

Enums become even more powerful when variants carry different types of data. In our network layer, we use enums to represent different message types, each with its own data structure:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum OpType {
    Tx,
    Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Version { best_height: usize },
    GetBlocks { from: usize },
    Inv { op_type: OpType, items: Vec<String> },
    GetData { op_type: OpType, items: Vec<String> },
    Block { block: Block },
    Tx { transaction: Transaction },
}
```

This `MessageType` enum demonstrates how enums can carry different data structures for each variant. A `Version` message carries a `best_height` field, while an `Inv` message carries an operation type and a list of items. The type system ensures we can only access the correct fields for each variant, preventing bugs from accessing non-existent data.

The benefits of this approach are significant:
- **Type Safety**: The compiler ensures we handle all possible message types. If we add a new variant, the compiler will point out all places that need updating.
- **Pattern Matching**: When we process messages with `match`, Rust requires exhaustive handling of all variants, catching bugs at compile time.
- **Flexible Data Association**: Each variant can carry different types and amounts of data, allowing us to model complex domain concepts naturally.

This pattern appears throughout our codebase whenever we need to represent values that can take multiple forms—from error types to network messages to operation types.

## Summary

Structs and enums are Rust's primary mechanisms for creating custom types. Structs group related data together, while enums represent values that can be one of several variants. Together, they enable us to model our blockchain's domain concepts naturally and safely.

The compiler's exhaustive checking ensures we handle all possible enum variants, preventing bugs from unhandled cases. Pattern matching (which we'll explore in detail later) makes working with these types expressive and safe.

In the next chapter, we'll see how traits allow us to define shared behavior across different types, enabling polymorphism and code reuse while maintaining type safety.

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

**📚 [← Rust Guide Index](README.md)** | **Structs and Enums** | **[← Previous](02-Ownership-and-Borrowing.md)** | **[Next →](04-Traits.md)** 📚

</div>

---


*This chapter covers structs and enums. Continue to [Traits](04-Traits.md) to learn how we define shared behavior across types.*