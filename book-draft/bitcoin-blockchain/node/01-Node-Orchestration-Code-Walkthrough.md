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
22. <a href="README.md">Chapter 22: Node Orchestration</a>
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

# Chapter 22A: Node Orchestration — Code Walkthrough (NodeContext + Mempool + Mining + Peers)

**Part I: Foundations & Core Implementation** | **Chapter 13.A: Node Orchestration — Code Walkthrough**

This chapter is a code-complete walkthrough of `bitcoin/src/node`. The intent is that you can read and understand the node runtime **without having the repository open**.

Two promises:

1. **Complete method coverage**: every function we reference is printed with its full method body in this chapter (or explicitly marked “defined earlier” and linked).
2. **No signature-only snippets**: any snippet that introduces a method is followed immediately by the full listing.

---

## 0. What this module does at runtime

`bitcoin/src/node` is the runtime glue. It is responsible for:

- accepting “events” (network messages, API calls, admin queries)
- routing those events into the correct subsystem
- ensuring expensive work (broadcast/mining) happens in background tasks

### Diagram: responsibilities and the control boundary

```text
Network (Chapter 12)
  - decodes TCP stream into `Package`
  - calls into NodeContext methods
            |
            v
Node orchestration (this chapter)
  - accepts tx into mempool (txmempool)
  - triggers mining (miner)
  - broadcasts inventory to peers (net/send_inv + peers list)
  - forwards blocks into chainstate (BlockchainService)
            |
            v
Chain / storage (Chapters 9–11)
  - validates + connects blocks
  - persists chain state
  - maintains UTXO
```

> **Methods involved:**
>
> - `NodeContext::process_transaction(...)`
> - `NodeContext::add_block(...)`
> - `NodeContext::submit_transaction_for_mining(...)`
> - `txmempool::add_to_memory_pool(...)`
> - `miner::{should_trigger_mining, process_mine_block}`

---

## 1. The façade: `NodeContext` is the coordinator

`NodeContext` is the central façade used by other layers (network server, web/API) to interact with the node.

It owns a `BlockchainService` handle and reaches out to:

- the global mempool (`GLOBAL_MEMORY_POOL`) via `txmempool`
- the peer set (`GLOBAL_NODES`) via `peers`
- the network send primitives (`send_inv`) to announce inventory
- the mining pipeline (`miner`) to build blocks and broadcast new tips

### 1.A Transaction ingestion: accept → relay → maybe mine

This is the path you will trace most often when debugging “why didn’t my transaction propagate / get mined?”.

### Diagram: the orchestration call graph for transactions

```text
Inbound tx bytes
  -> (Network) Package::Tx
  -> NodeContext::process_transaction
      -> txmempool::add_to_memory_pool
      -> spawn background task:
         NodeContext::submit_transaction_for_mining
             -> (if central) broadcast INV(txid) to peers
             -> (if mining threshold) miner::process_mine_block
```

> **Methods involved:**
>
> - `NodeContext::{new, process_transaction}`
> - `NodeContext::submit_transaction_for_mining` (internal)
> - `NodeContext::{btc_transaction, submit_transaction}` (front-door helpers)
> - `txmempool::{add_to_memory_pool, transaction_exists_in_pool}`
> - `send_inv(...)` (**defined earlier** in **[Chapter 12.A: Network Layer — Code Walkthrough](../net/01-Network-Operation-Code-Walkthrough.md)**)

### Listing 13-1.1 — `NodeContext` initialization and transaction acceptance (part 1) (`bitcoin/src/node/context.rs`)

