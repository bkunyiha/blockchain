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
12. **Chapter 2.7: Network Layer** ← *You are here*
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

An additional technical appendix explains the transport trade-offs and an actionable migration plan:

- **[Appendix: `std::net::TcpStream` vs `tokio::net::TcpStream`](02-Std-vs-Tokio-TcpStream.md)**

---

<div align="center">

**📚 [← Chapter 2.6: Block Acceptance (Whitepaper Step 5)](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)** | **Chapter 2.7: Network Layer** | **[Chapter 2.7.A: Network Layer — Code Walkthrough →](01-Network-Operation-Code-Walkthrough.md)** 📚

</div>

---

