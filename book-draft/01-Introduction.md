<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. **Chapter 1: Introduction & Overview** ← *You are here*
2. <a href="bitcoin-blockchain/README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="bitcoin-blockchain/Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="bitcoin-blockchain/primitives/README.md">Chapter 6: Primitives</a>
7. <a href="bitcoin-blockchain/util/README.md">Chapter 7: Utilities</a>
8. <a href="bitcoin-blockchain/crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="bitcoin-blockchain/chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="bitcoin-blockchain/store/README.md">Chapter 11: Storage Layer</a>
12. <a href="bitcoin-blockchain/net/README.md">Chapter 12: Network Layer</a>
13. <a href="bitcoin-blockchain/node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="bitcoin-blockchain/wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="bitcoin-blockchain/web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---

<div align="center">

# Rust Blockchain: A Full-Stack Implementation Guide

**[← Back to Main Book](README.md)** | **Chapter 1: Introduction & Overview** | **[Next: Introduction to Blockchain →](bitcoin-blockchain/README.md)**

</div>

---

## Chapter 1: Introduction & Overview

This book is a complete, self-contained guide to building a full-stack blockchain system in Rust, following the architecture defined by Bitcoin. By the time you finish, you will have walked through every module, every function, and every design decision in a working implementation — from raw byte serialization and cryptographic primitives all the way up to desktop wallet UIs and containerized deployment. You will not need to clone the repository or read external documentation to understand the system; the book *is* the documentation.

The target audience is **intermediate to advanced Rust developers** who want to understand how blockchain works at the implementation level. We assume you are comfortable with ownership, traits, generics, and async/await. If you need a refresher on any Rust concept, **Chapter 24: Rust Language Guide** at the end of the book serves as a standalone reference — read it before diving into the implementation chapters, or refer to it whenever you encounter unfamiliar syntax.

> **Note:** This project runs entirely on **Tokio**, the async runtime that powers most production Rust systems. You will use `async`/`await` throughout: spawning mining tasks, managing peer-to-peer TCP connections, orchestrating node subsystems with channels and `select!`, serving REST endpoints with Axum, and handling IPC commands in Tauri. If you want to learn async Rust through real code rather than toy examples, this book provides that from Chapter 9 onward.

> **What you will learn in this chapter:**
> - Understand the project architecture and how every crate in the workspace fits together
> - Identify the role of each system layer, from primitives through networking to user interfaces
> - Trace the data flow from a user action through the REST API, node context, and blockchain state
> - Set up the development environment and run the project for the first time

### How the book is organized

The book follows the same path data takes through a Bitcoin node:

- **Part I (Chapters 1–21)** builds the system from the ground up. We start with the Bitcoin whitepaper (Chapters 2–4) to understand *what* we are building and *why*. Then we implement it in Rust: primitives and byte encodings (6–7), cryptographic identity and authorization (8), blockchain state and consensus (9–10), persistent storage (11), networking (12), node orchestration (13), wallet (14), a REST API (Chapter 15), and multiple frontend interfaces — desktop UIs in Iced and Tauri (Chapters 16–19), an embedded encrypted database (Chapter 20), and a React web admin panel (Chapter 21).
- **Part II (Chapters 22–23)** covers deployment with Docker Compose and Kubernetes.
- **Part III (Chapter 24)** is the Rust language reference.

Each implementation chapter opens by explaining what problem the module solves, shows the complete annotated code, and closes by connecting forward to the next layer. The reading order is designed so that every concept is introduced before it is used.

### System Architecture

The diagram below shows how every module in the system connects. Data flows upward from raw bytes to user-facing interfaces. Keep this picture in mind as you read — each chapter builds one layer of this stack.

```text
┌──────────────────────────────────────────────────────┐
│                   DEPLOYMENT (Part II)               │
│  ┌────────────────────┐  ┌─────────────────────────┐ │
│  │ Ch 22: Docker      │  │ Ch 23: Kubernetes       │ │
│  │ Compose            │  │ Deployment              │ │
│  └────────────────────┘  └─────────────────────────┘ │
├──────────────────────────────────────────────────────┤
│                  USER INTERFACES                     │
│    ┌──────────┐ ┌──────────┐ ┌──────────┐            │
│    │ Ch 16    │ │ Ch 17    │ │ Ch 21    │            │
│    │ Iced     │ │ Tauri    │ │ React    │            │
│    │ Desktop  │ │ Desktop  │ │ Web UI   │            │
│    └─────┬────┘ └────┬─────┘ └────┬─────┘            │
│          └─────┬─────┘            │                  │
│                ▼                  ▼                  │
│    ┌────────────────────────────────────────────┐    │
│    │    Ch 15: Web API (Axum REST)              │    │
│    └──────────────────┬─────────────────────────┘    │
│                       │                              │
│    ┌──────────────────▼─────────────────────────┐    │
│    │    Ch 13: Node Orchestration               │    │
│    │    (message dispatch, coordination)        │    │
│    └──┬──────────┬──────────┬──────────────┬────┘    │
│       │          │          │              │         │
│       ▼          ▼          ▼              ▼         │
│    ┌───────┐ ┌────────┐ ┌────────┐ ┌───────────┐     │
│    │ Ch 14 │ │ Ch 12  │ │ Ch 11  │ │ Ch 9–10   │     │
│    │Wallet │ │Network │ │Storage │ │ Chain +   │     │
│    │       │ │ (TCP)  │ │ (sled) │ │ Consensus │     │
│    └──┬────┘ └────────┘ └────────┘ └─────┬─────┘     │
│       │                                  │           │
│       └────────────────┬─────────────────┘           │
│                        ▼                             │
│    ┌────────────────────────────────────────────┐    │
│    │    Ch 8: Cryptography                      │    │
│    │    (SHA-256, ECDSA, key derivation)        │    │
│    └──────────────────┬─────────────────────────┘    │
│                       ▼                              │
│    ┌────────────────────────────────────────────┐    │
│    │    Ch 6–7: Primitives + Utilities          │    │
│    │    (Block, Transaction, UTXO, helpers)     │    │
│    └────────────────────────────────────────────┘    │
├──────────────────────────────────────────────────────┤
│  Ch 20: Embedded DB (SQLCipher) — used by wallet UIs │
├──────────────────────────────────────────────────────┤
│  Ch 1–4: Concepts   │  Ch 24: Rust Language Guide    │
│  (Whitepaper,       │  (Reference — read anytime)    │
│   Bitcoin Intro)    │                                │
└──────────────────────────────────────────────────────┘
```

