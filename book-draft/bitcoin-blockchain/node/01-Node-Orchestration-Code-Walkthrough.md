<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. **Chapter 2.8: Node Orchestration** ← *You are here*
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

# Node Orchestration — Code Walkthrough (NodeContext + Mempool + Mining + Peers)

**Part I: Core Blockchain Implementation** | **Chapter 2.8.A: Node Orchestration — Code Walkthrough**

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

```
Network (Chapter 2.7)
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
Chain / storage (Chapters 2.4–2.6)
  - validates + connects blocks
  - persists chain state
  - maintains UTXO
```

> **Methods involved**
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

```
Inbound tx bytes
  -> (Network) Package::Tx
  -> NodeContext::process_transaction
      -> txmempool::add_to_memory_pool
      -> spawn background task:
         NodeContext::submit_transaction_for_mining
             -> (if central) broadcast INV(txid) to peers
             -> (if mining threshold) miner::process_mine_block
```

> **Methods involved**
>
> - `NodeContext::{new, process_transaction}`
> - `NodeContext::submit_transaction_for_mining` (internal)
> - `NodeContext::{btc_transaction, submit_transaction}` (front-door helpers)
> - `txmempool::{add_to_memory_pool, transaction_exists_in_pool}`
> - `send_inv(...)` (**defined earlier** in **[Chapter 2.7.A: Network Layer — Code Walkthrough](../net/01-Network-Operation-Code-Walkthrough.md)**)

### Code Listing 2.8A-1.1 — `NodeContext` (transaction pipeline) (`bitcoin/src/node/context.rs`)

