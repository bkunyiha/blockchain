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
# Iterators and Closures: Functional Programming in Rust

Iterators and closures enable functional programming in Rust. For comprehensive treatment, see [The Rust Book: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) and [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html).

## Iterators: Lazy, Composable Data Processing

From `bitcoin/src/net/net_processing.rs`:

```rust
nodes_to_add.iter().for_each(|node| {
    let node_addr = *node;
    tokio::spawn(async move {
        send_known_nodes(&node_addr, all_nodes).await;
    });
});
```

Iterators are lazy—operations don't execute until consumed. Methods:
- **`iter()`**: Immutable references
- **`iter_mut()`**: Mutable references
- **`into_iter()`**: Takes ownership
- **`map()`, `filter()`**: Transform and filter lazily
- **`collect()`**: Consume and collect into collection
- **`for_each()`**: Execute closure for each item

Chains compile to efficient code—often as fast as hand-written loops.

## Closures: Anonymous Functions with Capture

From `bitcoin/src/chain/chainstate.rs`:

```rust
async fn read<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(BlockchainFileSystem) -> Fut + Send,
    Fut: Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    let blockchain_guard = self.0.read().await;
    f(blockchain_guard.clone()).await
}
```

Three closure traits based on capture:
- **`FnOnce`**: Called once; takes ownership of captured variables
- **`FnMut`**: Multiple calls; requires mutable borrow
- **`Fn`**: Multiple calls; requires immutable borrow

Compiler automatically determines which trait applies.

## Iterator Chains

```rust
let result: Vec<i32> = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|&x| x % 2 == 0)    // Keep even
    .map(|x| x * 2)              // Double
    .collect();                  // Result: [4, 8]
```

Operations are lazy; no computation until `collect()`. Compiler optimizes to efficient code.

## Summary

Iterators provide lazy, composable data processing with zero-cost abstractions. Closures capture environment and pass behavior as values. Together they enable expressive, efficient functional programming patterns.
    .sum();

// Compiler optimizes to roughly:
// let mut sum = 0;
// for tx in transactions {
//     if tx.is_valid() {
//         sum += tx.value();
//     }
// }
```

**Key Performance Points:**
- **Lazy evaluation**: Iterators don't compute until consumed
- **Loop fusion**: Multiple iterator operations can be fused into a single loop
- **Zero allocations**: Iterator chains typically don't allocate intermediate collections
- **SIMD optimization**: Compiler can vectorize iterator operations when possible

### Advanced Iterator Patterns

**Pattern 1: Chunking**
```rust
// Process transactions in batches
for chunk in transactions.chunks(100) {
    process_batch(chunk).await?;
}
```

**Pattern 2: Windows**
```rust
// Compare adjacent blocks
for window in blocks.windows(2) {
    validate_chain(window[0], window[1])?;
}
```

**Pattern 3: Enumerate**
```rust
// Get index and value
for (index, transaction) in transactions.iter().enumerate() {
    process(index, transaction)?;
}
```

### Closure Capture Modes

Closures capture variables in different ways based on usage:

**Move Semantics:**
```rust
let data = vec![1, 2, 3];
let closure = move || {
    // Takes ownership of data
    println!("{:?}", data);
};
// data is no longer accessible here
```

**Borrow Semantics:**
```rust
let data = vec![1, 2, 3];
let closure = || {
    // Borrows data immutably
    println!("{:?}", data);
};
// data is still accessible
```

**Mutable Borrow:**
```rust
let mut data = vec![1, 2, 3];
let mut closure = || {
    // Borrows data mutably
    data.push(4);
};
```

### Iterator Adapters

Common iterator adapters used in our blockchain:

- **`map()`**: Transform each element
- **`filter()`**: Keep elements matching predicate
- **`filter_map()`**: Filter and transform in one step
- **`flat_map()`**: Flatten nested iterators
- **`take()` / `skip()`**: Limit or skip elements
- **`zip()`**: Combine two iterators
- **`chain()`**: Concatenate iterators
- **`fold()` / `reduce()`**: Accumulate values

**Example: Complex Processing**
```rust
let total_value: i64 = blocks
    .iter()
    .flat_map(|block| block.transactions.iter())
    .filter_map(|tx| tx.outputs.first())
    .map(|output| output.value as i64)
    .sum();
```

In our blockchain, iterators enable concise, efficient processing of transactions, blocks, and network messages, combining readability with performance.

## Summary

Iterators and closures enable functional programming patterns in Rust with zero runtime overhead. Iterators provide lazy, composable iteration that compiles to efficient loops. Closures capture their environment and can be passed as data, enabling flexible, expressive code.

The compiler optimizes iterator chains aggressively, often producing code as efficient as hand-written loops. Understanding closure capture modes helps us write efficient closures, and iterator adapters enable powerful data transformations.

In the next chapter, we'll explore type conversions, which enable converting between types safely and explicitly.

---

## Navigation

- **[← Previous: Modules](13-Modules.md)** - Code organization and visibility
- **[Next: Type Conversions →](15-Type-Conversions.md)** - Converting between types
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Pattern Matching](09-Pattern-Matching.md)** - Matching in iterator chains
- **[Generics](06-Generics.md)** - Generic iterator functions
- **[Testing](16-Testing.md)** - Testing iterators
- **[Best Practices](17-Best-Practices.md)** - Iterator patterns

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Iterators in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - Processing collections

---

<div align="center">

**[← Rust Guide Index](README.md)** | **Iterators and Closures** | **[← Previous](13-Modules.md)** | **[Next →](15-Type-Conversions.md)** 

</div>

---


*This chapter covers iterators and closures. Continue to [Type Conversions](15-Type-Conversions.md) to learn how to convert between types.*