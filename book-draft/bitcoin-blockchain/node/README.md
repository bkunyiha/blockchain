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
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. **Chapter 13: Node Orchestration** ← *You are here*
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

# Node Orchestration — Coordinating the Runtime

**Part I: Foundations & Core Implementation** | **Chapter 13: Node Orchestration**

This chapter explains the `bitcoin/src/node` module as a Rust implementer reads it: **a coordination layer** that turns “network messages and API calls” into “chainstate updates, mempool updates, mining, and peer relay”.

> **Prerequisites**: This chapter ties together every module built so far — primitives (6), crypto (8), chain state (9–10), storage (11), and networking (12). You should be comfortable with Tokio's `spawn`, `mpsc` channels, and `Arc<Mutex<T>>` patterns. If any of those are unfamiliar, the async primer in Chapter 12's opening or Chapter 24 (Rust Language Guide) covers them.

**What you will learn in this chapter:** How a single `NodeContext` struct coordinates message dispatch across the mempool, chainstate, mining, and peer relay subsystems — and why this coordination layer exists as a separate module rather than being spread across the network and chain code.

---

## What “node orchestration” means in our Rust Bitcoin implementation

At runtime, several subsystems must work together:

- **Network** receives bytes and produces typed messages (`Package`)
- **NodeContext** decides what subsystem should handle the message (mempool, chainstate, mining)
- **Chainstate** persists blocks and updates the canonical tip + UTXO
- **Mempool** stores unconfirmed transactions and marks outputs as “in mempool”
- **Mining** turns mempool contents into a block and relays the new tip
- **Peers** is the node’s peer set used for relay

```text
TCP -> Package -> NodeContext -> (mempool | add_block | mining | relay)
```

The key entry points are `NodeContext::process_transaction` (mempool admission), `NodeContext::add_block` (chain extension from a peer), and `NodeContext::mine_empty_block` (local mining trigger). These delegate to `txmempool::{add_to_memory_pool, remove_from_memory_pool}` for mempool state and `miner::{should_trigger_mining, process_mine_block, broadcast_new_block}` for the mining pipeline.

---

## Where the full code walkthrough lives

The code-complete walkthrough for `bitcoin/src/node` is in:

- **[Chapter 13.A: Node Orchestration — Code Walkthrough](01-Node-Orchestration-Code-Walkthrough.md)**

It includes full method bodies for:

- `NodeContext` (core coordinator)
- mempool helpers (`txmempool.rs`)
- mining pipeline (`miner.rs`)
- peer set (`peers.rs`)

---

<div align="center">

**[← Chapter 12: Network Layer](../net/README.md)** | **Chapter 13: Node Orchestration** | **[Chapter 13.A: Code Walkthrough →](01-Node-Orchestration-Code-Walkthrough.md)** 
</div>

---

> **Checkpoint:** This is a major milestone — the node can now run as a standalone process. Start it with `cargo run -p bitcoin` and you should see log output showing the node binding to a port and waiting for peer connections. If you have two terminals, start a second instance pointed at the first and verify that they exchange version handshakes and synchronize chain state.

---
