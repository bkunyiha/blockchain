<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. Chapter 1.4: Bitcoin Whitepaper In Rust - Bitcoin Whitepaper In Rust
5. **Chapter 2.0: Rust Blockchain Project** ← *You are here*
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. Chapter 2.5: Storage Layer - Persistent storage implementation
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
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
## Rust Project Index (Chapter 2 — Walking through Rust Bitcoin implementation)

In Chapter 1, we used the Bitcoin whitepaper to build a minimal mental model and a “Bitcoin-shaped” implementation. In this chapter, we switch from concepts to code. We read the Rust project as a working system and learn how the pieces connect in practice.

### Before we start: clone the repository

To follow along locally, clone the repository from GitHub ([`bkunyiha/blockchain`](https://github.com/bkunyiha/blockchain)) and open it in your editor.

In the repository, the Rust implementation we will walk through lives under the `bitcoin/` crate.

### How to use this chapter

As we read the codebase, we keep two questions in mind:

- **What exact bytes are parsed, serialized, hashed, and verified here?**
- **What state is updated, and how would we roll it back on a reorg?**

We recommend reading in roughly the same direction bytes flow through the system: primitives → crypto → PoW/validation → storage/chainstate → networking/node → wallet/client.

### Rust refresher (optional)

If we want to brush up on Rust before diving into the implementation, we can read the Rust guide:

- **Chapter 10: Rust Language Guide** — start with **Rust Installation & Setup**, then continue to **Introduction**.

### Guided tour (recommended reading order)

Each step below points to:

- the **book chapter** (documentation) that explains the subsystem
- the **code location** in the Rust project that implements it

1. **Business objects and encoding (block, header, transaction)**
   - Chapter section: Section 2.1: Primitives
   - Code: `bitcoin/src/primitives/` (block, header, transaction)

2. **Utility helpers (bytes, plumbing, small reusable building blocks)**
   - Chapter section: Section 2.2: Utilities
   - Code: `bitcoin/src/util/`

3. **Crypto primitives (hashes, keys, signatures, address/script helpers)**
   - Chapter section: Section 2.3: Cryptography
   - Code: `bitcoin/src/crypto/`

4. **Proof-of-work and block acceptance**
   - Chapter section: Section 2.4: Blockchain — From Transaction to Block Acceptance
   - Code: `bitcoin/src/pow.rs`, `bitcoin/src/chain/`

5. **Storage and persistence**
   - Chapter section: Section 2.5: Storage Layer
   - Code: `bitcoin/src/store/` (persisting chain/wallet data)

6. **Chainstate / UTXO updates**
   - Chapter section: Section 2.4.2: Blockchain State Management
   - Code: `bitcoin/src/chain/chainstate.rs`, `bitcoin/src/chain/utxo_set.rs`

7. **Networking and message handling**
   - Chapter section: Section 2.7: Network Layer
   - Code: `bitcoin/src/net/`, `bitcoin/src/net/net_processing.rs`

8. **Node orchestration**
   - Chapter section: Section 2.8: Node Orchestration
   - Code: `bitcoin/src/node/` (peers, server, miner, mempool integration)

9. **Wallet/client**
   - Chapter section: Section 2.9: Wallet System
   - Code: `bitcoin/src/wallet/` (wallet implementation + service layer)

### A small exercise (recommended)

To turn reading into understanding, pick one end-to-end flow and trace it through code:

- **Receiving and confirming a transaction**: network receives a `tx` message → parse → validate → mempool → mine/include in block → connect block → wallet updates confirmation depth.

If we can narrate that flow from memory and point to the relevant modules, we are ready for the deeper chapters that follow.


---

<div align="center">

**[← Bitcoin Whitepaper In Rust](whitepaper-rust/README.md)** | Walking through Rust Bitcoin implementation | **[Business Objects →](whitepaper-rust/00-business-objects.md)**

</div>