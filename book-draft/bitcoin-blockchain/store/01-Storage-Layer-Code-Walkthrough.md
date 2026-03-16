<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="README.md">Chapter 11: Storage Layer</a>
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

# Storage Layer — Code Walkthrough (sled + `BlockchainFileSystem`)

**Part I: Foundations & Core Implementation** | **Chapter 11.A: Storage Layer — Code Walkthrough**

This chapter is a code-complete walkthrough of `bitcoin/src/store`. The intent is that you can understand the storage model **without having the repository open**.

Two promises:

1. **Complete method coverage**: every function we reference is printed with its full method body in this chapter (or explicitly marked “defined earlier” and linked).
2. **No signature-only snippets**: any snippet that introduces a method is followed immediately by the full listing.

---

## 0. What this module stores (and what it deliberately does not)

This module is responsible for:

- opening an embedded database on disk (sled)
- persisting blocks (`block_hash -> bytes`)
- persisting the tip pointer (`"tip_block_hash" -> "<hash>"`)
- reading blocks / heights back out for network sync and UI queries

It deliberately does not try to be “a SQL schema”. It is a small key-value layout where the **code is the schema**.

### Diagram: write paths and atomicity boundaries

```text
Mining path (local):
  mine_block -> update_blocks_tree (atomic)
              -> update derived state (UTXO set)

Inbound block path (network):
  add_block -> insert (sled) -> consensus
            -> maybe set tip / reorganize
```

> **Methods involved**
>
> - `BlockchainFileSystem::{create_blockchain, open_blockchain, open_blockchain_empty}`
> - `BlockchainFileSystem::{get_tip_hash, set_tip_hash}`
> - `BlockchainFileSystem::update_blocks_tree` (internal atomic write)
> - `BlockchainFileSystem::{get_best_height, get_block, get_block_hashes}`
> - `BlockchainFileSystem::{mine_block, add_block}`

---

## 1. Initialization: creating or opening the DB

The storage location is chosen by environment variables:

- `TREE_DIR` (directory on disk; default `"data1"`)
- `BLOCKS_TREE` (sled tree name; default `"blocks1"`)

### Diagram: config → path → DB + tree

```text
TREE_DIR="data1"      BLOCKS_TREE="blocks1"
      |                    |
      v                    v
./data1/  (sled database)  Tree("blocks1")
```

> **Methods involved**
>
> - `BlockchainFileSystem::create_blockchain(...)`
> - `BlockchainFileSystem::open_blockchain(...)`
> - `BlockchainFileSystem::open_blockchain_empty(...)`

### Code Listing 10A-1.1 — Type definition and blockchain creation (part 1) (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
use crate::error::{BtcError, Result};
use crate::primitives::block::Block;
use crate::primitives::blockchain::Blockchain;
use crate::primitives::transaction::Transaction;
use crate::wallet::WalletAddress;
use sled::{Db, Tree};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

const DEFAULT_TIP_BLOCK_HASH_KEY: &str = “tip_block_hash”;
const DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE: &str = “empty”;
const DEFAULT_BLOCKS_TREE: &str = “blocks1”;
const DEFAULT_TREE_DIR: &str = “data1”;

#[derive(Clone, Debug)]
pub struct BlockchainFileSystem {
    blockchain: Blockchain<Db>,  // sled DB + tip pointer (cached in memory)
    file_system_tree_dir: String,
}

impl BlockchainFileSystem {
    pub async fn create_blockchain(
        genesis_address: &WalletAddress,
    ) -> Result<Self> {
        let blocks_tree_name = env::var(“BLOCKS_TREE”)
            .unwrap_or_else(|_| DEFAULT_BLOCKS_TREE.to_string());
        let dir = env::var(“TREE_DIR”)
            .unwrap_or_else(|_| DEFAULT_TREE_DIR.to_string());
        let path = std::env::current_dir()?
            .join(dir.clone());

        let db = sled::open(path)?;
        let blocks_tree = db.open_tree(blocks_tree_name.clone())?;

        // Check if chain exists, else create genesis
        let tip_hash =
            if let Some(data) = blocks_tree.get(DEFAULT_TIP_BLOCK_HASH_KEY)? {
            String::from_utf8(data.to_vec())?
        } else {
            // Initialize: create genesis block, persist + set tip atomically
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
            file_system_tree_dir: blocks_tree_name,
        })
    }
