<div align="left">

<details>
<summary><b>📑 Section Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Section 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Section 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Section 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. Section 1.4: Bitcoin Whitepaper In Rust - Bitcoin Whitepaper In Rust
5. Section 2.0: Rust Blockchain Project - Blockchain Project
6. Section 2.1: Primitives - Core data structures
7. Section 2.2: Utilities - Utility functions and helpers
8. Section 2.3: Cryptography - Cryptographic primitives and libraries
9. **Section 2.4: Blockchain (Technical Foundations)** ← *You are here*
10. Section 2.5: Storage Layer - Persistent storage implementation
11. Section 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
12. Section 2.7: Network Layer - Peer-to-peer networking and protocol
13. Section 2.8: Node Orchestration - Node context and coordination
14. Section 2.9: Wallet System - Wallet implementation and key management
15. Section 3: Web API Architecture - REST API implementation
16. Section 4: Desktop Admin Interface - Iced framework architecture
17. Section 5: Wallet User Interface - Wallet UI implementation
18. Section 6: Embedded Database & Persistence - SQLCipher integration
19. Section 7: Web Admin Interface - React/TypeScript web UI

### Part II: Deployment & Operations

20. Section 8: Docker Compose Deployment - Docker Compose guide
21. Section 9: Kubernetes Deployment - Kubernetes production guide
22. Section 10: Rust Language Guide - Rust programming language reference

</details>

</div>

---
# Section 2.4.2: Blockchain State Management

**Part I: Core Blockchain Implementation** | **Section 2.4.2: Blockchain State Management**

<div align="center">

**📚 [← Previous: Domain Model](01-Domain-Model.md)** | **Section 2.4.2 Blockchain State Management** | **[Next: Chain State and Storage →](03-Chain-State-and-Storage.md)** 📚

</div>

---

## Overview

This section focuses on the `chain/` module as the **public boundary** the rest of the node uses to read and mutate blockchain state.

In a Bitcoin-style node, multiple subsystems are active at once:

- the network handler receives blocks and transactions,
- the miner builds candidate blocks,
- the wallet queries spendability (UTXOs),
- the web/API layer reads chain information and triggers actions (e.g., “submit transaction”, “start mining”).

In this project, we concentrate “shared chain state” behind a single façade (`BlockchainService`) and control mutation using an async read/write lock. The façade then delegates persistence and fork-choice decisions to the storage layer (`BlockchainFileSystem`) and derived-state management to the UTXO set (`UTXOSet`).

## Scope within Section 2.4 (section flow)

This subsection covers **Section 2.4 (Blockchain — From Transaction to Block Acceptance) Steps 4–8** at the API boundary level: where reads vs writes are controlled, and where the node delegates to persistence and derived state.

## Key entry points in this subsection

- **Read boundary**: `BlockchainService::read(...)` (the shared-read lock pattern)
- **Write boundary (peer blocks)**: `BlockchainService::add_block(...)` → `BlockchainFileSystem::add_block(...)`
- **Write boundary (local mining)**: `BlockchainService::mine_block(...)` → `BlockchainFileSystem::mine_block(...)`
- **Derived-state update/rollback**: `BlockchainService::update_utxo_set(...)` / `rollback_utxo_set(...)` → `UTXOSet::{update, rollback_block}(...)`

### Who uses this façade? (node runtime + web layer)

It helps to be explicit about call sites. `BlockchainService` is a **high-level chain API** used in two ways:

- **Node runtime**: networking and mining code call `BlockchainService` directly (for example, to add a received block or mine a new one).
- **Web/API layer**: HTTP handlers call into `NodeContext` (`bitcoin/src/node/context.rs`), which holds a `BlockchainService` internally. In other words, the web layer uses the same chain façade **indirectly** through the node’s coordination boundary.

### Figure: Shared chain API used by both node and web layers

```
Web server / handlers (bitcoin/src/web/*)
        │  (State<Arc<NodeContext>>)
        ▼
NodeContext (coordination boundary)
        │  (owns a BlockchainService)
        ▼
BlockchainService  (Arc<TokioRwLock<BlockchainFileSystem>>)
        │                    │
        │ read() / write()   │ durable chain + tip pointer
        ▼                    ▼
UTXOSet (derived state)   BlockchainFileSystem (sled-backed storage)
```

## Step-by-step code walkthrough

**Goal**: understand how “chainstate” is structured in our implementation: the public façade (`BlockchainService`), the locking model, and how persistence + UTXO updates are invoked.

**Code walkthrough**:
- `bitcoin/src/chain/mod.rs`
- `bitcoin/src/chain/chainstate.rs`
- `bitcoin/src/chain/utxo_set.rs`

**Whitepaper anchors**:
- Section 5 (nodes accept blocks and update state; then build on the accepted tip)

### Figure: Where chain state lives (boundary + delegation)

```
Node subsystems (network / miner / wallet / API)
        │
        │ calls a small public API
        ▼
BlockchainService  (Arc<TokioRwLock<BlockchainFileSystem>>)
        │                    │
        │ read() / write()   │ owns the durable chain + tip pointer
        ▼                    ▼
UTXOSet (derived state)   BlockchainFileSystem (sled-backed storage)
```

### Step 1 — See what the `chain/` module exports (the public API boundary)

**Chain module public API code**: `bitcoin/src/chain/mod.rs`

The types we import (`BlockchainService`, `UTXOSet`) are re-exported here:

**Code Listing 2.4-2.1**: `chain/` module exports (public boundary)  
```rust
// Source: bitcoin/src/chain/mod.rs
pub mod chainstate;
pub mod utxo_set;

// Re-export main types for convenience
pub use chainstate::BlockchainService;
pub use utxo_set::UTXOSet;
```

