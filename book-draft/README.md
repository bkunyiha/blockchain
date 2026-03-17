<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Quick Start

0. <a href="00-Quick-Start.md">Chapter 0: Quick Start — See It Run</a>

### Part I: Foundations & Core Implementation

1. <a href="01-Introduction.md">Chapter 1: Introduction & Overview</a>
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

### Appendices

39. <a href="Glossary.md">Glossary</a>
40. <a href="Bibliography.md">Bibliography</a>
41. <a href="Appendix-Source-Reference.md">Appendix: Source Reference</a>

</details>

</div>

<div align="right">

**[← Back to Main Repository](../README.md)**

</div>

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                        HALF TITLE                          -->
<!-- ════════════════════════════════════════════════════════════ -->

<div align="center">

# Rust Blockchain

</div>

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                        TITLE PAGE                          -->
<!-- ════════════════════════════════════════════════════════════ -->

<div align="center">

# Rust Blockchain

### A Full-Stack Implementation Guide With Tokio/Async Rust, Axum, Iced, Tauri 2, Docker, and Kubernetes

**Bill Kunyiha**

*First Edition*

</div>

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                      COPYRIGHT PAGE                        -->
<!-- ════════════════════════════════════════════════════════════ -->

<div align="center">

**Rust Blockchain: A Full-Stack Implementation Guide**

First Edition

Copyright © 2026 Bill Kunyiha. All rights reserved.

No part of this publication may be reproduced, distributed, or transmitted in any form or by any means without the prior written permission of the author, except for brief quotations in reviews and certain noncommercial uses permitted by copyright law.

ISBN: 979-8-9954311-0-7

*Published by the author.*

</div>

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                       DEDICATION                           -->
<!-- ════════════════════════════════════════════════════════════ -->

<div align="center">

*To the contributors behind Rust, Bitcoin Core, and the countless open-source projects that made this work possible. You proved that trust can be built in code.*

</div>

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                     ACKNOWLEDGMENTS                        -->
<!-- ════════════════════════════════════════════════════════════ -->

## Acknowledgments

### Foundational Research

This book begins where blockchain itself began: with Satoshi Nakamoto's *Bitcoin: A Peer-to-Peer Electronic Cash System* (2008). That nine-page paper defined the problem, the solution, and the security model that Chapters 3, 4, and 10 of this book translate into Rust. The NIST Secure Hash Standard (FIPS 180-4) and the SEC 2 elliptic curve parameters underpin our cryptographic implementation in Chapter 8. Andreas Antonopoulos's *Mastering Bitcoin* and the Princeton *Bitcoin and Cryptocurrency Technologies* textbook provided invaluable reference throughout.

### Bitcoin Core

The Bitcoin Core C++ implementation served as the authoritative reference for this Rust blockchain project, built on Bitcoin's design. Its validation logic, UTXO management, peer-to-peer protocol, and consensus rules informed the design decisions throughout Chapters 6–14. Where our implementation simplifies (fixed difficulty, single-key wallets, no script system), it was Bitcoin Core's code that showed us what a production system requires.

### Frameworks and Tools

Every layer of this system stands on open-source foundations. The **Rust** language and the Cargo ecosystem made it possible to build a memory-safe blockchain in a single language. **Axum** and **Tower** power the REST API. **Iced** provides the pure Rust MVU desktop framework. **Tauri 2** bridges Rust and React for the hybrid desktop applications. **Sled** handles blockchain persistence, **SQLCipher** encrypts wallet data, and **Tokio** drives the async runtime. On the frontend, **React**, **TypeScript**, and **Zustand** provide the web and Tauri UI layers. **Docker** and **Kubernetes** handle deployment.

### The Open-Source Community

Every framework listed above is maintained by volunteers and contributors who answer issues, review pull requests, write documentation, and share their expertise freely. This book is a product of that culture. To the maintainers and contributors behind every crate, library, and tool referenced in these pages: thank you.

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                    TABLE OF CONTENTS                       -->
<!-- ════════════════════════════════════════════════════════════ -->

<a id="table-of-contents"></a>

## Table of Contents

<div align="center">

### **Part I: Foundations & Core Implementation**

