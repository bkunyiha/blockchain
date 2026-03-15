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
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. **Chapter 2.8: Node Orchestration** ← *You are here*
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

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
