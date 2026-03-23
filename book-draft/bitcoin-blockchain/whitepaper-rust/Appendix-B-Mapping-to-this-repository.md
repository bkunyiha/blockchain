<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
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
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
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
## Appendix B. Mapping to this repository
This whitepaper chapter explains *what* to build and *why*, with sample Rust implementation code. The next step is to read the actual Rust implementation and learn *how* the pieces connect in a real project.

In this appendix, we provide a practical map between:

- the whitepaper concepts we just studied
 - the implementation documentation in the next chapters
 - the Rust code in the repository ([`bkunyiha/blockchain`](https://github.com/bkunyiha/blockchain))

If you have not cloned the repository yet, start from the end of Section 12 and clone [`bkunyiha/blockchain`](https://github.com/bkunyiha/blockchain) so you can follow along locally.

### B.1 How to read the codebase (a guided tour order)

When learning an unfamiliar blockchain implementation, we recommend reading in the same direction bytes flow through the system:
In the next chapters, we move from the whitepaper concepts into the Rust implementation code that goes with this book.

1. **Business objects and encoding**
   - Chapter: `Business objects and encoding(block, header, transaction)`
   - Code: `bitcoin/src/primitives/` (block, header, transaction)
2. **Utility helpers (bytes, plumbing, small reusable building blocks)**
   - Docs: `Utility helpers (bytes, plumbing, small reusable building blocks)`
   - Code: `bitcoin/src/util/`
3. **Crypto primitives(hashes, keys, signatures, address/script helpers)**
   - Docs: `Crypto primitives(hashes, keys, signatures, address/script helpers)`
   - Code: `bitcoin/src/crypto/` (hashes, keys, signatures, address/script helpers)
4. **Proof-of-work and block acceptance**
   - Docs: `Proof-of-work and block acceptance`
   - Code: `bitcoin/src/pow.rs`, `bitcoin/src/chain/`
5. **Storage and persistence**
   - Docs: `Storage and persistence`
   - Code: `bitcoin/src/store/` (persisting chain/wallet data)
6. **Chainstate / UTXO updates**
   - Docs: `Chainstate / UTXO updates`
   - Code: `bitcoin/src/chain/chainstate.rs`, `bitcoin/src/chain/utxo_set.rs`
7. **Networking and message handling**
   - Docs: `Networking and message handling`
   - Code: `bitcoin/src/net/`, `bitcoin/src/net/net_processing.rs`
8. **Node orchestration**
   - Docs: `Node orchestration`
   - Code: `bitcoin/src/node/` (peers, server, miner, mempool integration)
9. **Wallet/client**
   - Docs: `Wallet/client`
   - Code: `bitcoin/src/wallet/` (wallet implementation + service layer)

### B.2 Concept map: whitepaper → code modules

Use this table as a “where do I look?” index while reading:

- **Transactions / UTXO model** (Sections 2, 9)
  - Code: `bitcoin/src/primitives/transaction.rs`, `bitcoin/src/chain/utxo_set.rs`
- **Block headers / chaining** (Section 3)
  - Code: `bitcoin/src/primitives/block.rs`, `bitcoin/src/primitives/blockchain.rs`
- **Proof-of-work and difficulty** (Section 4)
  - Code: `bitcoin/src/pow.rs`
- **Network operation** (Section 5)
  - Code: `bitcoin/src/net/`, `bitcoin/src/node/server.rs`, `bitcoin/src/node/peers.rs`
- **Incentives / fees** (Section 6)
  - Code: `bitcoin/src/node/miner.rs`, `bitcoin/src/node/txmempool.rs`
- **Reclaiming disk space / pruning mindset** (Section 7)
  - Code: `bitcoin/src/store/`, `bitcoin/src/chain/`
- **Merkle trees / SPV intuition** (Sections 7–8)
  - Code: `bitcoin/src/primitives/block.rs` (Merkle-related helpers if present), plus wallet/client logic where applicable
- **Privacy / address rotation** (Section 10)
  - Code: `bitcoin/src/wallet/`, `bitcoin/src/crypto/address.rs`
- **Confirmations / settlement policy** (Section 11)
  - Code: wallet/client policy layer (often under `bitcoin/src/wallet/` or service code that decides when to mark a tx “settled”)

### B.3 The documentation to keep open while reading

For deeper implementation details of what we just went through in the whitepaper, the chapter sections that follow are already aligned to these boundaries:

- **Chain state / UTXO**: Chapters 9--18: Blockchain — From Transaction to Block Acceptance
- **Crypto primitives**: Chapter 8: Cryptography
- **Primitives**: Chapter 6: Primitives
- **Networking**: Chapter 21: Network Layer
- **Node orchestration**: Chapter 22: Node Orchestration
- **Wallet**: Section 14: Wallet System

### B.4 A simple “follow the bytes” exercise

To turn reading into understanding, pick one path and trace it end-to-end in code:

- **Receiving and confirming a transaction**:
  - network receives a `tx` message → parse → validate → add to mempool → mine/include in block → connect block → wallet updates confirmation depth

As you do this, keep asking two questions that drive correct implementations:

- What exact bytes are hashed/verified here?
- What state is updated, and how do we roll it back on a reorg?

---

<div align="center">

**[[Start] ← Bitcoin White-paper (Rust Implementation/Encoding)](README)** | **[[Previous] ←  Appendix A: Object connectivity](Appendix-A-Object-connectivity-end-to-end-flow)** | Appendix B: Mapping to this repository

</div>
