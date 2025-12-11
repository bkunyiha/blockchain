# Pattern Matching: Exhaustive Case Handling

Pattern matching is one of Rust's most powerful features—a way to destructure data and handle different cases exhaustively. Pattern matching appears throughout Rust code, from error handling to data processing, and it's central to how we work with enums and other data structures.

The `match` expression provides exhaustive pattern matching, ensuring we handle all possible cases. The `if let` expression offers concise handling of single patterns. Together with destructuring, pattern matching enables expressive, safe code. In this chapter, we'll explore how pattern matching works and how we use it throughout our blockchain implementation.

### match Expressions: Exhaustive Case Handling

The `match` expression is Rust's primary pattern matching construct. It allows us to handle different cases based on the structure and value of data, with the compiler ensuring we cover all possibilities.

**Example from `bitcoin/src/node/server.rs`:**

```rust
accept_res = listener.accept() => {
    match accept_res {
        Ok((stream, _peer)) => {
            // Handle successful connection
            tokio::spawn(async move {
                // Process stream
            });
        }
        Err(e) => {
            error!("accept error: {}", e);
        }
    }
}
```

This example shows `match` handling the result of `listener.accept()`. The `match` expression has two arms: one for `Ok` (successful connection) and one for `Err` (connection error). The compiler ensures we handle both cases—we can't forget to handle errors.

Pattern matching in `match` provides several powerful features:
- **Exhaustive Checking**: The compiler requires handling all possible cases. If we add a new variant to an enum, the compiler will point out all `match` expressions that need updating.
- **Pattern Binding**: We can bind values from patterns to variables. In `Ok((stream, _peer))`, we destructure the tuple, binding `stream` and ignoring `_peer`.
- **Guards**: We can add additional conditions with `if`. For example, `Ok(value) if value > 0 => { ... }` only matches when the value is positive.

The exhaustiveness checking is particularly valuable in our blockchain code. When we add new message types or error variants, the compiler ensures we update all code that processes these types, preventing bugs from unhandled cases.

### if let: Handling Single Patterns

Sometimes we only care about one case of a pattern match. Writing a full `match` with a catch-all arm can be verbose. Rust's `if let` expression provides a concise way to handle a single pattern while optionally handling the remaining cases.

**Example from `bitcoin/src/store/file_system_db_chain.rs`:**

```rust
let data = blocks_tree
    .get(DEFAULT_TIP_BLOCK_HASH_KEY)
    .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;
let tip_hash = if let Some(data) = data {
    String::from_utf8(data.to_vec())
        .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?
} else {
    // Create genesis block
    let coinbase_tx = Transaction::new_coinbase_tx(genesis_address)?;
    let block = Block::generate_genesis_block(&coinbase_tx);
    Self::update_blocks_tree(&blocks_tree, &block).await?;
    String::from(block.get_hash())
};
```

This example shows `if let` extracting a value from an `Option`. If `data` is `Some`, we extract the inner value and process it. If it's `None`, we execute the `else` block to create a genesis block. This is more concise than a full `match` expression when we only care about one case.

The `if let` expression provides several benefits:
- **Conciseness**: Less verbose than `match` when handling a single pattern
- **Pattern Binding**: Like `match`, `if let` can bind values from patterns, extracting data from `Some`, `Ok`, or other patterns
- **Optional Else**: We can include an `else` clause to handle the remaining cases, or omit it if we don't need to handle other cases

In our blockchain code, `if let` appears frequently when we're checking for the presence of optional data or handling specific error cases. It makes the code more readable by focusing attention on the case we care about.

### Destructuring: Extracting Data from Structures

Pattern matching isn't limited to `match` and `if let`—we can destructure data in many contexts, including variable bindings and function parameters. Destructuring allows us to extract values from tuples, structs, and enums concisely.

```rust
// Destructuring tuples
let (txid, vout) = (vec![1, 2, 3], 0);

// Destructuring structs
let TXInput { txid, vout, .. } = input;  // .. ignores other fields

// Destructuring enums in match
match message {
    MessageType::Version { best_height } => {
        // best_height is extracted from the variant
    }
    MessageType::Inv { op_type, items } => {
        // op_type and items are extracted
    }
    // ... other variants
}
```

Destructuring appears throughout our codebase. When we process network messages, we destructure enum variants to extract their data. When we work with transaction inputs and outputs, we destructure structs to access individual fields. The `..` syntax allows us to ignore fields we don't need, making destructuring flexible.

This pattern makes our code more readable by extracting relevant data at the point of use, rather than accessing fields through dot notation repeatedly. It's also more efficient, as the compiler can optimize destructuring operations.

## Summary

Pattern matching enables exhaustive, expressive handling of different cases. The `match` expression ensures we handle all possible variants, while `if let` provides concise handling of single patterns. Destructuring allows us to extract data from structures naturally.

In our blockchain, pattern matching appears everywhere—from error handling to network message processing to transaction validation. The compiler's exhaustive checking ensures we never miss a case, preventing bugs from unhandled variants.

In the next chapter, we'll explore derive macros, which automatically generate trait implementations and reduce boilerplate.

---

## Navigation

- **[← Previous: Smart Pointers](08-Smart-Pointers.md)** - Shared ownership with Arc and Rc
- **[Next: Derive Macros →](10-Derive-Macros.md)** - Automatic trait implementations
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Data Structures](03-Data-Structures.md)** - Structs and Enums to match on
- **[Error Handling](05-Error-Handling.md)** - Matching on Result and Option
- **[Iterators and Closures](14-Iterators-Closures.md)** - Pattern matching in iterators

**Related Guides:**
- **[Transaction System](../bitcoin-blockchain/02-Transaction-System.md)** - Pattern matching in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Matching on request types

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Pattern Matching** | **[← Previous](08-Smart-Pointers.md)** | **[Next →](10-Derive-Macros.md)** 📚

</div>

---


*This chapter covers pattern matching and exhaustive case handling. Continue to [Derive Macros](10-Derive-Macros.md) to learn automatic trait implementations.*