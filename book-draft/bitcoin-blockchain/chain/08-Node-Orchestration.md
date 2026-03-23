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
10. <a href="01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. **Chapter 17: Node Orchestration** ← *You are here*
18. <a href="09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
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

# Chapter 17: Node Orchestration — Coordinating the Blockchain Runtime

In the previous sections, we've explored how transactions are constructed and signed (Chapter 14), how blocks are mined and persisted (Chapter 15), and how the UTXO set tracks spendable outputs (Chapter 13). But how does all of this come together at runtime? How does a node coordinate between incoming network messages, the mempool, mining operations, and blockchain state updates?

**What is a node?** A blockchain node is a running instance of the blockchain software that maintains its own local copy of the blockchain, validates transactions and blocks, participates in the peer-to-peer network, and (optionally) mines new blocks.

This is a crucial distributed systems concept: **each node stores its own complete copy of the blockchain** (as we saw in Chapter 12, where `BlockchainFileSystem` persists blocks to a local sled database). Nodes synchronize with each other by exchanging blocks and transactions over the network, but each node's storage is independent—there's no central database that all nodes share. This decentralization is what makes blockchains resilient: even if some nodes go offline, others continue operating with their own copies.

In our implementation, a node is the complete runtime system that coordinates blockchain state (stored locally in `BlockchainFileSystem`), transaction mempool, network communication, and mining operations.

**Node orchestration** is the coordination layer that wires these subsystems together—it routes network messages to the right handlers, manages shared state (like the mempool and peer list), coordinates between mining and network propagation, and ensures all components work together as a cohesive system. Think of it as the "conductor" that directs when and how different parts of the node interact.

This section answers those questions by walking through the **node orchestration layer**—the code that wires everything together and makes the blockchain node function as a cohesive system. We'll trace how external events (network messages, API requests) flow through the system, how global state is managed, and how different subsystems coordinate their work.

**What we'll learn**: How the node runtime coordinates blockchain operations, how global singletons manage shared state, how `NodeContext` serves as the central coordination point, and how network messages route into the appropriate subsystems. We'll understand the complete runtime architecture and be able to trace any operation from entry point to completion.