| # | Chapter | Description | Link |
|---|---------|-------------|------|
| 1 | **Introduction & Overview** | Book introduction, project structure, technical stack | **[Read Chapter 1 →](01-Introduction.md)** |
| 2 | **Introduction to Blockchain** | Blockchain fundamentals, including Bitcoin's role as the reference implementation, use cases, and technical foundations | **[Read Chapter 2 →](bitcoin-blockchain/README.md)** |
| 3 | **Bitcoin Whitepaper** | Whitepaper that defined Bitcoin | **[Read Chapter 3 →](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md)** |
| 4 | **Bitcoin Whitepaper In Rust** | Whitepaper → Rust encoding walkthrough: concrete data structures, hashing/serialization rules, and how the sections map into this repo. | **Index:** **[Rust Encoding Summary →](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)**<br>**Core building blocks:** **[Business Objects](bitcoin-blockchain/whitepaper-rust/00-business-objects.md)**<br>**Sections 1–4:** **[§1 Intro](bitcoin-blockchain/whitepaper-rust/01-Introduction-Bitcoin-Whitepaper-Section-1.md)** · **[§2 Transactions](bitcoin-blockchain/whitepaper-rust/02-Transactions-chain-of-signatures-Bitcoin-Whitepaper-Section-2.md)** · **[§3 Timestamp server](bitcoin-blockchain/whitepaper-rust/03-Timestamp-server-block-header-chaining-Bitcoin-Whitepaper-Section-3.md)** · **[§4 Proof-of-work](bitcoin-blockchain/whitepaper-rust/04-Proof-of-work-Bitcoin-Whitepaper-Section-4.md)** · **[nBits / target](bitcoin-blockchain/whitepaper-rust/04A-nBits-Target-Expansion.md)**<br>**Sections 5–9:** **[§5 Network](bitcoin-blockchain/whitepaper-rust/05-Network-operation-Bitcoin-Whitepaper-Section-5.md)** · **[§6 Incentives](bitcoin-blockchain/whitepaper-rust/06-Incentive-mechanism-Bitcoin-Whitepaper-Section-6.md)** · **[§7 Pruning](bitcoin-blockchain/whitepaper-rust/07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)** · **[§8 Merkle/SPV](bitcoin-blockchain/whitepaper-rust/08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)** · **[§9 Value](bitcoin-blockchain/whitepaper-rust/09-Combining-splitting-value-Bitcoin-Whitepaper-Section-9.md)**<br>**Sections 10–12:** **[§10 Privacy](bitcoin-blockchain/whitepaper-rust/10-Privacy-Bitcoin-Whitepaper-Section-10.md)** · **[§11 Confirmations](bitcoin-blockchain/whitepaper-rust/11-Confirmations-and-attacker-probability-Bitcoin-Whitepaper-Section-11.md)** · **[§12 Conclusion](bitcoin-blockchain/whitepaper-rust/12-Conclusion-Bitcoin-Whitepaper-Section-12.md)**<br>**Appendices:** **[A: End-to-end object connectivity](bitcoin-blockchain/whitepaper-rust/Appendix-A-Object-connectivity-end-to-end-flow.md)** · **[B: Mapping to this repository](bitcoin-blockchain/whitepaper-rust/Appendix-B-Mapping-to-this-repository.md)** |
| 5 | **Rust Blockchain Project** | Rust blockchain project structure, crate organization, and build configuration | **[Read Chapter 5 →](bitcoin-blockchain/Rust-Project-Index.md)** |
| 6 | **Primitives** | Core data structures (Block, Transaction, Blockchain) | **[Read Chapter 6 →](bitcoin-blockchain/primitives/README.md)** |
| 7 | **Utilities** | Utility functions and helpers | **[Read Chapter 7 →](bitcoin-blockchain/util/README.md)** |
| 8 | **Cryptography** | Cryptographic primitives, hash functions, signatures, and key pairs | **[Read Chapter 8 →](bitcoin-blockchain/crypto/README.md)** |
| 9 | **Chain (Code Walkthrough)** | End-to-end chain pipeline: model → state → storage → UTXO → mining → consensus → orchestration. | **Index:** **[9 →](bitcoin-blockchain/chain/README.md)**<br>**9.x:** **[9.1 Model](bitcoin-blockchain/chain/01-Domain-Model.md)** · **[9.2 State](bitcoin-blockchain/chain/02-Blockchain-State-Management.md)** · **[9.3 Storage](bitcoin-blockchain/chain/03-Chain-State-and-Storage.md)** · **[9.4 UTXO](bitcoin-blockchain/chain/04-UTXO-Set.md)**<br>**Flow:** **[9.5 Tx](bitcoin-blockchain/chain/05-Transaction-Lifecycle.md)** · **[9.6 Mine](bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md)** · **[9.7 Rules](bitcoin-blockchain/chain/07-Consensus-and-Validation.md)** · **[9.8 Node](bitcoin-blockchain/chain/08-Node-Orchestration.md)** · **[9.9 E2E](bitcoin-blockchain/chain/09-Transaction-To-Block.md)** |
| 10 | **Block Acceptance (WP §5/Step 5)** | "Valid + not already spent" (tx verify + UTXO + rules). | **Read:** **[10 →](bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md)**<br>**See also:** **[UTXO](bitcoin-blockchain/chain/04-UTXO-Set.md)** · **[Rules](bitcoin-blockchain/chain/07-Consensus-and-Validation.md)** · **[E2E](bitcoin-blockchain/chain/09-Transaction-To-Block.md)** |
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