```rust
use crate::chain::{BlockchainService, UTXOSet};
use crate::error::{BtcError, Result};
use crate::node::miner::*;
use crate::node::txmempool::*;
use crate::node::GLOBAL_NODES;
use crate::{Block, Transaction, WalletAddress};
use std::net::SocketAddr;

// Routes requests into mempool/miner/peers/chainstate subsystems.
#[derive(Clone, Debug)]
pub struct NodeContext {
    blockchain: BlockchainService,  // Only owned dependency
}

impl NodeContext {
    pub fn new(blockchain: BlockchainService) -> Self {
        Self { blockchain }
    }

    pub async fn btc_transaction(
        &self,
        wlt_frm_addr: &WalletAddress,
        wlt_to_addr: &WalletAddress,
        amount: i32,
    ) -> Result<String> {
        let utxo_set = UTXOSet::new(self.blockchain.clone());
        let utxo = Transaction::new_utxo_transaction(
            wlt_frm_addr, wlt_to_addr, amount, &utxo_set
        ).await?;
        let addr_from = crate::GLOBAL_CONFIG.get_node_addr();
        self.process_transaction(&addr_from, utxo).await
    }

    pub async fn process_transaction(
        &self,
        addr_from: &SocketAddr,
        utxo: Transaction,
    ) -> Result<String> {
        // 1) Dedupe check
        if transaction_exists_in_pool(&utxo) {
            return Err(BtcError::TransactionAlreadyExistsInMemoryPool(
                utxo.get_tx_id_hex(),
            ));
        }

        // 2) Accept into mempool (flags outputs as “in mempool”)
        add_to_memory_pool(utxo.clone(), &self.blockchain).await?;

        // 3) Fire-and-forget: broadcast + mining in background task
        let context = self.clone();
        let addr_copy = *addr_from;
        let tx = utxo.clone();
        tokio::spawn(async move {
            let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
        });

        Ok(utxo.get_tx_id_hex())
    }
```

After accepting a transaction into the mempool, the node spawns background tasks for relay and mining decisions:

### Listing 13-2 — Background relay and mining submission (part 2)

```rust
    async fn submit_transaction_for_mining(
        &self,
        addr_from: &SocketAddr,
        utxo: Transaction,
    ) -> Result<()> {
        // Background: relay to peers (if central node) and maybe trigger mining
        let my_node_addr = GLOBAL_CONFIG.get_node_addr();
        if my_node_addr == crate::node::CENTERAL_NODE {
            let nodes = self.get_nodes_excluding_sender(addr_from).await?;
            for node in &nodes {
                send_inv(
                    &node.get_addr(),
                    OpType::Tx,
                    &[utxo.get_id_bytes()],
                )
                .await;
            }
        }

        // Mining: trigger if mempool reaches threshold
        if should_trigger_mining() {
            if let Some(addr) = GLOBAL_CONFIG.get_mining_addr() {
                if let Ok(txs) = prepare_mining_utxo(&addr) {
                    if !txs.is_empty() {
                        process_mine_block(txs, &self.blockchain).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn get_nodes_excluding_sender(
        &self,
        addr_from: &SocketAddr,
    ) -> Result<Vec<Node>> {
        let nodes = GLOBAL_NODES
            .get_nodes()?
            .into_iter()
            .filter(|node| {
                let node_addr = node.get_addr();
                node_addr != *addr_from
                    && node_addr != GLOBAL_CONFIG.get_node_addr()
            })
            .collect();
        Ok(nodes)
    }

    pub async fn remove_from_memory_pool(&self, tx: Transaction) {
        remove_from_memory_pool(tx, &self.blockchain).await;
    }
}
```

---

### 1.B Block ingestion + chain/peer queries (node-facing API surface)

This is the subset of `NodeContext` that the network layer and UI/API use for:

- pushing blocks into chainstate (`add_block`)
- answering sync queries (height, hashes, block lookup)
- basic peer introspection (who are we connected to?)

> **Methods involved:**
>
> - `NodeContext::{get_blockchain, blockchain}` (chain handle access)
> - `NodeContext::{add_block, get_blockchain_height, get_block_hashes, get_block}`
> - `NodeContext::{get_peers, get_peer_count}`
> - `NodeContext::mine_empty_block(...)` (admin/testing convenience)
>
> The transaction pipeline methods are defined earlier in **Listing 13-1.1**.

