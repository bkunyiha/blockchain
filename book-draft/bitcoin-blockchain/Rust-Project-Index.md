<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. **Chapter 5: Rust Blockchain Project** ← *You are here*
6. <a href="primitives/README.md">Chapter 6: Primitives</a>
7. <a href="util/README.md">Chapter 7: Utilities</a>
8. <a href="crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="store/README.md">Chapter 11: Storage Layer</a>
12. <a href="net/README.md">Chapter 12: Network Layer</a>
13. <a href="node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
## Rust Project Index (Chapter 5 — Walking through Rust Bitcoin implementation)

In Chapter 1, we used the Bitcoin whitepaper to build a minimal mental model and a “Bitcoin-shaped” implementation. In this chapter, we switch from concepts to code and read the Rust project as a working system. We learn how the pieces connect in practice.

Chapter 4 closed with a five-step implementation roadmap: **Bytes → Identity → Authorization → State → Consensus**. The chapters ahead follow that same arc. Chapters 6–7 (Primitives and Utilities) cover *Bytes*—the canonical encodings every node must agree on. Chapter 8 (Cryptography) covers *Identity* (hashes as IDs) and *Authorization* (signatures that prove ownership). Chapter 9 (Blockchain Core) and 10 (Block Acceptance) cover *State* and *Consensus*—UTXO transitions, proof-of-work validation, and chain selection. From there, Chapters 11–14 build the surrounding infrastructure: storage, networking, node orchestration, and wallet. By the time we reach Chapter 15 (Web API), the full Bitcoin pipeline is implemented and we are exposing it to the outside world.

> **What you will learn in this chapter:**
> - Navigate the Cargo workspace and understand its crate organization
> - Explain the dependency graph between crates and why it is structured this way
> - Identify which crate owns each responsibility in the system
> - Understand the build configuration and feature flags used across the project

### Before we start: clone the repository

To follow along locally, clone the repository from GitHub ([`bkunyiha/blockchain`](https://github.com/bkunyiha/blockchain)) and open it in your editor.

In the repository, the Rust implementation we will walk through lives under the `bitcoin/` crate.

### How to use this chapter

As we read the codebase, we keep two questions in mind:

- **What exact bytes are parsed, serialized, hashed, and verified here?**
- **What state is updated, and how would we roll it back on a reorg?**

We recommend reading in the same direction bytes flow through the system: primitives → crypto → PoW/validation → storage/chainstate → networking/node → wallet/client.

### Rust refresher (read first if needed)

Chapter 24 (Rust Language Guide) appears at the end of the book, but it is designed to be read *before* the implementation chapters — not after. If ownership, traits, generics, or async/await feel rusty, read it now:

- **[Chapter 24: Rust Language Guide](../rust/README.md)** — start with **Rust Installation & Setup**, then continue to **Introduction**.

### Guided tour (recommended reading order)

Each step below points to:

- the **book chapter** (documentation) that explains the subsystem
- the **code location** in the Rust project that implements it

> **Note:** The crate names in this workspace (`bitcoin-blockchain`, `bitcoin-api`, etc.) are internal identifiers. They do not correspond to crates published on crates.io. All dependencies are local path dependencies within the workspace.

1. **Business objects and encoding (block, header, transaction)**
   - Chapter section: Section 6: Primitives
   - Code: `bitcoin/src/primitives/` (block, header, transaction)

2. **Utility helpers (bytes, plumbing, small reusable building blocks)**
   - Chapter section: Chapter 7: Utilities
   - Code: `bitcoin/src/util/`

3. **Crypto primitives (hashes, keys, signatures, address/script helpers)**
   - Chapter section: Chapter 8: Cryptography
   - Code: `bitcoin/src/crypto/`

4. **Proof-of-work and block acceptance**
   - Chapter section: Section 9: Blockchain — From Transaction to Block Acceptance
   - Code: `bitcoin/src/pow.rs`, `bitcoin/src/chain/`

5. **Storage and persistence**
   - Chapter section: Section 11: Storage Layer
   - Code: `bitcoin/src/store/` (persisting chain/wallet data)

6. **Chainstate / UTXO updates**
   - Chapter section: Section 9.2: Blockchain State Management
   - Code: `bitcoin/src/chain/chainstate.rs`, `bitcoin/src/chain/utxo_set.rs`

7. **Networking and message handling**
   - Chapter section: Section 12: Network Layer
   - Code: `bitcoin/src/net/`, `bitcoin/src/net/net_processing.rs`

8. **Node orchestration**
   - Chapter section: Section 13: Node Orchestration
   - Code: `bitcoin/src/node/` (peers, server, miner, mempool integration)

9. **Wallet/client**
   - Chapter section: Section 14: Wallet System
   - Code: `bitcoin/src/wallet/` (wallet implementation + service layer)

### A small exercise (recommended)

To turn reading into understanding, pick one end-to-end flow and trace it through code:

- **Receiving and confirming a transaction**: network receives a `tx` message → parse → validate → mempool → mine/include in block → connect block → wallet updates confirmation depth.

If we narrate that flow from memory and point to the relevant modules, we are ready for the deeper chapters that follow.

---

## Further Reading

- **[Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)** — The Rust Book's guide to multi-crate projects.
- **[Cargo Reference: Features](https://doc.rust-lang.org/cargo/reference/features.html)** — How conditional compilation and feature flags work in Cargo.

---

## Summary

- We navigated the Cargo workspace structure, identifying each crate and its responsibility within the blockchain system.
- We traced the dependency graph between crates, understanding why the architecture separates primitives, crypto, chain logic, and application layers.
- We examined the build configuration, feature flags, and workspace-level settings that coordinate compilation across the project.

In the next chapter, we dive into the primitives crate — the Block, Transaction, and Blockchain structures that form the foundation everything else builds on.

---

<div align="center">

**[← Bitcoin Whitepaper In Rust](whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)** | Walking through Rust Bitcoin implementation | **[Primitives →](primitives/README.md)**

</div>