### **Appendices**

| | Title | Description | Link |
|---|-------|-------------|------|
| — | **Glossary** | Key terms and definitions | **[Read Glossary →](Glossary.md)** |
| — | **Bibliography** | References to primary sources, language documentation, frameworks, and deployment tools | **[Read Bibliography →](Bibliography.md)** |
| — | **Source Reference** | Repository structure and companion chapter index | **[Read Appendix →](Appendix-Source-Reference.md)** |

</div>

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                         PREFACE                            -->
<!-- ════════════════════════════════════════════════════════════ -->

## Preface

Welcome to *Rust Blockchain: A Full-Stack Implementation Guide*. In this book, we build a complete blockchain system in Rust, modeled on Bitcoin's architecture and powered by Tokio's async runtime. Together, we explore every layer — from cryptographic primitives and consensus rules through async networking and REST APIs to desktop and web user interfaces, all the way to containerized deployment with Docker and Kubernetes.

### Who This Book Is For

This book is written for technical professionals who want to understand blockchain implementation at a deep level:

- **Software Engineers** building blockchain applications or learning Rust through a practical, full-stack project
- **Technologists learning Rust and its ecosystem** who want hands-on experience with Axum, Tokio, Iced, Tauri 2, Sled, SQLCipher, serde, and secp256k1 — all through one cohesive codebase rather than isolated tutorials
- **DevOps Engineers** who need to deploy and operate blockchain networks with Docker Compose and Kubernetes in production environments
- **Students and Researchers** studying blockchain implementation details and seeking hands-on experience with modern systems programming
- **Technical Professionals** who want to understand how blockchain systems work under the hood, using Bitcoin as the reference architecture — from cryptographic primitives to desktop and web user interfaces

### Technologies You Will Learn

This is not just a blockchain book. By building a single, cohesive system from whitepaper to production deployment, you will gain hands-on experience with the modern Rust ecosystem and its surrounding technologies:

| Technology | Role in This Book | Chapters |
|---|---|---|
| **Rust** | Systems language for the entire blockchain core, wallet, and desktop applications | 1–24 |
| **Tokio** | Async runtime powering the entire system: networking, mining, node orchestration, the web API, and desktop backends. Learn `async`/`await`, task spawning, channels, and `select!` through real production code | 9, 12, 13, 15, 16–19 |
| **Axum / Tower** | Production REST API with typed handlers, middleware, authentication, and OpenAPI | 15 |
| **Iced** | Pure Rust desktop UI framework using the Model-View-Update pattern | 16, 18 |
| **Tauri 2** | Hybrid desktop framework: Rust backend + React/TypeScript frontend via IPC | 17, 19 |
| **SQLCipher / rusqlite** | Encrypted embedded database for wallet persistence | 20 |
| **Sled** | Embedded key-value store for blockchain data persistence | 11 |
| **React / TypeScript** | Frontend for the Tauri desktop apps and the web admin interface | 17, 19, 21 |
| **Docker Compose** | Multi-node containerized deployment for development and testing | 22 |
| **Kubernetes** | Production-grade orchestration with autoscaling and high availability | 23 |
| **secp256k1** | Elliptic curve cryptography for key generation and transaction signing | 8, 14 |
| **serde** | Serialization framework used throughout every module | 6–15 |

**Async Rust with Tokio** is woven throughout the entire project — not isolated in a single chapter. You will use `async`/`await` for TCP peer connections, spawn concurrent mining tasks, orchestrate node subsystems with channels and `select!`, build a production web API on Axum (which runs on Tokio), and handle IPC commands in Tauri's async backend. By the time you finish the book, async Rust will feel natural because you will have used it in every context: networking, computation, database access, and UI integration.

Chapters 16–19 offer a unique side-by-side comparison: the same admin and wallet applications built in both **Iced** (pure Rust, MVU pattern) and **Tauri 2** (Rust + React, IPC bridge). By implementing identical functionality in two frameworks, you will understand the architectural trade-offs and choose the right tool for your own projects.