```rust
use crate::GLOBAL_CONFIG;
use crate::chain::{BlockchainService, UTXOSet};
use crate::error::{BtcError, Result};
use crate::net::net_processing::send_inv;
use crate::node::miner;
use crate::node::miner::{
    cleanup_invalid_transactions, prepare_mining_utxo, process_mine_block, should_trigger_mining,
};
use crate::node::txmempool::{
    add_to_memory_pool, remove_from_memory_pool, transaction_exists_in_pool,
};
use crate::node::{CENTERAL_NODE, GLOBAL_NODES, Node, OpType};
use crate::{Block, Transaction, WalletAddress};
use std::net::SocketAddr;
use tracing::{error, info, warn};

// `NodeContext` is the orchestration façade: other layers call into it
// instead of talking to mempool/miner/peers/chainstate directly.
#[derive(Clone, Debug)]
pub struct NodeContext {
    // The only owned dependency here: chainstate + storage + mining hooks.
    // Everything else is reached through module-level globals or helper modules.
    blockchain: BlockchainService,
}

impl NodeContext {
    pub fn new(blockchain: BlockchainService) -> Self {
        // Construction is intentionally cheap: the heavy state is inside `BlockchainService`.
        Self { blockchain }
    }

    pub async fn btc_transaction(
        &self,
        wlt_frm_addr: &WalletAddress,
        wlt_to_addr: &WalletAddress,
        amount: i32,
    ) -> Result<String> {
        // This is a high-level “create + submit” helper:
        // it builds a signed transaction from the UTXO view, then submits it to the normal pipeline.
        let utxo_set = UTXOSet::new(self.blockchain.clone());
        let utxo =
            Transaction::new_utxo_transaction(wlt_frm_addr, wlt_to_addr, amount, &utxo_set).await?;

        // For locally-created transactions, the sender address is “us”.
        let addr_from = crate::GLOBAL_CONFIG.get_node_addr();
        self.process_transaction(&addr_from, utxo).await
    }

    pub async fn submit_transaction(
        &self,
        addr_from: &std::net::SocketAddr,
        utxo: Transaction,
    ) -> Result<String> {
        // Convenience wrapper: treat the provided tx as already-built and just run acceptance.
        self.process_transaction(addr_from, utxo).await
    }

    pub async fn process_transaction(
        &self,
        addr_from: &std::net::SocketAddr,
        utxo: Transaction,
    ) -> Result<String> {
        // 1) Dedupe: refuse if mempool already contains this tx.
        if transaction_exists_in_pool(&utxo) {
            info!("Transaction: {:?} already exists", utxo.get_id());
            return Err(BtcError::TransactionAlreadyExistsInMemoryPool(
                utxo.get_tx_id_hex(),
            ));
        }

        // 2) Accept into mempool (and set the “in mempool” flag in the UTXO view).
        add_to_memory_pool(utxo.clone(), &self.blockchain).await?;

        // 3) Fire-and-forget: broadcast + trigger mining in a background task.
        // The API call returns immediately after mempool acceptance.
        let context = self.clone();
        let addr_copy = *addr_from;
        let tx = utxo.clone();
        tokio::spawn(async move {
            let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
        });

        // 4) Return txid for caller/UI.
        Ok(utxo.get_tx_id_hex())
    }

    async fn submit_transaction_for_mining(
        &self,
        addr_from: &std::net::SocketAddr,
        utxo: Transaction,
    ) -> Result<()> {
        // Background task: after mempool acceptance, we may relay and/or mine.
        let my_node_addr = GLOBAL_CONFIG.get_node_addr();

        // Relay policy: only the “central node” relays inventory to the peer set.
        if my_node_addr.eq(&CENTERAL_NODE) {
            let nodes = self.get_nodes_excluding_sender(addr_from).await?;
            self.broadcast_transaction_to_nodes(&nodes, utxo.get_id_bytes())
                .await;
        }

        // Mining policy: start mining when mempool reaches threshold (and we are a miner).
        if should_trigger_mining() {
            if let Some(mining_address) = GLOBAL_CONFIG.get_mining_addr() {
                match prepare_mining_utxo(&mining_address) {
                    Ok(txs) => {
                        if !txs.is_empty() {
                            // Mine + broadcast happens inside `process_mine_block`.
                            process_mine_block(txs, &self.blockchain).await.map(|_| ())
                        } else {
                            warn!("Mining triggered but no valid transactions to mine");
                            Ok(())
                        }
                    }
                    Err(e) => {
                        error!("Failed to prepare mining transactions: {}", e);
                        cleanup_invalid_transactions().await
                    }
                }
            } else {
                warn!("Mining triggered but no mining address configured");
                Ok(())
            }
        } else {
            // Nothing to do: relay/mining policies didn’t trigger.
            Ok(())
        }
    }

    async fn get_nodes_excluding_sender(
        &self,
        addr_from: &std::net::SocketAddr,
    ) -> Result<Vec<Node>> {
        // Avoid two common relay problems:
        // - echoing back to the sender
        // - sending to ourselves
        let nodes = GLOBAL_NODES
            .get_nodes()
            .expect("Global nodes get error")
            .into_iter()
            .filter(|node| {
                let node_addr = node.get_addr();
                let my_addr = GLOBAL_CONFIG.get_node_addr();
                node_addr != *addr_from && node_addr != my_addr
            })
            .collect();
        Ok(nodes)
    }

    async fn broadcast_transaction_to_nodes(&self, nodes: &[Node], txid: Vec<u8>) {
        // Broadcast pattern: INV(txid) to every peer (in parallel tasks).
        let txid_clone = txid.clone();
        nodes.iter().for_each(|node| {
            let node_addr = node.get_addr();
            let txid = txid_clone.clone();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Tx, &[txid]).await;
            });
        });
    }

    pub async fn remove_from_memory_pool(&self, tx: Transaction) {
        // Called after a transaction is confirmed in a block.
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

> **Methods involved**
>
> - `NodeContext::{get_blockchain, blockchain}` (chain handle access)
> - `NodeContext::{add_block, get_blockchain_height, get_block_hashes, get_block}`
> - `NodeContext::{get_peers, get_peer_count}`
> - `NodeContext::mine_empty_block(...)` (admin/testing convenience)
>
> The transaction pipeline methods are defined earlier in **Code Listing 2.8A-1.1**.

### Code Listing 2.8A-1.2 — `NodeContext` (block + chain queries) (`bitcoin/src/node/context.rs`)

```rust
use crate::chain::BlockchainService;
use crate::error::Result;
use crate::node::miner;
use crate::{Block, WalletAddress};
use std::net::SocketAddr;

// Continuation from Code Listing 2.8A-1.1: the `NodeContext` type and its `blockchain` field
// are defined earlier; here we focus on the node-facing chain/peer query surface.
impl NodeContext {
    pub fn get_blockchain(&self) -> &BlockchainService {
        // Exposes chainstate for subsystems that need direct access.
        &self.blockchain
    }

    pub fn blockchain(&self) -> &BlockchainService {
        // Alias kept for compatibility with earlier code.
        &self.blockchain
    }

    pub async fn add_block(&self, block: &Block) -> Result<()> {
        // Important boundary:
        // - the network layer hands a candidate block to the node
        // - chainstate decides whether it becomes state (validate → connect → persist)
        self.blockchain.add_block(block).await
    }

