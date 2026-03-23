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

**Key Performance Points:**
- **Lazy evaluation**: Iterators don't compute until consumed
- **Loop fusion**: Multiple iterator operations can be fused into a single loop
- **Zero allocations**: Iterator chains typically don't allocate
  intermediate collections
- **SIMD optimization**: Compiler can vectorize iterator operations
  when possible

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

In our blockchain, iterators enable concise, efficient processing of
transactions, blocks, and network messages, combining readability
with performance.

## Summary

Iterators and closures enable functional programming patterns in Rust
with zero runtime overhead. Iterators provide lazy, composable
iteration that compiles to efficient loops. Closures capture their
environment and can be passed as data, enabling flexible, expressive
code.

The compiler optimizes iterator chains aggressively, often producing
code as efficient as hand-written loops. Understanding closure capture
modes helps us write efficient closures, and iterator adapters enable
powerful data transformations.

In the next chapter, we'll explore type conversions, which enable
converting between types safely and explicitly.

---

## Navigation

- **[← Previous: Modules](13-Modules.md)** - Code organization and visibility
- **[Next: Type Conversions →](15-Type-Conversions.md)** - Converting
  between types
- **[Rust Guide Index](README.md)** - Complete guide overview and
  table of contents
- **[Pattern Matching](09-Pattern-Matching.md)** - Matching in iterator chains
- **[Generics](06-Generics.md)** - Generic iterator functions
- **[Testing](16-Testing.md)** - Testing iterators
- **[Best Practices](17-Best-Practices.md)** - Iterator patterns

**Related Guides:**
- Iterators in practice - see
  **[Transaction ID Format]
  (../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)**
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** -
  Processing collections

---

<div align="center">

**[← Rust Guide Index](README.md)** | **Iterators and Closures** |
**[← Previous](13-Modules.md)** | **[Next →](15-Type-Conversions.md)**

</div>

---


*This chapter covers iterators and closures. Continue to
[Type Conversions](15-Type-Conversions.md) to learn how to convert
between types.*