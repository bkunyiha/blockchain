<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Quick Start

0. <a href="00-Quick-Start.md">Chapter 0: Quick Start — See It Run</a>

### Part I: Foundations & Core Implementation

1. <a href="01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="bitcoin-blockchain/README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
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

### Appendices

39. <a href="Glossary.md">Glossary</a>
40. <a href="Appendix-Source-Reference.md">Appendix: Source Reference</a>

</details>

</div>

<div align="right">

**[← Back to Main Repository](../README.md)**

</div>

---

<div align="center">

# Building a Full-Stack Bitcoin Blockchain With Rust

## Complete Book Documentation

**Main Book Index**

[![Book](https://img.shields.io/badge/Book-24%20Chapters-blue)](README.md)
[![Part I](https://img.shields.io/badge/Part%20I-Core%20Implementation-green)](#chapter-1-introduction-overview)
[![Part II](https://img.shields.io/badge/Part%20II-Deployment-orange)](#chapter-22-docker-compose-deployment)
[![Part III](https://img.shields.io/badge/Part%20III-Language%20Reference-blue)](#chapter-24-rust-language-guide)

</div>

---

<a id="table-of-contents"></a>
## Quick Navigation - All Chapters

<div align="center">

### **Part I: Foundations & Core Implementation**

| # | Chapter | Description | Link |
|---|---------|-------------|------|
| 1 | **Introduction & Overview** | Book introduction, project structure, technical stack | **[Read Chapter 1 →](01-Introduction.md)** |
| 2 | **Introduction to Bitcoin & Blockchain** | Bitcoin and blockchain fundamentals, use cases, and technical foundations | **[Read Chapter 2 →](bitcoin-blockchain/README.md)** |
| 3 | **Bitcoin Whitepaper** | Whitepaper that defined Bitcoin | **[Read Chapter 3 →](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md)** |
| 4 | **Bitcoin Whitepaper In Rust** | Whitepaper → Rust encoding walkthrough: concrete data structures, hashing/serialization rules, and how the sections map into this repo. | **Index:** **[Rust Encoding Summary →](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)**<br>**Core building blocks:** **[Business Objects](bitcoin-blockchain/whitepaper-rust/00-business-objects.md)**<br>**Sections 1–4:** **[§1 Intro](bitcoin-blockchain/whitepaper-rust/01-Introduction-Bitcoin-Whitepaper-Section-1.md)** · **[§2 Transactions](bitcoin-blockchain/whitepaper-rust/02-Transactions-chain-of-signatures-Bitcoin-Whitepaper-Section-2.md)** · **[§3 Timestamp server](bitcoin-blockchain/whitepaper-rust/03-Timestamp-server-block-header-chaining-Bitcoin-Whitepaper-Section-3.md)** · **[§4 Proof-of-work](bitcoin-blockchain/whitepaper-rust/04-Proof-of-work-Bitcoin-Whitepaper-Section-4.md)** · **[nBits / target](bitcoin-blockchain/whitepaper-rust/04A-nBits-Target-Expansion.md)**<br>**Sections 5–9:** **[§5 Network](bitcoin-blockchain/whitepaper-rust/05-Network-operation-Bitcoin-Whitepaper-Section-5.md)** · **[§6 Incentives](bitcoin-blockchain/whitepaper-rust/06-Incentive-mechanism-Bitcoin-Whitepaper-Section-6.md)** · **[§7 Pruning](bitcoin-blockchain/whitepaper-rust/07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)** · **[§8 Merkle/SPV](bitcoin-blockchain/whitepaper-rust/08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)** · **[§9 Value](bitcoin-blockchain/whitepaper-rust/09-Combining-splitting-value-Bitcoin-Whitepaper-Section-9.md)**<br>**Sections 10–12:** **[§10 Privacy](bitcoin-blockchain/whitepaper-rust/10-Privacy-Bitcoin-Whitepaper-Section-10.md)** · **[§11 Confirmations](bitcoin-blockchain/whitepaper-rust/11-Confirmations-and-attacker-probability-Bitcoin-Whitepaper-Section-11.md)** · **[§12 Conclusion](bitcoin-blockchain/whitepaper-rust/12-Conclusion-Bitcoin-Whitepaper-Section-12.md)**<br>**Appendices:** **[A: End-to-end object connectivity](bitcoin-blockchain/whitepaper-rust/Appendix-A-Object-connectivity-end-to-end-flow.md)** · **[B: Mapping to this repository](bitcoin-blockchain/whitepaper-rust/Appendix-B-Mapping-to-this-repository.md)** |
| 5 | **Rust Blockchain Project** | Rust blockchain project structure, crate organization, and build configuration | **[Read Chapter 5 →](bitcoin-blockchain/Rust-Project-Index.md)** |
| 6 | **Primitives** | Core data structures (Block, Transaction, Blockchain) | **[Read Chapter 6 →](bitcoin-blockchain/primitives/README.md)** |
| 7 | **Utilities** | Utility functions and helpers | **[Read Chapter 7 →](bitcoin-blockchain/util/README.md)** |
| 8 | **Cryptography** | Cryptographic primitives, hash functions, signatures, and key pairs | **[Read Chapter 8 →](bitcoin-blockchain/crypto/README.md)** |
| 9 | **Chain (Code Walkthrough)** | End-to-end chain pipeline: model → state → storage → UTXO → mining → consensus → orchestration. | **Index:** **[9 →](bitcoin-blockchain/chain/README.md)**<br>**9.x:** **[9.1 Model](bitcoin-blockchain/chain/01-Domain-Model.md)** · **[9.2 State](bitcoin-blockchain/chain/02-Blockchain-State-Management.md)** · **[9.3 Storage](bitcoin-blockchain/chain/03-Chain-State-and-Storage.md)** · **[9.4 UTXO](bitcoin-blockchain/chain/04-UTXO-Set.md)**<br>**Flow:** **[9.5 Tx](bitcoin-blockchain/chain/05-Transaction-Lifecycle.md)** · **[9.6 Mine](bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md)** · **[9.7 Rules](bitcoin-blockchain/chain/07-Consensus-and-Validation.md)** · **[9.8 Node](bitcoin-blockchain/chain/08-Node-Orchestration.md)** · **[9.9 E2E](bitcoin-blockchain/chain/09-Transaction-To-Block.md)** |
| 10 | **Block Acceptance (WP §5/Step 5)** | “Valid + not already spent” (tx verify + UTXO + rules). | **Read:** **[10 →](bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md)**<br>**See also:** **[UTXO](bitcoin-blockchain/chain/04-UTXO-Set.md)** · **[Rules](bitcoin-blockchain/chain/07-Consensus-and-Validation.md)** · **[E2E](bitcoin-blockchain/chain/09-Transaction-To-Block.md)** |
| 11 | **Storage Layer** | Persistent storage implementation | **[Read Chapter 11 →](bitcoin-blockchain/store/README.md)** |
| 12 | **Network Layer** | Peer-to-peer networking and protocol | **[Read Chapter 12 →](bitcoin-blockchain/net/README.md)** |
| 13 | **Node Orchestration** | Node context and coordination | **[Read Chapter 13 →](bitcoin-blockchain/node/README.md)** |
| 14 | **Wallet System** | Wallet implementation and key management | **[Read Chapter 14 →](bitcoin-blockchain/wallet/README.md)** |
| 15 | **Web API Architecture** | REST API implementation, handlers, middleware, and security | **[Read Chapter 15 →](bitcoin-blockchain/web/README.md)** |
| 16 | **Desktop Admin Interface (Iced)** | Iced desktop admin UI — MVU architecture and implementation | **[Read Chapter 16 →](bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**<br>**Companions:** [16A Code Walkthrough](bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md) · [16B Update Loop](bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md) · [16C View Layer](bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md) |
| 17 | **Desktop Admin Interface (Tauri)** | Tauri desktop admin UI — Rust backend + React frontend | **[Read Chapter 17 →](bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md)**<br>**Companions:** [17A Rust Backend](bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md) · [17B Frontend Infrastructure](bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md) · [17C Frontend Pages](bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md) |
| 18 | **Wallet User Interface (Iced)** | Iced wallet UI — MVU architecture with encrypted storage | **[Read Chapter 18 →](bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md)**<br>**Companion:** [18A Code Listings](bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md) |
| 19 | **Wallet User Interface (Tauri)** | Tauri wallet UI — Rust backend + React frontend with SQLCipher | **[Read Chapter 19 →](bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md)**<br>**Companions:** [19A Rust Backend](bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md) · [19B Frontend Infrastructure](bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md) · [19C Frontend Pages](bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md) |
| 20 | **Embedded Database & Persistence** | SQLCipher integration — framework-agnostic, covers both Iced and Tauri | **[Read Chapter 20 →](embedded-database/06-Embedded-Database.md)**<br>**Companion:** [20A Code Listings](embedded-database/06A-Embedded-Database-Code-Listings.md) |
| 21 | **Web Admin Interface** | React/TypeScript web UI architecture and patterns | **[Read Chapter 21 →](bitcoin-web-ui/06-Web-Admin-UI.md)** |

### **Part II: Deployment & Operations**

| # | Chapter | Description | Link |
|---|---------|-------------|------|
| 22 | **Docker Compose Deployment** | Complete Docker Compose guide with scaling and examples | **[Read Chapter 22 →](ci/docker-compose/01-Introduction.md)** |
| 23 | **Kubernetes Deployment** | Production Kubernetes deployment and autoscaling | **[Read Chapter 23 →](ci/kubernetes/README.md)** |

### **Part III: Language Reference**

| # | Chapter | Description | Link |
|---|---------|-------------|------|
| 24 | **Rust Language Guide** | Comprehensive Rust language features and examples | **[Read Chapter 24 →](rust/README.md)** |

</div>

---

## About This Book

Welcome to *Building a Bitcoin Blockchain from Scratch*. In this book, we will embark on a comprehensive journey through building a complete Bitcoin blockchain implementation using Rust. Together, we'll explore every aspect of the system—from the fundamental concepts of blockchain technology to the intricate details of user interfaces and deployment strategies.

Our journey is structured to guide you through both understanding the theory and implementing the practice. Whether you're reading sequentially to build a complete understanding, or jumping to specific topics that interest you, each chapter builds upon foundational concepts while standing alone as a valuable resource.

### Book Structure

We've organized this book into three main parts that reflect the natural progression from implementation to deployment to language reference:

**Part I: Foundations & Core Implementation** (Chapters 1-21)

In the first part, we dive deep into the heart of blockchain technology. We'll explore the fundamental concepts, data structures, and implementation details that make a blockchain work. You'll learn about transaction systems, cryptographic primitives, and how to build user interfaces that interact with the blockchain — with both pure Rust (Iced) and Rust + React (Tauri) desktop implementations side by side. Each chapter provides deep technical insights into blockchain architecture, showing you not just what to build, but why we build it this way.

**Part II: Deployment & Operations** (Chapters 22-23)

Once we've built our blockchain, we need to deploy it. In the second part, we'll explore deployment strategies and operational procedures. We'll start with Docker Compose for development and local deployments, then move to Kubernetes for production-grade orchestration. You'll learn practical guidance for running and scaling the system in real-world scenarios.

**Part III: Language Reference** (Chapter 24)

In the final part, we provide a comprehensive reference guide to the Rust programming language features used throughout the blockchain implementation. This guide explains Rust concepts with examples from our codebase, helping you understand not just what Rust features exist, but how they work together to build reliable, performant systems software.

### How to Read This Book

This book is designed to accommodate different reading styles:

- **Sequential Reading:** If you're new to blockchain development or want a complete understanding, follow the chapters in order. Each chapter builds naturally on the previous ones.

- **Topic-Based Learning:** If you have specific interests—perhaps you're focused on UI development or deployment—use the learning paths above to focus on those areas while still understanding the broader context.

- **Reference Reading:** If you're already familiar with blockchain concepts and need specific information, feel free to jump to the chapters that interest you. Each chapter is written to be self-contained while referencing related concepts.

- **Learning by Example:** Throughout the book, you'll find practical code examples and explanations. We encourage you to read the code alongside the explanations, as understanding the implementation details is crucial to mastering blockchain development.

### Who This Book Is For

This book is written for technical professionals who want to understand blockchain implementation at a deep level:

- **Software Engineers** building blockchain applications or learning Rust through practical projects
- **DevOps Engineers** who need to deploy and operate blockchain networks in production environments
- **Students & Researchers** studying blockchain implementation details and seeking hands-on experience
- **Technical Professionals** who want to understand how Bitcoin-like blockchain systems work under the hood

---

## Getting Started

### For First-Time Readers

#### If we need a Rust refresher

If we are not very familiar with Rust (or we have not used it recently), we should first work through the Rust guide:

- **[Chapter 24: Rust Language Guide](rust/README.md)** — start with **[Rust Installation & Setup](rust/00-Rust-Installation-Setup.md)**, then continue to **[Introduction](rust/01-Introduction.md)**.

If you're new to blockchain development or this project, we recommend beginning your journey here:

1. **Start with [Chapter 1: Introduction & Overview](01-Introduction.md)** - We'll introduce you to the project structure, technical stack, and overall architecture. This foundation will help you understand how all the pieces fit together.

2. **Continue with [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md)** - Here, we'll explore what Bitcoin and blockchain are, their origins, advantages, and real-world use cases. This provides the foundational knowledge for understanding blockchain technology.

3. **Bitcoin Whitepaper [Chapter 3 → Bitcoin Whitepaper](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md)**

4. **Bitcoin Whitepaper In Rust  [Chapter 4 → Implemeting the Bitcoin Whitepaper in Rust](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)**

5. **Learn [Chapter 8: Cryptography](bitcoin-blockchain/crypto/README.md)** - We'll dive into the cryptographic primitives that secure blockchain: hash functions, digital signatures, key pairs, and address encoding.

6. **Explore [Chapter 9: Blockchain (Technical Foundations)](bitcoin-blockchain/chain/README.md)** - Here, we'll explore the fundamental blockchain concepts and transaction handling. This is where the magic of blockchain begins.

7. **Understand [Chapter 15: Web API Architecture](bitcoin-blockchain/web/README.md)** - We'll understand how the REST API is built, how requests flow through the system, and how we've created a secure, scalable web layer.

8. **Choose your path** based on what interests you most:
   - **Desktop UI (Pure Rust):** Continue with [Chapter 16: Desktop Admin (Iced)](bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md) for the MVU pattern, or [Chapter 18: Wallet (Iced)](bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md) for the wallet application.
   - **Desktop UI (Rust + React):** Continue with [Chapter 17: Desktop Admin (Tauri)](bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md) for the IPC-based approach, or [Chapter 19: Wallet (Tauri)](bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md) for the Tauri wallet.
   - **Web UI:** Jump to [Chapter 21: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) for the React/TypeScript web application.
   - **Deployment:** Jump ahead to [Chapter 22: Docker Compose](ci/docker-compose/01-Introduction.md) where we'll walk through deployment step by step.

### For Experienced Developers

If you're already familiar with blockchain concepts and Rust, you can dive deeper into specific areas:

- **Blockchain Implementation:** Start with [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md) for fundamentals, then [Chapter 8: Cryptography](bitcoin-blockchain/crypto/README.md) and [Transaction ID Format (Chapter 6)](bitcoin-blockchain/primitives/02-Transaction-ID-Format.md) to understand our implementation choices and design decisions.
- **UI Architecture:** Explore [Chapter 16: Desktop Admin (Iced)](bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md) for the pure Rust MVU approach, [Chapter 17: Desktop Admin (Tauri)](bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md) for the Rust + React IPC approach, or [Chapter 21: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) for the web UI.
- **Deployment:** Jump directly to [Chapter 22: Docker Compose](ci/docker-compose/01-Introduction.md) or [Chapter 23: Kubernetes](ci/kubernetes/README.md) to understand our deployment strategies.

### For Operations Teams

If your focus is on deploying and operating the system:

- **Quick Deployment:** Begin with [Chapter 22: Docker Compose Deployment](ci/docker-compose/01-Introduction.md), starting with Section 1: Introduction & Quick Start. We'll get you up and running quickly.
- **Production Setup:** For production deployments, [Chapter 23: Kubernetes Deployment](ci/kubernetes/README.md) provides a complete guide to production-grade orchestration and scaling.

---

## Cleanup (Stopping and Undeploying)

When you’re done experimenting, you can tear down resources cleanly. This is especially important on laptops, where local clusters can consume CPU/RAM/disk in the background.

### Docker Compose Cleanup

From `ci/docker-compose/configs/`:

```bash
docker compose down
```

### Kubernetes Cleanup (Minikube)

If you deployed the Kubernetes manifests (Chapter 23), you have two levels of cleanup:

**Option A: Undeploy just the blockchain resources (keep Minikube running):**

```bash
cd ci/kubernetes/manifests
./undeploy.sh
```

**Option B: Remove the entire namespace:**

```bash
kubectl delete namespace blockchain
```

**Stop Minikube (keeps the cluster on disk):**

```bash
minikube stop
```

**Delete Minikube completely (removes the cluster and its disk state):**

```bash
minikube delete
```

If you previously ran `eval $(minikube docker-env)`, you may also want to restore your shell to your normal Docker daemon:

```bash
eval $(minikube docker-env -u)
```

## Chapter 1: Introduction & Overview

**[→ Read Full Chapter 1: Introduction & Overview](01-Introduction.md)**

In Chapter 1, we introduce the book and project. You'll learn about the project structure, technical stack, and learning paths designed for different experience levels. This chapter provides the foundation for understanding how all the pieces of our blockchain implementation fit together.

---

## Chapter 2: Introduction to Bitcoin & Blockchain

**[→ Read Full Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md)**

In Chapter 2, we explore the fundamental concepts of Bitcoin and blockchain technology. We'll learn what Bitcoin and blockchain are, understand their origins and evolution, explore the advantages of decentralized systems across various domains (finance, voting, governance, social media, supply chain, and more), dive into technical foundations like cryptographic hash functions, digital signatures, and consensus mechanisms, and examine real-world use cases and applications. This chapter provides the essential knowledge needed to understand blockchain technology before diving into implementation details.

**[→ Read Full Chapter 3: Bitcoin Whitepaper](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md)**

In Chapter 3, we go through the bitcoin whitepaper.

**[→ Read Full Chapter 4: Bitcoin Whitepaper Rust Implementation](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)**

In Chapter 4, we go through the bitcoin whitepaper and implementation in Rust.


## Chapter 6: Primitives

**[→ Read Full Chapter 6: Primitives](bitcoin-blockchain/primitives/README.md)**

In Chapter 6, we examine the core data structures that form the foundation of the blockchain system. We explore the pure data structures for blocks, transactions, and the blockchain itself—the atomic building blocks from which all blockchain operations are constructed. This chapter covers the fundamental types used throughout the entire system, including the Transaction ID format and why we store transaction IDs as `Vec<u8>` rather than strings.

## Chapter 7: Utilities

**[→ Read Full Chapter 7: Utilities](bitcoin-blockchain/util/README.md)**

In Chapter 7, we explore the utility functions and helpers used throughout the blockchain system. We cover common functionality for timestamp generation, functional programming operations, and other cross-cutting concerns that don't belong to any specific domain but are needed across multiple modules.

## Chapter 8: Cryptography

**[→ Read Full Chapter 8: Cryptography](bitcoin-blockchain/crypto/README.md)**

In Chapter 8, we explore the cryptographic primitives that secure our blockchain. We understand how hash functions create transaction IDs and block hashes, how digital signatures authorize transactions, how key pairs are generated and managed, and how addresses are encoded. This chapter provides detailed technical documentation on the cryptographic libraries used, their implementation, and their role in blockchain security.

## Chapter 9: Chain (Code Walkthrough)

**[→ Read Full Chapter 9: Chain](bitcoin-blockchain/chain/README.md)**

In Chapter 9, we explore how blockchain state is managed and maintained. We understand how the UTXO (Unspent Transaction Output) set is tracked, how blockchain state operations are coordinated, and how the chain module provides services for querying and managing blockchain state. This chapter covers the end-to-end chain pipeline: domain model, state management, storage, UTXO, mining, consensus, node orchestration, and the transaction-to-block flow.

## Chapter 10: Block Acceptance

**[→ Read Full Chapter 10: Block Acceptance](bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md)**

In Chapter 10, we examine the whitepaper's Step 5 gate: "Nodes accept the block only if all transactions in it are valid and not already spent." This capstone chapter maps the whitepaper's safety rule to concrete code boundaries—validate first, connect only if valid—and shows the exact boundary where ownership and scarcity are enforced.

## Chapter 11: Storage Layer

**[→ Read Full Chapter 11: Storage Layer](bitcoin-blockchain/store/README.md)**

In Chapter 11, we explore how blockchain data is persistently stored on disk. We understand how the `BlockchainFileSystem` uses the Sled embedded database to store blocks, chain state, and UTXO data. This chapter covers the low-level storage operations that provide efficient persistence and retrieval of blockchain data.

## Chapter 12: Network Layer

**[→ Read Full Chapter 12: Network Layer](bitcoin-blockchain/net/README.md)**

In Chapter 12, we dive into the peer-to-peer networking protocol that enables blockchain nodes to communicate, synchronize, and maintain consensus. We explore how P2P messages are processed, how peer connections are managed, and how network operations coordinate blockchain synchronization. This chapter covers the network communication layer that enables distributed blockchain operation.

## Chapter 13: Node Orchestration

**[→ Read Full Chapter 13: Node Orchestration](bitcoin-blockchain/node/README.md)**

In Chapter 13, we explore the central coordination point for all blockchain node operations. We understand how `NodeContext` orchestrates interactions between blockchain state, transaction mempool, network operations, mining, and validation. This chapter covers how the node module provides a unified API that abstracts the complexity of coordinating multiple subsystems.

## Chapter 14: Wallet System

**[→ Read Full Chapter 14: Wallet System](bitcoin-blockchain/wallet/README.md)**

In Chapter 14, we explore the wallet system that provides functionality for creating, managing, and using cryptocurrency wallets. We understand how key pairs are generated, how addresses are created, how transactions are signed, and how wallets are persisted. This chapter covers the cryptographic and key management functionality needed for users to securely store and spend cryptocurrency.

**Reference Materials:**
- [Rust Language Guide](rust/README.md) - Rust programming language features used throughout
- [Tokio Runtime Guide](bitcoin-blockchain/Tokio.md) - Async runtime framework

---

## Chapter 15: Web API Architecture

**[→ Read Full Chapter 15: Web API Architecture](bitcoin-blockchain/web/README.md)**

In Chapter 15, we explore the web API layer that powers our blockchain node. We'll understand how the REST API is structured using Axum, how requests flow through handlers and middleware, and how we've built a secure, scalable web layer. We'll dive into authentication, routing, error handling, and OpenAPI documentation, seeing how all these components work together to create a production-ready API.

**Chapter Sections (read in order):**
1. [Section 1: Introduction & Architecture Overview](bitcoin-blockchain/web/01-Introduction.md) - Architecture overview and design principles
2. [Section 2: Server Setup and Configuration](bitcoin-blockchain/web/02-Server-Setup.md) - Server initialization and configuration
3. [Section 3: Routing System](bitcoin-blockchain/web/03-Routing.md) - Endpoint organization and route definitions
4. [Section 4: Request Handlers](bitcoin-blockchain/web/04-Handlers.md) - Request processing and business logic
5. [Section 5: Middleware Layer](bitcoin-blockchain/web/05-Middleware.md) - Authentication, CORS, logging, and error handling
6. [Section 6: Data Models](bitcoin-blockchain/web/06-Data-Models.md) - Request and response structures with type safety
7. [Section 7: Error Handling](bitcoin-blockchain/web/07-Error-Handling.md) - Comprehensive error management strategies
8. [Section 8: OpenAPI Documentation](bitcoin-blockchain/web/09-OpenAPI.md) - Automatic API documentation generation
9. [Section 9: Security Architecture](bitcoin-blockchain/web/10-Security.md) - Authentication, authorization, and security best practices
10. [Section 10: Best Practices and Patterns](bitcoin-blockchain/web/11-Best-Practices.md) - Design patterns and conventions

**Reference Materials:**
- [Web API Architecture Index](bitcoin-blockchain/web/README.md) - Overview and chapter summaries
- [Axum Framework Guide](bitcoin-blockchain/web/Axum.md) - Detailed explanations of Axum features
- [Tower Framework Guide](bitcoin-blockchain/web/Tower.md) - Middleware framework and tower_http components
- [Serde Framework Guide](bitcoin-blockchain/web/Serde.md) - Serialization and deserialization framework
- [Utoipa Framework Guide](bitcoin-blockchain/web/Utoipa.md) - OpenAPI documentation generation framework
- [Tracing Framework Guide](bitcoin-blockchain/web/Tracing.md) - Structured logging and diagnostics
- [Tokio Runtime Guide](bitcoin-blockchain/Tokio.md) - Async runtime framework

**Note:** For comprehensive Rust language reference, see [Chapter 24: Rust Language Guide](rust/README.md) in Part III.

---

## Chapters 16-19: Desktop Admin & Wallet User Interfaces

Chapters 16-19 cover the desktop admin interface and wallet applications, implemented in two frameworks that offer contrasting approaches to desktop development.

### Chapter 16: Desktop Admin Interface (Iced)

**[→ Read Full Chapter 16: Desktop Admin Interface (Iced)](bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

Chapter 16 takes us into the world of pure Rust desktop development using the Iced framework. We'll explore the Model-View-Update (MVU) pattern in detail, walking through each module—main.rs, app.rs, types.rs, update.rs, view.rs, api.rs, and runtime.rs—understanding how they work together. We'll learn about API integration and async operations, UI component design and styling, state management and data flow, and advanced features like popup menus, selectable text, and formatting systems.

**Companion chapters:** [16A Code Walkthrough](bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md) · [16B Update Loop](bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md) · [16C View Layer](bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md)

### Chapter 17: Desktop Admin Interface (Tauri)

**[→ Read Full Chapter 17: Desktop Admin Interface (Tauri)](bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md)**

Chapter 17 presents the same admin functionality built with Tauri 2, combining a Rust backend with a React/TypeScript frontend connected via IPC. We'll explore how Tauri commands bridge the two languages, how the React frontend uses Zustand for state and React Query for data fetching, and how the architecture contrasts with the pure Rust Iced approach. Both implementations use the same `bitcoin-api` crate for API communication.

**Companion chapters:** [17A Rust Backend](bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md) · [17B Frontend Infrastructure](bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md) · [17C Frontend Pages](bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md)

---


### Chapter 18: Wallet User Interface (Iced)

**[→ Read Full Chapter 18: Wallet User Interface (Iced)](bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md)**

Chapter 18 builds a user-facing wallet application with the Iced framework. We'll explore architecture and implementation details, learning about modular design patterns and code organization. We'll see how the Model-View-Update (MVU) pattern applies to a wallet use case, focusing on user experience, encrypted persistence, and blockchain backend integration.

**Companion chapter:** [18A Code Listings](bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md)

### Chapter 19: Wallet User Interface (Tauri)

**[→ Read Full Chapter 19: Wallet User Interface (Tauri)](bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md)**

Chapter 19 presents the same wallet functionality built with Tauri 2. We'll explore the Rust backend with SQLCipher database commands, the React frontend with form validation and React Query cache management, and how the Tauri wallet shares the same encrypted database file as the Iced wallet through a deterministic password generation scheme.

**Companion chapters:** [19A Rust Backend](bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md) · [19B Frontend Infrastructure](bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md) · [19C Frontend Pages](bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md)

---

## Chapter 20: Embedded Database & Persistence

**[→ Read Full Chapter 20: Embedded Database & Persistence](embedded-database/06-Embedded-Database.md)**

Chapter 20 addresses a critical aspect of wallet applications: secure data persistence. This is a framework-agnostic chapter that covers both the Iced and Tauri wallet implementations. We'll integrate SQLCipher for encrypted storage, learning about encryption strategies and deterministic password generation. We'll design database schemas and migrations, explore persistence patterns for wallet data (settings, addresses, user profiles), and understand how both wallets share the same encrypted database file. We'll tackle thread safety with `OnceLock<Mutex<Connection>>`, the singleton table pattern, upsert operations, and the table-recreation migration technique.

**Companion chapter:** [20A Code Listings](embedded-database/06A-Embedded-Database-Code-Listings.md) — Complete verbatim source from both Iced and Tauri wallets, tests, and the shared password generation function.

---

## Chapter 21: Web Admin Interface

**[→ Read Full Chapter 21: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md)**

Chapter 21 brings us into the modern web development world. We'll build a React-based web interface, exploring component architecture and state management with React Query. We'll learn about API integration and authentication patterns, understand the build process and deployment strategies, and see how TypeScript provides type safety in modern development practices. We'll work with React Hooks, routing, and styling, and discuss recent improvements and lessons learned from bug fixes.

---

## Chapter 22: Docker Compose Deployment

**[→ Read Full Chapter 22: Docker Compose Deployment](ci/docker-compose/01-Introduction.md)**

In Chapter 22, we move from building our blockchain to deploying it. We'll learn how to deploy and manage the blockchain network using Docker Compose, starting with quick start guides and progressing to advanced networking internals. This chapter will take you from zero to a fully operational blockchain network running in containers.

**Chapter Sections (read in order):**
1. [Section 1: Introduction & Quick Start](ci/docker-compose/01-Introduction.md) - Main chapter entry point with quick start guide
2. [Section 2: Architecture & Execution Flow](ci/docker-compose/02-Architecture-and-Execution.md) - Container naming, instance detection, volume structure, and startup process
3. [Section 3: Deployment Topology](ci/docker-compose/03-Deployment-Topology.md) - Network topology, port mapping, and scaling strategies
4. [Section 4: Deployment Scenarios & Operations](ci/docker-compose/04-Deployment-Scenarios-and-Operations.md) - Practical deployment examples and production procedures
5. [Section 5: Accessing Webserver](ci/docker-compose/05-Accessing-Webserver.md) - Web interface access and authentication
6. [Section 6: DNS Resolution Mechanism](ci/docker-compose/06-DNS-Resolution-Mechanism.md) - Advanced networking internals

---

## Chapter 23: Kubernetes Deployment

**[→ Read Full Chapter 23: Kubernetes Deployment](ci/kubernetes/README.md)**

Chapter 23 takes us into production-grade deployment. We'll learn how to deploy and manage the blockchain network on Kubernetes, exploring production-grade orchestration, autoscaling, high availability, and advanced operational topics. This is where we transform our development system into a production-ready, scalable blockchain network.

**Chapter Sections (read in order):**
1. [Section 1: Introduction & Quick Start](ci/kubernetes/README.md) - Main chapter entry point with quick start guide
2. [Section 2: Architecture & Core Concepts](ci/kubernetes/02-Architecture.md) - Kubernetes architecture and core concepts
3. [Section 3: Migration Guide](ci/kubernetes/03-Migration.md) - Migration guide from Docker Compose
4. [Section 4: Kubernetes Manifests](ci/kubernetes/04-Manifests.md) - Complete manifest examples
5. [Section 5: Deployment & Operations](ci/kubernetes/05-Deployment.md) - Deployment process and operations
6. [Section 6: Autoscaling](ci/kubernetes/06-Autoscaling.md) - HPA configuration and scaling
7. [Section 7: Production & Advanced Topics](ci/kubernetes/07-Production.md) - Production considerations and troubleshooting

---

## Chapter 24: Rust Language Guide

**[→ Read Full Chapter 24: Rust Language Guide](rust/README.md)**

Chapter 24 provides a comprehensive reference guide to the Rust programming language features used throughout our blockchain implementation. This guide explains Rust concepts with examples from our codebase, helping you understand not just what Rust features exist, but how they work together to build reliable, performant systems software. We'll explore ownership and borrowing, data structures, traits, error handling, generics, lifetimes, smart pointers, pattern matching, derive macros, async/await, concurrency, modules, iterators and closures, type conversions, testing, and best practices. Each section includes practical examples from our blockchain codebase, making this guide both a language reference and a practical implementation guide.

**Chapter Sections (read in order):**
1. [Section 1: Introduction](rust/01-Introduction.md) - Guide overview and Rust fundamentals
2. [Section 2: Ownership and Borrowing](rust/02-Ownership-and-Borrowing.md) - Memory safety and borrowing rules
3. [Section 3: Data Structures](rust/03-Data-Structures.md) - Structs, enums, and collections
4. [Section 4: Traits](rust/04-Traits.md) - Trait definitions and implementations
5. [Section 5: Error Handling](rust/05-Error-Handling.md) - Result, Option, and error propagation
6. [Section 6: Generics](rust/06-Generics.md) - Generic types and functions
7. [Section 7: Lifetimes](rust/07-Lifetimes.md) - Lifetime annotations and elision
8. [Section 8: Smart Pointers](rust/08-Smart-Pointers.md) - Box, Rc, Arc, and RefCell
9. [Section 9: Pattern Matching](rust/09-Pattern-Matching.md) - Match expressions and patterns
10. [Section 10: Derive Macros](rust/10-Derive-Macros.md) - Common derive macros and custom derives
11. [Section 11: Async/Await](rust/11-Async-Await.md) - Asynchronous programming
12. [Section 12: Concurrency](rust/12-Concurrency.md) - Threads, channels, and synchronization
13. [Section 13: Modules](rust/13-Modules.md) - Module system and visibility
14. [Section 14: Iterators and Closures](rust/14-Iterators-Closures.md) - Functional programming patterns
15. [Section 15: Type Conversions](rust/15-Type-Conversions.md) - From, Into, and custom conversions
16. [Section 16: Testing](rust/16-Testing.md) - Unit tests, integration tests, and test organization
17. [Section 17: Best Practices](rust/17-Best-Practices.md) - Rust idioms and coding standards

---

## Key Concepts Covered

### Blockchain Fundamentals
- Transaction structure and validation
- UTXO (Unspent Transaction Output) model
- Script execution and verification
- Merkle trees and block structure
- Consensus mechanisms
- Network protocols and peer-to-peer communication

### User Interfaces
- **Desktop Admin UI (Iced)**: Full-featured administrative interface using pure Rust Iced framework (MVU pattern)
- **Desktop Admin UI (Tauri)**: Same admin features using Tauri 2 with Rust backend + React/TypeScript frontend
- **Wallet UI (Iced)**: User-facing wallet application with encrypted storage using Iced
- **Wallet UI (Tauri)**: Same wallet features using Tauri 2 with SQLCipher persistence
- **Web Admin UI**: Modern React-based web interface

### Data Persistence
- SQLCipher encrypted database
- File system storage for blockchain data
- Settings and configuration persistence
- User profile management

### Deployment
- **Docker Compose**: Local development and simple deployments
- **Kubernetes**: Production-grade orchestration with autoscaling

---

## Additional Resources

- [Rust Official Documentation](https://doc.rust-lang.org/)
- [Iced Framework Documentation](https://docs.rs/iced/)
- [Tauri 2 Documentation](https://v2.tauri.app/)
- [React Documentation](https://react.dev/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)

---

## Contributing

This book is part of an ongoing writing process. As the codebase evolves and we learn more, we continuously update these chapters to reflect:
- New features and capabilities we've added
- Refinements to existing implementations based on real-world use
- Lessons learned during development that might help others
- Best practices we've discovered through experience

We believe that the best technical books are living documents that grow and improve alongside the systems they describe.

---

<div align="center">

**Local Navigation - Table of Contents**

| [← First Section: Chapter 1: Introduction & Overview](01-Introduction.md) | [↑ Table of Contents](#table-of-contents) | [Last Section: Chapter 24: Rust Language Guide →](#chapter-24-rust-language-guide) |
|:---:|:---:|:---:|
| *Start of Book* | *Current Chapter* | *End of Book* |

</div>

---

*This documentation is continuously updated as the book writing process progresses. For the most current information, please refer to the latest version of each chapter.*