### Listing 13-3 — `NodeContext` (block and chain queries) (`bitcoin/src/node/context.rs`)

```rust
use crate::chain::BlockchainService;
use crate::error::Result;
use crate::node::miner;
use crate::{Block, WalletAddress};
use std::net::SocketAddr;

impl NodeContext {
    pub fn get_blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }

    pub fn blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }

    pub async fn add_block(&self, block: &Block) -> Result<()> {
        self.blockchain.add_block(block).await
    }

    pub async fn get_blockchain_height(&self) -> Result<usize> {
        self.blockchain.get_best_height().await
    }

    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        self.blockchain.get_block_hashes().await
    }

    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.blockchain.get_block(block_hash).await
    }
    // ... (continues with mining and peer methods)
}
```

Additional methods on `NodeContext` provide mining and peer discovery functionality:

```rust
    pub async fn mine_empty_block(
        &self,
        wallet_address: &WalletAddress,
    ) -> Result<Block> {
        // Mining convenience for testing
        miner::mine_empty_block(&self.blockchain, wallet_address).await
    }

    pub fn get_peer_count(&self) -> Result<usize> {
        use crate::node::GLOBAL_NODES;
        let nodes = GLOBAL_NODES.get_nodes()?;
        Ok(nodes.len())
    }

    pub fn get_peers(&self) -> Result<Vec<SocketAddr>> {
        use crate::node::GLOBAL_NODES;
        let nodes = GLOBAL_NODES.get_nodes()?;
        Ok(nodes.into_iter().map(|n| n.get_addr()).collect())
    }
}
```

---

## 2. The mempool primitives (`txmempool.rs`)

The mempool module is deliberately small: it is a thin wrapper around `GLOBAL_MEMORY_POOL` plus a UTXO-side flag update so outputs can be marked “in mempool”.

> **Methods involved:**
>
> - `add_to_memory_pool(...)`
> - `remove_from_memory_pool(...)`
> - `transaction_exists_in_pool(...)`

### Listing 13-6 — Mempool helpers (`bitcoin/src/node/txmempool.rs`)

```rust
use crate::error::Result;
use crate::node::GLOBAL_MEMORY_POOL;
use crate::{BlockchainService, Transaction, UTXOSet};

pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain_service: &BlockchainService,
) -> Result<()> {
    // Store in pool + mark outputs “in mempool” to prevent double-spend
    GLOBAL_MEMORY_POOL.add(tx.clone())?;

    let utxo_set = UTXOSet::new(blockchain_service.clone());
    utxo_set.set_global_mem_pool_flag(&tx, true).await?;

    Ok(())
}

pub async fn remove_from_memory_pool(
    tx: Transaction,
    blockchain: &BlockchainService,
) {
    // Remove from pool + clear “in mempool” flags
    let _ = GLOBAL_MEMORY_POOL.remove(tx.clone());
    let utxo_set = UTXOSet::new(blockchain.clone());
    let _ = utxo_set.set_global_mem_pool_flag(&tx, false).await;
}

pub fn transaction_exists_in_pool(tx: &Transaction) -> bool {
    GLOBAL_MEMORY_POOL.contains_transaction(tx).unwrap_or(false)
}
```

---

## 3. Mining pipeline (`miner.rs`)

Mining is triggered by **policy** (`should_trigger_mining`) and executed by `process_mine_block`. The output of mining is a new block which is then announced to peers using `INV(block_hash)`.

### Diagram: mining trigger → mining → relay

```text
NodeContext::submit_transaction_for_mining
  |
  | if should_trigger_mining()
  v
prepare_mining_utxo -> txs + coinbase
  |
  v
process_mine_block -> blockchain.mine_block(txs)
  |
  | remove txs from mempool
  v
broadcast_new_block -> send_inv(op=Block, items=[block_hash])
```

