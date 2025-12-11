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
- **[Transaction System](../bitcoin-blockchain/02-Transaction-System.md)** - See data structures in action
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Data models and serialization

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Structs and Enums** | **[← Previous](02-Ownership-and-Borrowing.md)** | **[Next →](04-Traits.md)** 📚

</div>

---


*This chapter covers structs and enums. Continue to [Traits](04-Traits.md) to learn how we define shared behavior across types.*