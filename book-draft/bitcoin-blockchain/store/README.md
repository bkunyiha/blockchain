<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. Chapter 1.4: Bitcoin Whitepaper In Rust - Bitcoin Whitepaper In Rust
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. **Chapter 2.5: Storage Layer** ← *You are here*
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)
12. Chapter 2.7: Network Layer - Peer-to-peer networking and protocol
13. Chapter 2.8: Node Orchestration - Node context and coordination
14. Chapter 2.9: Wallet System - Wallet implementation and key management
15. Chapter 3: Web API Architecture - REST API implementation
16. Chapter 4: Desktop Admin Interface - Iced framework architecture
17. Chapter 5: Wallet User Interface - Wallet UI implementation
18. Chapter 6: Embedded Database & Persistence - SQLCipher integration
19. Chapter 7: Web Admin Interface - React/TypeScript web UI

### Part II: Deployment & Operations

20. Chapter 8: Docker Compose Deployment - Docker Compose guide
21. Chapter 9: Kubernetes Deployment - Kubernetes production guide
22. Chapter 10: Rust Language Guide - Rust programming language reference

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

