<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. **Chapter 2.5: Storage Layer** ← *You are here*
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

# Storage Layer — Persistence for Blocks, Tip, and Derived State

**Part I: Core Blockchain Implementation** | **Chapter 2.5: Storage Layer**

This chapter explains `bitcoin/src/store` as an implementer reads it: **how blocks become durable bytes on disk**, how we track the **tip**, and which write paths must be **atomic** to keep the node consistent after crashes.

The goal is that you can read this chapter without the repository open:

- every referenced method is printed in full in the code walkthrough chapter below, or is explicitly marked **defined earlier** with a link
- each section uses a consistent **Methods involved** box
- diagrams illustrate the DB layout and the “write path” for mined blocks vs inbound blocks

---

## What “storage” means in our Rust Bitcoin implementation

In this codebase, “storage” is not a separate service; it is an embedded key-value database (**sled**) with a small set of conventions:

- a **blocks tree** that maps `block_hash -> serialized Block bytes`
- a stable **tip key** (`"tip_block_hash"`) that points at the canonical tip hash
- atomic updates for “insert block + move tip” (sled transactions)

> **Methods involved**
>
> - `BlockchainFileSystem::{create_blockchain, open_blockchain}`
> - `BlockchainFileSystem::{get_tip_hash, get_last_block}`
> - `BlockchainFileSystem::{get_block, get_best_height}`
> - `BlockchainFileSystem::update_blocks_tree` (internal atomic write)

---

## Diagram: sled layout (the minimum schema)

```
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

**📚 [← Chapter 2.4: Blockchain (Technical Foundations)](../chain/README.md)** | **Chapter 2.5: Storage Layer** | **[Next: Chapter 2.5.A: Storage Layer — Code Walkthrough →](01-Storage-Layer-Code-Walkthrough.md)** 📚

</div>

---

