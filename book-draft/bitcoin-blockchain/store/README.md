<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. **Chapter 11: Storage Layer** ← *You are here*
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. <a href="../node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---

# Storage Layer — Persistence for Blocks, Tip, and Derived State

**Part I: Foundations & Core Implementation** | **Chapter 11: Storage Layer**

This chapter explains `bitcoin/src/store` as an implementer reads it: **how blocks become durable bytes on disk**, how we track the **tip**, and which write paths must be **atomic** to keep the node consistent after crashes.

> **Prerequisites**: This chapter builds on the block and transaction types from Chapter 6 (Primitives) and the validation logic from Chapters 9–10. You should be comfortable with Rust's `Result` type and basic async/await — the storage layer uses both extensively. No prior database experience is required; we introduce sled from scratch below.

**What you will learn in this chapter:** How blocks and chain metadata are persisted to disk using an embedded key-value store, why atomicity matters when updating the tip, and how the storage API is designed so that higher layers (node orchestration, wallet) never deal with raw bytes directly.

**Why storage matters for Bitcoin.** Without durable storage, a node would lose all blockchain history every time it restarts — it would have to re-download and re-validate the entire chain from peers. The storage layer also enforces a critical invariant: the tip hash and the blocks tree must always be consistent. If the node crashes halfway through writing a new block, the database must either contain both the block and the updated tip, or neither. This is why atomicity is the central design concern of this chapter.

---

## What “storage” means in our Rust Bitcoin implementation

In this codebase, “storage” is not a separate service; it is an embedded key-value database (**sled**) with a small set of conventions.

**What is sled?** Sled is a pure-Rust embedded database — think of it as a persistent `BTreeMap<Vec<u8>, Vec<u8>>` backed by a log-structured merge tree on disk. It requires no external process, no TCP connection, and no configuration file: you open a directory path and get a `sled::Db` handle. We chose it because it gives us atomic batch writes (critical for “insert block + move tip” in one operation) and compiles on every platform Rust targets. If you have used RocksDB or LevelDB before, sled fills the same niche with a Rust-native API.

Our conventions on top of sled:

- a **blocks tree** that maps `block_hash -> serialized Block bytes`
- a stable **tip key** (`"tip_block_hash"`) that points at the canonical tip hash
- atomic updates for “insert block + move tip” (sled transactions)

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



<div align="center">

**[← Chapter 10: Block Acceptance](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)** | **Chapter 11: Storage Layer** | **[Next: Chapter 11.A: Storage Layer — Code Walkthrough →](01-Storage-Layer-Code-Walkthrough.md)** 
</div>

---

> **Checkpoint:** After reading this chapter, you can verify the storage layer works by running the node, mining a few blocks, stopping it, and restarting. The chain height and tip hash should survive the restart. If they do not, check that `update_blocks_tree` is using a sled transaction (not separate writes) and that the tip key is being written atomically with the block data.

---

### Common Errors

> **Troubleshooting: Storage Layer**
>
> **"sled lock file already exists"** — Another process (or a crashed previous run) still holds the database lock. Delete the `db/__sled__/lock` file manually, or ensure the previous node process has fully exited.
>
> **Deserialization panic after code changes** — If you modify the `Block` struct and restart without clearing the database, sled will try to deserialize old bytes into the new struct layout. Delete the `db/` directory to start fresh, or implement a migration path.
>
> **"tree not found" on a fresh database** — Ensure `create_blockchain` is called before `open_blockchain`. The blocks tree must be explicitly created on first run.

---

### Further Reading

- **[sled documentation](https://docs.rs/sled)** — The official sled API reference, including transactions, iterators, and configuration options.
- **RocksDB comparison** — If you are considering a different embedded database, the `rust-rocksdb` crate provides a mature alternative with different performance trade-offs (write amplification vs. read latency).
- **Bitcoin Core's LevelDB usage** — The production Bitcoin client uses LevelDB for its chainstate and block index databases. Studying its key layout (`CDBWrapper`) reveals how a production system organizes block metadata.

---

