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
10. Chapter 2.5: Storage Layer - Persistent storage implementation
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
12. **Chapter 2.7: Network Layer** ← *You are here*
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

# Network Layer (P2P) — Message Routing, Inventory, and Sync

**Part I: Core Blockchain Implementation** | **Chapter 2.7: Network Layer**

This chapter explains the network layer as an implementer reads it in Rust: **as a pipeline of concrete methods** that transform bytes on a TCP stream into node actions (mempool admission, block download, block connection).

The goal is that you can read this chapter without the repository open:

- every method we reference is printed in full in the walkthrough chapter below
- every flow is accompanied by a diagram and a “Methods involved” box

---

## Chapter map (what “network” means in this repo)

This repository’s P2P layer is deliberately small:

- **Transport**: TCP streams
- **Encoding**: JSON via `serde_json`
- **Message model**: `Package` enum + `OpType` enum
- **Dispatcher**: `process_stream(...)` (routes each inbound package to the right action)
- **Send primitives**: `send_data(...)` and typed wrappers (`send_inv`, `send_get_data`, `send_block`, `send_tx`, `send_version`, …)
- **Peer bootstrap**: central-node concept + “known nodes” exchange

The core implementation lives in:

- `bitcoin/src/net/net_processing.rs`
- `bitcoin/src/node/server.rs` (TCP accept loop + bootstrap wiring)

---

## What you will be able to do after this chapter

You will be able to:

- trace **INV → GETDATA → (TX|BLOCK)** in code, by method name
- explain how `process_stream(...)` turns a TCP stream into a sequence of `Package` values
- identify where peer discovery happens (`KnownNodes` + `process_known_nodes`)
- understand which parts of the pipeline are **network-only** vs delegated to node/chain logic

---

## Diagram: the minimal protocol loop in this implementation

```
Peer A has an object (tx or block)
  |
  | 1) announce inventory (hash only)
  v
INV { op_type, items=[id] }  ----->  Peer B
                                     |
                                     | 2) request bytes for missing object
                                     v
                               GETDATA { op_type, id }  ----->  Peer A
                                                                |
                                                                | 3) send full bytes
                                                                v
                                                         (TX | BLOCK)  ----->  Peer B
                                                                                 |
                                                                                 | 4) hand off to node logic
                                                                                 v
                                                                           mempool / add_block
```

This loop is the core of the “gossip + fetch” strategy used throughout Bitcoin-like systems.

> **Methods involved**
>
> - `process_stream(...)` (dispatcher)
> - `send_inv(...)`, `send_get_data(...)` (announce + request)
> - `send_tx(...)`, `send_block(...)` (deliver full bytes)
> - `process_known_nodes(...)` (peer discovery)
>
> Full listings: **Chapter 2.7.A** (linked below).

---

## Where the full walkthrough lives

The full, code-centric walkthrough (with complete method listings) is in:

- **[Chapter 2.7.A: Network Layer — Code Walkthrough](01-Network-Operation-Code-Walkthrough.md)**

---

<div align="center">

**📚 [← Chapter 2.6: Block Acceptance (Whitepaper Step 5)](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)** | **Chapter 2.7: Network Layer** | **[Chapter 2.7.A: Network Layer — Code Walkthrough →](01-Network-Operation-Code-Walkthrough.md)** 📚

</div>

---

