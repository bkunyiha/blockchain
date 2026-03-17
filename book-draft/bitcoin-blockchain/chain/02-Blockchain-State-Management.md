<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. <a href="../node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
# Section 9.2: Blockchain State Management

**Part I: Foundations & Core Implementation** | **Section 9.2: Blockchain State Management**

<div align="center">

**[← Previous: Domain Model](01-Domain-Model.md)** | **Section 9.2 Blockchain State Management** | **[Next: Chain State and Storage →](03-Chain-State-and-Storage.md)** 

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

## Scope within Section 9 (section flow)

This subsection covers **Section 9 (Blockchain — From Transaction to Block Acceptance) Steps 4–8** at the API boundary level: where reads vs writes are controlled, and where the node delegates to persistence and derived state.

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

```text
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

```text
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

**Code Listing 9-6**: `chain/` module exports (public boundary)  
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

**Code Listing 9-7**: The façade type (`BlockchainService`)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
#[derive(Debug)]
pub struct BlockchainService(
    Arc<TokioRwLock<BlockchainFileSystem>>
);

impl BlockchainService {
    pub async fn initialize(
        genesis_address: &WalletAddress
    ) -> Result<BlockchainService> {
        let blockchain =
            BlockchainFileSystem::create_blockchain(
                genesis_address
            ).await?;
        Ok(BlockchainService(
            Arc::new(TokioRwLock::new(blockchain))
        ))
    }

    pub async fn default()
        -> Result<BlockchainService>
    {
        let blockchain =
            BlockchainFileSystem::open_blockchain()
                .await?;
        Ok(BlockchainService(
            Arc::new(TokioRwLock::new(blockchain))
        ))
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

**Code Listing 9-8**: Read helper (read lock + delegation)
```rust
// Source: bitcoin/src/chain/chainstate.rs
async fn read<F, Fut, T>(&self, f: F) -> Result<T>
where
    F: FnOnce(BlockchainFileSystem) -> Fut + Send,
    Fut: Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    // Acquire shared (read) lock…
    let blockchain_guard = self.0.read().await;
    // …clone underlying chainstate handle,
    // run the operation.
    f(blockchain_guard.clone()).await
}
```

- **What to notice**
  - The public “read” methods (`get_tip_hash`, `get_block`, `find_transaction`, etc.) are thin wrappers around `read(...)`.
  - We intentionally funnel reads through a single mechanism so it is easy to audit which operations require exclusive access.

### Step 4 — Find where writes take the lock (example: adding a block)

**Write-lock boundary (`add_block`) code**: `bitcoin/src/chain/chainstate.rs`

**Code Listing 9-9**: Write-lock boundary example (`BlockchainService::add_block`)  
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

**Code Listing 9-10**: Mining boundary (`BlockchainService::mine_block`)  
```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Validate candidate transactions before we persist anything.
    for transaction in transactions {
        let is_valid = transaction.verify(self).await?;
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

**Code Listing 9-11**: Derived-state delegation (UTXO update + rollback)  
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

Next, in Section 9.3 (Chain State and Storage), we trace the storage layer’s write paths that make blocks durable on disk. Then, in Section 9.4 (UTXO Set), we study update and rollback rules in detail.

## References and further reading

- Tokio `RwLock` (read/write lock semantics): `https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html`
- `Arc<T>` (shared ownership in Rust): `https://doc.rust-lang.org/std/sync/struct.Arc.html`
- sled transactions: `https://docs.rs/sled/latest/sled/struct.Tree.html#method.transaction`

## Recap

- **`BlockchainService` is the shared chain API**: one façade used across node runtime and indirectly by the web layer via `NodeContext`.
- **Read vs write is explicit**: reads go through a shared lock; mutations take an exclusive lock before delegating to persistence.
- **Persistence and derived state are separate**: the storage layer persists blocks/tip; the UTXO set tracks spendability updates/rollbacks.

<div align="center">

**[← Previous: Domain Model](01-Domain-Model.md)** | **Blockchain State Management** | **[Next: Chain State and Storage →](03-Chain-State-and-Storage.md)** 

</div>

---

**In this subsection, we explored the `bitcoin/src/chain` boundary: what it exports, where it takes locks, and how it delegates persistence and derived-state updates. Next, in Section 9.3, we trace the concrete write paths that make blocks durable state on disk.**