### Book Structure

We have organized this book into three main parts that reflect the natural progression from implementation to deployment to language reference.

**Part I: Foundations & Core Implementation** (Chapters 1–21) dives deep into the heart of blockchain technology. We explore the fundamental concepts, data structures, and implementation details that make a blockchain work. You will learn about transaction systems, cryptographic primitives, and how to build user interfaces that interact with the blockchain — with both pure Rust (Iced) and Rust + React (Tauri) desktop implementations side by side. Each chapter provides deep technical insights into blockchain architecture, showing you not just what to build, but why we build it this way.

**Part II: Deployment & Operations** (Chapters 22–23) takes us from building our blockchain to deploying it. We start with Docker Compose for development and local deployments, then move to Kubernetes for production-grade orchestration. You will learn practical guidance for running and scaling the system in real-world scenarios.

**Part III: Language Reference** (Chapter 24) provides a comprehensive reference guide to the Rust programming language features used throughout the blockchain implementation. This guide explains Rust concepts with examples from our codebase, helping you understand not just what Rust features exist, but how they work together to build reliable, performant systems software.

### How to Read This Book

This book is designed to accommodate different reading styles.

**Sequential reading.** If you are new to blockchain development or want a complete understanding, follow the chapters in order. Each chapter builds naturally on the previous ones.

**Topic-based learning.** If you have specific interests — perhaps you are focused on UI development or deployment — use the suggested paths below to focus on those areas while still understanding the broader context.

**Reference reading.** If you are already familiar with blockchain concepts and need specific information, feel free to jump to the chapters that interest you. Each chapter is written to be self-contained while referencing related concepts.

**Learning by example.** Throughout the book, you will find practical code examples and explanations. We encourage you to read the code alongside the explanations, as understanding the implementation details is crucial to mastering blockchain development.

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                    CHAPTER PREREQUISITES                     -->
<!-- ════════════════════════════════════════════════════════════ -->

## Chapter Prerequisites

The table below shows which chapters are required reading before starting each chapter. "Recommended" chapters provide helpful context but are not strictly necessary.

| Chapter | Required Reading | Recommended |
|---------|-----------------|-------------|
| Ch 1: Introduction | None | — |
| Ch 2: Introduction to Blockchain | None | — |
| Ch 3: Bitcoin Whitepaper | None | Ch 2 |
| Ch 4: Whitepaper in Rust | Ch 3 | Ch 2 |
| Ch 5: Rust Project Index | Ch 1 | — |
| Ch 6: Primitives | Ch 1 | Ch 4 |
| Ch 7: Utilities | Ch 6 | — |
| Ch 8: Cryptography | Ch 6 | Ch 3, Ch 4 |
| Ch 9: Chain | Ch 6, Ch 8 | Ch 2, Ch 3, Ch 4 |
| Ch 10: Block Acceptance | Ch 9 | Ch 3 |
| Ch 11: Storage | Ch 6 | Ch 9 |
| Ch 12: Network | Ch 6, Ch 9 | Ch 11 |
| Ch 13: Node Orchestration | Ch 9, Ch 11, Ch 12 | Ch 14 |
| Ch 14: Wallet | Ch 6, Ch 8 | Ch 9 |
| Ch 15: Web API | Ch 13 | Ch 9 |
| Ch 16: Desktop Admin (Iced) | Ch 15 | Ch 24 (if new to Rust) |
| Ch 17: Desktop Admin (Tauri) | Ch 15 | Ch 16 (for comparison) |
| Ch 18: Wallet UI (Iced) | Ch 14, Ch 15 | Ch 16 |
| Ch 19: Wallet UI (Tauri) | Ch 14, Ch 15 | Ch 17, Ch 18 |
| Ch 20: Embedded Database | Ch 18 or Ch 19 | — |
| Ch 21: Web Admin Interface | Ch 15 | Ch 16 or Ch 17 |
| Ch 22: Docker Compose | Ch 0 (Quick Start) | Ch 1 |
| Ch 23: Kubernetes | Ch 22 | — |
| Ch 24: Rust Language Guide | None | — |

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                     SUGGESTED PATHS                        -->
<!-- ════════════════════════════════════════════════════════════ -->

## Suggested Reading Paths

### First-Time Readers

If you need a Rust refresher — or have not used Rust recently — start with the Rust guide: **[Chapter 24: Rust Language Guide](rust/README.md)**, starting with **[Rust Installation & Setup](rust/00-Rust-Installation-Setup.md)**.

