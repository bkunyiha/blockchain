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
# Error Handling: Result and Option

Error handling is explicit and type-safe in Rust. For comprehensive treatment, see [The Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

`Result<T, E>` represents operations that can fail; `Option<T>` represents values that might not exist. Both force explicit handling, preventing silently ignored errors.

### Result Type: Explicit Success or Failure

The `Result<T, E>` type represents the outcome of an operation that can either succeed, producing a value of type `T`, or fail, producing an error of type `E`. This explicit representation makes error handling a first-class concern in Rust programs.

**Example from `bitcoin/src/primitives/transaction.rs`:**

```rust
impl TXOutput {
    pub fn new(value: i32, address: &WalletAddress) -> Result<TXOutput> {
        let mut output = TXOutput {
            value,
            in_global_mem_pool: false,
            pub_key_hash: vec![],
        };
        output.lock(address)?;  // ← ? operator propagates errors
        Ok(output)
    }

    fn lock(&mut self, address: &WalletAddress) -> Result<()> {
        let pub_key_hash = get_pub_key_hash(address)?;  // ← Error propagation
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }
}
```

This example shows how `Result` types flow through our code. The `new` function returns `Result<TXOutput>`, indicating it might fail (perhaps if the address is invalid). Inside the function, we call `lock()` which also returns `Result<()>`. The `?` operator automatically propagates errors—if `lock()` returns an error, the function immediately returns that error. If it succeeds, execution continues.

Rust provides several methods for working with `Result` types:
- **`?` operator**: The most common way to handle errors. If the result is `Ok`, it unwraps the value. If it's `Err`, it returns the error immediately, propagating it up the call stack.
- **`unwrap()`**: Unwraps an `Ok` value or panics on error. Useful in tests or when we're certain an operation cannot fail, but should be used sparingly in production code.
- **`expect()`**: Like `unwrap()`, but allows providing a custom panic message for debugging.
- **`map()`**: Transforms the `Ok` value if present, leaving errors unchanged.
- **`map_err()`**: Transforms the `Err` value if present, leaving successes unchanged.

The `?` operator is particularly powerful because it makes error handling concise while remaining explicit. Every `?` is a potential early return, making error paths visible in the code.

### Option Type: Representing Absence

While `Result` represents operations that can fail, `Option<T>` represents values that might not exist. This is Rust's way of handling null values safely—instead of allowing null pointers that can crash programs, Rust requires explicit handling of missing values.

**Example from `bitcoin/src/node/peers.rs`:**

```rust
pub fn first(&self) -> Result<Option<Node>> {
    let inner = self
        .inner
        .read()
        .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
    Ok(inner.iter().next().cloned())  // ← Returns Option<Node>
}
```

The `first()` method returns `Result<Option<Node>>`, showing how `Result` and `Option` can be combined. The outer `Result` represents potential errors (like lock poisoning), while the inner `Option` represents whether a node was found. The `next()` method on an iterator returns `Option<T>`, which we then wrap in `Ok()`.

Working with `Option` requires explicit handling of the `None` case. Rust provides several methods for this:
- **`unwrap_or()`**: Provides a default value if `None`, useful when a sensible default exists.
- **`unwrap_or_else()`**: Computes a default value lazily if `None`, useful when the default is expensive to compute.
- **`map()`**: Transforms the `Some` value if present, similar to `Result::map()`.
- **`and_then()`**: Chains operations that return `Option`, useful for operations that might fail.
- **`match`**: Exhaustive pattern matching ensures we handle both `Some` and `None` cases.

The key insight is that `Option` forces us to think about the absence case. We can't accidentally use a null value—we must explicitly handle the `None` case, preventing null pointer exceptions that plague other languages.

### Error Propagation: The `?` Operator

One of Rust's most elegant features is the `?` operator, which makes error propagation concise and readable. Instead of writing verbose `match` statements for every error case, `?` automatically propagates errors up the call stack.

**Example from `bitcoin/src/primitives/block.rs`:**

```rust
pub fn deserialize(bytes: &[u8]) -> Result<Block> {
    bincode::serde::decode_from_slice(bytes, bincode::config::standard())
        .map_err(|e| BtcError::BlockDeserializationError(e.to_string()))
        .map(|(block, _)| block)
}
```

The `?` operator here does several things automatically. First, it checks if the `decode_from_slice` operation succeeded. If it returned `Ok`, the `?` operator unwraps the value and execution continues. If it returned `Err`, the `?` operator immediately returns that error from the function.

The `map_err()` call converts the bincode error into our custom `BtcError` type, and `map()` extracts just the block from the tuple returned by bincode. This chaining of operations is idiomatic Rust—each operation transforms the result, building up the final value step by step.

The `?` operator is equivalent to writing a `match` statement with early return, but it's much more concise. More importantly, it makes error paths visible—every `?` in the code is a potential early return, making it easy to see where errors can occur.

### Custom Error Types: Domain-Specific Errors

While Rust's standard library provides basic error types, real applications need domain-specific errors that carry context about what went wrong. Our blockchain uses the `thiserror` crate to define a comprehensive error type that represents all possible failure modes in our system.

**Example from `bitcoin/src/error.rs`:**

```rust
use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum BtcError {
    #[error("Blockchain not found error: {0}")]
    BlockchainNotFoundError(String),

    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("Not enough funds")]
    NotEnoughFunds,
    // ... many more variants
}
```

Our `BtcError` enum uses the `thiserror::Error` derive macro, which automatically implements the `std::error::Error` trait and provides convenient error message formatting. Each variant can carry context—like the `BlockchainNotFoundError(String)` variant that includes details about what wasn't found.

This approach provides several advantages over generic error types:
- **Type Safety**: We can match on specific error variants, allowing different handling for different failure modes. Code can check `if let Err(BtcError::NotEnoughFunds) = result` to handle insufficient funds specially.
- **Rich Context**: Error variants can carry detailed information about what went wrong, making debugging easier. The `#[error(...)]` attribute provides human-readable error messages.
- **Error Conversion**: The `?` operator can automatically convert between error types when we implement the `From` trait, allowing seamless error propagation through layers that use different error types.

This error handling strategy ensures that errors are never silently ignored and that we have the context needed to diagnose and fix issues when they occur.

### Error Handling Performance

Rust's error handling is designed to be zero-cost when there are no errors. The `Result` type is the same size as the success value plus a discriminant byte, and error paths only execute when errors occur:

```rust
// Zero-cost in the success path
pub fn get_blockchain_height(&self) -> Result<usize> {
    // Success path: no overhead
    // Error path: only executes on error
}
```

**Performance Characteristics:**
- **Success path**: No runtime overhead compared to returning values directly
- **Error path**: Only pays cost when errors occur
- **No exceptions**: Unlike exceptions in other languages, Rust's error handling doesn't require stack unwinding in the success case
- **Optimization**: The compiler can optimize error handling aggressively

## Patterns: Early Return, Transformation, Context

```rust
// Early return with ?
pub fn process_block(block: Block) -> Result<()> {
    validate(&block)?;
    add_to_chain(block)?;
    Ok(())
}

// Error transformation
pub fn deserialize(bytes: &[u8]) -> Result<Block> {
    bincode::decode(bytes)
        .map_err(|e| BtcError::DeserializationError(e.to_string()))
}

// Error context with ok_or_else
pub fn get_block(height: usize) -> Result<Block> {
    self.blocks
        .get(height)
        .ok_or_else(|| BtcError::BlockNotFound(height))?
}
```

## Best Practices

- **`Result`** for recoverable errors; **`Option`** for missing values
- Propagate with **`?`** operator for conciseness
- Add context with **`map_err()`**
- Match on specific error types for different handling

## Summary

Rust's error handling is explicit and type-safe. `Result` and `Option` force explicit handling, preventing silently ignored errors. The `?` operator enables concise propagation. Custom error types provide domain-specific context.

---

## Navigation

- **[← Previous: Traits](04-Traits.md)** - Polymorphism and code reuse
- **[Next: Generics →](06-Generics.md)** - Type parameters and code reuse
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Type Conversions](15-Type-Conversions.md)** - TryFrom and TryInto for fallible conversions
- **[Pattern Matching](09-Pattern-Matching.md)** - Matching on Result and Option
- **[Testing](16-Testing.md)** - Testing error cases
- **[Best Practices](17-Best-Practices.md)** - Error handling patterns

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Error handling in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - API error responses

---

<div align="center">

**[← Rust Guide Index](README.md)** | **Error Handling** | **[← Previous](04-Traits.md)** | **[Next →](06-Generics.md)** 

</div>

---


*This chapter covers error handling with Result and Option. Continue to [Generics](06-Generics.md) to learn how to write reusable, type-safe code.*