> **Methods involved:**
>
> - `should_trigger_mining()`
> - `prepare_mining_utxo(...)`
> - `process_mine_block(...)`
> - `broadcast_new_block(...)`
> - `mine_empty_block(...)`
> - `cleanup_invalid_transactions(...)`

### Listing 13-3.1 — Mining trigger and block construction (part 1) (`bitcoin/src/node/miner.rs`)

```rust
use super::txmempool::remove_from_memory_pool;
use crate::error::Result;
use crate::net::net_processing::send_inv;
use crate::node::{GLOBAL_MEMORY_POOL, GLOBAL_NODES, OpType};
use crate::{
    Block, BlockchainService, GLOBAL_CONFIG, Transaction,
    WalletAddress,
};

const TRANSACTION_THRESHOLD: usize = 3;

pub fn should_trigger_mining() -> bool {
    // Policy: mine if miner configured AND mempool size exceeds threshold
    GLOBAL_MEMORY_POOL.len().unwrap_or(0) >= TRANSACTION_THRESHOLD
        && GLOBAL_CONFIG.is_miner()
}

pub async fn prepare_mining_utxo(
    mining_address: &WalletAddress,
    blockchain: &BlockchainService,   // validates inputs against UTXO set
) -> Result<Vec<Transaction>> {
    // Snapshot mempool and validate each tx's inputs are still unspent.
    // This prevents mining blocks with already-spent inputs when a
    // competing block has been accepted during the race between miners.
    let txs = GLOBAL_MEMORY_POOL.get_all()?;
    let db = blockchain.get_db().await?;
    let utxo_tree = db.open_tree("chainstate")?;
    let mut valid_txs = Vec::new();
    for tx in txs {
        if tx.is_coinbase() { continue; }
        // ... (validate each input exists in UTXO tree)
        // Stale transactions are removed from mempool
    }
    valid_txs.push(Transaction::new_coinbase_tx(mining_address)?);
    Ok(valid_txs)
}
```

Once we have a validated transaction set, `process_mine_block` performs the actual proof-of-work, clears the mempool, and announces the result to peers:

### Listing 13-3.1b — Mining execution and broadcast (`bitcoin/src/node/miner.rs`)

```rust
pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    // Prevent concurrent mining
    if MINING_IN_PROGRESS.compare_exchange(false, true, ...).is_err() {
        return Err("Mining already in progress");
    }
    reset_mining_cancellation();

    let result = async {
        // Check cancellation before mining (competing block may have arrived)
        if is_mining_cancelled() { return Err("Mining cancelled"); }

        // 1) Mine: PoW + chainstate integration (with stale-mining protection)
        let new_block = blockchain.mine_block(&txs).await?;

        // CRITICAL: Do NOT cancel after block creation —
        // the block is in our chain and MUST be broadcast,
        // or we create a permanent minority fork.

        // 2) Remove confirmed txs from mempool
        for tx in &txs {
            remove_from_memory_pool(tx.clone(), blockchain).await;
        }

        // 3) Announce block hash to peers (inventory-first)
        broadcast_new_block(&new_block).await?;
        Ok(new_block)
    }.await;

    MINING_IN_PROGRESS.store(false, Ordering::SeqCst); // Always release lock
    result
}

pub async fn broadcast_new_block(block: &Block) -> Result<()> {
    let my_addr = GLOBAL_CONFIG.get_node_addr();
    let nodes = GLOBAL_NODES.get_nodes()?;
    for node in nodes {
        if node.get_addr() != my_addr {
            let addr = node.get_addr();
            let hash = block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&addr, OpType::Block, &[hash]).await;
            });
        }
    }
    Ok(())
}
```

Empty block mining and transaction cleanup follow the main mining pipeline:

### Listing 13-3.2 — Empty block mining and cleanup (part 2)

