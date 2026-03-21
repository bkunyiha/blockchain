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

# Section 9.3: Chain State and Storage — How Blocks Become Persistent State

**Important distributed systems concept**: In a blockchain network, each node maintains its own local copy of the blockchain. When we discuss "chain state and storage" in this section, we're describing how one node stores blocks locally—not a shared database that all nodes access. Each node's `BlockchainFileSystem` persists blocks to its own local sled database, and nodes synchronize with each other by exchanging blocks over the network (as we'll see in Section 9.8: Node Orchestration). This decentralization is fundamental to blockchain's resilience: each node operates independently with its own copy of the chain.

## Code walkthrough

### Step 0 — Entry points: a chain read for “status” and a chain write for “append a block”

#### Figure 9-3-S0: Call flow (the two entry points shown below)

```text
GET /api/v1/blockchain  (status read)
  router → get_blockchain_info(handler)
    → NodeContext::get_blockchain_height()
      → BlockchainService::get_best_height()
        → read(...) [shared lock]
        → BlockchainFileSystem::get_best_height()
          → Blockchain<Db>::db (sled) +
            Blockchain<Db>::tip_hash
    → NodeContext::blockchain().get_last_block()
      → BlockchainService::get_last_block()
        → read(...) [shared lock]
        → BlockchainFileSystem::get_last_block()
          → get_tip_hash (tip from cache)
            → sled.get(tip_hash) → Block

POST /api/v1/mining/generatetoaddress  (append-a-block write)
  router → generate_to_address(handler)
    → NodeContext::mine_block(txs)
      → BlockchainService::mine_block(txs) → write lock [exclusive]
        → BlockchainFileSystem::mine_block(txs)
          → update_blocks_tree(new_block) + cache tip_hash + update_utxo_set
```

- **What this figure shows**
  - **What is calling into storage**:
    - Axum router dispatches `GET /api/v1/blockchain`
      to `web/handlers/blockchain.rs::
      get_blockchain_info(...)` (read-heavy status).
    - Axum router dispatches `POST
      /api/v1/mining/generatetoaddress` to
      `web/handlers/mining.rs::generate_to_address
      (...)`, which calls `node.mine_block(...)`.
      (write-heavy “append block” operation)
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


> **Source:** `blockchain.rs` — router → get_blockchain_info(handler)

```rust
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    let height = node
        .get_blockchain_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let last_block = node
        .blockchain()
        .get_last_block()
        .await
        // ... error handling
    let last_block_hash = last_block
        .map(|block| block.get_hash().to_string())
        .unwrap_or_else(|| "genesis".to_string());

    let mempool_size = node.get_mempool_size()?;

    // ... build response ...
    Ok(Json(ApiResponse::success(info)))
}
```


> **Source:** `mining.rs` — router → generate_to_address(handler)

```rust
let mined_block = node
    .mine_block(&transactions)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
```

#### Flow A — Status read (height + tip block): full call chain + code
```text
GET /api/v1/blockchain  (status read)
  router → get_blockchain_info(handler)
    → NodeContext::get_blockchain_height()
      → BlockchainService::get_best_height() → read(...) [shared lock]
        → BlockchainFileSystem::get_best_height()
          → Blockchain<Db>::db (sled) + Blockchain<Db>::tip_hash
```
##### Height read: `NodeContext::get_blockchain_height` → `BlockchainService::get_best_height` → `BlockchainFileSystem::get_best_height`
> **Source:** `context.rs` — get_blockchain_info(handler) → NodeContext::get_blockchain_height

```rust
pub async fn get_blockchain_height(&self) -> Result<usize> {
    // NodeContext is a coordinator; it delegates chain reads to
    // BlockchainService.
    // → BlockchainService::get_best_height(...)
    self.blockchain.get_best_height().await
}
```

> **Source:** `chainstate.rs` — NodeContext::get_blockchain_height → BlockchainService::get_best_height

```rust
pub async fn get_best_height(&self) -> Result<usize> {
    // Shared-lock boundary: many concurrent
    // readers are allowed here.
    self.read(
        // Delegate DB lookup to persistence engine.
        |blockchain: BlockchainFileSystem|
        async move {
            blockchain.get_best_height().await
        },
    )
    .await
}
```

> **Source:** `file_system_db_chain.rs` — BlockchainService
>> ::get_best_height → BlockchainFileSystem
>> ::get_best_height

```rust
pub async fn get_best_height(&self) -> Result<usize> {
    if self.is_empty() {
        Ok(0)
    } else {
        let block_tree = self.blockchain.db
            .open_tree(self.get_blocks_tree_path())?;
        let tip_block_bytes = block_tree
            .get(self.get_tip_hash().await?)?
            // ... error handling
        let tip_block = Block::deserialize(
            tip_block_bytes.as_ref()
        )?;
        Ok(tip_block.get_height())
    }
}
```

**Explain the code**
- `BlockchainService::get_best_height(...)` is a **shared-lock read**: it uses `read(...)` (shared lock) and delegates to `BlockchainFileSystem`.
- `BlockchainFileSystem::get_best_height(...)` hits sled via `self.blockchain.db` and uses the current tip hash (`self.get_tip_hash()`) to load the tip block bytes.

##### Tip-block read: `NodeContext::blockchain().get_last_block` → `BlockchainService::get_last_block` → `BlockchainFileSystem::get_last_block`
```text
    → NodeContext::blockchain().get_last_block()
      → BlockchainService::get_last_block() → read(...) [shared lock]
        → BlockchainFileSystem::get_last_block()
          → get_tip_hash() → sled.get(tip_hash) → Block
```

> **Source:** `file_system_db_chain.rs` — BlockchainService
>> ::get_last_block → BlockchainFileSystem
>> ::get_last_block

```rust
pub async fn get_last_block(&self) -> Result<Option<Block>> {
    // 1) Read cached tip pointer (String).
    let tip_hash = self.get_tip_hash().await?;
    // 2) Use tip hash as sled key to load and
    // deserialize the last block.
    let block =
        self.get_block(tip_hash.as_bytes()).await?;
    Ok(block)
}
```

> **Source:** `chainstate.rs` — NodeContext
>> ::blockchain().get_last_block →
>> BlockchainService::get_last_block

```rust
pub async fn get_last_block(&self) -> Result<Option<Block>> {
    // Shared-lock boundary +
    // delegation into persistence engine.
    self.read(
        |blockchain: BlockchainFileSystem|
        async move {
            blockchain.get_last_block().await
        },
    )
    .await
}
```

> **Source:** `file_system_db_chain.rs` — BlockchainFileSystem::get_last_block → BlockchainFileSystem::get_tip_hash

```rust
pub async fn get_tip_hash(&self) -> Result<String> {
    // This is reading the `tip_hash` field of `Blockchain<T>` (in-memory
    // cache).
    let tip_hash = self.blockchain.tip_hash.read().await;
    Ok(tip_hash.clone())
}
```

**Explain the code**
- `BlockchainService::get_last_block(...)` is also a **shared-lock read**.
- `BlockchainFileSystem::get_last_block(...)` is a “tip pointer + lookup” pattern: read tip hash → get block bytes from sled → deserialize.

##### Where chain state lives: `BlockchainFileSystem` owns `Blockchain<Db>` (tip cache + sled handle)

> **Source:** `file_system_db_chain.rs` — BlockchainFileSystem (owns Blockchain<Db>)

```rust
#[derive(Clone, Debug)]
pub struct BlockchainFileSystem {
    // type is defined in `bitcoin/src/primitives/blockchain.rs`
    blockchain: Blockchain<Db>,
    file_system_tree_dir: String,
}
```

> **Source:** `blockchain.rs` — Blockchain<T> (chain container)

```rust
pub struct Blockchain<T> {
    // in-memory cache of the persisted tip pointer
    pub tip_hash: Arc<TokioRwLock<String>>,
    pub db: T,                              // sled Db handle (persistence)
    pub is_empty: bool,
}
```

#### Flow B — Append a block (write): full call chain + code
```text
    POST /api/v1/mining/generatetoaddress  (append-a-block write)
      router → generate_to_address(handler)
        → NodeContext::mine_block(txs)
          → BlockchainService::mine_block(txs) → write lock [exclusive]
            → BlockchainFileSystem::mine_block(txs)
            → update_blocks_tree(new_block) + cache tip_hash + update_utxo_set
```

##### Write entry point: `node.mine_block(&transactions)`
```text
    router → generate_to_address(handler)
      → NodeContext::mine_block(txs)
```
##### NodeContext delegates into the chainstate API
```text
    → NodeContext::mine_block(txs)
      → BlockchainService::mine_block(txs) → write lock [exclusive]
        → BlockchainFileSystem::mine_block(txs)
        → update_blocks_tree(new_block) + cache tip_hash + update_utxo_set
```
> **Source:** `context.rs` — generate_to_address(handler) → NodeContext::mine_block

```rust
pub async fn mine_block(
    &self,
    transactions: &[Transaction],
) -> Result<Block> {
    // Delegate the write to BlockchainService
    // (which owns the exclusive-lock boundary).
    self.blockchain.mine_block(transactions).await
}
```

##### Exclusive-lock boundary + delegation: `BlockchainService::mine_block(...)`
> **Source:** `chainstate.rs` — NodeContext
>> ::mine_block → BlockchainService::mine_block

```rust
pub async fn mine_block(
    &self,
    transactions: &[Transaction],
) -> Result<Block> {
    // Gate: validate before mutating state.
    for transaction in transactions {
        // `verify(...)` may perform chain lookups;
        // at BlockchainService level, we're
        // still pre-persistence.
        let is_valid = transaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }

    // Exclusive lock boundary (write path):
    // only one writer can append/update state
    // at a time.
    let blockchain_guard = self.0.write().await;
    // Delegate to persistence engine to
    // perform actual DB writes.
    blockchain_guard.mine_block(transactions).await
}
```

##### Persistence engine performs the write: `BlockchainFileSystem::mine_block(...)`
> **Source:** `file_system_db_chain.rs` — BlockchainService
>> ::mine_block → BlockchainFileSystem::mine_block

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    let best_height = self.get_best_height().await?;

    // 1) Build the new block in memory
    // (PoW happens inside Block::new_block).
    let block = Block::new_block(
        self.get_tip_hash().await?,
        transactions,
        best_height + 1
    );
    let block_hash = block.get_hash();

    // 2) Persist `<block_hash> -> bytes(Block)` and `"tip_block_hash" ->
    // <block_hash>` in one transaction.
    let blocks_tree = self.blockchain.db
        .open_tree(self.get_blocks_tree_path())?;
    Self::update_blocks_tree(&blocks_tree, &block)
        .await?;

    // 3) Update in-memory tip cache to match
    // what we just persisted.
    self.set_tip_hash(block_hash).await?;

    // 4) Update derived state (UTXO set)
    // after block bytes + tip pointer are
    // persisted.
    self.update_utxo_set(&block).await?;

    Ok(block)
}
```

#### Note:
  - `BlockchainService::mine_block(...)` is the
    **exclusive lock boundary** for the “append
    block” write path.
  - `BlockchainFileSystem::mine_block(...)` is
    where persistence happens: create block →
    write block bytes + tip key (sled transaction)
    → update in-memory tip cache → update
    derived state (UTXO).

---

### Step 1 — Construct the chainstate API handle (create vs open)
#### Figure 9-3-S1: Call flow (construct the chainstate API handle)

```text
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

