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
10. **Chapter 2.5: Storage Layer** ← *You are here*
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

# Storage Layer — Code Walkthrough (sled + `BlockchainFileSystem`)

**Part I: Core Blockchain Implementation** | **Chapter 2.5.A: Storage Layer — Code Walkthrough**

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

```
Mining path (local):
  mine_block -> update_blocks_tree (atomic insert + tip move) -> update derived state (UTXO set)

Inbound block path (network):
  add_block -> insert block (sled transaction) -> consensus decision -> (maybe) set tip / reorganize
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

```
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

### Code Listing 2.5A-1.1 — Create/open helpers (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
use crate::error::{BtcError, Result};
use crate::primitives::block::{Block, GENESIS_BLOCK_PRE_BLOCK_HASH};
use crate::primitives::blockchain::Blockchain;
use crate::primitives::transaction::Transaction;
use crate::wallet::WalletAddress;
use sled::transaction::TransactionResult;
use sled::{Db, IVec, Tree};
use std::env;
use std::env::current_dir;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

// Key used inside the blocks tree to point at the current canonical tip hash.
const DEFAULT_TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
// Used when we intentionally want a “no chain yet” handle (tests / bootstrap paths).
const DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE: &str = "empty";
// Default sled tree name for block storage (can be overridden via env var).
const DEFAULT_BLOCKS_TREE: &str = "blocks1";
// Default on-disk directory for sled (can be overridden via env var).
const DEFAULT_TREE_DIR: &str = "data1";

#[derive(Clone, Debug)]
pub struct BlockchainFileSystem {
    // Generic “blockchain handle” that bundles: the sled DB, the tip pointer, and an empty flag.
    blockchain: Blockchain<Db>,
    // Name of the sled Tree that stores blocks (hash -> bytes).
    file_system_tree_dir: String,
}

impl BlockchainFileSystem {
    pub async fn create_blockchain(genesis_address: &WalletAddress) -> Result<Self> {
        // Decide where the sled database lives on disk.
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        // Decide which sled Tree inside the DB will store our blocks.
        let file_system_tree_dir =
            env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string());
        // Build the absolute DB path as: current_dir()/TREE_DIR.
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        // Open (or create) the sled DB at that path.
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        // Open (or create) the “blocks tree”.
        let blocks_tree = db
            .open_tree(file_system_tree_dir.clone())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        // Check whether a tip key already exists. If it does, we’re reopening an existing chain.
        let data = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;
        let tip_hash = if let Some(data) = data {
            // Tip is stored as UTF-8 bytes containing the hash string.
            String::from_utf8(data.to_vec())
                .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?
        } else {
            // No tip key means: initialize chain by creating genesis.
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address)?;
            let block = Block::generate_genesis_block(&coinbase_tx);
            // Persist genesis block + set tip atomically.
            Self::update_blocks_tree(&blocks_tree, &block).await?;
            String::from(block.get_hash())
        };

        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                // Tip is cached in memory so “what’s the tip?” doesn’t hit the DB every time.
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: false,
            },
            file_system_tree_dir,
        })
    }

    pub async fn open_blockchain() -> Result<BlockchainFileSystem> {
        // Same config and open steps as create_blockchain(), except we *require* the tip key.
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        let file_system_tree_dir =
            env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string());
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let blocks_tree = db
            .open_tree(file_system_tree_dir.clone())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        // If the tip key is missing, we treat this as “no chain found”.
        let tip_bytes = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
            .ok_or(BtcError::BlockchainNotFoundError(
                "No existing blockchain found. Connect to a blcock chain cluster first."
                    .to_string(),
            ))?;
        // Decode the tip hash string from bytes.
        let tip_hash = String::from_utf8(tip_bytes.to_vec())
            .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?;
        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: false,
            },
            file_system_tree_dir,
        })
    }

    pub async fn open_blockchain_empty() -> Result<BlockchainFileSystem> {
        // Creates a handle that points at a DB, but marks it as empty.
        // This is useful in some test/setup flows where we *don’t* want genesis auto-created.
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        let file_system_tree_dir =
            env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string());
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let tip_hash = DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE.to_string();

        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: true,
            },
            file_system_tree_dir,
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