For those new to blockchain development or this project, we recommend beginning here:

1. **[Chapter 1: Introduction & Overview](01-Introduction.md)** — Project structure, technical stack, and overall architecture.
2. **[Chapter 2: Introduction to Blockchain](bitcoin-blockchain/README.md)** — Bitcoin and blockchain fundamentals, origins, advantages, and use cases.
3. **[Chapter 3: Bitcoin Whitepaper](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md)** — The whitepaper that defined Bitcoin.
4. **[Chapter 4: Bitcoin Whitepaper In Rust](bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)** — Implementing the whitepaper in Rust.
5. **[Chapter 8: Cryptography](bitcoin-blockchain/crypto/README.md)** — Hash functions, digital signatures, key pairs, and address encoding.
6. **[Chapter 9: Chain](bitcoin-blockchain/chain/README.md)** — The end-to-end chain pipeline and transaction handling.
7. **[Chapter 15: Web API Architecture](bitcoin-blockchain/web/README.md)** — REST API structure, request flow, and security.
8. **Choose your path** based on what interests you most:
   - **Desktop UI (Pure Rust):** [Chapter 16: Desktop Admin (Iced)](bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md) or [Chapter 18: Wallet (Iced)](bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md)
   - **Desktop UI (Rust + React):** [Chapter 17: Desktop Admin (Tauri)](bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md) or [Chapter 19: Wallet (Tauri)](bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md)
   - **Web UI:** [Chapter 21: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md)
   - **Deployment:** [Chapter 22: Docker Compose](ci/docker-compose/01-Introduction.md)

### Experienced Developers

If you are already familiar with blockchain concepts and Rust, you can dive deeper into specific areas:

- **Blockchain Implementation:** Start with [Chapter 2](bitcoin-blockchain/README.md) for fundamentals, then [Chapter 8: Cryptography](bitcoin-blockchain/crypto/README.md) and [Transaction ID Format (Chapter 6)](bitcoin-blockchain/primitives/02-Transaction-ID-Format.md) for implementation choices and design decisions.
- **Async Rust / Tokio:** Follow [Chapter 9: Chain](bitcoin-blockchain/chain/README.md) (task spawning, channels) → [Chapter 12: Network](bitcoin-blockchain/net/README.md) (async TCP, peer connections) → [Chapter 13: Node Orchestration](bitcoin-blockchain/node/README.md) (`select!`, message routing) → [Chapter 15: Web API](bitcoin-blockchain/web/README.md) (Axum on Tokio). This path shows Tokio in four production contexts.
- **UI Architecture:** Explore [Chapter 16: Desktop Admin (Iced)](bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md) for the pure Rust MVU approach, [Chapter 17: Desktop Admin (Tauri)](bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md) for the Rust + React IPC approach, or [Chapter 21: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) for the web UI.
- **Deployment:** Jump directly to [Chapter 22: Docker Compose](ci/docker-compose/01-Introduction.md) or [Chapter 23: Kubernetes](ci/kubernetes/README.md).

### Operations Teams

If your focus is on deploying and operating the system:

- **Quick Deployment:** Begin with [Chapter 22: Docker Compose Deployment](ci/docker-compose/01-Introduction.md) to get up and running quickly.
- **Production Setup:** For production deployments, [Chapter 23: Kubernetes Deployment](ci/kubernetes/README.md) provides a complete guide to production-grade orchestration and scaling.

---

<!-- ════════════════════════════════════════════════════════════ -->
<!--                        CLEANUP                             -->
<!-- ════════════════════════════════════════════════════════════ -->

## Cleanup (Stopping and Undeploying)

When you're done experimenting, you can tear down resources cleanly. This is especially important on laptops, where local clusters can consume CPU/RAM/disk in the background.

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

---

## Additional Resources

- [Rust Official Documentation](https://doc.rust-lang.org/)
- [Iced Framework Documentation](https://docs.rs/iced/)
- [Tauri 2 Documentation](https://v2.tauri.app/)
- [React Documentation](https://react.dev/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)

---

<div align="center">

**Local Navigation — Table of Contents**

| [← First Chapter: Introduction & Overview](01-Introduction.md) | [↑ Table of Contents](#table-of-contents) | [Last Chapter: Rust Language Guide →](rust/README.md) |
|:---:|:---:|:---:|
| *Start of Book* | *Current Page* | *End of Book* |

</div>

---

*This documentation is continuously updated as the book writing process progresses. For the most current information, please refer to the latest version of each chapter.*
