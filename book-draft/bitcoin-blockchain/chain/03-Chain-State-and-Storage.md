<div align="left">

<details>
<summary><b>📑 Section Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Section 1: Introduction & Overview
2. Section 1.2: Introduction to Bitcoin & Blockchain
3. Section 1.3: Bitcoin Whitepaper
4. Section 1.4: Bitcoin Whitepaper In Rust
5. Section 2.0: Rust Blockchain Project
6. Section 2.1: Primitives
7. Section 2.2: Utilities
8. Section 2.3: Cryptography
9. **Section 2.4: Blockchain (Technical Foundations)** ← *You are here*
10. Section 2.5: Storage Layer

</details>

</div>

---

# Section 2.4.3: Chain State and Storage — How Blocks Become Persistent State

**Important distributed systems concept**: In a blockchain network, each node maintains its own local copy of the blockchain. When we discuss "chain state and storage" in this section, we're describing how one node stores blocks locally—not a shared database that all nodes access. Each node's `BlockchainFileSystem` persists blocks to its own local sled database, and nodes synchronize with each other by exchanging blocks over the network (as we'll see in Section 2.4.8: Node Orchestration). This decentralization is fundamental to blockchain's resilience: each node operates independently with its own copy of the chain.

## Code walkthrough

### Step 0 — Entry points: a chain read for “status” and a chain write for “append a block”

#### Figure 2.4-3-S0: Call flow (the two entry points shown below)

```
GET /api/v1/blockchain  (status read)
  router → get_blockchain_info(handler)
    → NodeContext::get_blockchain_height()
      → BlockchainService::get_best_height() → read(...) [shared lock]
        → BlockchainFileSystem::get_best_height()
          → Blockchain<Db>::db (sled) + Blockchain<Db>::tip_hash
    → NodeContext::blockchain().get_last_block()
      → BlockchainService::get_last_block() → read(...) [shared lock]
        → BlockchainFileSystem::get_last_block()
          → get_tip_hash() (Blockchain<Db>::tip_hash) → sled.get(tip_hash) → Block

POST /api/v1/mining/generatetoaddress  (append-a-block write)
  router → generate_to_address(handler)
    → NodeContext::mine_block(txs)
      → BlockchainService::mine_block(txs) → write lock [exclusive]
        → BlockchainFileSystem::mine_block(txs)
          → update_blocks_tree(new_block) + cache tip_hash + update_utxo_set
```

- **What this figure shows**
  - **What is calling into storage**:
    - The Axum router dispatches `GET /api/v1/blockchain` into `web/handlers/blockchain.rs::get_blockchain_info(...)` (a read-heavy “status” endpoint).
    - The Axum router dispatches `POST /api/v1/mining/generatetoaddress` into `web/handlers/mining.rs::generate_to_address(...)`, which calls `node.mine_block(...)` (a write-heavy “append a block” operation).
  - **Where the lock boundary is**:
    - Reads go through `BlockchainService::read(...)` (**shared lock**): many concurrent readers can query height/tip.
    - Writes go through `BlockchainService`’s `write` lock (**exclusive lock**): only one task can append/update chain state at a time.
  - **What persisted keys/records are touched**:
    - Reads: `"tip_block_hash"` → `<tip_hash>`; then sled `get(<tip_hash>)` → `bytes(Block)` → deserialize.
    - Writes: sled `insert(<new_hash> -> bytes(Block))` and `insert("tip_block_hash" -> <new_hash>)` in one transaction (plus derived-state update after).
  - **How to read the walkthrough below**:
    - Flow A shows `NodeContext::get_blockchain_height → BlockchainService::get_best_height → BlockchainFileSystem::get_best_height`.
    - Flow A shows `BlockchainService::get_last_block → BlockchainFileSystem::get_last_block → get_tip_hash + sled lookup`.
    - Flow B shows `NodeContext::mine_block → BlockchainService::mine_block (exclusive lock) → BlockchainFileSystem::mine_block → update_blocks_tree`.


```rust
// router → get_blockchain_info(handler): bitcoin/src/web/handlers/blockchain.rs
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    // STATUS READ: "what is the current chain height?"
    let height = node
        .get_blockchain_height() // delegate into NodeContext (which delegates to BlockchainService)
        .await // await the async chain read
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // convert domain error → HTTP 500

    // STATUS READ: "what is the current tip block?"
    let last_block = node
        .blockchain() // fetch the chainstate API handle (BlockchainService) from NodeContext
        .get_last_block() // shared-lock read: read cached tip → sled.get(tip_hash) → deserialize Block
        .await // await the async chain read
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // convert domain error → HTTP 500

    let last_block_hash = last_block
        .map(|block| block.get_hash().to_string()) // if present, format the tip hash for API output
        .unwrap_or_else(|| "genesis".to_string()); // if None, we are at (or before) genesis in this API view

    let mempool_size = node
        .get_mempool_size() // mempool is in-memory runtime state, not sled persistence
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // convert domain error → HTTP 500

    // ... build response ...
    Ok(Json(ApiResponse::success(info)))
}
```


```rust
// router → generate_to_address(handler): bitcoin/src/web/handlers/mining.rs
// Mine the block (already adds to blockchain and updates UTXO)
let mined_block = node
    .mine_block(&transactions) // coordinator call → NodeContext::mine_block → BlockchainService::mine_block (write lock)
    .await // await local mining + persistence
    .map_err(|e| {
        // Convert a domain error into an HTTP 500 and log the details server-side.
        error!("Failed to mine block {}: {}", block_num + 1, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
```

#### Flow A — Status read (height + tip block): full call chain + code
```
GET /api/v1/blockchain  (status read)
  router → get_blockchain_info(handler)
    → NodeContext::get_blockchain_height()
      → BlockchainService::get_best_height() → read(...) [shared lock]
        → BlockchainFileSystem::get_best_height()
          → Blockchain<Db>::db (sled) + Blockchain<Db>::tip_hash
```
##### Height read: `NodeContext::get_blockchain_height` → `BlockchainService::get_best_height` → `BlockchainFileSystem::get_best_height`
```rust
// get_blockchain_info(handler) → NodeContext::get_blockchain_height: bitcoin/src/node/context.rs
pub async fn get_blockchain_height(&self) -> Result<usize> {
    // NodeContext is a coordinator; it delegates chain reads to BlockchainService.
    self.blockchain.get_best_height().await // → BlockchainService::get_best_height(...)
}
```

```rust
// NodeContext::get_blockchain_height → BlockchainService::get_best_height: bitcoin/src/chain/chainstate.rs
pub async fn get_best_height(&self) -> Result<usize> {
    // Shared-lock boundary: many concurrent readers are allowed here.
    self.read(
        // Delegate the actual DB lookup to the persistence engine.
        |blockchain: BlockchainFileSystem| async move { blockchain.get_best_height().await },
    )
    .await
}
```

```rust
// BlockchainService::get_best_height → BlockchainFileSystem::get_best_height: bitcoin/src/store/file_system_db_chain.rs
pub async fn get_best_height(&self) -> Result<usize> {
    // Persistence engine reads the tip block from sled and returns its height.
    if self.is_empty() {
        Ok(0)
    } else {
        // Open the blocks tree and load the tip block bytes.
        let block_tree = self.blockchain.db.open_tree(self.get_blocks_tree_path())?;
        let tip_block_bytes = block_tree
            // NOTE: `get_tip_hash()` reads the in-memory cached tip pointer, then we use it as a sled key.
            .get(self.get_tip_hash().await?)?
            .ok_or(BtcError::GetBlockchainError("tip is invalid".to_string()))?;
        // Deserialize the bytes into a Block so we can read its height.
        let tip_block = Block::deserialize(tip_block_bytes.as_ref())?;
        Ok(tip_block.get_height())
    }
}
```

**Explain the code**
- `BlockchainService::get_best_height(...)` is a **shared-lock read**: it uses `read(...)` (shared lock) and delegates to `BlockchainFileSystem`.
- `BlockchainFileSystem::get_best_height(...)` hits sled via `self.blockchain.db` and uses the current tip hash (`self.get_tip_hash()`) to load the tip block bytes.

##### Tip-block read: `NodeContext::blockchain().get_last_block` → `BlockchainService::get_last_block` → `BlockchainFileSystem::get_last_block`
```
    → NodeContext::blockchain().get_last_block()
      → BlockchainService::get_last_block() → read(...) [shared lock]
        → BlockchainFileSystem::get_last_block()
          → get_tip_hash() (Blockchain<Db>::tip_hash) → sled.get(tip_hash) → Block
```

```rust
// BlockchainService::get_last_block → BlockchainFileSystem::get_last_block: bitcoin/src/store/file_system_db_chain.rs
pub async fn get_last_block(&self) -> Result<Option<Block>> {
    // 1) Read the cached tip pointer (String).
    let tip_hash = self.get_tip_hash().await?;
    // 2) Use the tip hash as the sled key to load and deserialize the last block.
    let block = self.get_block(tip_hash.as_bytes()).await?;
    Ok(block)
}
```

```rust
// NodeContext::blockchain().get_last_block → BlockchainService::get_last_block: bitcoin/src/chain/chainstate.rs
pub async fn get_last_block(&self) -> Result<Option<Block>> {
    // Shared-lock boundary + delegation into persistence engine.
    self.read(
        |blockchain: BlockchainFileSystem| async move { blockchain.get_last_block().await },
    )
    .await
}
```

```rust
// BlockchainFileSystem::get_last_block → BlockchainFileSystem::get_tip_hash: bitcoin/src/store/file_system_db_chain.rs
pub async fn get_tip_hash(&self) -> Result<String> {
    // This is reading the `tip_hash` field of `Blockchain<T>` (in-memory cache).
    let tip_hash = self.blockchain.tip_hash.read().await;
    Ok(tip_hash.clone())
}
```

**Explain the code**
- `BlockchainService::get_last_block(...)` is also a **shared-lock read**.
- `BlockchainFileSystem::get_last_block(...)` is a “tip pointer + lookup” pattern: read tip hash → get block bytes from sled → deserialize.

##### Where chain state lives: `BlockchainFileSystem` owns `Blockchain<Db>` (tip cache + sled handle)

```rust
// BlockchainFileSystem (owns Blockchain<Db>): bitcoin/src/store/file_system_db_chain.rs
#[derive(Clone, Debug)]
pub struct BlockchainFileSystem {
    blockchain: Blockchain<Db>, // type is defined in `bitcoin/src/primitives/blockchain.rs`
    file_system_tree_dir: String,
}
```

```rust
// Blockchain<T> (chain container): bitcoin/src/primitives/blockchain.rs
pub struct Blockchain<T> {
    pub tip_hash: Arc<TokioRwLock<String>>, // in-memory cache of the persisted tip pointer
    pub db: T,                              // sled Db handle (persistence)
    pub is_empty: bool,
}
```

#### Flow B — Append a block (write): full call chain + code
```
    POST /api/v1/mining/generatetoaddress  (append-a-block write)
      router → generate_to_address(handler)
        → NodeContext::mine_block(txs)
          → BlockchainService::mine_block(txs) → write lock [exclusive]
            → BlockchainFileSystem::mine_block(txs)
            → update_blocks_tree(new_block) + cache tip_hash + update_utxo_set
```          

##### Write entry point: `node.mine_block(&transactions)`
```
    router → generate_to_address(handler)
      → NodeContext::mine_block(txs)
```
##### NodeContext delegates into the chainstate API
```
    → NodeContext::mine_block(txs)
      → BlockchainService::mine_block(txs) → write lock [exclusive]
        → BlockchainFileSystem::mine_block(txs)
        → update_blocks_tree(new_block) + cache tip_hash + update_utxo_set
```
```rust
// generate_to_address(handler) → NodeContext::mine_block: bitcoin/src/node/context.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Delegate the write to BlockchainService (which owns the exclusive-lock boundary).
    self.blockchain.mine_block(transactions).await
}
```

##### Exclusive-lock boundary + delegation: `BlockchainService::mine_block(...)`
```rust
// NodeContext::mine_block → BlockchainService::mine_block: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Gate: validate before mutating persistent state.
    for trasaction in transactions {
        // `verify(...)` may perform chain lookups; at the BlockchainService level, we're still pre-persistence.
        let is_valid = trasaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }

    // Exclusive lock boundary (write path): only one writer can append/update chain state at a time.
    let blockchain_guard = self.0.write().await;
    // Delegate to persistence engine to perform the actual DB writes.
    blockchain_guard.mine_block(transactions).await
}
```

##### Persistence engine performs the write: `BlockchainFileSystem::mine_block(...)`
```rust
// BlockchainService::mine_block → BlockchainFileSystem::mine_block: bitcoin/src/store/file_system_db_chain.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    let best_height = self.get_best_height().await?;

    // 1) Build the new block in memory (PoW happens inside Block::new_block).
    let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
    let block_hash = block.get_hash();

    // 2) Persist `<block_hash> -> bytes(Block)` and `"tip_block_hash" -> <block_hash>` in one transaction.
    let blocks_tree = self.blockchain.db.open_tree(self.get_blocks_tree_path())?;
Self::update_blocks_tree(&blocks_tree, &block).await?;

    // 3) Update in-memory tip cache to match what we just persisted.
    self.set_tip_hash(block_hash).await?;

    // 4) Update derived state (UTXO set) after the block bytes + tip pointer are persisted.
    self.update_utxo_set(&block).await?;

    Ok(block)
}
```

#### Note:
  - `BlockchainService::mine_block(...)` is the **exclusive lock boundary** for the “append block” write path.
  - `BlockchainFileSystem::mine_block(...)` is where persistence happens: create block → write block bytes + tip key (sled transaction) → update in-memory tip cache → update derived state (UTXO).

---

### Step 1 — Construct the chainstate API handle (create vs open)
#### Figure 2.4-3-S1: Call flow (construct the chainstate API handle)

```
caller (node startup)
  │
  ├─ BlockchainService::default()
  │    └─> BlockchainFileSystem::open_blockchain()
  │          └─> Blockchain<Db>::db (sled) + Blockchain<Db>::tip_hash cache
  │
  └─ BlockchainService::initialize(genesis_address)
       └─> BlockchainFileSystem::create_blockchain(genesis_address)
             ├─> (maybe) update_blocks_tree(...)  // write genesis + tip key
             └─> Blockchain<Db>::db + tip_hash cache
```

- **What this figure shows**
  - **Two entry points into persistence**:
    - `BlockchainService::default()` is the “open only” path: it expects `"tip_block_hash"` to exist and fails otherwise.
    - `BlockchainService::initialize(genesis_address)` is the “create if missing” path: it will create genesis and write `"tip_block_hash"` if absent.
  - **What state is created/cached**:
    - Both paths return a `BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>)` so later reads/writes all share the same chain handle.
    - The `BlockchainFileSystem` owns `Blockchain<Db>`: `db` (sled handle) + `tip_hash` (in-memory cache of the persisted tip key).
  - **What the code walkthrough below shows next**:
    - The concrete constructor code in `bitcoin/src/chain/chainstate.rs` that chooses `open_blockchain` vs `create_blockchain`.
    - Later steps then open/create the DB and show how the `"tip_block_hash"` key is read/written.

#### Code walkthrough

```rust
// caller (node startup) → BlockchainService::{default, initialize}: bitcoin/src/chain/chainstate.rs

#[derive(Debug)] // Enable `{:?}` formatting for logs/debug output.
pub struct BlockchainService(
    // Shared, async-safe handle to the persistence engine.
    // - `Arc<T>`: multiple async tasks can own the same chain handle.
    // - `TokioRwLock<T>`: many concurrent readers OR one exclusive writer.
    Arc<TokioRwLock<BlockchainFileSystem>>,
);

impl BlockchainService {
    // “Create if needed”: may bootstrap genesis, then return a usable chain handle.
    pub async fn initialize(genesis_address: &WalletAddress) -> Result<BlockchainService> {
        // Delegate to the persistence engine:
        // - If the chain exists, this will open it.
        // - If not, this will create genesis and persist the initial tip pointer.
        let blockchain = BlockchainFileSystem::create_blockchain(genesis_address).await?;
        // Wrap the engine in Arc + TokioRwLock so all callers share one consistent chain handle.
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }

    // “Open only”: requires an already-initialized DB with a persisted tip key.
    pub async fn default() -> Result<BlockchainService> {
        // Delegate to the persistence engine to open an existing DB.
        // If the "tip_block_hash" key is missing, this returns an error (Step 3).
        let blockchain = BlockchainFileSystem::open_blockchain().await?;
        // Same wrapping: Arc + TokioRwLock provides safe shared access across async tasks.
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
}
```

#### Note:
  - `Arc<TokioRwLock<...>>` is the concurrency boundary: **shared reads** vs **exclusive writes**.
  - `initialize(...)` delegates to `BlockchainFileSystem::create_blockchain(...)` (genesis bootstrapping lives there).
  - `default()` delegates to `BlockchainFileSystem::open_blockchain(...)` (no genesis; fail if missing tip key).

---

### Step 1.5 — Read boundary: shared-lock helper + one concrete read method
This “read boundary” is **not Bitcoin-specific**, and it is **not described in the Bitcoin whitepaper**. It exists because our implementation is written in **Rust** and runs on an **asynchronous runtime** (`tokio`), where many tasks may try to access chain state at the same time.

- **What problem it solves (Rust concurrency, not protocol design)**:
  - A web request might read the tip block while a background mining task is persisting a new block.
  - Without synchronization, we could read partially-updated state (e.g., “tip hash changed” while “block bytes not yet visible”), or we could race on shared memory.
- **What we use**: `Arc<TokioRwLock<...>>`
  - `Arc<T>` lets multiple async tasks share the same chain handle safely (reference-counted shared ownership).
  - `TokioRwLock<T>` provides a **read/write lock**:
    - Many readers can hold the lock at once (**shared lock**) for queries like “height” and “last block”.
    - Writers must hold the lock exclusively (**write lock**) for mutations like “mine block” and “add block”.
- **Why this matters for state + storage**:
  - Our chain state is a combination of **in-memory cached fields** (like the tip hash) and **durable storage** (sled). The lock boundary ensures reads see a coherent snapshot while writes are in progress.

#### Figure 2.4-3-S1.5: Call flow (shared-lock read boundary)

```
caller
  │
  ▼
BlockchainService::<read method>()
  └─> BlockchainService::read(...)  [shared lock]
       └─> BlockchainFileSystem::<read method>()
            └─> Blockchain<Db>::db / Blockchain<Db>::tip_hash
```

- **What this figure shows**
  - **The recurring shape of every read in this section**:
    - Callers invoke a method on `BlockchainService` (never on `BlockchainFileSystem` directly).
    - `BlockchainService::read(...)` acquires a **shared lock** on the chainstate handle.
    - The closure delegates to a `BlockchainFileSystem` method which performs the actual DB work.
  - **Where the data comes from**:
    - Reads typically use `Blockchain<Db>::tip_hash` (cached pointer) and `Blockchain<Db>::db` (sled) to locate and deserialize blocks.
  - **What the code walkthrough below shows next**:
    - The generic `read(...)` helper, then a concrete example (`get_last_block`) that uses it to reach `BlockchainFileSystem::get_last_block`.

#### Code walkthrough

```rust
// BlockchainService::<read method>() → BlockchainService::read(...): bitcoin/src/chain/chainstate.rs
impl BlockchainService {
    /// Read helper: acquire a shared lock and delegate to the persistence engine.
    async fn read<F, Fut, T>(&self, f: F) -> Result<T>
    where
        // The caller provides a one-shot function that consumes a `BlockchainFileSystem`
        // handle and returns an async computation (a `Future`).
        F: FnOnce(BlockchainFileSystem) -> Fut + Send,
        // That future must produce `Result<T>` when awaited.
        Fut: Future<Output = Result<T>> + Send,
        // The result type must be safe to move across tasks/await points.
        T: Send + 'static,
    {
        // Acquire the shared lock: many readers can run concurrently, but writers are blocked.
        let blockchain_guard = self.0.read().await;
        // Clone the persistence engine handle so we can move it into the async closure.
        // (This is typically cheap: the underlying sled handle is internally shared.)
        let blockchain = blockchain_guard.clone();
        // Run the caller-supplied operation against the persistence engine.
        f(blockchain).await
    }

    pub async fn get_last_block(&self) -> Result<Option<Block>> {
        // `get_last_block` is a read-only query, so it uses the shared-lock helper.
        self.read(|blockchain: BlockchainFileSystem| async move {
            // Delegate the actual DB work to the persistence engine.
            blockchain.get_last_block().await
        })
        .await
    }
}
```

#### Note:
  - The lock boundary is inside `BlockchainService`: callers never touch `BlockchainFileSystem` directly.
  - The closure receives a **clone** of the persistence engine handle and runs the real read operation.

---

### Step 2 — Identify the persisted keys that define “chain exists”
#### Figure 2.4-3-S2: Call flow (what “exists on disk”)

```
persisted state in sled
  ├─ "tip_block_hash"  -> "<block_hash>"         // head pointer
  └─ "<block_hash>"    -> bytes(Block)           // historical log record
runtime cache
  └─ Blockchain<Db>::tip_hash (TokioRwLock<String>)
```

- **What this figure shows**
  - **Two representations of “where we are on the chain”**:
    - Persisted pointer: the sled key `"tip_block_hash"` stores the canonical head hash on disk.
    - Runtime cache: `Blockchain<Db>::tip_hash` caches that value in memory behind a Tokio `RwLock`.
  - **How this affects reads**:
    - Most reads do not need to hit sled to learn the tip hash; they read the cached `tip_hash`, then fetch blocks by hash from sled.
  - **How this affects creation**:
    - If `"tip_block_hash"` is missing, `create_blockchain(...)` boots genesis and writes it (Step 4).
  - **What the code walkthrough below shows next**:
    - The exact constant names used for `"tip_block_hash"`, blocks tree name, and DB directory.

#### Code walkthrough

```rust
// BlockchainFileSystem storage constants (tip key + tree names): bitcoin/src/store/file_system_db_chain.rs
const DEFAULT_TIP_BLOCK_HASH_KEY: &str = "tip_block_hash"; // persisted head pointer (sled key)
const DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE: &str = "empty";  // placeholder used by this codebase
const DEFAULT_BLOCKS_TREE: &str = "blocks1"; // sled tree: "<block_hash>" -> bytes(Block)
const DEFAULT_TREE_DIR: &str = "data1";      // on-disk directory for the default sled DB
```

#### Note:
  - The persisted head pointer is **one key**: `"tip_block_hash"`.
  - The historical log is a **tree** that maps `<block_hash> -> serialized Block bytes`.

---

### Step 3 — Open path: load tip from sled into memory (`open_blockchain`)
- **Call chain**: `BlockchainService::default()` → `BlockchainFileSystem::open_blockchain()`

#### Figure 2.4-3-S3: Call flow (open existing chain)

```
BlockchainService::default()
  └─> BlockchainFileSystem::open_blockchain()
       ├─> open Blockchain<Db>::db (sled)
       ├─> blocks_tree.get("tip_block_hash") -> "<tip_hash>"
       └─> cache: Blockchain<Db>::tip_hash = "<tip_hash>"
```

- **What this figure shows**
  - **Which function is being explained**: `BlockchainFileSystem::open_blockchain()` — the persistence-layer “open existing DB” path.
  - **Which persisted key is required**:
    - `blocks_tree.get("tip_block_hash")` must succeed; otherwise `open_blockchain()` returns `BlockchainNotFoundError`.
  - **What state is reconstructed**:
    - `db = sled::open(path)` reconstructs the DB handle.
    - `tip_hash = String::from_utf8(tip_bytes)` reconstructs the canonical head pointer.
    - `Blockchain<Db>::tip_hash = tip_hash` caches the pointer in memory for subsequent reads.
  - **How this sets up the code below**:
    - The next listing shows the precise sequence: open db → open tree → get `"tip_block_hash"` → decode → cache in `Blockchain { tip_hash, db, ... }`.

#### Code walkthrough

```rust
pub async fn open_blockchain() -> Result<BlockchainFileSystem> {
    // 1) Resolve on-disk location and open sled DB (creates the DB if missing on disk).
    let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;

    // 2) Open the blocks tree (where both block records and the "tip_block_hash" pointer live).
    let blocks_tree = db
        .open_tree(file_system_tree_dir.clone())
        .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

    // 3) Read the persisted tip pointer. If missing, we treat this as “no existing chain”.
    let tip_bytes = blocks_tree
        .get(DEFAULT_TIP_BLOCK_HASH_KEY)
        .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
        .ok_or(BtcError::BlockchainNotFoundError(
            "No existing blockchain found. Connect to a blcock chain cluster first.".to_string(),
        ))?;

    // 4) Decode the tip hash bytes back into a String.
    let tip_hash = String::from_utf8(tip_bytes.to_vec())
        .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?;

    // 5) Cache the tip hash in memory behind a lock for fast reads during runtime.
    Ok(BlockchainFileSystem {
        blockchain: Blockchain {
            tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
            db,
            is_empty: false,
        },
        file_system_tree_dir,
    })
}
```

#### Note:
  - The **only “open” state we must recover** is the tip hash (`tip_block_hash` key).
  - After this returns, reads like `get_tip_hash()` do **not** need to hit sled for the tip pointer (they read the in-memory lock).

---

### Step 4 — Create path: bootstrap genesis iff the tip key is missing (`create_blockchain`)
- **Call chain**: `BlockchainService::initialize(genesis_address)` → `BlockchainFileSystem::create_blockchain(genesis_address)`

#### Figure 2.4-3-S4: Call flow (create chain; maybe write genesis)

```
BlockchainService::initialize(genesis_address)
  └─> BlockchainFileSystem::create_blockchain(genesis_address)
       ├─> if blocks_tree.get("tip_block_hash") exists: load + cache
       └─> else:
            ├─> build genesis (coinbase tx + genesis block)
            ├─> update_blocks_tree(genesis)  // writes block bytes + tip key atomically
            └─> cache: Blockchain<Db>::tip_hash = genesis_hash
```

- **What this figure shows**
  - **The existence check**: `blocks_tree.get("tip_block_hash")` decides whether we already have a chain on disk.
  - **The bootstrap branch (no tip key)**:
    - Create the genesis *content* (coinbase tx → genesis block).
    - Commit the genesis *persistence* by calling `update_blocks_tree(genesis)` (writes genesis bytes + `"tip_block_hash"` in one transaction).
    - Cache the new head pointer in `Blockchain<Db>::tip_hash`.
  - **The open branch (tip key exists)**:
    - Decode the stored bytes to a `String` tip hash and cache it (no genesis write happens).
  - **How to read the code below**:
    - Watch the `if let Some(data) = data { ... } else { ... }` branch: it is literally the “open vs bootstrap” decision.

#### Code walkthrough

```rust
// BlockchainService::initialize(genesis_address) → BlockchainFileSystem::create_blockchain: bitcoin/src/store/file_system_db_chain.rs
pub async fn create_blockchain(genesis_address: &WalletAddress) -> Result<Self> {
    // 1) Open (or create) the sled DB (path resolution omitted here for brevity).
    let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;

    // 2) Open the blocks tree where we store:
    //    - "<block_hash>"   -> bytes(Block)
    //    - "tip_block_hash" -> "<block_hash>"
    let blocks_tree = db
        .open_tree(file_system_tree_dir.clone())
        .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

    // 3) Existence check: is there already a persisted tip pointer?
    let data = blocks_tree
        .get(DEFAULT_TIP_BLOCK_HASH_KEY)
        .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;

    // 4) Decide which tip hash we will cache in memory.
    let tip_hash = if let Some(data) = data {
        // 4a) Existing chain: decode the persisted tip hash bytes into a String.
        String::from_utf8(data.to_vec())
            .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?
    } else {
        // 4b) No tip key yet: create genesis deterministically from a coinbase transaction.
        let coinbase_tx = Transaction::new_coinbase_tx(genesis_address)?;
        let block = Block::generate_genesis_block(&coinbase_tx);

        // 5) Persist genesis block bytes AND set the tip key in one atomic sled transaction.
        Self::update_blocks_tree(&blocks_tree, &block).await?;

        // 6) Return the new tip hash so we can cache it in memory.
        String::from(block.get_hash())
    };

    // 7) Construct the persistence engine with an in-memory cached tip pointer.
    Ok(BlockchainFileSystem {
        blockchain: Blockchain {
            tip_hash: Arc::new(TokioRwLock::new(tip_hash)), // runtime cache of persisted tip
            db,                                             // sled handle (durable storage)
            is_empty: false,
        },
        file_system_tree_dir, // which sled tree name/path we use for blocks
    })
}
```

#### Note:
  - Genesis is created **exactly once** (when the tip key is absent).
  - The persisted “chain exists” signal is: the `"tip_block_hash"` key is present.

---

### Step 5 — Atomic persistence unit: write block bytes + tip key (`update_blocks_tree`)
- **Call chain (when mining)**: `BlockchainService::mine_block(...)` → `BlockchainFileSystem::mine_block(...)` → `update_blocks_tree(...)`
- **Call chain (when creating genesis)**: `BlockchainService::initialize(...)` → `BlockchainFileSystem::create_blockchain(...)` → `update_blocks_tree(...)`

#### Figure 2.4-3-S5: Call flow (atomic persistence unit)

```
update_blocks_tree(block):
  blocks_tree.transaction {
    insert("<block_hash>" -> bytes(Block))
    insert("tip_block_hash" -> "<block_hash>")
  }
```

- **What this figure shows**
  - **Exactly what is committed atomically** (inside one sled transaction):
    - `insert("<block_hash>" -> bytes(Block))` (the historical log record)
    - `insert("tip_block_hash" -> "<block_hash>")` (the canonical head pointer)
  - **Exactly what is *not* committed atomically here**:
    - In-memory cache updates (`Blockchain<Db>::tip_hash`) happen after the transaction (e.g. in `mine_block`).
    - Derived state (UTXO set) is updated after persistence (separate tree/step).
  - **How this sets up the code below**:
    - The listing immediately below shows the closure passed to `blocks_tree.transaction(|tx_db| ...)` where both inserts happen.

#### Code walkthrough

```rust
async fn update_blocks_tree(blocks_tree: &Tree, block: &Block) -> Result<()> {
    // Key: the block hash string used to locate this block in sled.
    let block_hash = block.get_hash();
    // Value: the serialized bytes of the Block stored under that hash.
    let block_ivec = IVec::try_from(block.clone())?;

    // sled transaction: both inserts commit together (or neither commits).
    let transaction_result: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
        // 1) Persist the historical log record: "<block_hash>" -> bytes(Block)
        let _ = tx_db.insert(block_hash, block_ivec.clone())?;
        // 2) Persist the head pointer: "tip_block_hash" -> "<block_hash>"
        let _ = tx_db.insert(DEFAULT_TIP_BLOCK_HASH_KEY, block_hash)?;
        Ok(())
    });

    transaction_result
        .map(|_| ())
        .map_err(|e| BtcError::BlockchainDBconnection(format!("{:?}", e)))
}
```

#### Note:
  - “Atomic” in this subsection means: **block bytes + tip pointer** are updated together.
  - Derived state (UTXO set) is **not** part of this transaction (it is updated as a later step).

---

### Step 6 — Local extension: chainstate API boundary → persistence engine (`mine_block`)

#### Figure 2.4-3-S6: Call flow (append block via local mining)

```
NodeContext::mine_block(txs)
  └─> BlockchainService::mine_block(txs)
       ├─> verify(txs)                 // gate
       └─> write lock (exclusive)
            └─> BlockchainFileSystem::mine_block(txs)
                 ├─> update_blocks_tree(new_block)
                 ├─> cache: Blockchain<Db>::tip_hash = new_hash
                 └─> update_utxo_set(new_block)
```

- **What this figure shows**
  - **The end-to-end “append block” write path** and where each responsibility lives:
    - `NodeContext` is the coordinator (web/runtime calls it).
    - `BlockchainService` is the mutation boundary (exclusive lock + transaction verification gate).
    - `BlockchainFileSystem` is the persistence engine (sled writes + in-memory tip cache update).
  - **The two state transitions this path must keep consistent**:
    - Persisted: sled log record `<new_hash> -> bytes(Block)` and pointer `"tip_block_hash" -> <new_hash>`.
    - Runtime: cached `Blockchain<Db>::tip_hash = <new_hash>` must match the persisted tip key after the write.
  - **How this sets up the code below**:
    - First listing: `BlockchainService::mine_block` (verify → `write` lock → delegate).
    - Second listing: `BlockchainFileSystem::mine_block` (create block → `update_blocks_tree` → `set_tip_hash` → `update_utxo_set`).

#### Code walkthrough

```rust
// NodeContext::mine_block → BlockchainService::mine_block: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Gate: verify candidate transactions before we persist anything.
    for trasaction in transactions {
        let is_valid = trasaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }

    // Exclusive lock boundary for the chain write path.
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions).await
}
```

```rust
// BlockchainService::mine_block → BlockchainFileSystem::mine_block: bitcoin/src/store/file_system_db_chain.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    let best_height = self.get_best_height().await?;

    // 1) Create the block (PoW is performed inside Block::new_block).
    let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
    let block_hash = block.get_hash();

    // 2) Persist block bytes + update persisted tip key (atomic unit).
    let blocks_tree = self
        .blockchain
        .db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
    Self::update_blocks_tree(&blocks_tree, &block).await?;

    // 3) Update the in-memory cached tip hash to match what we persisted.
    self.set_tip_hash(block_hash).await?;

    // 4) Update derived state (separate tree/step).
    self.update_utxo_set(&block).await?;

    Ok(block)
}
```

#### Note:
  - `BlockchainService::mine_block(...)` is the **exclusive lock boundary** for local chain extension.
  - The gate is: every transaction must pass `Transaction::verify(...)` **before** we acquire the write lock and persist.
  - Persistence is split into:
    - **Atomic persisted unit**: write `<block_hash> -> bytes(Block)` + `"tip_block_hash" -> <block_hash>` (Step 5).
    - **Runtime cache update**: `set_tip_hash(...)` keeps `Blockchain<Db>::tip_hash` consistent with the persisted tip key.
    - **Derived state update**: UTXO updates happen after the block is persisted (separate tree/step).

---

### Step 7 — Iterate the chain through the API (iterator → next → “blocks by height”)

#### Figure 2.4-3-S7: Call flow (iterate blocks from tip backward)

```
caller
  └─> BlockchainService::iterator()
       └─> BlockchainService::read(...)  [shared lock]
            └─> BlockchainFileSystem::iterator()
                 └─> BlockchainIterator::next()
                      └─> sled.get(current_hash) -> Block
                           current_hash = pre_block_hash
```

- **What this figure shows**
  - **Where iteration starts**: `BlockchainFileSystem::iterator()` seeds the iterator with the *current* tip hash (from cached `Blockchain<Db>::tip_hash`).
  - **What each iterator step does**:
    - Use the current hash as a sled key: `db.open_tree(...).get(current_hash)` → bytes(Block)
    - Deserialize → produce a `Block`
    - Update the cursor: `current_hash = block.pre_block_hash` and repeat
  - **What state/storage this touches**:
    - Reads the cached tip pointer (`tip_hash`) once at iterator creation.
    - Performs repeated sled reads keyed by block hash (no tip-key writes; no UTXO updates).
  - **How this sets up the code below**:
    - First listing: `BlockchainService::iterator` (shared lock + delegation).
    - Second listing: `BlockchainFileSystem::iterator` (seed `BlockchainIterator`).
    - Third listing: `BlockchainIterator::next` (the cursor-following loop).

#### Code walkthrough

```rust
// caller → BlockchainService::iterator: bitcoin/src/chain/chainstate.rs
pub async fn iterator(&self) -> Result<BlockchainIterator> {
    // API boundary: shared-lock read + delegation into persistence engine.
    self.read(|blockchain: BlockchainFileSystem| async move { blockchain.iterator().await })
        .await
}
```

```rust
// BlockchainService::iterator → BlockchainFileSystem::iterator: bitcoin/src/store/file_system_db_chain.rs
pub async fn iterator(&self) -> Result<BlockchainIterator> {
    // Seed iteration from the current canonical head pointer.
    let hash = self.get_tip_hash().await?;
    Ok(BlockchainIterator::new(
        hash,
        self.blockchain.db.clone(),
        self.get_blocks_tree_path(), // which sled tree to read from
    ))
}
```

```rust
// BlockchainFileSystem::iterator → BlockchainIterator::{new, next}: bitcoin/src/store/file_system_db_chain.rs
pub struct BlockchainIterator {
    db: Db, // sled DB handle used for lookups
    file_system_blocks_tree: String, // which tree stores "<block_hash>" -> bytes(Block)
    current_hash: String, // cursor: the hash key we will read next
}

impl BlockchainIterator {
    fn new(tip_hash: String, db: Db, file_system_blocks_tree: String) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: tip_hash, // cursor starts at the current canonical tip
            file_system_blocks_tree,
            db,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        // 1) Read the current hash key from the blocks tree.
        // NOTE: this code uses `unwrap()`; in production code we would handle/open-tree errors explicitly.
        let block_tree = self.db.open_tree(self.file_system_blocks_tree.clone()).unwrap();
        let data = match block_tree.get(self.current_hash.clone()) {
            Ok(Some(d)) => d,
            Ok(None) => return None, // empty chain or missing block
            Err(_) => return None,   // DB read error
        };

        // 2) Deserialize block bytes into a Block value.
        let block = match Block::deserialize(data.to_vec().as_slice()) {
            Ok(b) => b,
            Err(_) => return None,
        };

        // 3) Move cursor “backward” to the previous block hash and return this block.
        self.current_hash = block.get_pre_block_hash().clone();
        Some(block)
    }
}
```

```rust
// caller → BlockchainService::get_blocks_by_height: bitcoin/src/chain/chainstate.rs
pub async fn get_blocks_by_height(
    &self,
    initial_height: usize, // lower bound (inclusive)
    height: usize,         // upper bound (inclusive)
) -> Result<Vec<Block>> {
    let mut blocks = Vec::new(); // results accumulator
    let mut iterator = self.iterator().await?; // start at tip and walk backward

    while let Some(block) = iterator.next() {
        let h = block.get_height(); // extract height once to avoid repeating calls

        // Keep blocks in the requested [initial_height, height] range.
        if h <= height && h >= initial_height {
            blocks.push(block.clone());
        }

        // Stop early once we've walked outside the range.
        // (Because we're iterating tip → ... → genesis, heights are monotonically decreasing.)
        if h > height || h < initial_height {
            break;
        }
    }

    Ok(blocks) // return blocks found in range
}
```

#### Note:
  - The iterator is exposed at the **API boundary** (`BlockchainService::iterator`) and implemented in the persistence engine (`BlockchainFileSystem::iterator`).
  - Iteration starts at the current tip hash and repeatedly follows `pre_block_hash` “backward”.
  - `get_blocks_by_height(...)` is a concrete example of a **higher-level API** built on top of that iterator.

---

### Step 8 — Network insertion: chainstate API boundary → persistence engine (`add_block`)

#### Figure 2.4-3-S8: Call flow (append candidate from network)

```
NodeContext::add_block(peer_block)
  └─> BlockchainService::add_block(peer_block)
       └─> write lock (exclusive)
            └─> BlockchainFileSystem::add_block(peer_block)
                 └─> blocks_tree.transaction {
                      insert("<peer_hash>" -> bytes(Block))
                      if peer.height > tip.height:
                        insert("tip_block_hash" -> "<peer_hash>")
                    }
```

- **What this figure shows**
  - **Where the network block enters the chainstate API**:
    - The runtime receives a block from the network and eventually calls `NodeContext::add_block(...)`.
    - `NodeContext` delegates into `BlockchainService::add_block(...)`, which is the exclusive-lock mutation boundary.
  - **What gets written to storage**:
    - Always: persist the candidate block bytes under its hash key (`"<peer_hash>" -> bytes(Block)"`).
    - Sometimes: advance the persisted tip pointer by writing `"tip_block_hash" -> "<peer_hash>"` (only if the candidate “wins” by height in this simplified rule).
  - **What this does *not* do yet** (important for interpretation):
    - It persists the candidate before the full “validate → connect” gate is enforced (that hard gate is still a FIXME elsewhere in the codebase).
  - **How to read the code below**:
    - First listing shows `BlockchainService::add_block` acquiring the exclusive write lock and delegating.
    - Second listing shows `BlockchainFileSystem::add_block` doing idempotence check → serialize → sled transaction → height comparison → possible tip update.

#### Code walkthrough

```rust
// NodeContext::add_block(peer_block) → BlockchainService::add_block: bitcoin/src/chain/chainstate.rs
pub async fn add_block(&self, block: &Block) -> Result<()> {
    // Exclusive lock boundary: network insertion mutates persisted chain state.
    let mut blockchain_guard = self.0.write().await;
    // Delegate to persistence engine for the DB transaction and fork-choice update.
    blockchain_guard.add_block(block).await
}
```

```rust
// BlockchainService::add_block(peer_block) → BlockchainFileSystem::add_block: bitcoin/src/store/file_system_db_chain.rs
pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
    let block_tree = self
        .blockchain
        .db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

    // 1) Idempotence: if we already have this block hash, do nothing.
    if block_tree
        .get(new_block.get_hash())
        .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
        .is_some()
    {
        return Ok(());
    }

    // 2) Serialize the incoming block (what we persist).
    let block_bytes = new_block.serialize()?;
    let tip_hash = self.get_tip_hash().await?;

    // 3) Atomic storage: persist candidate block bytes; update tip key if it “wins” by height.
    let transaction_result: TransactionResult<(), ()> = block_tree.transaction(|transaction| {
        let _ = transaction.insert(new_block.get_hash(), block_bytes.clone())?;

        // Load current tip block inside the transaction so height comparison is consistent.
        let tip_block_bytes = transaction.get(tip_hash.clone())?.ok_or(
            UnabortableTransactionError::Storage(sled::Error::CollectionNotFound(IVec::from(
                tip_hash.as_bytes(),
            ))),
        )?;
        let tip_block = Block::deserialize(tip_block_bytes.as_ref()).map_err(|e| {
            UnabortableTransactionError::Storage(sled::Error::Unsupported(e.to_string()))
        })?;

        // Fork-choice (simplified): higher height becomes the new persisted tip.
        if new_block.get_height() > tip_block.get_height() {
            let _ = transaction.insert(DEFAULT_TIP_BLOCK_HASH_KEY, new_block.get_hash())?;
        }

        Ok(())
    });

    if transaction_result.is_err() {
        return Err(BtcError::BlockchainDBconnection(format!(
            "Transaction failed: {:?}",
            transaction_result
        )));
    }

    // ... consensus tie-break + possible reorg logic follows ...
    Ok(())
}
```

#### Note:
  - The “persist” operation happens **before** any richer “validate → connect” gate (that hard gate is still a FIXME in this implementation; the capstone shows how to finish it).
  - Tip updates are done inside the same sled transaction as the candidate write, but the fork-choice rule is simplified (height-based).

---

<div align="center">

**📚 [← Previous: Blockchain State Management](02-Blockchain-State-Management.md)** | **Chain State and Storage** | **[Next: UTXO Set →](04-UTXO-Set.md)** 📚

</div>

