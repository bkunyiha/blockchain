<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. **Chapter 10: Rust Language Guide** ← *You are here*

</details>

</div>

---
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
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Pattern matching in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Matching on request types

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Pattern Matching** | **[← Previous](08-Smart-Pointers.md)** | **[Next →](10-Derive-Macros.md)** 📚

</div>

---


*This chapter covers pattern matching and exhaustive case handling. Continue to [Derive Macros](10-Derive-Macros.md) to learn automatic trait implementations.*