    pub async fn get_blockchain_height(&self) -> Result<usize> {
        // “Best height” means the current canonical tip height.
        self.blockchain.get_best_height().await
    }

    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        // Used by the network for sync: it can send an INV list of hashes.
        self.blockchain.get_block_hashes().await
    }

    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        // Used by the network to answer GETDATA(Block).
        self.blockchain.get_block(block_hash).await
    }

    pub async fn mine_empty_block(&self, wallet_address: &WalletAddress) -> Result<Block> {
        // Administrative/testing convenience: mine a block with only coinbase.
        miner::mine_empty_block(&self.blockchain, wallet_address).await
    }

    pub fn get_peer_count(&self) -> Result<usize> {
        use crate::node::GLOBAL_NODES;
        // `GLOBAL_NODES` is a shared peer set used by relay + mining announcements.
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

> **Methods involved**
>
> - `add_to_memory_pool(...)`
> - `remove_from_memory_pool(...)`
> - `transaction_exists_in_pool(...)`

### Code Listing 2.8A-2.1 — Mempool helpers (`bitcoin/src/node/txmempool.rs`)

```rust
use crate::error::Result;
use crate::node::GLOBAL_MEMORY_POOL;
use crate::{BlockchainService, Transaction, UTXOSet};
use tracing::debug;

pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain_service: &BlockchainService,
) -> Result<()> {
    // Logging banner (helpful during demos/tests).
    debug!("\n");
    debug!(
        "******************************************************************************************************"
    );
    debug!(
        "Adding transaction to memory pool: {:?}",
        tx.get_tx_id_hex()
    );
    debug!(
        "******************************************************************************************************\n"
    );

    // 1) Store the transaction in the global in-memory pool.
    GLOBAL_MEMORY_POOL
        .add(tx.clone())
        .expect("Memory pool add error");

    // 2) Mark the referenced outputs as “in mempool” in the UTXO view.
    // This repo uses the flag as a lightweight double-spend prevention within the mempool.
    let utxo_set = UTXOSet::new(blockchain_service.clone());
    utxo_set.set_global_mem_pool_flag(&tx.clone(), true).await?;

    Ok(())
}

pub async fn remove_from_memory_pool(tx: Transaction, blockchain: &BlockchainService) {
    // 1) Remove tx from the global pool.
    GLOBAL_MEMORY_POOL
        .remove(tx.clone())
        .expect("Memory pool remove error");

    // 2) Clear the “in mempool” flags in the UTXO view.
    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), false)
        .await
        .expect("Failed to get blockchain");
}

pub fn transaction_exists_in_pool(tx: &Transaction) -> bool {
    // Fast membership check to prevent duplicates.
    GLOBAL_MEMORY_POOL.contains_transaction(tx).unwrap_or(false)
}
```

---

## 3. Mining pipeline (`miner.rs`)

Mining is triggered by **policy** (`should_trigger_mining`) and executed by `process_mine_block`. The output of mining is a new block which is then announced to peers using `INV(block_hash)`.

### Diagram: mining trigger → mining → relay

```
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

> **Methods involved**
>
> - `should_trigger_mining()`
> - `prepare_mining_utxo(...)`
> - `process_mine_block(...)`
> - `broadcast_new_block(...)`
> - `mine_empty_block(...)`
> - `cleanup_invalid_transactions(...)`

### Code Listing 2.8A-3.1 — Mining implementation (`bitcoin/src/node/miner.rs`)

```rust
use super::txmempool::remove_from_memory_pool;
use crate::error::{BtcError, Result};
use crate::net::net_processing::send_inv;
use crate::node::{GLOBAL_MEMORY_POOL, GLOBAL_NODES, OpType};
use crate::{Block, BlockchainService, GLOBAL_CONFIG, Transaction, WalletAddress};
use tracing::info;

const TRANSACTION_THRESHOLD: usize = 3;

fn create_mining_coinbase_transaction(to: &WalletAddress) -> Result<Transaction> {
    // Coinbase is the “block reward” transaction (newly minted coins to the miner).
    Transaction::new_coinbase_tx(to)
}

pub fn should_trigger_mining() -> bool {
    // Mining policy in this implementation:
    // - only mine if we are configured as a miner
    // - only mine if mempool size crosses a small threshold
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    let is_miner = GLOBAL_CONFIG.is_miner();
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}

pub fn prepare_mining_utxo(mining_address: &WalletAddress) -> Result<Vec<Transaction>> {
    // Take a snapshot of mempool transactions (copy into a Vec).
    let txs = GLOBAL_MEMORY_POOL.get_all()?;

    info!("Preparing to mine with {} valid transactions", txs.len());

    // Always include coinbase in the block template.
    let coinbase_tx = create_mining_coinbase_transaction(mining_address)?;
    let mut final_txs = txs;
    final_txs.push(coinbase_tx);

    Ok(final_txs)
}

pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    // 1) Ask chainstate to mine a block for this transaction list.
    // `blockchain.mine_block` encapsulates PoW and chainstate integration.
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    let new_block = blockchain.mine_block(&txs).await?;

    info!(
        "New block {} is mined by node {}!",
        new_block.get_hash(),
        my_node_addr
    );

    // 2) Remove transactions from mempool now that they are confirmed.
    for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }

    // 3) Announce the new block to peers (INV(block_hash)).
    broadcast_new_block(&new_block).await?;
    Ok(new_block)
}

