# Ownership and Borrowing

Ownership is Rust's most distinctive feature and the foundation of its memory safety guarantees. Understanding ownership is essential for writing effective Rust code. In this chapter, we'll explore how ownership works, why it matters, and how we use it in our blockchain implementation.

When we first began implementing our blockchain, we faced a fundamental challenge: how do we manage memory safely without a garbage collector, while maintaining the performance required for a production blockchain system? Rust's answer is ownership—a compile-time system that tracks who owns each piece of data and ensures memory is freed exactly once, automatically.

Ownership might seem restrictive at first, but it's this very restriction that enables Rust's safety guarantees. The compiler enforces three simple rules that prevent entire classes of bugs that plague other systems languages:

1. **Each value has exactly one owner**: When you assign a value to a variable, that variable becomes the owner. Ownership can be transferred, but only one owner exists at any time.
2. **Only one owner at a time**: This prevents the ambiguity of shared ownership that leads to use-after-free errors and double-free bugs.
3. **Values are automatically dropped when their owner goes out of scope**: Rust's compiler inserts cleanup code automatically, ensuring memory is freed without manual intervention.

These rules might seem abstract, but they have concrete implications for how we structure our code. Let's see how ownership works in practice within our blockchain implementation.

### Ownership in Practice: Transaction Inputs

Consider how we represent transaction inputs in our blockchain. In `bitcoin/src/primitives/transaction.rs`, the `TXInput` struct owns its transaction ID data:

```rust
impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),  // ← Creates a new Vec<u8> owned by TXInput
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    pub fn get_txid(&self) -> &[u8] {  // ← Returns a borrowed reference
        self.txid.as_slice()
    }
}
```

When we create a new `TXInput`, the `new` function receives a borrowed slice (`&[u8]`) representing the transaction ID. Rather than storing a reference (which would require the original data to outlive our struct), we call `to_vec()` to create a new vector that our struct owns. This ownership transfer ensures that the `TXInput` struct controls the lifetime of its transaction ID data.

When we need to read the transaction ID, `get_txid()` returns a borrowed reference (`&[u8]`). This allows callers to read the data without taking ownership, and the borrow checker ensures the reference remains valid as long as the `TXInput` struct exists. This pattern—owning data internally and providing borrowed access—is common throughout our codebase.

### Borrowing: Temporary Access Without Ownership

While ownership ensures memory safety, requiring ownership transfer for every data access would be impractical. Rust's borrowing system allows temporary access to data without taking ownership, enabling efficient data sharing while maintaining safety guarantees.

Borrowing comes in two forms: immutable borrows for reading data, and mutable borrows for modifying data. The borrow checker enforces rules that prevent data races and use-after-free errors at compile time.

**Immutable Borrows (`&T`):**

When we need to read data without modifying it, we use immutable borrows. In our transaction code, we see this pattern frequently:

```rust
// In bitcoin/src/primitives/transaction.rs
pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
    let locking_hash = hash_pub_key(self.pub_key.as_slice());  // ← Borrows self immutably
    locking_hash.eq(pub_key_hash)  // ← Borrows pub_key_hash immutably
}
```

The `&self` parameter means we're borrowing the struct immutably—we can read its fields but cannot modify them. Similarly, `pub_key_hash: &[u8]` is an immutable borrow of the caller's data. Multiple functions can hold immutable borrows simultaneously, enabling concurrent reads without conflicts.

**Mutable Borrows (`&mut T`):**

When we need to modify data, we use mutable borrows. The borrow checker ensures only one mutable borrow exists at a time, preventing data races:

```rust
// In bitcoin/src/primitives/transaction.rs
fn lock(&mut self, address: &WalletAddress) -> Result<()> {
    let pub_key_hash = get_pub_key_hash(address)?;
    self.pub_key_hash = pub_key_hash;  // ← Mutates self via mutable borrow
    Ok(())
}
```

The `&mut self` parameter gives us exclusive mutable access to the struct. The borrow checker ensures no other references (mutable or immutable) exist while we hold this mutable borrow, preventing data races at compile time.

**Borrowing Rules in Practice:**

These rules might seem restrictive, but they prevent entire classes of bugs:
- **Multiple immutable borrows**: Safe because reading doesn't conflict with other reads
- **Single mutable borrow**: Prevents data races by ensuring exclusive write access
- **No mixing**: Prevents reading data that's being modified elsewhere