```

The creation path for genesis is followed by methods to open an existing blockchain or open a handle without loading state:

### Code Listing 10A-2 — Open existing or empty blockchain (part 2)

```rust
    pub async fn open_blockchain() -> Result<Self> {
        let blocks_tree_name = env::var(“BLOCKS_TREE”)
            .unwrap_or_else(|_| DEFAULT_BLOCKS_TREE.to_string());
        let dir = env::var(“TREE_DIR”)
            .unwrap_or_else(|_| DEFAULT_TREE_DIR.to_string());
        let path = std::env::current_dir()?.join(dir);

        let db = sled::open(path)?;
        let blocks_tree = db.open_tree(blocks_tree_name.clone())?;

        let tip_bytes = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)?
            .ok_or(BtcError::BlockchainNotFoundError(
                “No blockchain found”.into(),
            ))?;
        let tip_hash = String::from_utf8(tip_bytes.to_vec())?;

        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: false,
            },
            file_system_tree_dir: blocks_tree_name,
        })
    }

    pub async fn open_blockchain_empty() -> Result<Self> {
        let blocks_tree_name = env::var(“BLOCKS_TREE”)
            .unwrap_or_else(|_| DEFAULT_BLOCKS_TREE.to_string());
        let dir = env::var(“TREE_DIR”)
            .unwrap_or_else(|_| DEFAULT_TREE_DIR.to_string());
        let path = std::env::current_dir()?.join(dir);

        let db = sled::open(path)?;
        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(“empty”.to_string())),
                db,
                is_empty: true,
            },
            file_system_tree_dir: blocks_tree_name,
        })
    }
}
```

---

## 2. Atomic storage primitive: `update_blocks_tree` (insert + tip move)

When we create genesis or mine locally, we want “block insert” and “tip pointer update” to happen as a single atomic unit.

> **Methods involved**
>
> - `BlockchainFileSystem::update_blocks_tree(...)`

### Code Listing 10A-6 — Atomic insert + tip update (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
async fn update_blocks_tree(blocks_tree: &Tree, block: &Block) -> Result<()> {
    // Hash is the key; a serialized block is the value.
    let block_hash = block.get_hash();
    let block_ivec = IVec::try_from(block.clone())?;
    // sled transactions give us atomicity across multiple inserts into the same
    // Tree.
    let transaction_result: TransactionResult<(), ()> =
        blocks_tree.transaction(|tx_db| {
        // 1) Persist the block bytes under its hash.
        let _ = tx_db.insert(block_hash, block_ivec.clone())?;
        // 2) Move the “tip pointer” to the same hash.
        // If the process crashes after this transaction commits, both updates
        // are present.
        let _ = tx_db.insert(DEFAULT_TIP_BLOCK_HASH_KEY, block_hash)?;
        Ok(())
    });
    transaction_result
        .map(|_| ())
        .map_err(|e| BtcError::BlockchainDBconnection(format!("{:?}", e)))
}
```

---

## 3. Tip pointer API: `get_tip_hash` and `set_tip_hash`

The tip is also cached in memory (an async `RwLock<String>`) for fast access during runtime.

> **Methods involved**
>
> - `BlockchainFileSystem::get_tip_hash(...)`
> - `BlockchainFileSystem::set_tip_hash(...)` (internal)

### Code Listing 10A-3.1 — Tip pointer helpers (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn get_tip_hash(&self) -> Result<String> {
    // Read-lock: multiple readers can read the cached tip concurrently.
    let tip_hash = self.blockchain.tip_hash.read().await;
    Ok(tip_hash.clone())
}

async fn set_tip_hash(&self, new_tip_hash: &str) -> Result<()> {
    // Write-lock: tip updates are exclusive.
    let mut tip_hash = self.blockchain.tip_hash.write().await;
    // Update the in-memory cached tip (DB tip key updates happen elsewhere).
    *tip_hash = String::from(new_tip_hash);
    Ok(())
}
```

---

## 4. Read path: height, get-by-hash, and “hash inventory”

The network layer and UI need:

- current height (`get_best_height`)
- fetch-by-hash (`get_block`)
- list hashes (`get_block_hashes`) for sync inventory

> **Methods involved**
>
> - `BlockchainFileSystem::get_best_height(...)`
> - `BlockchainFileSystem::get_block(...)`
> - `BlockchainFileSystem::get_block_hashes(...)`

### Code Listing 10A-16 — Read path (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn get_best_height(&self) -> Result<usize> {
    if self.is_empty() {
        // An “empty handle” reports height 0 (genesis is height 1 in this
        // codebase).
        info!("Blockchain is empty, returning height 0");
        Ok(0)
    } else {
        // Open the blocks tree so we can fetch the tip block bytes.
        let block_tree = self
            .blockchain
            .db
            .open_tree(self.get_blocks_tree_path())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;
        // Use the cached tip hash to look up the corresponding block record.
        let tip_block_bytes = block_tree
            .get(self.get_tip_hash().await?)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
            .ok_or(BtcError::GetBlockchainError("tip is invalid".to_string()))?;
        // Deserialize bytes back into a Block so we can read its height.
        let tip_block = Block::deserialize(tip_block_bytes.as_ref())?;
        Ok(tip_block.get_height())
    }
}

pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
    // Read-by-hash is a straight key-value lookup.
    let block_tree = self
        .blockchain
        .db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;
    let block_bytes = block_tree
        .get(block_hash)
        .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;

    if let Some(block_bytes) = block_bytes {
        // Found a record: decode it back into a Block.
        let block = Block::deserialize(block_bytes.as_ref())?;
        Ok(Some(block))
    } else {
        // Missing key: caller gets None (not an error).
        Ok(None)
    }
}

pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
    // “Inventory” for sync: walk the chain backward and collect hashes.
    let mut iterator = self.iterator().await?;
    let mut blocks = vec![];
    loop {
        match iterator.next() {
            None => break,
            Some(block) => {
                blocks.push(block.get_hash_bytes());
            }
        }
    }
    Ok(blocks)
}
```

