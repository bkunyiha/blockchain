<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. **Chapter 4: Desktop Admin Interface** ← *You are here*
17. <a href="../bitcoin-wallet-ui/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../bitcoin-wallet-ui/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

<div align="right">

**[← Back to Main Book](../README.md#table-of-contents)**

</div>

---

# Chapter 4: Desktop Admin Interface — Architecture and Code Walkthrough

This chapter explains the `bitcoin-desktop-ui` crate as a Rust developer reads it: **how an Iced MVU app boots**, **how messages flow through the update loop**, and **how the UI triggers async HTTP calls** to our Rust Bitcoin implementation’s admin API.

The goal is that you can read this chapter without having the project open:

- **complete method coverage** lives in the walkthrough chapter below
- each section uses a consistent **Methods involved** box
- diagrams make the event loop and async boundary explicit

<div align="center">

**📚 [← Chapter 3: Web API Architecture](../bitcoin-blockchain/web/README.md)** | **Chapter 4: Desktop Admin Interface** | **[Next: Chapter 5 (Wallet UI) →](../bitcoin-wallet-ui/04-Wallet-UI.md)** 📚

</div>

---

## What this UI is (in one sentence)

The desktop admin UI is a **thin MVU shell**: it renders state (`view.rs`), mutates state in response to messages (`update.rs`), and performs async admin calls (`api.rs`) on a Tokio runtime (`runtime.rs`).

> **Methods involved**
>
> - `main` (boot + theme + app start)
> - `init_runtime`, `spawn_on_tokio` (async boundary)
> - `AdminApp::{new, clear_related_data}` (state and hygiene)
> - `update` (message dispatcher)
> - `view` and `view_*` helpers (rendering)

---

## Diagram: MVU event loop (Iced)

```
User input (click / type)
   |
   v
Message (enum variant)  -------------------------+
   |                                            |
   v                                            |
update(&mut AdminApp, Message)                  |
   |                                            |
   |-- pure state change -> Task::none()        |
   |-- async request  -> Task::perform(...) ----+--> (later) Message::XxxLoaded(Result<...>)
   |
   v
view(&AdminApp) -> Element<Message>  (re-render)
```

---

## Diagram: async boundary (HTTP requests)

```
update.rs (sync)             runtime.rs (tokio)                api.rs (async)
------------------          ------------------                -------------
Task::perform(              spawn_on_tokio(fut)  ----await-->  AdminClient::new(cfg)
  spawn_on_tokio(fut),                                      -> client.some_admin_endpoint().await
  Message::XxxLoaded
)
```

---

Together, these chapters walk through implementaion in:

- `bitcoin-desktop-ui/src/main.rs`
- `bitcoin-desktop-ui/src/runtime.rs`
- `bitcoin-desktop-ui/src/types.rs`
- `bitcoin-desktop-ui/src/app.rs`
- `bitcoin-desktop-ui/src/api.rs`
- `bitcoin-desktop-ui/src/update.rs`
- `bitcoin-desktop-ui/src/view.rs`

---

<div align="center">

**📚 [← Chapter 3: Web API Architecture](../bitcoin-blockchain/web/README.md)** | **Chapter 4: Desktop Admin Interface** | **[Chapter 4.A: Boot + runtime + types + state + API client →](03A-Desktop-Admin-UI-Code-Walkthrough.md)** 📚

</div>

---