### What This Book Does Not Cover

This is an educational implementation, not a production Bitcoin client. To keep the book focused and the codebase readable, we deliberately exclude:

- **Production mining difficulty** — our proof-of-work uses a simplified difficulty target. Real Bitcoin's difficulty adjustment algorithm (recalculated every 2,016 blocks) is not implemented.
- **NAT traversal and peer discovery** — nodes connect via configured addresses. The DNS seeding, UPnP, and NAT hole-punching that production Bitcoin uses for peer discovery are outside our scope.
- **BIP-32 HD wallets** — our wallet generates standalone key pairs. Hierarchical Deterministic key derivation (BIP-32/39/44) is covered in the Further Reading section of the Wallet chapter.
- **Lightning Network** — layer-2 payment channels are an entirely separate protocol built on top of Bitcoin. We focus exclusively on the layer-1 blockchain.
- **Smart contracts and scripting** — Bitcoin's Script language for transaction conditions is not implemented. Our transactions use a simplified signature-based authorization model.
- **Multi-signature wallets** — all transactions require a single signature. Multi-sig (P2SH, P2WSH) is a natural extension but adds complexity beyond our teaching goals.

Each exclusion is a deliberate scope decision. The Further Reading sections at the end of relevant chapters point you to specifications and crates that cover these topics.

### Project Structure

```text
blockchain/
├── bitcoin/                    # Core blockchain implementation
│   ├── src/
│   │   ├── primitives/        # Transaction, Block, UTXO structures
│   │   ├── node/              # Node context and networking
│   │   ├── store/             # Blockchain storage (file system, database)
│   │   └── web/               # REST API server (Axum)
│   └── ...
├── bitcoin-desktop-ui-iced/        # Desktop admin interface (Iced)
├── bitcoin-wallet-ui-iced/          # Wallet user interface (Iced)
├── bitcoin-web-ui/             # Web admin interface (React/TypeScript)
├── ci/
│   ├── docker-compose/        # Docker Compose deployment
│   └── kubernetes/            # Kubernetes deployment
└── book-draft/                 # This documentation
```

> **Tip:** If you want to see the system running before reading further, jump to Chapter 0: Quick Start. You can return here afterward for the full architectural picture.

### Technical Stack

#### Backend (Rust)
- **Tokio**: Async runtime for non-blocking I/O
- **SQLCipher**: Encrypted SQLite database
- **Rusqlite**: Rust bindings for SQLite
- **Serde**: Serialization framework
- **Axum**: Web framework for REST APIs
- **Reqwest**: HTTP client

#### Desktop UIs (Rust)
- **Iced**: Cross-platform GUI framework
- **Model-View-Update (MVU)**: Architecture pattern
- **Desktop Admin Interface**: Admin UI for blockchain management (Iced)
- **Wallet User Interface**: User-facing wallet application (Iced)

#### Web UI (TypeScript/React)
- **React 18**: UI framework
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool and dev server
- **React Query**: Server state management
- **React Router**: Client-side routing
- **Tailwind CSS**: Utility-first CSS framework
- **Axios**: HTTP client

---

## What to read next

If you are reading the book front-to-back, continue to **[Chapter 2: Introduction to Blockchain](bitcoin-blockchain/README.md)** — it builds the conceptual vocabulary (transactions, blocks, consensus, UTXO) that every implementation chapter depends on.

If you want to brush up on Rust first, jump to **[Chapter 24: Rust Language Guide](rust/README.md)** and return here when you are ready. The **[Tokio Runtime Guide](bitcoin-blockchain/Tokio.md)** is also useful preparation for the async code that appears from Chapter 12 onward.

---

## Further Reading

- **[The Rust Programming Language](https://doc.rust-lang.org/book/)** — The official Rust book, covering everything from installation to advanced features.
- **[Cargo Reference](https://doc.rust-lang.org/cargo/)** — Comprehensive guide to Rust's package manager and build system.
- **[Bitcoin Developer Guide](https://developer.bitcoin.org/devguide/)** — Bitcoin.org's technical documentation for developers.

---

## Summary

- We surveyed the full project architecture: a Cargo workspace of specialized crates spanning primitives, cryptography, chain logic, networking, storage, wallet, web API, and multiple UI frontends.
- We identified the four system layers — core blockchain, API/services, desktop UIs, and deployment — and how data flows between them.
- We outlined the technical stack: Rust for the backend, Iced and Tauri for desktop UIs, React/TypeScript for the web UI, and Docker Compose and Kubernetes for deployment.
- We established the learning paths that guide different readers — first-timers, experienced developers, and operations teams — through the material.

In the next chapter, we explore the fundamental concepts of Bitcoin and blockchain technology — what they are, why they matter, and the cryptographic building blocks that make them work.

---

<div align="center">

**[← Back to Main Book](README.md)** | **Introduction & Overview** | **[Next: Introduction to Blockchain →](bitcoin-blockchain/README.md)**

</div>

