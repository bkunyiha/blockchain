<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. **Chapter 22: Node Orchestration** ← *You are here*
23. <a href="../wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="../web/README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---

# Chapter 22: Node Orchestration — Coordinating the Runtime

**Part I: Foundations & Core Implementation** | **Chapter 22: Node Orchestration**

This chapter explains the `bitcoin/src/node` module as a Rust implementer reads it: **a coordination layer** that turns “network messages and API calls” into “chainstate updates, mempool updates, mining, and peer relay”.

> **Prerequisites:**: This chapter ties together every module built so far — primitives (Chapter 7), crypto (Chapter 9), chain state (Chapters 10–18), storage (Chapter 20), and networking (Chapter 21). You should be comfortable with Tokio's `spawn`, `mpsc` channels, and `Arc<Mutex<T>>` patterns. If any of those are unfamiliar, the async primer in Chapter 21's opening or Chapter 33 (Rust Language Guide) covers them.

> **What you will learn in this chapter:**
> - Coordinate blockchain state, mempool, network, mining, and validation through the NodeContext API
> - Understand how the node module provides a unified interface over multiple subsystems
> - Trace how an incoming request flows through the node context to the appropriate handler
> - Explain why centralized orchestration simplifies the interaction between blockchain components

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

> **Important:** NodeContext is the single entry point for all blockchain operations. Every user-facing interface — the web API, the desktop admin UIs, and the wallet applications — communicates with the blockchain exclusively through NodeContext. Understanding this module is essential for understanding how the system's components fit together.

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

## Exercises

1. **Request Flow Tracing** — Pick any REST API endpoint (e.g., submit transaction) and trace the request from the HTTP handler through NodeContext to the final blockchain state change. Identify every module boundary the request crosses.

2. **Subsystem Interaction Map** — Draw a diagram showing how NodeContext interacts with each subsystem: chain state, mempool, network, mining, and validation. For each interaction, label the function called and the data exchanged.

---

## Further Reading

- **[Tokio Documentation](https://tokio.rs/)** — The async runtime that powers node orchestration.
- **[Bitcoin Core Architecture](https://developer.bitcoin.org/devguide/p2p_network.html)** — How Bitcoin Core organizes its node operations.

---

## Summary

- We built the NodeContext that serves as the central coordination point for all blockchain node operations.
- We unified interactions between blockchain state, transaction mempool, network operations, mining, and validation behind a single API.
- We saw how the node module abstracts the complexity of coordinating multiple subsystems into clean, testable interfaces.

In the next chapter, we build the wallet system — key generation, address derivation, and transaction signing — that gives users the ability to hold and spend cryptocurrency.

---

<div align="center">

**[← Chapter 30: Network Layer](../net/README.md)** | **Chapter 22: Node Orchestration** | **[Chapter 13.A: Code Walkthrough →](01-Node-Orchestration-Code-Walkthrough.md)**
</div>

---

> **Checkpoint:** This is a major milestone — the node can now run as a standalone process. Start it with `cargo run -p bitcoin` and you should see log output showing the node binding to a port and waiting for peer connections. If you have two terminals, start a second instance pointed at the first and verify that they exchange version handshakes and synchronize chain state.

---