In our blockchain, these rules ensure that transaction validation can safely read blockchain state while miners write new blocks, all without runtime synchronization overhead.

### Ownership Transfer: Moving Data Between Contexts

Ownership transfer occurs when we pass values between functions or assign them to variables. In our node context implementation, we see a clear example of ownership transfer:

```rust
// In bitcoin/src/node/context.rs
pub fn new(blockchain: BlockchainService) -> NodeContext {
    NodeContext {
        blockchain,  // ← Ownership of BlockchainService transferred to NodeContext
    }
}
```

When we create a new `NodeContext`, the `blockchain` parameter is moved into the struct. The caller no longer owns the `BlockchainService`—ownership has been transferred to the `NodeContext`. This transfer is explicit and checked at compile time, ensuring we can't accidentally use the `blockchain` after transferring ownership.

This ownership model provides several benefits that are crucial for our blockchain implementation:

- **Clear Responsibility**: The owner is unambiguously responsible for cleanup. When `NodeContext` is dropped, it automatically cleans up the `BlockchainService`.
- **No Double-Free Errors**: The compiler prevents us from freeing memory twice, a common source of crashes in C/C++ code.
- **Memory Safety Guarantees**: These checks happen at compile time, not runtime, meaning there's no performance penalty for these safety guarantees.

In practice, this means our blockchain code can safely manage complex data structures like the blockchain state, transaction mempool, and network connections without worrying about memory leaks or use-after-free errors.

### Performance Implications of Ownership

Understanding ownership helps us write efficient code. When we transfer ownership, Rust moves the data—it doesn't copy it. This means ownership transfers are zero-cost operations:

```rust
// In bitcoin/src/node/context.rs
pub fn new(blockchain: BlockchainService) -> NodeContext {
    NodeContext {
        blockchain,  // ← Move, not copy - zero cost
    }
}
```

**Key Performance Points:**
- **Moves are cheap**: Ownership transfer doesn't copy data, just transfers the pointer
- **Copy types**: Types implementing `Copy` (like `i32`, `bool`) are copied automatically
- **Clone explicitly**: For types that don't implement `Copy`, use `.clone()` when you need a copy
- **Borrow when possible**: Borrowing avoids ownership transfer entirely, making it the most efficient option

In our blockchain, we carefully choose between ownership, borrowing, and cloning based on performance needs. Transaction validation borrows blockchain state (efficient), while creating new transactions takes ownership (necessary for isolation).

### Common Ownership Patterns in Our Codebase

**Pattern 1: Builder Pattern with Ownership**
```rust
// Ownership allows chaining while maintaining safety
let tx = Transaction::new()
    .add_input(input1)  // Takes ownership
    .add_output(output1)  // Takes ownership
    .build()?;  // Returns owned Transaction
```

**Pattern 2: Borrowing for Validation**
```rust
// Validation borrows data, doesn't take ownership
pub fn validate(&self, blockchain: &Blockchain) -> Result<()> {
    // Can validate without taking ownership
}
```

**Pattern 3: Ownership Transfer for Isolation**
```rust
// Taking ownership ensures data isolation
pub fn process_transaction(tx: Transaction) -> Result<()> {
    // Owned transaction can't be modified elsewhere
}
```

These patterns appear throughout our blockchain implementation, demonstrating how ownership enables both safety and efficiency.

## Summary

Ownership and borrowing form the foundation of Rust's memory safety guarantees. By tracking ownership at compile time, Rust prevents entire classes of bugs—memory leaks, use-after-free errors, and data races—without runtime overhead. The borrowing system allows efficient data sharing while maintaining these safety guarantees.

As we've seen in our blockchain implementation, ownership enables us to:
- **Manage memory safely**: Automatic cleanup without garbage collection overhead
- **Share data efficiently**: Borrowing allows multiple readers without copying
- **Prevent data races**: The borrow checker ensures exclusive mutable access
- **Write performant code**: Zero-cost abstractions mean no runtime penalty

In the next chapter, we'll explore how we use structs and enums to model our blockchain's data structures, building on the ownership concepts we've learned here.

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

**📚 [← Rust Guide Index](README.md)** | **Ownership and Borrowing** | **[← Previous: Introduction](01-Introduction.md)** | **[Next: Data Structures →](03-Data-Structures.md)** 📚

</div>

---


*This chapter covers Rust's ownership system. Continue to [Data Structures](03-Data-Structures.md) to learn how we model blockchain data with structs and enums.*