<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
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
## Appendix B. Mapping to this repository
This whitepaper chapter explains *what* to build and *why*, with sample Rust implementation code. The next step is to read the actual Rust implementation and learn *how* the pieces connect in a real project.

In this appendix, we provide a practical map between:

- the whitepaper concepts we just studied
 - the implementation documentation in the next chapters
 - the Rust code in the repository ([`bkunyiha/blockchain`](https://github.com/bkunyiha/blockchain))

If you have not cloned the repository yet, start from the end of Section 12 and clone [`bkunyiha/blockchain`](https://github.com/bkunyiha/blockchain) so you can follow along locally.

### B.1 How to read the codebase (a guided tour order)

When learning an unfamiliar blockchain implementation, we recommend reading in the same direction bytes flow through the system:
In the next chapters, we move from the whitepaper concepts into the Rust implementation code that goes with this book.

1. **Business objects and encoding**
   - Chapter: `Business objects and encoding(block, header, transaction)`
   - Code: `bitcoin/src/primitives/` (block, header, transaction)
2. **Utility helpers (bytes, plumbing, small reusable building blocks)**
   - Docs: `Utility helpers (bytes, plumbing, small reusable building blocks)`
   - Code: `bitcoin/src/util/`
3. **Crypto primitives(hashes, keys, signatures, address/script helpers)**
   - Docs: `Crypto primitives(hashes, keys, signatures, address/script helpers)`
   - Code: `bitcoin/src/crypto/` (hashes, keys, signatures, address/script helpers)
4. **Proof-of-work and block acceptance**
   - Docs: `Proof-of-work and block acceptance`
   - Code: `bitcoin/src/pow.rs`, `bitcoin/src/chain/`
5. **Storage and persistence**
   - Docs: `Storage and persistence`
   - Code: `bitcoin/src/store/` (persisting chain/wallet data)
6. **Chainstate / UTXO updates**
   - Docs: `Chainstate / UTXO updates`
   - Code: `bitcoin/src/chain/chainstate.rs`, `bitcoin/src/chain/utxo_set.rs`
7. **Networking and message handling**
   - Docs: `Networking and message handling`
   - Code: `bitcoin/src/net/`, `bitcoin/src/net/net_processing.rs`
8. **Node orchestration**
   - Docs: `Node orchestration`
   - Code: `bitcoin/src/node/` (peers, server, miner, mempool integration)
9. **Wallet/client**
   - Docs: `Wallet/client`
   - Code: `bitcoin/src/wallet/` (wallet implementation + service layer)

### B.2 Concept map: whitepaper → code modules

Use this table as a “where do I look?” index while reading:

- **Transactions / UTXO model** (Sections 2, 9)
  - Code: `bitcoin/src/primitives/transaction.rs`, `bitcoin/src/chain/utxo_set.rs`
- **Block headers / chaining** (Section 3)
  - Code: `bitcoin/src/primitives/block.rs`, `bitcoin/src/primitives/blockchain.rs`
- **Proof-of-work and difficulty** (Section 4)
  - Code: `bitcoin/src/pow.rs`
- **Network operation** (Section 5)
  - Code: `bitcoin/src/net/`, `bitcoin/src/node/server.rs`, `bitcoin/src/node/peers.rs`
- **Incentives / fees** (Section 6)
  - Code: `bitcoin/src/node/miner.rs`, `bitcoin/src/node/txmempool.rs`
- **Reclaiming disk space / pruning mindset** (Section 7)
  - Code: `bitcoin/src/store/`, `bitcoin/src/chain/`
- **Merkle trees / SPV intuition** (Sections 7–8)
  - Code: `bitcoin/src/primitives/block.rs` (Merkle-related helpers if present), plus wallet/client logic where applicable
- **Privacy / address rotation** (Section 10)
  - Code: `bitcoin/src/wallet/`, `bitcoin/src/crypto/address.rs`
- **Confirmations / settlement policy** (Section 11)
  - Code: wallet/client policy layer (often under `bitcoin/src/wallet/` or service code that decides when to mark a tx “settled”)

### B.3 The documentation to keep open while reading

For deeper implementation details of what we just went through in the whitepaper, the chapter sections that follow are already aligned to these boundaries:

- **Chain state / UTXO**: Section 2.4: Blockchain — From Transaction to Block Acceptance
- **Crypto primitives**: Section 2.3: Cryptography
- **Primitives**: Section 2.1: Primitives
- **Networking**: Section 2.7: Network Layer
- **Node orchestration**: Section 2.8: Node Orchestration
- **Wallet**: Section 2.9: Wallet System

### B.4 A simple “follow the bytes” exercise

To turn reading into understanding, pick one path and trace it end-to-end in code:

- **Receiving and confirming a transaction**:
  - network receives a `tx` message → parse → validate → add to mempool → mine/include in block → connect block → wallet updates confirmation depth

As you do this, keep asking two questions that drive correct implementations:

- What exact bytes are hashed/verified here?
- What state is updated, and how do we roll it back on a reorg?

---

<div align="center">

**[[Start] ← Bitcoin White-paper (Rust Implementation/Encoding)](README)** | **[[Previous] ←  Appendix A: Object connectivity](Appendix-A-Object-connectivity-end-to-end-flow)** | Appendix B: Mapping to this repository

</div>
