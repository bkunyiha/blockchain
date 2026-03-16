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
# Pattern Matching: Exhaustive Case Handling

# Pattern Matching: Exhaustive Case Handling

Pattern matching destructures data and handles cases exhaustively. For comprehensive treatment, see [The Rust Book: Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html).

## match Expressions

The `match` expression handles all possible cases. From `bitcoin/src/node/server.rs`:

```rust
match listener.accept() {
    Ok((stream, _peer)) => {
        tokio::spawn(async move { /* process stream */ });
    }
    Err(e) => error!("accept error: {}", e),
}
```

The compiler enforces exhaustiveness—all variants must be handled. New enum variants trigger compile errors in all relevant `match` statements.

## if let: Single Patterns

`if let` concisely handles one pattern:

```rust
let tip_hash = if let Some(data) = data {
    process(data)
} else {
    default_value()
};
```

Less verbose than `match` when handling one case.

## Destructuring

Extract data from tuples, structs, and enums:

```rust
let (txid, vout) = (vec![1, 2, 3], 0);
let TXInput { txid, vout, .. } = input;  // .. ignores other fields

match message {
    MessageType::Version { best_height } => { /* ... */ }
    MessageType::Inv { op_type, items } => { /* ... */ }
}
```

## Summary

Pattern matching ensures exhaustive case handling via compiler-enforced `match` statements. `if let` provides concise single-pattern handling. Destructuring extracts data from structures naturally. Exhaustiveness prevents bugs from unhandled variants.

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

**[← Rust Guide Index](README.md)** | **Pattern Matching** | **[← Previous](08-Smart-Pointers.md)** | **[Next →](10-Derive-Macros.md)** 

</div>

---


*This chapter covers pattern matching and exhaustive case handling. Continue to [Derive Macros](10-Derive-Macros.md) to learn automatic trait implementations.*