### Code Listing 2.5A-2.1 — Atomic insert + tip update (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
async fn update_blocks_tree(blocks_tree: &Tree, block: &Block) -> Result<()> {
    // Hash is the key; a serialized block is the value.
    let block_hash = block.get_hash();
    let block_ivec = IVec::try_from(block.clone())?;
    // sled transactions give us atomicity across multiple inserts into the same Tree.
    let transaction_result: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
        // 1) Persist the block bytes under its hash.
        let _ = tx_db.insert(block_hash, block_ivec.clone())?;
        // 2) Move the “tip pointer” to the same hash.
        // If the process crashes after this transaction commits, both updates are present.
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

### Code Listing 2.5A-3.1 — Tip pointer helpers (`bitcoin/src/store/file_system_db_chain.rs`)

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

### Code Listing 2.5A-4.1 — Read path (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn get_best_height(&self) -> Result<usize> {
    if self.is_empty() {
        // An “empty handle” reports height 0 (genesis is height 1 in this codebase).
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

### Code Listing 2.5A-5.1 — Mining write path (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Height is derived from the current tip.
    let best_height = self.get_best_height().await?;

    // Construct a new Block that points to the current tip as its parent.
    let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
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

    // Update derived state (UTXO set) so spending rules and balance queries stay correct.
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

### Code Listing 2.5A-6.1 — Inbound block handling (`bitcoin/src/store/file_system_db_chain.rs`)

```rust
pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
    // Inbound blocks arrive from the network. We persist first, then decide whether it becomes tip.
    let block_tree = self
        .blockchain
        .db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

    if self.is_empty() {
        // Special-case: if the chain was empty, any first block becomes the tip.
        info!("Blockchain is empty, adding block");

        self.set_not_empty();
        info!("Blockchain is now not empty");
        // Atomic insert + DB tip key update.
        Self::update_blocks_tree(&block_tree, new_block).await?;
        // Keep in-memory cached tip consistent with DB.
        self.set_tip_hash(new_block.get_hash()).await?;

        // Update derived state so the chainstate matches the canonical chain.
        self.update_utxo_set(new_block).await?;

        let best_height = self.get_best_height().await?;
        info!(
            "Blockchain is now not empty, best height is {}",
            best_height
        );
        return Ok(());
    } else {
        // Dedup: if we already have this block, treat as no-op.
        let block_bytes = block_tree
            .get(new_block.get_hash())
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;
        if block_bytes.is_some() {
            return Ok(());
        }

        // Persist the new block bytes first, inside a sled transaction.
        // Note: the full “validate → connect” rules are covered in Chapter 2.6.
        // FIXME: From bitcoint whitepaper, only add block if:
        // A) “All transactions in it are valid”
        // B) “Not already spent”
        // See book-draft/bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md
        let block_bytes = new_block.serialize()?;
        // We snapshot the current tip hash for comparison inside the transaction closure.
        let tip_hash = self.get_tip_hash().await?;
        let transaction_result: TransactionResult<(), ()> =
            block_tree.transaction(|transaction| {
                // Store the block record by hash.
                let _ = transaction.insert(new_block.get_hash(), block_bytes.clone())?;

                // Read the current tip block bytes *within the same transaction view*.
                let tip_block_bytes = transaction.get(tip_hash.clone())?.ok_or(
                    UnabortableTransactionError::Storage(sled::Error::CollectionNotFound(
                        IVec::from(tip_hash.as_bytes()),
                    )),
                )?;

                // Decode the tip block so we can compare heights.
                let tip_block = Block::deserialize(tip_block_bytes.as_ref()).map_err(|e| {
                    UnabortableTransactionError::Storage(sled::Error::Unsupported(
                        e.to_string(),
                    ))
                })?;

                if self.is_empty() || new_block.get_height() > tip_block.get_height() {
                    // If the new block is strictly higher, we can eagerly move the DB-side tip key.
                    info!("Block height is higher, updating tip in transaction");
                    let _ =
                        transaction.insert(DEFAULT_TIP_BLOCK_HASH_KEY, new_block.get_hash())?;
                } else {
                    // Otherwise we defer: consensus logic after the transaction may reorganize.
                    info!("Block height is same or lower, will use tie-breaking logic");
                    info!(
                        "Block {:?} not added because its height is less than mine",
                        new_block.get_hash()
                    );
                }

                Ok(())
            });

        // Check if transaction succeeded
        if transaction_result.is_err() {
            return Err(BtcError::BlockchainDBconnection(format!(
                "Transaction failed: {:?}",
                transaction_result
            )));
        }

        // After persistence, decide whether this block becomes part of the canonical chain.
        // This is the “fork choice” boundary (height → work → tie-break).
        if !self.is_empty() {
            // Read current tip/height *after* the insert transaction completed.
            let current_tip = self.get_tip_hash().await?;
            let current_height = self.get_best_height().await?;

            match new_block.get_height().cmp(&current_height) {
                Ordering::Greater => {
                    // Higher height wins immediately in this simplified longest-chain rule.
                    self.set_tip_hash(new_block.get_hash()).await?;

                    // Update UTXO set when accepting block with higher height
                    self.update_utxo_set(new_block).await?;

                    info!(
                        "Block {} accepted: higher height ({} > {}) - longest chain rule",
                        new_block.get_hash(),
                        new_block.get_height(),
                        current_height
                    );
                }
                Ordering::Equal => {
                    // Competing blocks at the same height require work comparison.
                    if new_block.get_pre_block_hash() == current_tip {
                        // If it extends our tip, it isn’t a competitor; it’s the next block.
                        info!(
                            "Block {} is the next block in our chain (height {}), accepting without reorganization",
                            new_block.get_hash(),
                            new_block.get_height()
                        );
                        return Ok(());
                    }

                    // Compare cumulative work for the two competing tips.
                    let current_work = self.get_chain_work(&current_tip).await?;

                    // The block was inserted in the transaction above, but we re-check presence.
                    let block_already_exists = self
                        .get_block(new_block.get_hash().as_bytes())
                        .await?
                        .is_some();

                    // If the block did not already exist, we temporarily insert to compute work.
                    let temp_block_tree = if !block_already_exists {
                        let block_bytes = new_block.serialize()?;
                        let tree = self
                            .blockchain
                            .db
                            .open_tree(self.get_blocks_tree_path())
                            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;
                        tree.insert(new_block.get_hash(), block_bytes)
                            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
                        Some(tree)
                    } else {
                        None
                    };

                    let new_work = self.get_chain_work(new_block.get_hash()).await?;

                    match new_work.cmp(&current_work) {
                        Ordering::Greater => {
                            // Stronger competing branch: reorganize to it.
                            info!(
                                "Reorganizing chain: new work {} > current work {} - stronger competing chain",
                                new_work, current_work
                            );
                            self.reorganize_chain(new_block.get_hash()).await?;
                        }
                        Ordering::Equal => {
                            // Equal work: deterministic tie-break so all nodes converge.
                            if self
                                .accept_new_block_tie_break(new_block, &current_tip)
                                .await?
                            {
                                info!(
                                    "Reorganizing chain via tie-breaking: new work {} == current work {}",
                                    new_work, current_work
                                );
                                self.reorganize_chain(new_block.get_hash()).await?;
                                info!(
                                    "Block {} accepted via tie-breaking",
                                    new_block.get_hash()
                                );
                            } else {
                                // Tie-break rejected the new branch; remove any temporary insert.
                                info!(
                                    "Block {} rejected via tie-breaking",
                                    new_block.get_hash()
                                );
                                if let Some(tree) = &temp_block_tree {
                                    tree.remove(new_block.get_hash()).map_err(|e| {
                                        BtcError::BlockchainDBconnection(e.to_string())
                                    })?;
                                }
                            }
                        }
                        Ordering::Less => {
                            // Weaker branch: reject and cleanup any temporary insert.
                            info!(
                                "Block {} rejected: work {} < current work {} - weaker competing chain",
                                new_block.get_hash(),
                                new_work,
                                current_work
                            );
                            if let Some(tree) = &temp_block_tree {
                                tree.remove(new_block.get_hash()).map_err(|e| {
                                    BtcError::BlockchainDBconnection(e.to_string())
                                })?;
                            }
                        }
                    }
                }
                Ordering::Less => {
                    // Shorter chain: reject.
                    info!(
                        "Block {} rejected: height {} < current height {} - shorter chain",
                        new_block.get_hash(),
                        new_block.get_height(),
                        current_height
                    );
                }
            }
        }
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

**📚 [← Chapter 2.5: Storage Layer](README.md)** | **Chapter 2.5.A: Code Walkthrough** | **[Next: Chapter 2.6 (Block Acceptance) →](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)** 📚

</div>

---