- **What to notice**
  - `chain/mod.rs` is intentionally small: it exposes the façade + UTXO set without leaking internal layout.
- **Whitepaper mapping**
  - **§5**: the node needs a clear boundary between “network events” and “state updates”; this module is that boundary in our implementation.

### Step 2 — Understand the façade (`BlockchainService` wraps the storage layer)

**Chain façade (BlockchainService) code**: `bitcoin/src/chain/chainstate.rs`

**Code Listing 2.4-2.2**: The façade type (`BlockchainService`)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
#[derive(Debug)]
pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl BlockchainService {
    pub async fn initialize(genesis_address: &WalletAddress) -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::create_blockchain(genesis_address).await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }

    pub async fn default() -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::open_blockchain().await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
}
```

- **What to notice**
  - `Arc<T>` makes the service clonable and shareable across tasks without copying the underlying chainstate.
  - `TokioRwLock<T>` allows many readers concurrently (reads), but only one writer (mutations such as adding blocks).
- **Whitepaper mapping**
  - **§5**: nodes must concurrently receive/validate/broadcast while maintaining a consistent chain tip.

### Step 3 — Read vs write: the lock boundary you should look for first

The easiest way to understand the locking model is to find the helper used for reads, and then compare it to write paths.

**Code Listing 2.4-2.3**: Read helper (read lock + delegation)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
async fn read<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(BlockchainFileSystem) -> Fut + Send,
    Fut: Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    // Acquire a shared (read) lock…
    let blockchain_guard = self.0.read().await;
    // …clone the underlying chainstate handle and run the operation.
    f(blockchain_guard.clone()).await
}
```

- **What to notice**
  - The public “read” methods (`get_tip_hash`, `get_block`, `find_transaction`, etc.) are thin wrappers around `read(...)`.
  - We intentionally funnel reads through a single mechanism so it is easy to audit which operations require exclusive access.

### Step 4 — Find where writes take the lock (example: adding a block)

**Write-lock boundary (`add_block`) code**: `bitcoin/src/chain/chainstate.rs`

**Code Listing 2.4-2.4**: Write-lock boundary example (`BlockchainService::add_block`)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn add_block(&self, block: &Block) -> Result<()> {
    let mut blockchain_guard = self.0.write().await;
    blockchain_guard.add_block(block).await
}
```

- **What to notice**
  - `add_block` is intentionally short: it takes the write lock and delegates to `BlockchainFileSystem::add_block(...)`.
  - This delegation is important: **the storage layer is the acceptance boundary** where “validate → connect” belongs.
- **Whitepaper mapping**
  - **§5 step 5**: “accept only if…” must be enforced before connecting state; this is the call path that eventually performs that mutation.

### Step 5 — Mining is also a write path (and it verifies before it mutates)

Mining constructs a new block, persists it, advances the tip, and updates the UTXO set. That is why `mine_block` is also guarded by the write lock.

**Code Listing 2.4-2.5**: Mining boundary (`BlockchainService::mine_block`)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Validate candidate transactions before we persist anything.
    for trasaction in transactions {
        let is_valid = trasaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }

    // Mining + persistence happens inside the storage layer.
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions).await
}
```

- **What to notice**
  - Verification happens first; persistence happens only after verification passes.
  - We use a write lock even though `BlockchainFileSystem::mine_block` takes `&self`: internally it mutates durable state (DB + tip) and updates derived state.

### Step 6 — See how derived state is updated (UTXO update + rollback)

**Derived-state delegation (UTXO update/rollback) code**: `bitcoin/src/chain/chainstate.rs`

**Code Listing 2.4-2.6**: Derived-state delegation (UTXO update + rollback)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn update_utxo_set(&self, block: &Block) -> Result<()> {
    let utxo_set = UTXOSet::new(self.clone());
    utxo_set.update(block).await
}

pub async fn rollback_utxo_set(&self, block: &Block) -> Result<()> {
    let utxo_set = UTXOSet::new(self.clone());
    utxo_set.rollback_block(block).await
}
```

At this point, we have identified the most important “state boundaries” in code:

- read-only operations are funneled through `read(...)`,
- state mutations take the write lock and delegate to the storage layer,
- derived state is updated through the UTXO set.

Next, in Section 2.4.3 (Chain State and Storage), we trace the storage layer’s write paths that make blocks durable on disk. Then, in Section 2.4.4 (UTXO Set), we study update and rollback rules in detail.

## References and further reading

- Tokio `RwLock` (read/write lock semantics): `https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html`
- `Arc<T>` (shared ownership in Rust): `https://doc.rust-lang.org/std/sync/struct.Arc.html`
- sled transactions: `https://docs.rs/sled/latest/sled/struct.Tree.html#method.transaction`

## Recap

- **`BlockchainService` is the shared chain API**: one façade used across node runtime and indirectly by the web layer via `NodeContext`.
- **Read vs write is explicit**: reads go through a shared lock; mutations take an exclusive lock before delegating to persistence.
- **Persistence and derived state are separate**: the storage layer persists blocks/tip; the UTXO set tracks spendability updates/rollbacks.

<div align="center">

**📚 [← Previous: Domain Model](01-Domain-Model.md)** | **Blockchain State Management** | **[Next: Chain State and Storage →](03-Chain-State-and-Storage.md)** 📚

</div>

---

**In this subsection, we explored the `bitcoin/src/chain` boundary: what it exports, where it takes locks, and how it delegates persistence and derived-state updates. Next, in Section 2.4.3, we trace the concrete write paths that make blocks durable state on disk.**