> **Source:** `chainstate.rs` — caller (node startup) → BlockchainService::{default, initialize}

```rust
#[derive(Debug)]
pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl BlockchainService {
    pub async fn initialize(
        genesis_address: &WalletAddress,
    ) -> Result<BlockchainService> {
        let blockchain =
            BlockchainFileSystem::create_blockchain(genesis_address).await?;
        Ok(BlockchainService(Arc::new(
            TokioRwLock::new(blockchain)
        )))
    }

    pub async fn default() -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::open_blockchain().await?;
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

#### Figure 9-3-S1.5: Call flow (shared-lock read boundary)

```text
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

> **Source:** `chainstate.rs` — BlockchainService::<read method>() → BlockchainService::read(...)

```rust
impl BlockchainService {
    async fn read<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(BlockchainFileSystem) -> Fut + Send,
        Fut: Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let blockchain_guard = self.0.read().await;
        let blockchain = blockchain_guard.clone();
        f(blockchain).await
    }

    pub async fn get_last_block(&self) -> Result<Option<Block>> {
        self.read(|blockchain| async move {
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
#### Figure 9-3-S2: Call flow (what “exists on disk”)

```text
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

> **Source:** `file_system_db_chain.rs` — BlockchainFileSystem storage constants (tip key + tree names)