**Primary code files** (we'll walk through these):
- `bitcoin/src/node/server.rs` — global singletons and TCP server loop
- `bitcoin/src/node/context.rs` — central coordination API (`NodeContext`)
- `bitcoin/src/net/net_processing.rs` — network message routing and handling
- `bitcoin/src/node/txmempool.rs` — mempool management
- `bitcoin/src/node/miner.rs` — mining coordination

## Scope (section flow)

This chapter covers the **orchestration layer** that coordinates **the blockchain pipeline (Blockchain — From Transaction to Block Acceptance) Steps 4–8**: how web/network events route into mempool admission, mining triggers, propagation, and block acceptance. While previous sections focused on *what* happens (transaction lifecycle, block mining), this section focuses on *how* it all connects at runtime.

### Diagram — runtime event flow (how orchestration coordinates events)

This diagram shows how external events flow through the orchestration layer and get routed to the appropriate subsystems. This is the "runtime wiring" we'll explore in detail:

External Events (arrive at runtime)
  │
  ├─> Web API Request (HTTP)
  │     │
  │     ├─> GET /api/balance
  │     │     └─> NodeContext::get_balance
  │     │           └─> BlockchainService → UTXOSet → Storage
  │     │
  │     └─> POST /api/transaction
  │           └─> NodeContext::process_transaction
  │                 ├─> Check duplicate (GLOBAL_MEMORY_POOL)
  │                 ├─> Add to mempool (txmempool)
  │                 └─> Spawn background: broadcast + mining trigger
  │
  └─> Network Message Router (Peer-to-Peer(P2P))
        │
        │ Messages arrive via TCP connections from other nodes in the network.
        │ Each connection spawns a task that calls process_stream to deserialize
        │ and route messages based on Package enum type.
        │
        ├─> Package::Tx (transaction from peer)
        │     └─> process_stream → NodeContext::process_transaction
        │           └─> [same flow as web API transaction]
        │
        ├─> Package::Block (block from peer)
        │     └─> process_stream → NodeContext::add_block
        │           ├─> BlockchainService::add_block (validate + persist)
        │           ├─> Remove txs from mempool (confirmed)
        │           └─> Request next block (if syncing)
        │
        ├─> Package::Inv (inventory announcement)
        │     ├─> OpType::Block → Add to download queue → Request via GETDATA
        │     └─> OpType::Tx → Check mempool → Request if missing
        │
        └─> Package::GetData (peer requests data)
              ├─> OpType::Block → Lookup in blockchain → Send block
              └─> OpType::Tx → Lookup in mempool → Send transaction

Background Tasks (spawned by orchestration)
  │
  ├─> Transaction Broadcast (if central node)
  │     └─> submit_transaction_for_mining → send_inv to all peers
  │
  └─> Mining Trigger (if threshold met)
        └─> prepare_mining_utxo → process_mine_block
              ├─> BlockchainService::mine_block (verify + mine)
              └─> broadcast_new_block → send_inv to peers

**What this diagram shows**: The orchestration layer (`NodeContext` and `process_stream`) acts as a **message router and coordinator** that:

- **Receives events from multiple sources**: Web API requests (HTTP handlers) and network messages (P2P peers) arrive concurrently and need routing to the right handlers.

- **Routes events to appropriate subsystems**:
  - Transaction events → mempool admission (`add_to_memory_pool`) → background broadcast/mining
  - Block events → blockchain validation (`add_block`) → mempool cleanup → chain state update
  - Query events → blockchain reads (`get_balance`, `get_blockchain_height`) → UTXO set queries

- **Coordinates background work asynchronously**: Long-running operations (network broadcast, mining) are spawned as separate tasks using `tokio::spawn`, allowing the node to respond immediately to API requests while background work continues.

- **Manages shared state**: Global singletons (`GLOBAL_MEMORY_POOL`, `GLOBAL_NODES`, `GLOBAL_BLOCKS_IN_TRANSIT`) are accessed by multiple subsystems concurrently, and the orchestration layer ensures consistent access patterns.

- **Handles concurrency**: Multiple peers can send messages simultaneously, multiple API requests can arrive concurrently, and mining/broadcast tasks run in parallel—the orchestration layer coordinates all of this safely.

This is the "runtime wiring" we'll explore step-by-step in the code walkthrough below, showing exactly how `NodeContext` methods and `process_stream` implement this coordination.

## How to read this section

To get the most out of this section, we'll follow this approach:

- **Start with the architecture diagrams** to understand the big picture before diving into code details
- **Read each step in order** — each step builds on the previous one, showing how different pieces connect
- **Follow the flow**: architecture overview → global state → coordination API → runtime paths → code walkthrough
- **Keep our mental model simple**: The node is a coordinator that routes events to the right subsystems and manages shared state

**Code listings** are copied from the project and annotated with inline comments to explain *what* each line does and *why* it matters. All methods are shown in full so we can read this book without having the project repository open.

### Rust concepts we'll encounter

This section uses several Rust-specific patterns that are essential for async network programming:

- **`async`/`await`**: Functions marked `async` return `Future`s. The `await` keyword suspends execution until the future completes, allowing other tasks to run. This enables efficient concurrent I/O without blocking threads.

- **`tokio::spawn`**: Spawns a new async task that runs concurrently. Tasks are lightweight (unlike OS threads) and can number in the thousands.

- **`tokio::select!`**: A macro that waits for multiple futures and executes the branch for the first one that completes. Used for handling multiple concurrent events (e.g., shutdown signal vs incoming connection).

- **`Lazy`** (from `once_cell`): Thread-safe lazy initialization of static variables. Initializes on first access and ensures single initialization.

- **`Clone`**: Allows copying types. In async Rust, we often clone values to move them into closures/tasks. `NodeContext` implements `Clone` to allow sharing across tasks.

- **`Send + Sync`**: Marker traits for thread safety. `Send` means a type can be moved between threads. `Sync` means a type can be shared between threads via `&T`.

- **Pattern matching (`match`)**: Exhaustively handles enum variants. Used extensively for routing network messages based on their type.

- **`Result<T, E>`**: Rust's error handling type. `Ok(T)` for success, `Err(E)` for errors. The `?` operator propagates errors.

- **Iterator chains**: Rust's iterator API (`iter()`, `filter()`, `map()`, `collect()`) provides a functional programming style for data transformations.

---

## Overview: Node architecture at a glance

Before diving into the code, we first lay out the big picture. A blockchain node is fundamentally a **coordinator** that manages multiple subsystems: blockchain state, transaction mempool, network communication, and mining operations. Understanding how these pieces fit together will make our detailed code walkthrough much easier to follow.

### Diagram — high-level architecture (subsystems and coordination)

This diagram shows the overall architecture: external interfaces communicate with `NodeContext`, which coordinates between subsystems, and all persistent state ultimately lives in the storage layer.

┌───────────────────────────────────────────────────────────────────┐
│                      External Interfaces                          │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────┐  ┌──────────┐ │
│  │   Web API    │  │  Wallet UI   │  │  Admin UI  │  │  Network │ │
│  │  (HTTP/REST) │  │              │  │            │  │  (P2P)   │ │
│  └──────┬───────┘  └──────-┬──────┘  └──────┬─────┘  └─────┬────┘ │
│         │                  │                │              │      │
└─────────┼─────────────────-┼────────────────┼──────────────┼──────┘
          │                  │                │              │
          └──────────────────┼────────────────┼──────────────┘
                             │                │
                    ┌────────▼────────────────▼───────┐
                    │         NodeContext             │
                    │    (Central Coordination)       │
                    └────────┬────────────────────────┘
                             │
        ┌────────────────────┼─────────────────────┐
        │                    │                     │
   ┌────▼────┐         ┌─────▼────┐         ┌──────▼──────┐
   │ Chain   │         │ Mempool  │         │  Network    │
   │ Service │         │          │         │   Layer     │
   │         │         │          │         │             │
   │ - Blocks│         │ - Pending│         │ - Peers     │
   │ - UTXO  │         │   TXs    │         │ - Messages  │
   │ - State │         │ - Flags  │         │ - Sync      │
   └────┬────┘         └─────┬────┘         └──────┬──────┘
        │                    │                     │
   ┌────▼────┐               │              ┌────▼─────┐
   │ UTXO Set│               │              │  Peers   │
   │         │               │              │  Manager │
   │ - Query │               │              └──────────┘
   │ - Update│               │
   └────┬────┘               │
        │                    │
        └──────────┬─────────┘
                   │
            ┌──────▼────────┐
            │ Storage Layer │
            │   (sled DB)   │
            │               │
            │ - Blocks      │
            │ - Chainstate  │
            │ - Metadata    │
            └───────────────┘

**What this diagram shows**: The separation of concerns in our implementation. External interfaces (web API, wallet UI, network peers) all communicate through `NodeContext`, which acts as a central coordinator. `NodeContext` delegates to specialized subsystems (chain state, mempool, network), and all persistent data ultimately lives in the storage layer.

### Diagram — dependency graph (who calls whom)

This diagram shows the call relationships: how external code calls `NodeContext`, how `NodeContext` delegates to subsystems, and where global state is accessed.

External Layer (web/clients/network)
  │
  ├─> NodeContext (central API)
  │     │
  │     ├─> BlockchainService (chain state operations)
  │     │     └─> UTXOSet (query/update spendable outputs)
  │     │
  │     ├─> GLOBAL_MEMORY_POOL (via txmempool functions)
  │     │     └─> add_to_memory_pool / remove_from_memory_pool
  │     │
  │     ├─> miner functions (mining coordination)
  │     │     ├─> should_trigger_mining
  │     │     ├─> prepare_mining_utxo
  │     │     └─> process_mine_block
  │     │
  │     └─> net_processing (network message handling)
  │           ├─> send_inv (announce transactions/blocks)
  │           ├─> send_block (transmit block data)
  │           └─> send_tx (transmit transaction data)
  │
  └─> Server::run_with_shutdown (TCP accept loop)
        └─> net_processing::process_stream (per-connection handler)

Global Singletons (shared across all subsystems):
  ├─> GLOBAL_MEMORY_POOL (pending transactions)
  ├─> GLOBAL_NODES (peer list)
  ├─> GLOBAL_BLOCKS_IN_TRANSIT (download queue)
  └─> CENTERAL_NODE (bootstrap node address)

All persistent state ultimately lives in:
  └─> BlockchainFileSystem (sled database)

**What this diagram shows**: The call hierarchy and shared state. `NodeContext` is the primary entry point, but subsystems also access global singletons directly (e.g., `GLOBAL_MEMORY_POOL`). The server loop spawns per-connection tasks that handle network messages.

### Diagram — example flow: "get balance" query

This diagram traces a concrete example: how a balance query flows from the web API through the system.

Client HTTP Request
  │
  ├─> GET /api/balance?address=1A1zP1eP...
  │
  └─> Web Handler (HTTP layer)
        │
        └─> NodeContext::get_balance(address)
              │
              └─> BlockchainService (via NodeContext)
                    │
                    └─> UTXOSet::get_balance(address)
                          │
                          ├─> Query UTXO set (sled DB)
                          │     └─> Filter outputs matching address
                          │
                          └─> Sum output values
                                │
                                └─> Return balance (satoshis)
                                      │
                                      └─> HTTP Response: {"balance": 50000}

**What this diagram shows**: A simple read operation that doesn't modify state. The query flows through `NodeContext` to the blockchain service, which queries the UTXO set, and the result flows back up the same path.

### Diagram — example flow: "send bitcoin" from wallet address A → wallet address B

This diagram traces a concrete write operation: create a signed transaction spending UTXOs from **A** (`from_address`) to **B** (`to_address`), accept it into the mempool, then propagate it to peers (and potentially trigger mining).

Wallet UI (desktop/web)
  │
  ├─> bitcoin_api::client::WalletClient::send_transaction(&SendTransactionRequest)
  │     └─> POST /api/v1/transactions
  │           body: { from_address: "A", to_address: "B", amount: N }
  │
  └─> bitcoin::web::handlers::transaction::send_transaction(State<Arc<NodeContext>>, Json<SendTransactionRequest>)
        │
        ├─> NodeContext::btc_transaction(&WalletAddress /*A*/, &WalletAddress /*B*/, amount: i32)
        │     │
        │     ├─> UTXOSet::new(self.blockchain.clone())                 <-- Initialize a UTXO view over chainstate
        │     │
        │     ├─> Transaction::new_utxo_transaction(A, B, amount, &UTXOSet).await   <-- Build + sign a spend transaction
        │     │     ├─> Select spendable UTXOs owned by A (inputs)
        │     │     ├─> Create outputs:
        │     │     │     - pay-to B (amount)
        │     │     │     - change back to A (if needed)
        │     │     └─> Sign inputs (prove A can spend them)
        │     │
        │     └─> NodeContext::process_transaction(&addr_from, Transaction).await   <-- Mempool acceptance entry point
        │           │
        │           ├─> txmempool::transaction_exists_in_pool(&Transaction)         <-- Duplicate check
        │           │     └─> Reject duplicates early
        │           │
        │           ├─> txmempool::add_to_memory_pool(Transaction, &BlockchainService).await  <-- Add to mempool (unconfirmed set)
        │           │     ├─> GLOBAL_MEMORY_POOL.add(tx)
        │           │     └─> UTXOSet::set_global_mem_pool_flag(&tx, true).await   <-- Mark inputs as "reserved" by mempool
        │           │
        │           ├─> tokio::spawn(NodeContext::submit_transaction_for_mining(&addr_from, tx))  <-- Background: propagate + maybe mine
        │           │     │
        │           │     ├─> (network propagation; only if this node is CENTERAL_NODE)
        │           │     │     ├─> NodeContext::get_nodes_excluding_sender(&addr_from).await
        │           │     │     └─> NodeContext::broadcast_transaction_to_nodes(&[Node], txid_bytes)
        │           │     │           └─> net::net_processing::send_inv(&peer_addr, OpType::Tx, &[txid]).await   <-- "I have txid"
        │           │     │
        │           │     └─> (optional mining trigger)
        │           │           ├─> miner::should_trigger_mining()                  <-- Mempool threshold reached?
        │           │           ├─> miner::prepare_mining_utxo(&mining_address)
        │           │           └─> miner::process_mine_block(txs, &BlockchainService).await
        │           │
        │           └─> Return txid immediately (HTTP 202): SendBitCoinResponse { txid, timestamp }
        │
        └─> Peer follow-up (how the tx spreads)
              ├─> Peers receive INV in net::net_processing::process_stream(...)
              ├─> Peers request the full tx: net::net_processing::send_get_data(&origin, OpType::Tx, txid).await  <-- "send me tx"
              └─> Origin serves the tx: net::net_processing::send_tx(&peer, &Transaction).await                   <-- full transaction bytes

**What this diagram shows**: A state-changing write path. The API handler delegates to `NodeContext::btc_transaction`, which builds a signed UTXO transaction, accepts it into the global mempool, and then uses a background task to (a) broadcast an INV to peers and (b) optionally trigger mining if the mempool threshold is met.

---

## Step-by-step code walkthrough

Now that we understand the architecture, we walk through the code that implements it. We'll start with global state management, then examine the coordination API, and finally trace the runtime paths for transactions, blocks, and mining.

**Our goal**: Understand the runtime wiring—where global state lives, which entry points receive network messages, and how those messages route into chainstate/mempool/mining.

**Code walkthrough**:
- `bitcoin/src/node/server.rs` — global singletons and server loop
- `bitcoin/src/node/context.rs` — `NodeContext` coordination API
- `bitcoin/src/net/net_processing.rs` — network message routing
- `bitcoin/src/node/txmempool.rs` — mempool operations (referenced)
- `bitcoin/src/node/miner.rs` — mining coordination (referenced)

**Whitepaper anchors**:
- **Section 5** (Network operation loop): nodes continuously receive transactions and blocks while extending the chain
- **Section 8** (SPV idea): motivates inventory-style propagation (INV → GETDATA → BLOCK/TX)

### Roadmap (how the steps connect)

Here's the path we'll follow through the code. Each step leads naturally to the next:

- **Step 1**: Global singletons (`GLOBAL_MEMORY_POOL`, `GLOBAL_NODES`, etc.) → Listing **9-7.1**
- **Step 2**: `NodeContext` initialization and structure → Listing **9-7.2**
- **Step 3**: Transaction processing path (`process_transaction`) → Listing **9-7.3**
- **Step 4**: Block processing path (`add_block`) → Listing **9-7.4**
- **Step 5**: Network message routing (`process_stream`) → Listing **9-7.5**
- **Step 6**: Server runtime loop (wiring it all together) (`run_with_shutdown`) → Listing **9-7.6**

---

### Step 1 — Global singletons: shared state across subsystems

**Node runtime globals code**: `bitcoin/src/node/server.rs`

The node uses several process-wide singletons that are shared across all subsystems. These globals provide a single source of truth for shared state like the mempool, peer list, and blocks in transit.

**Why globals?** In Rust, global state is typically avoided in favor of dependency injection. However, blockchain nodes have legitimate shared state that multiple subsystems need to access concurrently. Using `Lazy` static variables with thread-safe types allows safe concurrent access without passing references through every function call. This is a pragmatic choice that matches Bitcoin Core's architecture.

**Rust concepts we'll see**:
- **`Lazy`** (from `once_cell::sync::Lazy`): Initializes a static variable lazily (on first access) and ensures thread-safe initialization. This is safer than regular `static mut` because it guarantees single initialization.
- **`static`**: Process-wide storage that lives for the entire program lifetime. All threads can access it.
- **`const`**: Compile-time constants that are inlined wherever used.

### Listing 9-7.1: Process-wide globals (node identity, peer set, mempool, blocks-in-transit)

> **Source:** `server.rs` — Source

```rust
pub const NODE_VERSION: usize = 1;
pub const TRANSACTION_THRESHOLD: usize = 3;

// Central/bootstrap node address (env var or default to localhost:2001)
pub static CENTERAL_NODE: Lazy<SocketAddr> = Lazy::new(|| {
    let addr_str = env::var("CENTERAL_NODE")
        .unwrap_or_else(|_| "127.0.0.1:2001".to_string());
    // ... (parse with fallback to default)
    addr_str.parse().expect("Invalid CENTERAL_NODE address")
});
```

Next, we set up the global node list and mempool. The central node is registered as the initial peer, and the mempool tracks pending transactions waiting to be mined.

```rust
pub static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();
    nodes.add_node(*CENTERAL_NODE).expect("Node add error");
    nodes
});

pub static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> =
    Lazy::new(MemoryPool::new);
pub static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> =
    Lazy::new(BlockInTransit::new);
```

### Listing 9-7.1 explanation:

- **`NODE_VERSION`**: Protocol version used in the
  version handshake when nodes first connect.
  This allows nodes to detect incompatible
  protocol versions.

- **`CENTERAL_NODE`**: The bootstrap node address. New nodes connect to this node first to discover other peers. Defaults to `127.0.0.1:2001` if not set via environment variable.

- **`TRANSACTION_THRESHOLD`**: Mining trigger threshold. When the mempool reaches this many transactions, the node will attempt to mine a new block. This is a simplified mining trigger—Bitcoin uses more sophisticated logic based on block time and difficulty.

- **`GLOBAL_NODES`**: The peer list. Initialized with the central node, then expanded as peers connect and exchange known nodes. Used by both network sync and mining broadcast operations.

- **`GLOBAL_MEMORY_POOL`**: The transaction mempool. This is the single source of truth for pending transactions. All subsystems (network, mining, web API) read from and write to this global pool.

- **`GLOBAL_BLOCKS_IN_TRANSIT`**: Download queue for blocks. When a node receives an `INV` message announcing blocks it doesn't have, it adds them here and requests them one by one via `GETDATA`. This prevents requesting the same block multiple times.

**Whitepaper mapping**:
- **§5**: Nodes keep a pool of unconfirmed transactions (`GLOBAL_MEMORY_POOL`) and communicate with peers (`GLOBAL_NODES`).
- **§8**: The SPV discussion motivates inventory-style announcements—nodes announce what they have (`INV`) before transmitting full data, which is why we need `GLOBAL_BLOCKS_IN_TRANSIT` to track what we've requested.

**Checkpoint (we should be able to explain)**:
- What global singletons exist and why they're process-wide (shared across all subsystems)
- The purpose of each global: `GLOBAL_MEMORY_POOL`, `GLOBAL_NODES`, `GLOBAL_BLOCKS_IN_TRANSIT`, `CENTERAL_NODE`
- How `TRANSACTION_THRESHOLD` triggers mining
- Why globals are accessed directly by subsystems rather than through `NodeContext`

**Diagram — globals and their consumers**

GLOBAL_MEMORY_POOL
  ├─> txmempool::add_to_memory_pool (add pending transactions)
  ├─> txmempool::remove_from_memory_pool (remove confirmed transactions)
  ├─> miner::prepare_mining_utxo (select transactions for mining)
  ├─> NodeContext::process_transaction (reject duplicates)
  └─> net_processing::process_stream (serve tx if requested)

GLOBAL_NODES
  ├─> net_processing::send_inv (broadcast INV to peers)
  ├─> net_processing::send_block (broadcast BLOCK to peers)
  ├─> miner broadcast (announce new block via INV)
  └─> NodeContext::get_peers (query peer list)

GLOBAL_BLOCKS_IN_TRANSIT
  ├─> net_processing::process_stream (enqueue unknown block hash on INV)
  ├─> net_processing::process_stream (dequeue when BLOCK received)
  └─> net_processing::process_stream (request next queued block via GETDATA)

CENTERAL_NODE
  ├─> Server::run_with_shutdown (bootstrap connection)
  ├─> net_processing::send_version (version handshake)
  └─> GLOBAL_NODES initialization (add to peer list)

---

### Step 2 — NodeContext: the central coordination API

**NodeContext code**: `bitcoin/src/node/context.rs`

`NodeContext` is the primary entry point for all node operations. It provides a clean, unified API that abstracts the complexity of coordinating multiple subsystems. Following Bitcoin Core's architecture, it serves a similar role to Bitcoin's validation context and node state manager.

### Listing 9-7.2: NodeContext structure and initialization

> **Source:** `context.rs` — Source

```rust
/// Central coordination point for all node operations (facade pattern)
#[derive(Clone, Debug)]
pub struct NodeContext {
    blockchain: BlockchainService,
}

impl NodeContext {
    pub fn new(blockchain: BlockchainService) -> Self {
        Self { blockchain }
    }

    pub fn get_blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }

    pub fn blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }
}
```

### Listing 9-7.2 explanation:

- **`NodeContext` struct**: A thin wrapper around `BlockchainService`. The coordination happens through methods on `NodeContext` that delegate to subsystems (mempool, miner, network) rather than storing all subsystems as fields.

- **`new`**: Creates a new `NodeContext` with a `BlockchainService`. The `BlockchainService` manages all persistent state (blocks, UTXO set) via the storage layer.

- **`get_blockchain` / `blockchain`**: Accessors for the underlying blockchain service. These are provided for cases where direct blockchain access is needed, but most operations should go through `NodeContext` methods.

**Design pattern**: `NodeContext` follows the **facade pattern**—it provides a simplified interface to a complex subsystem. External code doesn't need to know about `GLOBAL_MEMORY_POOL`, miner functions, or network helpers; it just calls `NodeContext` methods.

**Rust concepts explained**:
- **`#[derive(Clone, Debug)]`**: Derive macros automatically implement traits. `Clone` allows copying the struct (cheap because it only clones the `BlockchainService` reference, not the data). `Debug` enables formatting with `{:?}` for logging.
- **`Send + Sync`**: These marker traits indicate thread safety:
  - **`Send`**: The type can be safely transferred between threads (moved)
  - **`Sync`**: The type can be safely shared between threads (via `&T`)
  - `NodeContext` is `Send + Sync` because `BlockchainService` uses internal synchronization (`Arc`, `RwLock`, etc.)
- **Why `Clone`?**: In async Rust, we often need to move values into closures. `Clone` allows creating multiple handles to the same `NodeContext` for different async tasks without borrowing issues.

**Checkpoint (we should be able to explain)**:
- What `NodeContext` is and why it exists (central coordination point)
- How `NodeContext` relates to `BlockchainService` (thin wrapper that delegates)
- Why `NodeContext` is `Clone` + `Send` + `Sync` (safe sharing across async tasks)
- The facade pattern: how `NodeContext` simplifies the API for external code
- Why `Clone` is needed for async Rust (moving values into closures)

---

### Step 3 — Transaction processing path: from network to mempool

**Transaction processing code**: `bitcoin/src/node/context.rs`, `bitcoin/src/net/net_processing.rs`

When a transaction arrives (either from the network or via the web API), it flows through `NodeContext::process_transaction`, which handles mempool admission, network broadcast, and mining triggers.

### Listing 9-7.3: Transaction processing (`process_transaction` and `submit_transaction_for_mining`)

> **Source:** `context.rs` — Source

```rust
pub async fn process_transaction(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<String> {
    if transaction_exists_in_pool(&utxo) {
        return Err(
            BtcError::TransactionAlreadyExistsInMemoryPool(
                utxo.get_tx_id_hex()
            )
        );
    }
    add_to_memory_pool(utxo.clone(), &self.blockchain).await?;
    let context = self.clone();
    let addr_copy = *addr_from;
    let tx = utxo.clone();
    tokio::spawn(async move {
        let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
    });
    Ok(utxo.get_tx_id_hex())
}
```

After accepting the transaction into the mempool, we spawn a background task to handle broadcast and mining. The next function manages this asynchronous work, broadcasting the transaction to peers and triggering mining if we have enough transactions.

```rust
async fn submit_transaction_for_mining(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<()> {
    // Broadcast tx to peers if this is central node
    if GLOBAL_CONFIG.get_node_addr().eq(&CENTERAL_NODE) {
        let nodes = self.get_nodes_excluding_sender(addr_from).await?;
        self.broadcast_transaction_to_nodes(&nodes, utxo.get_id_bytes()).await;
    }
```

After broadcasting the transaction to peers, we check if we should trigger mining. If the mempool now has enough transactions, we prepare them and start mining a new block.

```rust
    if should_trigger_mining() {
        if let Some(mining_address) = GLOBAL_CONFIG.get_mining_addr() {
            if let Ok(txs) = prepare_mining_utxo(&mining_address) {
                if !txs.is_empty() {
                    process_mine_block(txs, &self.blockchain).await?;
                }
            } else {
                cleanup_invalid_transactions().await;
            }
        }
    }
    Ok(())
}
```

### Listing 9-7.3 explanation:

- **`process_transaction`**: The main entry point for transaction processing. It performs duplicate checking, adds the transaction to the mempool, and spawns background work for broadcast and mining. Returns immediately with the transaction ID, following Bitcoin's pattern of immediate acceptance.

- **Duplicate check**: Before adding to mempool, we check if the transaction already exists. This prevents processing the same transaction multiple times and avoids network loops. The check uses `transaction_exists_in_pool`, which queries `GLOBAL_MEMORY_POOL`.

- **Mempool admission**: `add_to_memory_pool` stores the transaction and marks its referenced outputs as "in mempool" in the UTXO set. This prevents local double-spending—if another transaction tries to spend the same output, it will be rejected. The function is `async` because it may need to read from the blockchain to validate outputs.

- **Background work**: Broadcast and mining are spawned as background tasks using `tokio::spawn`. This ensures the API response is immediate—the caller doesn't wait for network propagation or mining to complete. The spawned task runs independently and errors are logged but don't affect the response.

- **Rust async/await**: The `async` keyword makes a function return a `Future`. The `await` keyword suspends execution until the future completes. `tokio::spawn` runs the future concurrently with other tasks.

- **`submit_transaction_for_mining`**: Handles the background work. If this node is the central node, it broadcasts the transaction to all peers (except the sender). If the mining threshold is met, it triggers mining. This function is `async` because network operations and mining are async.

- **Mining trigger**: `should_trigger_mining` checks if we're a miner and if the mempool has enough transactions (`TRANSACTION_THRESHOLD`). If so, it prepares candidate transactions and mines a block. The mining process itself is async and may take time (proof-of-work).

**Network message routing (P2P socket path)**: Nodes communicate by transmiting transactions and blocks over a **TCP peer connection** (the node's P2P socket). The server accept loop (`Server::run_with_shutdown`) hands each inbound stream to `net_processing::process_stream`, which decodes a `Package::Tx { ... }` and routes it into `NodeContext::process_transaction`.

**Type definition (what `Package::Tx` means)**: In this codebase, the P2P protocol is modeled as a single enum: `Package`. Each `Package` variant represents a **distinct network message type** that peers can send to each other (e.g., inventory announcements, data requests, full transactions, full blocks). `process_stream` deserializes incoming bytes into this enum and then dispatches behavior by matching on the variant. `OpType` is a small discriminator used by messages like `Inv` and `GetData` to indicate whether the referenced IDs are **transaction IDs** or **block hashes**.

```rust
// Source: bitcoin/src/node/server.rs (excerpt)
#[derive(Debug, Serialize, Deserialize)]
pub enum OpType {
    Tx,
    Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Package {
    Inv {
        addr_from: SocketAddr,
        op_type: OpType,
        items: Vec<Vec<u8>>,
    },
    // Data request: "Send me the full bytes for this id"
    GetData {
        addr_from: SocketAddr,
        op_type: OpType,
        id: Vec<u8>,
    },
    // Full transaction bytes
    Tx {
        addr_from: SocketAddr,
        transaction: Vec<u8>,
    },
    // Full block bytes
    Block {
        addr_from: SocketAddr,
        block: Vec<u8>,
    },
    // ... other variants omitted ...
}
```

The server dispatches each incoming `Package` by pattern-matching its variant. The `process_stream` function deserializes the TCP byte stream into a `Package` and routes it to the appropriate handler:

```rust
// Source: bitcoin/src/net/net_processing.rs (excerpt)

pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    ...

    for pkg in pkg_reader {
        let pkg = pkg?;

        match pkg {
            ...

            Package::Tx {
                addr_from,
                transaction,
            } => {
                // Deserialize the transaction bytes
                let tx = Transaction::deserialize(transaction.as_slice())
                    .expect("Transaction deserialization error");

                // Route to NodeContext for processing
                match node_context.process_transaction(&addr_from, tx).await {
                    Ok(_) => (),
                    Err(
                        BtcError::TransactionAlreadyExistsInMemoryPool(
                            txid
                        )
                    ) => {
                        // Transaction already in mempool—normal
                        // log it
                    }
                    Err(e) => Err(e)?,
                }
            }

            ...
        }
    ...
}
```

**Diagram — transaction processing flow**

Transaction arrives (network or API)
  │
  ├─> Network: Package::Tx → process_stream
  │     └─> Deserialize transaction bytes
  │
  └─> API: HTTP POST /api/transaction
        └─> Web handler
              │
              └─> NodeContext::process_transaction
                    │
                    ├─> Step 1: Check duplicate (transaction_exists_in_pool)
                    │     └─> If duplicate: return error
                    │
                    ├─> Step 2: Add to mempool (add_to_memory_pool)
                    │     ├─> Store in GLOBAL_MEMORY_POOL
                    │     └─> Mark outputs as "in mempool" in UTXO set
                    │
                    ├─> Step 3: Spawn background work (tokio::spawn)
                    │     └─> submit_transaction_for_mining
                    │           │
                    │           ├─> If central node: broadcast to peers (send_inv)
                    │           │     └─> Send INV(OpType::Tx, txid) to all peers
                    │           │
                    │           └─> If mining threshold met: trigger mining
                    │                 ├─> prepare_mining_utxo (select transactions)
                    │                 └─> process_mine_block (mine block)
                    │
                    └─> Step 4: Return txid immediately

**Whitepaper mapping**:
- **§5 step 1**: "new transactions are broadcast" — this is the code path that receives and admits them. The background broadcast follows Bitcoin's pattern of immediate acceptance with async propagation.

**Checkpoint (we should be able to explain)**:
- The complete flow of `process_transaction`: duplicate check → mempool admission → background work → immediate return
- Why broadcast and mining are done asynchronously (don't block the API response)
- How `submit_transaction_for_mining` coordinates broadcast (central node only) and mining triggers
- The difference between immediate acceptance (mempool) and async propagation (network/mining)
- How network messages (`Package::Tx`) route to `process_transaction`

---

### Step 4 — Block processing path: from network to chainstate

**Block processing code**: `bitcoin/src/node/context.rs`, `bitcoin/src/net/net_processing.rs`

When a block arrives from the network, it flows through `NodeContext::add_block`, which validates and adds it to the chain, then cleans up the mempool.

### Listing 9-7.4: Block processing (`add_block` and network routing)

> **Source:** `context.rs` — Source

```rust
/// Add a block to the blockchain
///
/// Adds a validated block to the blockchain and updates the chain state.
///
/// # Arguments
///
///
/// # Returns
///
/// * `Ok(())` - Block successfully added
/// * `Err(_)` - Block validation failed or storage error
pub async fn add_block(&self, block: &Block) -> Result<()> {
    // Block Addition: Delegate to BlockchainService
    // NodeContext is a thin wrapper - it delegates actual blockchain operations
    //
    // BlockchainService::add_block performs:
    // 1. **Block validation**: Check height, previous hash, proof-of-work, etc.
    // 3. **Persistence**: Write block to sled database (persistent storage)
    // outputs
    //
    // `&block`: Pass reference (don't take ownership, caller might need it)
    // `.await`: Wait for async operation to complete
    // Returns Result<()> - Ok(()) on success, Err on validation/storage failure
    self.blockchain.add_block(block).await
}
```

**Network message routing**: When a block arrives via the network, it's handled in `net_processing::process_stream`:

```rust
// Source: bitcoin/src/net/net_processing.rs (excerpt)

Package::Block { addr_from, block } => {
    // Deserialize the block bytes
    let block = Block::deserialize(block.as_slice())
        .expect("Block deserialization error");

    // Add block to blockchain
    // chain,
    // and updates the UTXO set
    node_context
        .add_block(&block)
        .await
        .expect("Blockchain write error");

    let added_block_hash = block.get_hash_bytes();

    // Remove transactions in block from memory pool
    for tx in block.get_transactions().await? {
        node_context.remove_from_memory_pool(tx.clone()).await;
    }

    // Remove block from blocks-in-transit queue
    let removed_block_hash = GLOBAL_BLOCKS_IN_TRANSIT
        .remove(added_block_hash.as_ref())
        .expect("Block removal error");

    if GLOBAL_BLOCKS_IN_TRANSIT
        .is_not_empty()
        .expect("Blocks in transit error")
    {
        let block_hash = GLOBAL_BLOCKS_IN_TRANSIT
            .first()
            .expect("Blocks in transit error")
            .expect("Blocks in transit error");
        send_get_data(&addr_from, OpType::Block, &block_hash).await;
    }
}
```

### Listing 9-7.4 explanation:

- **`add_block`**: Delegates to `BlockchainService::add_block`, which handles block validation, chain reorganization, persistence, and UTXO set updates. This is a thin wrapper that provides a clean API.

- **Block deserialization**: Network messages contain serialized block bytes. We deserialize them before processing.

- **Mempool cleanup**: After adding a block, we remove its transactions from the mempool. These transactions are now confirmed, so they shouldn't be in the pending set.

- **Blocks-in-transit management**: When we receive a block we requested, we remove it from `GLOBAL_BLOCKS_IN_TRANSIT`. If there are more blocks in the queue, we request the next one. This implements sequential block download during initial sync.

**Diagram — block processing flow**

Block arrives (network)
  │
  └─> Network: Package::Block → process_stream
        │
        ├─> Deserialize block bytes
        │
        ├─> NodeContext::add_block
        │     │
        │     └─> BlockchainService::add_block
        │           │
        │           ├─> Validate block (height, previous hash, etc.)
        │           │
        │           ├─> Check if extends best chain
        │           │     │
        │           │     ├─> If yes: update chain tip
        │           │     │     ├─> Persist block to storage
        │           │     │     └─> Update UTXO set
        │           │     │
        │           │     └─> If no: store block but don't update tip
        │           │           (block is part of a side chain)
        │           │
        │           └─> Return success
        │
        ├─> Remove transactions from mempool
        │     └─> For each tx in block:
        │           └─> remove_from_memory_pool(tx)
        │                 └─> Remove from GLOBAL_MEMORY_POOL
        │
        ├─> Remove block from blocks-in-transit queue
        │     └─> GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash)
        │
        └─> Request next block (if queue not empty)
              └─> send_get_data(OpType::Block, next_block_hash)

**Whitepaper mapping**:
- **§5 steps 4–6**: Broadcast blocks; accept if valid; continue building on the accepted chain. The block processing path implements this—we validate blocks, update the chain tip, and continue requesting more blocks if needed.

**Checkpoint (we should be able to explain)**:
- How `add_block` delegates to `BlockchainService::add_block` (validation, chain reorganization, persistence)
- Why mempool cleanup happens after block acceptance (remove confirmed transactions)
- How `GLOBAL_BLOCKS_IN_TRANSIT` manages sequential block download during initial sync
- The flow: receive block → add to chain → clean mempool → request next block (if queue not empty)
- How network messages (`Package::Block`) route to `add_block`

---

### Step 5 — Network message routing: the message dispatcher

**Network processing code**: `bitcoin/src/net/net_processing.rs`

The network layer receives serialized messages from peers over **TCP sockets** and routes them to the appropriate handlers. This is the "message dispatcher" that connects P2P network events to node operations. It also handles **outbound P2P propagation**: when we broadcast transactions/blocks to other nodes (INV/GETDATA/TX/BLOCK), that happens via `net_processing::send_inv`, `net_processing::send_get_data`, `net_processing::send_tx`, and `net_processing::send_block` — **not** through the web (HTTP) layer.

### Listing 9-7.5: Network message routing (`process_stream`)

> **Source:** `net_processing.rs` — Source

```rust
pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(&stream);
    let pkg_reader = Deserializer::from_reader(reader).into_iter::<Package>();
    for pkg in pkg_reader {
        let pkg = pkg?;
        match pkg {
```

Now we enter the main message processing loop. The following sections handle different message types, starting with full block responses from peers.

```rust
            // Message Type: Block (Full block data)
            // full block
            // Pattern matching: `Package::Block { addr_from, block }` extracts
            // fields
            Package::Block { addr_from, block } => {
                // Step 1: Deserialize block bytes into Block struct
                // Network sends serialized bytes, we need the Block struct
                let block = Block::deserialize(block.as_slice())
                    .expect("Block deserialization error");

                // Step 2: Add block to blockchain
                node_context
                    .add_block(&block)
                    .await
                    .expect("Blockchain write error");

                // Step 3: Clean up mempool
                // Transactions in this block are now confirmed - remove from
                // pending set
                for tx in block.get_transactions().await? {
                    node_context.remove_from_memory_pool(tx.clone()).await;
                }

                // Step 4: Update download queue
                // Remove this block from blocks-in-transit (we've received it)
                let added_block_hash = block.get_hash_bytes();
                GLOBAL_BLOCKS_IN_TRANSIT
                    .remove(added_block_hash.as_ref())
                    .expect("Block removal error");

                // Step 5: Continue sequential download
                if GLOBAL_BLOCKS_IN_TRANSIT
                    .is_not_empty()
                    .expect("Blocks in transit error")
                {
                    let block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                        .first()
                        .expect("Blocks in transit error")
                        .expect("Blocks in transit error");
                    send_get_data(&addr_from, OpType::Block, &block_hash).await;
                }
            }
```

When a full block is received, we validate it, update the blockchain, and then request the next block if there are more in our download queue. The following section handles incoming transactions that we requested from peers.

```rust
            // Message Type: Transaction (Full transaction data)
            // full transaction
            Package::Tx {
                addr_from,
                transaction,
            } => {
                // Step 1: Deserialize transaction bytes into Transaction struct
                // `transaction` is Vec<u8> (serialized bytes), we need
                // Transaction struct
                let tx = Transaction::deserialize(transaction.as_slice())
                    .expect("Transaction deserialization error");

                // Step 2: Process the transaction
                // triggers mining
                // gracefully
                match node_context.process_transaction(&addr_from, tx).await {
                    Ok(_) => {
                        // Success: transaction added to mempool
                        // No action needed, processing continues
                    }
                    Err(
                        BtcError::TransactionAlreadyExistsInMemoryPool(
                            txid
                        )
                    ) => {
                        // Duplicate transaction - normal
                        // error
                        // Can happen if:
                        // Just log it for debugging, don't treat as error
                    }
                    Err(e) => {
                        // Other errors (validation failures, etc.) - propagate
                        // up
                        // potentially close
                        Err(e)?
                    }
                }
            }
```

After processing a full transaction, we add it to our mempool or log if it's a duplicate. Next, we handle inventory announcements where peers inform us of blocks and transactions they have available.

```rust
            // Peer is telling us "I have these blocks/transactions" (by ID
            // only)
            // GETDATA → BLOCK/TX)
            Package::Inv {
                addr_from,
                op_type,  // OpType::Block or OpType::Tx
                // Vec<Vec<u8>> - list of block/tx hashes (32 bytes each)
                items,
            } => match op_type {
                // Peer announced blocks they have - we need to download them
                OpType::Block => {
                    // Step 1: Add all announced blocks to download queue
                    // `items` is a list of block hashes (32-byte Vec<u8> each)
                    // sequential download
                    GLOBAL_BLOCKS_IN_TRANSIT
                        // Add all block hashes to queue
                        .add_blocks(items.as_slice())
                        .expect("Blocks in transit add error");

                    // Step 2: Request the first block
                    let block_hash = items
                        .first()
                        .expect("Blocks in transit add error");
                    send_get_data(
                        &addr_from,
                        OpType::Block,
                        block_hash
                    ).await;
                }

                // Peer announced transactions they have - check if we need them
                OpType::Tx => {
                    // Get the first transaction ID
                    let txid = items
                        .first()
                        .expect("Blocks in transit add error");

                    // Convert transaction ID from bytes to hex
                    // GLOBAL_MEMORY_POOL uses hex strings as keys
                    let txid_hex = HEXLOWER.encode(txid);

                    // Step: Only request if we don't already have it
                    // Check GLOBAL_MEMORY_POOL - if we have it, no need to
                    // request
                    if !GLOBAL_MEMORY_POOL
                        // Check if txid exists in mempool
                        .contains(txid_hex.as_str())
                        .expect("Memory pool contains error")
                    {
                        send_get_data(&addr_from, OpType::Tx, txid).await;
                    }
                    // processed)
                }
            },
```

After handling inventory announcements, we request the full data for items we need. Next, we handle peer requests for full block and transaction data that we have in our storage or mempool.

```rust
            // Message Type: GetData - Request for full object data
            // Peer is requesting a specific block or transaction by ID
            // GETDATA → BLOCK/TX)
            Package::GetData {
                addr_from,  // Who is requesting
                op_type,    // OpType::Block or OpType::Tx
                // Vec<u8> - the hash/ID of the requested object (32 bytes)
                id,
            } => match op_type {
                // GetData Type: Block
                // Peer wants a specific block - look it up in our blockchain
                OpType::Block => {
                    // Look up block by hash in our blockchain storage
                    // `id.as_slice()`: Convert Vec<u8> to &[u8] for lookup
                    // `.await`: Async database read
                    // Returns Option<Block> - Some if found, None if not
                    if let Some(block) = node_context
                        .get_block(id.as_slice())
                        .await
                        .expect("Blockchain read error")
                    {
                        // `send_block` serializes the block and sends
                        // Package::Block
                        send_block(&addr_from, &block).await;
                    }
                    // we don't have)
                }

                // GetData Type: Transaction
                // Peer wants a specific transaction - look it up in our mempool
                OpType::Tx => {
                    // Convert transaction ID from bytes to hex string
                    // GLOBAL_MEMORY_POOL uses hex strings as keys
                    let txid_hex = HEXLOWER.encode(id.as_slice());

                    // Look up transaction in mempool
                    // Returns Option<Transaction> - Some if found, None if not
                    if let Some(tx) = GLOBAL_MEMORY_POOL
                        .get(txid_hex.as_str())  // Lookup by hex string key
                        .expect("Memory pool get error")
                    {
                        // `send_tx` serializes the transaction and sends
                        // Package::Tx
                        send_tx(&addr_from, &tx).await;
                    } else {
                        // Transaction not in mempool - likely already mined
                        // sent INV,
                        // but got mined before peer requested it
                    }
                }
            },
```

After responding to data requests, we handle peers requesting a list of all our block hashes for initial synchronization.

```rust
            // Message Type: GetBlocks - Request for all block hashes (initial
            // sync)
            // Peer is doing initial sync - they want to know what blocks we
            // have
            Package::GetBlocks { addr_from } => {
                // Step 1: Get all block hashes from our blockchain
                // Returns Vec<Vec<u8>> - list of block hashes (each is 32
                // bytes)
                // Ordered from genesis to tip (chronological order)
                let blocks = node_context
                    .get_block_hashes()
                    .await
                    .expect("Blockchain read error");

                // Step 2: Send inventory message with all block hashes
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
```

Finally, we handle the remaining protocol messages: version negotiation and peer list exchange, which are typically handled during the initial handshake but may be received again.

```rust
            // Message Type: Version - Protocol version handshake
            // Sent when nodes first connect to negotiate protocol version
            // Handled during connection setup (before process_stream)
            // processed)
            Package::Version { .. } => {
                // Version handshake is handled during connection setup
                // here
            }

            // Message Type: KnownNodes - Peer list exchange
            // Sent to help nodes discover more peers in the network
            // Handled during connection setup
            // processed)
            Package::KnownNodes { .. } => {
                // Known nodes exchange is handled during connection setup
                // here
            }
        }
    }

    Ok(())
}
```

### Listing 9-7.5 explanation:

- **`process_stream`**: The main network message dispatcher. It reads messages from a TCP stream, deserializes them, and routes them to appropriate handlers based on message type.

- **Message deserialization**: Messages are serialized as JSON and read from the stream using a `Deserializer`. Each message is a `Package` enum variant.

- **Block handling**: When we receive a full block (`Package::Block`), we add it to the blockchain, remove its transactions from the mempool, and request the next block if there are more in the download queue.

- **Transaction handling**: When we receive a full transaction (`Package::Tx`), we process it through `NodeContext::process_transaction`, which adds it to the mempool and triggers broadcast/mining.

- **Inventory handling**: When we receive an inventory (`Package::Inv`), we add announced blocks to the download queue and request them, or request transactions we don't have.

- **GetData handling**: When a peer requests data (`Package::GetData`), we look up the block or transaction and send it.

- **GetBlocks handling**: When a peer requests all block hashes (`Package::GetBlocks`), we send an inventory message with all our block hashes. This is used during initial sync.

**Diagram — network message routing**

TCP Stream (peer connection)
  │
  └─> process_stream (deserialize messages)
        │
        ├─> Package::Block
        │     └─> add_block → remove from mempool → request next
        │
        ├─> Package::Tx
        │     └─> process_transaction → mempool → broadcast/mining
        │
        ├─> Package::Inv
        │     ├─> OpType::Block → add to queue → request first
        │     └─> OpType::Tx → check mempool → request if missing
        │
        ├─> Package::GetData
        │     ├─> OpType::Block → lookup → send_block
        │     └─> OpType::Tx → lookup mempool → send_tx
        │
        ├─> Package::GetBlocks
        │     └─> get_block_hashes → send_inv
        │
        └─> Package::Version / KnownNodes
              └─> (handled during connection setup)

**Whitepaper mapping**:
- **§5**: Network operation requires propagation of transactions and blocks. The message routing implements this—nodes exchange inventory, request data, and transmit full blocks/transactions.
- **§8**: Inventory-style propagation (INV → GETDATA → BLOCK/TX) is motivated by SPV thinking—nodes announce what they have before transmitting full data, reducing bandwidth.

**Checkpoint (we should be able to explain)**:
- How `process_stream` deserializes messages and routes them based on `Package` enum variants
- The purpose of each message type: `Block`, `Tx`, `Inv`, `GetData`, `GetBlocks`
- How inventory-style propagation works (INV announces IDs → GETDATA requests bytes → BLOCK/TX delivers data)
- How `GLOBAL_BLOCKS_IN_TRANSIT` is used for sequential block download
- How `GLOBAL_MEMORY_POOL` is checked before requesting transactions (avoid duplicate requests)

---

### Step 6 — Server runtime loop: accepting connections and spawning handlers

**Server code**: `bitcoin/src/node/server.rs`

The server binds a TCP listener and spawns a task per incoming connection. Each connection is handled by `process_stream`, which processes messages until the connection closes.

### Listing 9-7.6: Server runtime loop (`run_with_shutdown`)

> **Source:** `server.rs` — Source

```rust
/// Run the server with graceful shutdown support
///
/// spawns a task per connection to handle network messages.
///
/// # Arguments
///
/// * `connect_nodes` - Set of nodes to connect to on startup
/// * `shutdown` - Broadcast receiver for shutdown signal
///
/// # Process Flow
///
/// 1. **Bind TCP listener** - Start listening on the specified address
/// 3. **Accept loop** - Continuously accept connections and spawn handlers
/// 4. **Shutdown** - Gracefully shutdown on signal
#[instrument(skip(self, addrs, connect_nodes, shutdown))]
pub async fn run_with_shutdown(
    &self,
    addrs: &SocketAddr,
    connect_nodes: HashSet<ConnectNode>,
    mut shutdown: tokio::sync::broadcast::Receiver<()>,
) {
    // Step 1: Bind TCP listener
    // `TcpListener::bind` is async - it returns a Future that resolves when
    // binding completes
    // `.await` suspends execution until the bind operation completes
    // `.expect` panics if binding fails (e.g., port already in use)
    let listener = TcpListener::bind(addrs)
        .await
        .expect("TcpListener bind error");

        "Server listening on {:?}",
        listener.local_addr().expect("TcpListener local_addr error")
    );
```

With the TCP listener bound, we now perform bootstrap connectivity. Non-central nodes announce themselves to the bootstrap node, while central nodes register their configured remote peers.

```rust
    // Step 2: Bootstrap connectivity
    // node
    // to announce ourselves and start the connection handshake
    if !addrs.eq(&CENTERAL_NODE) {
        // Get our current blockchain height for the version message
        let best_height = self
            .node_context
            .get_blockchain_height()
            .await
            .expect("Blockchain read error");

        // Send version message to central node
        send_version(&CENTERAL_NODE, best_height).await;
    } else {

        // Filter to only remote nodes (exclude "local")
        // Rust iterator chain: iter → filter → map → collect
        // - `iter()`: Create iterator over HashSet
        // - `filter`: Keep only remote nodes (exclude "local")
        // - `map`: Transform ConnectNode to SocketAddr
        // - `collect()`: Build new HashSet from iterator
        let remote_nodes: HashSet<SocketAddr> = connect_nodes
            .iter()
            .filter(|node| node.is_remote())  // Keep only remote nodes
            .map(|node| node.get_addr())       // Extract SocketAddr
            .collect();                        // Build HashSet

        // Add remote nodes to global peer list
        GLOBAL_NODES
            .add_nodes(remote_nodes.clone())
            .expect("Global nodes add error");

        // Send known nodes list to each remote node
        for remote_node in remote_nodes {
            send_known_nodes(
                &remote_node,
                GLOBAL_NODES
                    .get_nodes()
                    .expect("Global nodes get error")
                    .iter()
                    .map(|node| node.get_addr())
                    .collect(),
            )
            .await;
        }
    }
```

With the TCP listener bound and bootstrap handshakes complete, we enter the main accept loop that listens for incoming connections and spawns handlers. The loop continues until a shutdown signal is received.

```rust
    // Step 3: Accept loop with graceful shutdown
    // `tokio::select!` is a macro that waits for multiple async operations and
    // executes the branch corresponding to the first one that completes
    // concurrently
    loop {
        tokio::select! {
            // Shutdown signal: exit the loop gracefully
            // `shutdown.recv()` returns a Future that completes when shutdown
            // is signaled
            _ = shutdown.recv() => {
                break;  // Exit the loop
            }

            // `listener.accept()` returns a Future that completes when a
            // connection arrives
            // `accept_res` receives the Result<(TcpStream, SocketAddr), Error>
            accept_res = listener.accept() => {
                match accept_res {
                    Ok((stream, _peer)) => {
                        // Pattern matching: `Ok((stream, _peer))` destructures
                        // the Result
                        // `stream` is the TcpStream, `_peer` is the peer
                        // address (we ignore it)

                        // Clone node context for the handler task
                        // NodeContext is Clone + Send + Sync, so this is safe
                        // task
                        let blockchain = self.node_context.clone();

                        // Spawn Connection Handler Task
                        // concurrently
                        // simultaneously
                        // `tokio::spawn` returns a JoinHandle, but we don't
                        // await it
                        // logs and exits
                        tokio::spawn(async move {
                            // `async move` closure takes ownership of captured
                            // variables
                            // - `blockchain`: Cloned NodeContext (moved into
                            // task)
                            // - `stream`: TcpStream (moved into task)
                            // from the accept loop

                            // std::net::TcpStream (not tokio)
                            // code predates tokio
                            // `into_std()` consumes the tokio stream and
                            // converts it
                            match stream.into_std() {
                                Ok(std_stream) => {
                                    // Set blocking mode (required for BufReader
                                    // in process_stream)
                                    // `set_nonblocking(false)` makes the stream
                                    // blocking
                                    // BufReader expects blocking I/O - it reads
                                    // in chunks
                                    // process_stream will handle it
                                    let _ = std_stream.set_nonblocking(false);
```

With the stream converted to a standard TCP stream in blocking mode, we now pass it to the message processing loop which will handle all messages from this peer until the connection closes.

```rust
                                    // `process_stream` will:
                                    // 1. Read messages from the stream
                                    // (deserialize JSON)
                                    // 2. Route each message to the appropriate
                                    // handler
                                    // 3. Continue until the connection closes
                                    // or errors
                                    //
                                    // - Peer closes connection (normal)
                                    // - Deserialization error (malformed
                                    // message)
                                    // - Network error (connection lost)
                                    //
                                    // Errors are logged but don't crash
                                    // server
                                    if let Err(e) =
                                        net_processing::process_stream(
                                            blockchain,
                                            std_stream
                                        ).await
                                    {
                                        // Log the error for debugging
                                    }
                                    // (connection closed)
                                }
                                Err(e) => {
                                    // Stream conversion failed (rare, but
                                    // handle it)
                                }
                            }
                            // Task exits here (connection handler done)
                        });
                        // accept loop
                    }
                    Err(e) => {
                        // Accept failed (e.g., too many file descriptors,
                        // network error)
                        // Log it but continue the loop - server keeps trying to
                        // accept
                    }
                }
            }
        }
    }
    // Loop exits here when shutdown signal received
    // Server gracefully shuts down
}
```

### Listing 9-7.6 explanation:

- **TCP listener binding**: The server binds a TCP listener on the specified address. This starts listening for incoming connections.

- **Bootstrap logic**: If this node is not the central node, it sends a version message to the central node to announce itself. If it is the central node, it registers the connect nodes and sends them the known nodes list.

- **Accept loop**: The main loop uses `tokio::select!` to handle two events: shutdown signal or incoming connection. This allows graceful shutdown.

- **Per-connection tasks**: Each incoming connection spawns a new task that handles that connection independently. This provides concurrency—the server can handle multiple peers simultaneously.

- **Stream conversion**: Tokio's `TcpStream` is converted to `std::net::TcpStream` because the `net_processing` module uses standard library types. This is a compatibility layer.

- **Message processing**: Each connection task calls `process_stream`, which processes all messages from that peer until the connection closes.

**Diagram — server runtime architecture**

Server Startup
  │
  ├─> Bind TCP listener (listen on address)
  │
  ├─> Bootstrap connectivity
  │     ├─> If not central node: send_version to central node
  │     └─> If central node: register connect_nodes, send_known_nodes
  │
  └─> Enter accept loop
        │
        ├─> tokio::select! (wait for events)
        │     │
        │     ├─> shutdown.recv() → exit loop
        │     │
        │     └─> listener.accept() → spawn handler task
        │           │
        │           └─> tokio::spawn(async move {
        │                 │
        │                 ├─> Convert tokio stream to std stream
        │                 │
        │                 └─> process_stream (handle all messages)
        │                       │
        │                       └─> Loop until connection closes
        │                             │
        │                             ├─> Deserialize message
        │                             │
        │                             └─> Route to handler (Block/Tx/Inv/etc.)
        │
        └─> (loop continues, handling multiple connections concurrently)

**Concurrency model**: The server uses **task-based concurrency**. Each connection is handled by a separate task, allowing the server to process multiple peers simultaneously. This is more efficient than thread-based concurrency for I/O-bound operations like network communication.

**Rust async/await explained**:
- **Tasks**: `tokio::spawn` creates a new task (lightweight thread) that runs concurrently
- **No blocking**: When a task waits for I/O (e.g., `listener.accept().await`), it yields control to other tasks
- **Efficient**: Thousands of tasks can run on a small number of OS threads
- **`tokio::select!`**: Waits for multiple futures and executes the first one that completes
- **Ownership**: The `move` keyword transfers ownership of values into closures/tasks

**Whitepaper mapping**:
- **§5**: Nodes are continuously receiving transactions and blocks while also extending the chain. The async accept loop with per-connection tasks is the practical expression of this continuous operation.

**Checkpoint (we should be able to explain)**:
- How the server loop binds a TCP listener and enters an accept loop
- The bootstrap logic: how non-central nodes connect to the central node, and how the central node registers connect nodes
- Why each connection spawns a separate task (concurrency: handle multiple peers simultaneously)
- How `tokio::select!` enables graceful shutdown (wait for shutdown signal or incoming connection)
- The conversion from tokio streams to std streams (compatibility with `net_processing`)
- How the accept loop coordinates with `process_stream` to handle all messages from a peer

---

## Summary: how it all fits together

Now that we've walked through the code, we summarize how the node orchestration layer coordinates all the subsystems:

1. **Global state**: Process-wide singletons (`GLOBAL_MEMORY_POOL`, `GLOBAL_NODES`, `GLOBAL_BLOCKS_IN_TRANSIT`) provide shared state accessible to all subsystems.

2. **Central coordination**: `NodeContext` serves as the primary API, routing operations to the appropriate subsystems and managing the blockchain service.

3. **Network routing**: `process_stream` deserializes network messages and routes them to handlers based on message type (Block, Tx, Inv, GetData, etc.).

4. **Concurrent handling**: The server loop spawns a task per connection, allowing multiple peers to be handled simultaneously.

5. **Event flow**: External events (network messages, API requests) flow through `NodeContext` methods, which coordinate with subsystems and update global state.

**The complete picture**: External interfaces → `NodeContext` → subsystems → global state → storage layer. This architecture provides clean separation of concerns while maintaining a unified coordination point.

---

## Navigation

- **Previous**: Chapter 16 (Consensus and Validation)
- **Next**: Chapter 18 (Transaction to Block — End-to-End Runtime Walkthrough)

---

<div align="center">

**[← Previous: Consensus and Validation](07-Consensus-and-Validation.md)** | **Node Orchestration** | **[Next: Transaction to Block (End-to-End) →](09-Transaction-To-Block.md)**

</div>