---

## 5. Mining write path: `mine_block` (persist + move tip + update UTXO)

Local mining creates a new `Block`, writes it into sled, updates the in-memory tip, then updates derived state.

> **Methods involved**
>
> - `BlockchainFileSystem::mine_block(...)`
> - `BlockchainFileSystem::update_blocks_tree(...)` (defined earlier)
> - `BlockchainFileSystem::set_tip_hash(...)` (defined earlier)

### Code Listing 10A-18 — Mining write path (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Height is derived from the current tip.
    let best_height = self.get_best_height().await?;

    // Construct a new Block that points to the current tip as its parent.
    let block = Block::new_block(
        self.get_tip_hash().await?,
        transactions,
        best_height + 1,
    );
    let block_hash = block.get_hash();

    // Open the blocks tree for writing.
    let blocks_tree = self
        .blockchain
        .db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
    // Atomically insert the block and update the DB-side tip key.
    Self::update_blocks_tree(&blocks_tree, &block).await?;
    // Update the in-memory cached tip pointer to match the DB.
    self.set_tip_hash(block_hash).await?;

    // Update derived state (UTXO set) so spending rules and balance queries
    // stay correct.
    self.update_utxo_set(&block).await?;

    Ok(block)
}
```

---

## 6. Inbound write path: `add_block` (persist + consensus + possible reorg)

Inbound blocks are first persisted, then the node runs consensus decisions to determine whether the tip should move or whether a reorganization is required.

This method is intentionally printed in full so the reader can trace storage + consensus boundaries without the repo open.

> **Methods involved**
>
> - `BlockchainFileSystem::add_block(...)`

### Code Listing 10A-6.1 — Inbound block persistence and dedup (part 1) (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
    // Inbound: persist first → then consensus decision → maybe reorg.
    let block_tree = self.blockchain.db.open_tree(self.get_blocks_tree_path())?;

    if self.is_empty() {
        // First block always becomes tip
        self.set_not_empty();
        Self::update_blocks_tree(&block_tree, new_block).await?;
        self.set_tip_hash(new_block.get_hash()).await?;
        self.update_utxo_set(new_block).await?;
        return Ok(());
    }

    // Dedup: skip if already present
    if block_tree.get(new_block.get_hash())?.is_some() {
        return Ok(());
    }

    // Atomic insert within transaction
    let block_bytes = new_block.serialize()?;
    let tip_hash = self.get_tip_hash().await?;
    block_tree.transaction(|txn| {
        txn.insert(new_block.get_hash(), block_bytes.clone())?;
        // Eagerly move tip if block is strictly higher
        if new_block.get_height() > Block::deserialize(
            txn.get(tip_hash.clone())?.ok_or(...)?)?. get_height() {
            txn.insert(DEFAULT_TIP_BLOCK_HASH_KEY, new_block.get_hash())?;
        }
        Ok(())
    })?;
```

After persisting the block, we run consensus rules to decide whether to update the tip or reorganize:

### Code Listing 10A-6.2 — Fork choice and reorg decision (part 2)

```rust
    // Fork choice: compare height, work, then tie-break
    let current_tip = self.get_tip_hash().await?;
    let current_height = self.get_best_height().await?;

    match new_block.get_height().cmp(&current_height) {
        Ordering::Greater => {
            // Longest chain rule: higher always wins
            self.set_tip_hash(new_block.get_hash()).await?;
            self.update_utxo_set(new_block).await?;
        }
        Ordering::Equal => {
            if new_block.get_pre_block_hash() == current_tip {
                // Extends our tip: accept naturally
                return Ok(());
            }
            // Competing tips: compare cumulative work
            let current_work = self.get_chain_work(&current_tip).await?;
            let new_work = self.get_chain_work(new_block.get_hash()).await?;
            match new_work.cmp(&current_work) {
                Ordering::Greater => {
            self.reorganize_chain(new_block.get_hash()).await?
        }
                Ordering::Equal => {
                    // Deterministic tie-break for consensus
                    if self
                        .accept_new_block_tie_break(new_block, &current_tip)
                        .await?
                    {
                        self.reorganize_chain(new_block.get_hash()).await?;
                    }
                }
                Ordering::Less => {}
            }
        }
        Ordering::Less => {}
    }
    Ok(())
}
```

---

## Summary: what to remember

- sled is the persistence engine; the “schema” is just key conventions in code.
- `update_blocks_tree` is the atomic primitive used when we can deterministically move tip.
- `mine_block` is “local write path”: create block → persist → move tip → update derived state.
- `add_block` is “inbound path”: persist → consensus decision → maybe reorg.

---

<div align="center">

**[← Chapter 11: Storage Layer](README.md)** | **Chapter 11.A: Code Walkthrough** | **[Next: Chapter 12 (Network Layer) →](../net/README.md)** 

</div>

---

