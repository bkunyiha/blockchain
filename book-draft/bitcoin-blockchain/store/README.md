<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. **Chapter 20: Storage Layer** ← *You are here*
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="../web/README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---

# Chapter 20: Storage Layer — Persistence for Blocks, Tip, and Derived State

**Part I: Foundations & Core Implementation** | **Chapter 20: Storage Layer**

This chapter explains `bitcoin/src/store` as an implementer reads it: **how blocks become durable bytes on disk**, how we track the **tip**, and which write paths must be **atomic** to keep the node consistent after crashes.

> **Prerequisites:**: This chapter builds on the block and transaction types from Chapter 6 (Primitives) and the validation logic from Chapters 9–10. You should be comfortable with Rust's `Result` type and basic async/await — the storage layer uses both extensively. No prior database experience is required; we introduce sled from scratch below.

**Why storage matters for Bitcoin.** Without durable storage, a node would lose all blockchain history every time it restarts — it would have to re-download and re-validate the entire chain from peers. The storage layer also enforces a critical invariant: the tip hash and the blocks tree must always be consistent. If the node crashes halfway through writing a new block, the database must either contain both the block and the updated tip, or neither. This is why atomicity is the central design concern of this chapter.

> **What you will learn in this chapter:**
> - Use the Sled embedded database for persistent block and chain state storage
> - Understand how the BlockchainFileSystem abstracts low-level storage operations
> - Implement efficient persistence and retrieval of blocks, chain state, and UTXO data
> - Explain the trade-offs in the storage design and why Sled was chosen

---

## What “storage” means in our Rust Bitcoin implementation

In this codebase, “storage” is not a separate service; it is an embedded key-value database (**sled**) with a small set of conventions.

**What is sled?** Sled is a pure-Rust embedded database — think of it as a persistent `BTreeMap<Vec<u8>, Vec<u8>>` backed by a log-structured merge tree on disk. It requires no external process, no TCP connection, and no configuration file: you open a directory path and get a `sled::Db` handle. We chose it because it gives us atomic batch writes (critical for “insert block + move tip” in one operation) and compiles on every platform Rust targets. If you have used RocksDB or LevelDB before, sled fills the same niche with a Rust-native API.

Our conventions on top of sled:

- a **blocks tree** that maps `block_hash -> serialized Block bytes`
- a stable **tip key** (`”tip_block_hash”`) that points at the canonical tip hash
- atomic updates for “insert block + move tip” (sled transactions)

Think of sled as a persistent in-memory data structure (like a `BTreeMap`) that writes changes to disk atomically. If the process crashes, the database either contains both the block and the updated tip, or neither — it never gets stuck in an inconsistent state.

> **Tip:** When debugging storage issues, Sled's `Db::export()` and `Db::import()` methods can dump the entire database contents for inspection. This is invaluable during development.

The key methods are `BlockchainFileSystem::create_blockchain` and `open_blockchain` (lifecycle), `get_tip_hash` and `get_last_block` (reads), and `update_blocks_tree` (the internal atomic write that inserts a block and advances the tip in a single sled transaction).

## Diagram: sled layout (the minimum schema)

```text
sled::Db at ./<TREE_DIR>/
  |
  └─ Tree "<BLOCKS_TREE>"  (default: "blocks1")
       |
       |-- key: "<block_hash_string>"  -> value: serialized Block (bytes)
       |
       └-- key: "tip_block_hash"       -> value: "<block_hash_string>" (bytes)
```

This chapter is about making those two invariants true:

- if a block exists, it can be fetched by hash
- tip always points at a block that exists (or a known “empty” sentinel)

---

## Exercises

1. **Storage Round-Trip Test** — Write a test that creates a block, stores it using BlockchainFileSystem, retrieves it, and verifies all fields match the original. Pay attention to serialization: does every field survive the round trip?

2. **UTXO Retrieval Performance** — Examine how UTXO lookups work in the Sled storage layer. If the blockchain has 10,000 transactions with an average of 2 outputs each, how many entries are in the UTXO set? What data structure does Sled use internally that makes lookups efficient?

---

## Summary

- We explored how the BlockchainFileSystem uses the Sled embedded database to persist blocks, chain state, and UTXO data to disk.
- We implemented the storage operations that allow efficient insertion and retrieval of blockchain data.
- We examined the storage abstraction that decouples chain logic from persistence details, making it possible to swap storage backends.

In the next chapter, we build the networking layer that enables nodes to communicate, propagate blocks, and maintain consensus across a distributed network.

---

<div align=”center”>

**[← Chapter 28: Block Acceptance](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)** | **Chapter 20: Storage Layer** | **[Next: Chapter 11.A: Storage Layer — Code Walkthrough →](01-Storage-Layer-Code-Walkthrough.md)**
</div>

---

> **Checkpoint:** After reading this chapter, you can verify the storage layer works by running the node, mining a few blocks, stopping it, and restarting. The chain height and tip hash should survive the restart. If they do not, check that `update_blocks_tree` is using a sled transaction (not separate writes) and that the tip key is being written atomically with the block data.

---

### Common Errors

> **Troubleshooting: Storage Layer**
>
> **"sled lock file already exists"** — Another process (or a crashed previous run) still holds the database lock. Delete the `db/__sled__/lock` file manually, or ensure the previous node process has fully exited.
>
> **Troubleshooting:** **Deserialization panic after code changes** — If you modify the `Block` struct and restart without clearing the database, sled will try to deserialize old bytes into the new struct layout. Delete the `db/` directory to start fresh, or implement a migration path.
>
> **"tree not found" on a fresh database** — Ensure `create_blockchain` is called before `open_blockchain`. The blocks tree must be explicitly created on first run.

---

### Further Reading

- **[sled documentation](https://docs.rs/sled)** — The official sled API reference, including transactions, iterators, and configuration options.
- **RocksDB comparison** — If you are considering a different embedded database, the `rust-rocksdb` crate provides a mature alternative with different performance trade-offs (write amplification vs. read latency).
- **Bitcoin Core's LevelDB usage** — The production Bitcoin client uses LevelDB for its chainstate and block index databases. Studying its key layout (`CDBWrapper`) reveals how a production system organizes block metadata.

---

