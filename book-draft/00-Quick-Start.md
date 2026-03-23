<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. **Chapter 1: Quick Start** ← *You are here*
2. <a href="01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="bitcoin-blockchain/README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="bitcoin-blockchain/Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="bitcoin-blockchain/primitives/README.md">Chapter 7: Primitives</a>
8. <a href="bitcoin-blockchain/util/README.md">Chapter 8: Utilities</a>
9. <a href="bitcoin-blockchain/crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="bitcoin-blockchain/chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="bitcoin-blockchain/chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="bitcoin-blockchain/chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="bitcoin-blockchain/chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="bitcoin-blockchain/chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="bitcoin-blockchain/chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="bitcoin-blockchain/chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="bitcoin-blockchain/chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="bitcoin-blockchain/store/README.md">Chapter 20: Storage Layer</a>
21. <a href="bitcoin-blockchain/net/README.md">Chapter 21: Network Layer</a>
22. <a href="bitcoin-blockchain/node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="bitcoin-blockchain/wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="bitcoin-blockchain/web/README.md">Chapter 24: Web API Architecture</a>
25. <a href="bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="Glossary.md">Glossary</a>
49. <a href="Bibliography.md">Bibliography</a>
50. <a href="Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---

# Chapter 1: Quick Start — See It Run

**Time required: under 5 minutes.** You need only Docker and Docker Compose installed.

This page exists so you can see the finish line before starting the journey. By the end of these steps, you will have a multi-node blockchain network, like the one Bitcoin runs on, running on your machine — mining blocks, propagating them to peers, and serving a web admin interface. No Rust installation required.

> **What you will learn in this chapter:**
> - Deploy a multi-node blockchain network with a single command using Docker Compose
> - Observe blocks being mined, hashed, and propagated between peers in real time
> - Query the blockchain REST API and view the web admin dashboard
> - Understand which chapters of the book correspond to each running component

---

## Step 1: Clone the repository

```bash
git clone https://github.com/bkunyiha/rust-blockchain.git
cd rust-blockchain
```

## Step 2: Start the network

```bash
cd ci/docker-compose/configs
docker compose up --build
```

This builds the Rust node from source (first run takes a few minutes while Cargo compiles), then starts two containers: a **miner** node and a **webserver** node.

## Step 3: Watch it work

Once the build finishes, you will see output like this in your terminal:

```text
miner-1     | [INFO] Starting miner on port 8010...
miner-1     | [INFO] Mining block #1 ...
miner-1     | [INFO] Block mined: hash=00a3f2...  height=1  txns=1
webserver-1 | [INFO] Connected to peer miner-1:8010
miner-1     | [INFO] Mining block #2 ...
miner-1     | [INFO] Block mined: hash=0017cb...  height=2  txns=1
webserver-1 | [INFO] Received block #2 from miner-1
```

Blocks are being mined, hashed, and propagated across the network in real time.

## Step 4: Open the admin dashboard

While the network is running, open your browser to:

```text
http://localhost:8080
```

If the dashboard prompts you to configure an API key, use the default **Admin API key**: `admin-secret` (unless you’ve overridden it via `BITCOIN_API_ADMIN_KEY`).

**API keys (what they are):** the webserver protects its REST endpoints with shared secrets passed on each request. In Docker Compose there are two keys:
- **Admin key** (`BITCOIN_API_ADMIN_KEY`, default `admin-secret`): full access to `/api/admin/*` endpoints used by the admin dashboard.
- **Wallet key** (`BITCOIN_API_WALLET_KEY`, default `wallet-secret`): access to wallet-related endpoints.

You will see the Web Admin Interface showing the current chain height, connected peers, recent blocks, and the mempool. This is the same React dashboard we build in Chapter 21.

## Step 5: Query the API

In another terminal, try:

```bash
curl http://localhost:8080/api/admin/blockchain-info | python3 -m json.tool
```

You will get a JSON response with the chain height, tip hash, peer count, and mempool size — the same API we build in Chapter 24.

---

## What just happened?

In those few commands, you ran the entire system this book teaches you to build:

- **Primitives** (Ch 6) — the `Block` and `Transaction` structs that were serialized into each mined block
- **Cryptography** (Ch 8) — the SHA-256 hashing and ECDSA signing that produced each block hash
- **Chain validation** (Ch 9–10) — the consensus rules that accepted each block into the canonical chain
- **Storage** (Ch 11) — the sled database that persisted blocks to disk
- **Networking** (Ch 12) — the TCP protocol that propagated blocks between the miner and webserver nodes
- **Node orchestration** (Ch 13) — the coordinator that routed messages between subsystems
- **Web API** (Ch 15) — the Axum REST API that served the `/api/admin/*` endpoints
- **Web Admin UI** (Ch 21) — the React dashboard you just opened in your browser
- **Docker Compose** (Ch 22) — the deployment configuration that wired everything together

The rest of this book explains how every piece of that system works, line by line.

---

## Clean up

```bash
docker compose down -v    # Stop containers and remove volumes
```

---

Continue to **[Chapter 2: Introduction & Overview](01-Introduction.md)** to understand the architecture, then follow the reading order through the implementation. To brush up on Rust first, jump to **[Chapter 33: Rust Language Guide](rust/README.md)**.

<div align="center">

**Quick Start** | **[Next: Chapter 2: Introduction & Overview →](01-Introduction.md)**
</div>