pub async fn broadcast_new_block(block: &Block) -> Result<()> {
    // Relay is “inventory-first”: send hashes; peers request full bytes via GETDATA.
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();
    let nodes = GLOBAL_NODES.get_nodes().expect("Global nodes get error");
    nodes
        .iter()
        // Don’t send inventory to ourselves.
        .filter(|node| !my_node_addr.eq(&node.get_addr()))
        .for_each(|node| {
            let node_addr = node.get_addr();
            let block_hash = block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Block, &[block_hash]).await;
            });
        });
    Ok(())
}

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
    // Placeholder: production nodes revalidate mempool and evict invalid txs over time.
    info!("Cleaning up invalid transactions from memory pool");
    Ok(())
}
```

---

## 4. Peer set (`peers.rs`)

The peer set is a thread-safe wrapper around a `HashSet<Node>` guarded by an `RwLock`.

> **Methods involved**
>
> - `Node::get_addr()`
> - `Nodes::{add_node, add_nodes, evict_node, get_nodes, node_is_known}`

### Code Listing 2.8A-4.1 — Peer data structures (`bitcoin/src/node/peers.rs`)

```rust
use crate::error::{BtcError, Result};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::RwLock;

// A `Node` is a thin wrapper around `SocketAddr` so it can be stored in a `HashSet`.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    addr: SocketAddr,
}

impl Node {
    fn new(addr: SocketAddr) -> Node {
        Node { addr }
    }

    pub fn get_addr(&self) -> SocketAddr {
        // Used heavily by relay/mining to compute broadcast targets.
        self.addr
    }
}

// `Nodes` is a thread-safe peer set:
// - reads are frequent (broadcast target lists)
// - writes happen on peer discovery or eviction
pub struct Nodes {
    inner: RwLock<HashSet<Node>>,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            inner: RwLock::new(HashSet::new()),
        }
    }

    pub fn add_node(&self, addr: SocketAddr) -> Result<()> {
        // Acquire write lock and insert (deduped by HashSet semantics).
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        inner.insert(Node::new(addr));
        Ok(())
    }

    pub fn add_nodes(&self, nodes: HashSet<SocketAddr>) -> Result<()> {
        // Batch insert multiple peers.
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        for node in nodes {
            inner.insert(Node::new(node));
        }
        Ok(())
    }

    pub fn evict_node(&self, addr: &SocketAddr) -> Result<bool> {
        // Remove a peer. Used when outbound connections fail.
        let mut inner = self
            .inner
            .write()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.remove(&Node::new(*addr)))
    }

    pub fn get_nodes(&self) -> Result<Vec<Node>> {
        // Snapshot the peer set into a Vec for iteration/broadcast.
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.iter().cloned().collect())
    }

    pub fn node_is_known(&self, addr: &SocketAddr) -> Result<bool> {
        // Membership query used by peer discovery logic.
        let inner = self
            .inner
            .read()
            .map_err(|e| BtcError::NodesInnerPoisonedLockError(e.to_string()))?;
        Ok(inner.iter().any(|x| x.get_addr().eq(addr)))
    }
}

impl Default for Nodes {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 5. Integration point: network routing uses NodeContext

The network router is responsible for **decoding** and **dispatching**, and then calling into `NodeContext` for stateful operations.

> **Methods involved**
>
> - `process_stream(...)` (**defined earlier** in **[Chapter 2.7.A: Network Layer — Code Walkthrough](../net/01-Network-Operation-Code-Walkthrough.md)**)
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

**📚 [← Chapter 2.8: Node Orchestration](README.md)** | **Chapter 2.8.A: Code Walkthrough** | **[Chapter 2.9: Wallet System →](../wallet/README.md)** 📚

</div>

---

