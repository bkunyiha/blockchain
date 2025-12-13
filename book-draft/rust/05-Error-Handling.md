# Error Handling: Result and Option

Error handling is a critical aspect of any production system. In traditional languages, errors are often handled through exceptions or return codes, leading to error paths that are easy to ignore or forget. Rust takes a fundamentally different approach: errors are explicit in function signatures, and the type system forces us to handle them. This design prevents entire classes of bugs where errors are silently ignored.

In this chapter, we'll explore Rust's error handling mechanisms: the `Result` type for operations that can fail, the `Option` type for values that might not exist, and the `?` operator for concise error propagation. We'll see how these features work together to create robust, maintainable error handling in our blockchain.

Rust provides two primary types for representing potential failures: `Result<T, E>` for operations that can fail, and `Option<T>` for operations that might not produce a value. Both types force explicit handling, making error paths visible and preventing accidental error propagation.

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

### Error Propagation Patterns

**Pattern 1: Early Return with `?`**
```rust
pub fn process_block(block: Block) -> Result<()> {
    validate_block(&block)?;  // Early return on error
    add_to_chain(block)?;     // Early return on error
    Ok(())
}
```

**Pattern 2: Error Transformation**
```rust
pub fn deserialize(bytes: &[u8]) -> Result<Block> {
    bincode::decode(bytes)
        .map_err(|e| BtcError::DeserializationError(e.to_string()))
}
```

**Pattern 3: Error Context**
```rust
pub fn get_block(height: usize) -> Result<Block> {
    self.blocks
        .get(height)
        .ok_or_else(|| BtcError::BlockNotFound(height))?
}
```

### Error Handling Best Practices

1. **Use `Result` for recoverable errors**: Operations that can fail should return `Result`
2. **Use `Option` for missing values**: When a value might not exist, use `Option`
3. **Propagate with `?`**: Use `?` for concise error propagation
4. **Add context**: Use `map_err` to add context to errors
5. **Match on specific errors**: Use pattern matching to handle different error cases differently

In our blockchain, these patterns ensure robust error handling throughout the system, from transaction validation to network communication.

## Summary

Rust's error handling is explicit and type-safe. The `Result` type represents operations that can fail, while `Option` represents values that might not exist. Both types force explicit handling, preventing bugs from silently ignored errors.

The `?` operator makes error propagation concise and readable, while custom error types enable domain-specific error handling with rich context. This approach ensures errors are never silently ignored and provides the information needed to diagnose and fix issues.

In the next chapter, we'll explore generics, which allow us to write reusable code that works with multiple types while maintaining type safety—another key feature that enables flexible, maintainable code.

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

**📚 [← Rust Guide Index](README.md)** | **Error Handling** | **[← Previous](04-Traits.md)** | **[Next →](06-Generics.md)** 📚

</div>

---


*This chapter covers error handling with Result and Option. Continue to [Generics](06-Generics.md) to learn how to write reusable, type-safe code.*