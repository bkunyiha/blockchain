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
12. Chapter 2.7: Network Layer - Peer-to-peer networking and protocol
13. **Chapter 2.8: Node Orchestration** ← *You are here*
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

# Node Orchestration — Coordinating the Runtime

**Part I: Core Blockchain Implementation** | **Chapter 2.8: Node Orchestration**

This chapter explains the `bitcoin/src/node` module as a Rust implementer reads it: **a coordination layer** that turns “network messages and API calls” into “chainstate updates, mempool updates, mining, and peer relay”.

The goal is that you can read this chapter without the repository open:

- every referenced method is printed in full in the code walkthrough chapter below, or is explicitly marked **defined earlier** with a link
- each section contains a consistent **Methods involved** box
- diagrams are used to make message routing and task concurrency obvious

---

## What “node orchestration” means in our Rust Bitcoin implementation

At runtime, several subsystems must work together:

- **Network** receives bytes and produces typed messages (`Package`)
- **NodeContext** decides what subsystem should handle the message (mempool, chainstate, mining)
- **Chainstate** persists blocks and updates the canonical tip + UTXO
- **Mempool** stores unconfirmed transactions and marks outputs as “in mempool”
- **Mining** turns mempool contents into a block and relays the new tip
- **Peers** is the node’s peer set used for relay

```
TCP stream -> Package -> NodeContext -> (mempool | add_block | mining | peer relay)
```

> **Methods involved**
>
> - `NodeContext::process_transaction(...)`
> - `NodeContext::add_block(...)`
> - `NodeContext::mine_empty_block(...)`
> - `txmempool::{add_to_memory_pool, remove_from_memory_pool}`
> - `miner::{should_trigger_mining, process_mine_block, broadcast_new_block}`

---

## Where the full code walkthrough lives

The code-complete walkthrough for `bitcoin/src/node` is in:

- **[Chapter 2.8.A: Node Orchestration — Code Walkthrough](01-Node-Orchestration-Code-Walkthrough.md)**

It includes full method bodies for:

- `NodeContext` (core coordinator)
- mempool helpers (`txmempool.rs`)
- mining pipeline (`miner.rs`)
- peer set (`peers.rs`)

---

<div align="center">

**📚 [← Chapter 2.7: Network Layer](../net/README.md)** | **Chapter 2.8: Node Orchestration** | **[Chapter 2.8.A: Code Walkthrough →](01-Node-Orchestration-Code-Walkthrough.md)** 📚

</div>

---