```rust
// persisted head pointer (sled key)
const DEFAULT_TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
// placeholder used by this codebase
const DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE: &str = "empty";
// sled tree: "<block_hash>" -> bytes(Block)
const DEFAULT_BLOCKS_TREE: &str = "blocks1";
// on-disk directory for the default sled DB
const DEFAULT_TREE_DIR: &str = "data1";
```

#### Note:
  - The persisted head pointer is **one key**: `"tip_block_hash"`.
  - The historical log is a **tree** that maps `<block_hash> -> serialized Block bytes`.

---

### Step 3 — Open path: load tip from sled into memory (`open_blockchain`)
- **Call chain**: `BlockchainService::default()` → `BlockchainFileSystem::open_blockchain()`

#### Figure 9-3-S3: Call flow (open existing chain)

```text
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
    - `db = sled::open(path)` reconstructs DB.
    - `tip_hash = String::from_utf8(tip_bytes)`
      reconstructs the canonical head pointer.
    - `Blockchain<Db>::tip_hash = tip_hash`
      caches the pointer in memory for
      subsequent reads.
  - **How this sets up the code below**:
    - The next listing shows: open db → open
      tree → get `"tip_block_hash"` → decode →
      cache in `Blockchain { tip_hash, db, ... }`.

#### Code walkthrough

```rust
pub async fn open_blockchain()
    -> Result<BlockchainFileSystem>
{
    let db = sled::open(path)?;
    let blocks_tree = db
        .open_tree(file_system_tree_dir.clone())?;

    let tip_bytes = blocks_tree
        .get(DEFAULT_TIP_BLOCK_HASH_KEY)?
        .ok_or(BtcError::BlockchainNotFoundError(
            “No existing blockchain found.”.to_string(),
        ))?;

    let tip_hash = String::from_utf8(tip_bytes.to_vec())?;

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

#### Figure 9-3-S4: Call flow (create chain; maybe write genesis)

```text
BlockchainService::initialize(genesis_address)
  └─> BlockchainFileSystem::create_blockchain(genesis_address)
       ├─> if blocks_tree.get("tip_block_hash") exists: load + cache
       └─> else:
            ├─> build genesis (coinbase tx + genesis block)
            ├─> update_blocks_tree(genesis)  // atomic write
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

> **Source:** `file_system_db_chain.rs` — BlockchainService::initialize(genesis_address) → BlockchainFileSystem::create_blockchain

```rust
pub async fn create_blockchain(
    genesis_address: &WalletAddress,
) -> Result<Self> {
    let db = sled::open(path)?;
    let blocks_tree =
        db.open_tree(file_system_tree_dir.clone())?;

    let data = blocks_tree.get(DEFAULT_TIP_BLOCK_HASH_KEY)?;

    let tip_hash = if let Some(data) = data {
        String::from_utf8(data.to_vec())?
    } else {
        let coinbase_tx = Transaction::new_coinbase_tx(genesis_address)?;
        let block = Block::generate_genesis_block(&coinbase_tx);
        Self::update_blocks_tree(&blocks_tree, &block).await?;
        String::from(block.get_hash())
    };

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
  - Genesis is created **exactly once** (when the tip key is absent).
  - The persisted “chain exists” signal is: the `"tip_block_hash"` key is present.

---

### Step 5 — Atomic persistence unit: write block bytes + tip key (`update_blocks_tree`)
- **Call chain (when mining)**: `BlockchainService::mine_block(...)` → `BlockchainFileSystem::mine_block(...)` → `update_blocks_tree(...)`
- **Call chain (when creating genesis)**: `BlockchainService::initialize(...)` → `BlockchainFileSystem::create_blockchain(...)` → `update_blocks_tree(...)`

#### Figure 9-3-S5: Call flow (atomic persistence unit)

```text
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
    let block_hash = block.get_hash();
    let block_ivec = IVec::try_from(block.clone())?;

    let transaction_result = blocks_tree.transaction(|tx_db| {
        tx_db.insert(block_hash, block_ivec.clone())?;
        tx_db.insert(DEFAULT_TIP_BLOCK_HASH_KEY, block_hash)?;
        Ok(())
    });

    transaction_result
        .map(|_| ())
        .map_err(|e| BtcError::BlockchainDBconnection(format!("{:?}", e)))
}
```

#### Note:
  - “Atomic” in this subsection means:
    **block bytes + tip pointer** are updated
    together.
  - Derived state (UTXO set) is **not** part
    of this transaction (it is updated as a
    later step).

---

### Step 6 — Local extension: chainstate API boundary → persistence engine (`mine_block`)

#### Figure 9-3-S6: Call flow (append block via local mining)

```text
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

> **Source:** `chainstate.rs` — NodeContext::mine_block → BlockchainService::mine_block

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    for tx in transactions {
        if !tx.verify(self).await? {
            return Err(BtcError::InvalidTransaction);
        }
    }
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions).await
}
```

> **Source:** `file_system_db_chain.rs` — BlockchainService::mine_block → BlockchainFileSystem::mine_block

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    let best_height = self.get_best_height().await?;
    let block = Block::new_block(
        self.get_tip_hash().await?,
        transactions,
        best_height + 1,
    );
    let block_hash = block.get_hash();

    let blocks_tree = self.blockchain.db
        .open_tree(self.get_blocks_tree_path())?;
    Self::update_blocks_tree(&blocks_tree, &block).await?;
    self.set_tip_hash(block_hash).await?;
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

#### Figure 9-3-S7: Call flow (iterate blocks from tip backward)

```text
caller
  └─> BlockchainService::iterator()
       └─> BlockchainService::read(...)  [shared lock]
            └─> BlockchainFileSystem::iterator()
                 └─> BlockchainIterator::next()
                      └─> sled.get(current_hash) -> Block
                           current_hash = pre_block_hash
```

- **What this figure shows**
  - **Where iteration starts**: `BlockchainFileSystem
    ::iterator()` seeds the iterator with the
    *current* tip hash (from cached
    `Blockchain<Db>::tip_hash`).
  - **What each iterator step does**:
    - Use current hash as sled key:
      `db.open_tree(...).get(current_hash)`
      → bytes(Block)
    - Deserialize → produce a `Block`
    - Update cursor:
      `current_hash = block.pre_block_hash`
      and repeat
  - **What state/storage this touches**:
    - Reads cached tip pointer (`tip_hash`)
      once at iterator creation.
    - Performs repeated sled reads keyed by
      block hash (no tip-key writes; no UTXO
      updates).
  - **How this sets up the code below**:
    - First listing: `BlockchainService
      ::iterator` (shared lock + delegation).
    - Second listing: `BlockchainFileSystem
      ::iterator` (seed `BlockchainIterator`).
    - Third listing: `BlockchainIterator::next`
      (the cursor-following loop).

#### Code walkthrough

> **Source:** `chainstate.rs` — caller → BlockchainService::iterator

```rust
pub async fn iterator(
    &self,
) -> Result<BlockchainIterator> {
    self.read(|blockchain| async move {
        blockchain.iterator().await
    })
    .await
}
```

> **Source:** `file_system_db_chain.rs` —
>> BlockchainService::iterator →
>> BlockchainFileSystem::iterator

```rust
pub async fn iterator(
    &self,
) -> Result<BlockchainIterator> {
    let hash = self.get_tip_hash().await?;
    Ok(BlockchainIterator::new(
        hash,
        self.blockchain.db.clone(),
        self.get_blocks_tree_path(),
    ))
}
```

> **Source:** `file_system_db_chain.rs` — BlockchainFileSystem::iterator → BlockchainIterator::{new, next}

```rust
pub struct BlockchainIterator {
    db: Db,
    file_system_blocks_tree: String,
    current_hash: String,
}

impl BlockchainIterator {
    fn new(
        tip_hash: String,
        db: Db,
        file_system_blocks_tree: String,
    ) -> Self {
        BlockchainIterator {
            current_hash: tip_hash,
            file_system_blocks_tree,
            db,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self.db
            .open_tree(self.file_system_blocks_tree.clone())
            .ok()?;
        let data = block_tree
            .get(self.current_hash.clone())
            .ok()
            .flatten()?;
        let block = Block::deserialize(data.to_vec().as_slice())
            .ok()?;
        self.current_hash = block.get_pre_block_hash().clone();
        Some(block)
    }
}
```

> **Source:** `chainstate.rs` — caller → BlockchainService::get_blocks_by_height

```rust
pub async fn get_blocks_by_height(
    &self,
    initial_height: usize,
    height: usize,
) -> Result<Vec<Block>> {
    let mut blocks = Vec::new();
    let mut iterator = self.iterator().await?;

    while let Some(block) = iterator.next() {
        let h = block.get_height();
        if h <= height && h >= initial_height {
            blocks.push(block.clone());
        }
        if h < initial_height {
            break;
        }
    }
    Ok(blocks)
}
```

#### Note:
  - The iterator is exposed at the **API boundary** (`BlockchainService::iterator`) and implemented in the persistence engine (`BlockchainFileSystem::iterator`).
  - Iteration starts at the current tip hash and repeatedly follows `pre_block_hash` “backward”.
  - `get_blocks_by_height(...)` is a concrete example of a **higher-level API** built on top of that iterator.

---

### Step 8 — Network insertion: chainstate API boundary → persistence engine (`add_block`)

#### Figure 9-3-S8: Call flow (append candidate from network)

```text
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

> **Source:** `chainstate.rs` — NodeContext::add_block(peer_block) → BlockchainService::add_block

```rust
pub async fn add_block(&self, block: &Block) -> Result<()> {
    let mut blockchain_guard = self.0.write().await;
    blockchain_guard.add_block(block).await
}
```

> **Source:** `file_system_db_chain.rs` — BlockchainService::add_block(peer_block) → BlockchainFileSystem::add_block

```rust
pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
    let block_tree = self.blockchain.db.open_tree(self.get_blocks_tree_path())?;

    if block_tree.get(new_block.get_hash())?.is_some() {
        return Ok(());
    }

    let block_bytes = new_block.serialize()?;
    let tip_hash = self.get_tip_hash().await?;

    let transaction_result = block_tree.transaction(|tx| {
        tx.insert(new_block.get_hash(), block_bytes.clone())?;
        // ... (load tip block and compare heights)
        if new_block.get_height() > /* tip height */ {
            tx.insert(DEFAULT_TIP_BLOCK_HASH_KEY, new_block.get_hash())?;
        }
        Ok(())
    });

    transaction_result
        .map_err(|e| BtcError::BlockchainDBconnection(format!(“{:?}”, e)))?;

    // ... consensus tie-break + reorg logic ...
    Ok(())
}
```

#### Note:
  - The “persist” operation happens **before** any richer “validate → connect” gate (that hard gate is still a FIXME in this implementation; the capstone shows how to finish it).
  - Tip updates are done inside the same sled transaction as the candidate write, but the fork-choice rule is simplified (height-based).

---

<div align="center">

**[← Previous: Blockchain State Management](02-Blockchain-State-Management.md)** | **Chain State and Storage** | **[Next: UTXO Set →](04-UTXO-Set.md)**

</div>