```rust
pub async fn mine_empty_block(
    blockchain: &BlockchainService,
    wallet_address: &WalletAddress,
) -> Result<Block> {
    if GLOBAL_CONFIG.is_miner() {
        // An “empty” block still has one transaction: coinbase.
        let coinbase_tx = create_mining_coinbase_transaction(wallet_address)?;
        let txs = vec![coinbase_tx];
        process_mine_block(txs, blockchain).await
    } else {
        Err(BtcError::NotAMiner)
    }
}

pub async fn cleanup_invalid_transactions() -> Result<()> {
    // Placeholder for revalidation over time.
    info!(“Cleaning up invalid transactions from memory pool”);
    Ok(())
}
```

---

## 4. Peer set (`peers.rs`)

The peer set is a thread-safe wrapper around a `HashSet<Node>` guarded by an `RwLock`.

> **Methods involved:**
>
> - `Node::get_addr()`
> - `Nodes::{add_node, add_nodes, evict_node, get_nodes, node_is_known}`

### Listing 13-16 — Peer data structures (`bitcoin/src/node/peers.rs`)

```rust
use crate::error::Result;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::RwLock;

// Thin wrapper to store SocketAddr in HashSet (must be Eq, Hash, etc.)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node { addr: SocketAddr }

impl Node {
    fn new(addr: SocketAddr) -> Self { Node { addr } }
    pub fn get_addr(&self) -> SocketAddr { self.addr }
}

// Thread-safe peer set: many reads, occasional writes
pub struct Nodes { inner: RwLock<HashSet<Node>> }

impl Nodes {
    pub fn new() -> Self {
        Nodes { inner: RwLock::new(HashSet::new()) }
    }

    pub fn add_node(&self, addr: SocketAddr) -> Result<()> {
        self.inner.write()?.insert(Node::new(addr));
        Ok(())
    }

    pub fn add_nodes(&self, nodes: HashSet<SocketAddr>) -> Result<()> {
        let mut inner = self.inner.write()?;
        for node in nodes {
            inner.insert(Node::new(node));
        }
        Ok(())
    }

    pub fn evict_node(&self, addr: &SocketAddr) -> Result<bool> {
        Ok(self.inner.write()?.remove(&Node::new(*addr)))
    }

    pub fn get_nodes(&self) -> Result<Vec<Node>> {
        Ok(self.inner.read()?.iter().cloned().collect())
    }

    pub fn node_is_known(&self, addr: &SocketAddr) -> Result<bool> {
        Ok(self.inner.read()?.iter().any(|x| x.get_addr() == *addr))
    }
}

impl Default for Nodes {
    fn default() -> Self { Self::new() }
}
```

---

## 5. Integration point: network routing uses NodeContext

The network router is responsible for **decoding** and **dispatching**, and then calling into `NodeContext` for stateful operations.

> **Methods involved:**
>
> - `process_stream(...)` (**defined earlier** in **[Chapter 12.A: Network Layer — Code Walkthrough](../net/01-Network-Operation-Code-Walkthrough.md)**)
> - `NodeContext::process_transaction(...)` (printed above)
> - `NodeContext::add_block(...)` (printed above)

At the key boundary:

- `Package::Tx` → `NodeContext::process_transaction(...)`
- `Package::Block` → `NodeContext::add_block(...)`

This is the “runtime wiring” that makes the node feel like a single system rather than a set of disconnected modules.

---

## Summary: what to remember

- `NodeContext` is a façade: it routes high-level requests into mempool, mining, relay, and chainstate.
- `process_transaction` is intentionally **fast**: it accepts to mempool and then spawns relay/mining in the background.
- Mining is policy-driven (`should_trigger_mining`) and ends by relaying an `INV` for the new block.
- The peer set is a simple `RwLock<HashSet<Node>>` and is used by both relay and mining.

---

<div align="center">

**[← Chapter 31: Node Orchestration](README.md)** | **Chapter 13.A: Code Walkthrough** | **[Chapter 23: Wallet System →](../wallet/README.md)**

</div>

---

