<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

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
17. <a href="bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
18. <a href="bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
19. <a href="bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
20. <a href="embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
21. <a href="bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>

### Part II: Deployment & Operations

22. <a href="ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
23. <a href="ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>

### Part III: Language Reference

24. <a href="rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---

# Quick Start — See It Run

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

You will see the Web Admin Interface showing the current chain height, connected peers, recent blocks, and the mempool. This is the same React dashboard we build in Chapter 21.

## Step 5: Query the API

In another terminal, try:

```bash
curl http://localhost:8080/api/admin/blockchain-info | python3 -m json.tool
```

You will get a JSON response with the chain height, tip hash, peer count, and mempool size — the same API we build in Chapter 15.

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

Continue to **[Chapter 1: Introduction & Overview](01-Introduction.md)** to understand the architecture, then follow the reading order through the implementation. To brush up on Rust first, jump to **[Chapter 24: Rust Language Guide](rust/README.md)**.

<div align="center">

**Quick Start** | **[Next: Chapter 1: Introduction & Overview →](01-Introduction.md)**
</div>
