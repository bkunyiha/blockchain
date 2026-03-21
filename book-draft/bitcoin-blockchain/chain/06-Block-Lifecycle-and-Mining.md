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

## Section 9.6: Block Lifecycle and Mining

In the previous section (Section 9.5, **Transaction Lifecycle**), we followed a transaction from construction and signing, through mempool admission and propagation. That brings us to the natural next question: **what happens to those pending transactions next?**

This section picks up from that boundary and traces the rest of the path: how the node assembles a candidate block from the mempool, performs proof-of-work, persists the new tip, advances the UTXO set, and relays the new block to peers. We will walk through the code: key methods are printed, and each listing is followed by a succinct explanation of what the code is doing and why.

## Scope within Section 9 (section flow)

This subsection covers **Section 9 (Blockchain — From Transaction to Block Acceptance) Steps 5–7**: candidate assembly, proof-of-work, persistence, and the UTXO state transition after a block is connected.

## Reader promise (what you will understand after this section)

After reading this section, you should be able to explain—using concrete method names—how the code moves from **pending transactions** to a **mined, persisted, and relayed block**, including where the implementation:

- Builds the candidate transaction list (mempool snapshot + coinbase).
- Constructs a block header and commits to transaction data.
- Performs proof-of-work (the nonce search and the bytes being hashed).
- Persists the new tip and applies the UTXO state transition.
- Announces blocks to peers and handles block download (INV → GETDATA → BLOCK).

### Quick terminology (used throughout)

- **mempool**: an in-memory pool of **accepted transactions** that are **waiting to be included in a block** (implemented here as `GLOBAL_MEMORY_POOL`)
- **coinbase transaction**: by convention, the **first transaction in a block** is a coinbase transaction. It has no real inputs (there are no earlier coins to spend), and it **creates new coins owned by the miner** as the block reward—this is the Bitcoin whitepaper’s incentive mechanism (**§6: “the first transaction in a block is a special transaction that starts a new coin owned by the creator of the block”**). In this implementation it is created with `Transaction::new_coinbase_tx(...)`.
- **tip**: the current best block hash this node considers canonical (the “head” of the chain)
- **blocks tree**: the on-disk key/value store of blocks (hash → serialized block)
- **UTXO set / chainstate**: derived state answering “what outputs are spendable right now?” (stored under the `"chainstate"` tree)
- **INV / GETDATA / BLOCK**: announce an object by id → request bytes → transmit full bytes

### The three boundaries to watch for

This section is easiest to follow if you keep three “boundaries” in mind:

- **mining boundary**: the “last step” before doing proof-of-work—every candidate transaction must be verified before we are willing to mine it (`BlockchainService::mine_block`)
- **persistence + state-transition boundary**: the moment a mined block becomes part of our local chain—write the block as the new tip, then update the UTXO set so spendability matches the new tip (`BlockchainFileSystem::mine_block` → `update_utxo_set`)
- **peer acceptance boundary**: the moment we receive a full block from the network and attempt to add it to our chain (`process_stream` handling `Package::Block` → `node_context.add_block`)

## Primary files (the walkthrough goes through code in)

- `bitcoin/src/node/miner.rs`
- `bitcoin/src/chain/chainstate.rs`
- `bitcoin/src/store/file_system_db_chain.rs`
- `bitcoin/src/primitives/block.rs`
- `bitcoin/src/pow.rs`
- `bitcoin/src/net/net_processing.rs`

## Orientation: the full block lifecycle at a glance (two zoom levels)

### A. Diagram — high-level lifecycle (stages)

```text
Pending transactions (mempool)
  |
  |  (candidate transaction assembly)
  v
 Candidate transaction list
  (transactions from mempool + coinbase)
  |
  |  (block construction)
  v
Constructed block header
  (includes hash commitment to txs)
  |
  |  (proof-of-work)
  v
Mined block (Nonce search until hash < target)
  |
  |  (persist + state transition)
  v
Add block to the blockchain
  (Persist as new tip + update UTXO set)
  |
  |  (announce to peers nodes)
  v
Relay new block to peers
  (INV announces hash → GETDATA requests
   bytes → BLOCK delivers full block)
  |
  |  (peer acceptance boundary +
      housekeeping)
  v
Add new block to chainstate;
remove its txs from local mempool
```

In this diagram, “selected from the mempool for this block” means a fixed list of transactions the miner will try to include in the next block; those transactions are still unconfirmed until the block is mined and accepted.

### B. Diagram — code-level call sequence (method call sequence)

```text
miner::should_trigger_mining
  // Check if mempool size >= threshold
  -> miner::prepare_mining_utxo
       // Snapshot mempool + append coinbase
       -> chainstate::BlockchainService::mine_block
            // Mining boundary: verify sigs
            -> store::BlockchainFileSystem::mine_block
                 // Persist block as tip
                 -> Block::new_block
                      // Construct header + PoW
                      -> ProofOfWork::new_proof_of_work
                      -> ProofOfWork::run
                           // Search nonce
       -> miner::broadcast_new_block
            // Announce to network
            -> net_processing::send_inv(OpType::Block, [hash])

net_processing::process_stream
  // Network message handler
  Package::Inv(Block)  -> send_get_data(Block, hash)
       // On INV: request full block
  Package::GetData     -> send_block(block_bytes)
       // On GETDATA: send serialized block
  Package::Block       -> node_context.add_block(&block)
       // On BLOCK: add + cleanup mempool
```

## Whitepaper connections (what to keep in mind while reading code)

### Timestamp chaining (Whitepaper §3)

The “timestamp server” concept creates an immutable, chronological chain by linking each block to its predecessor through cryptographic hashing. The key insight is that changing any block would require redoing all subsequent proof-of-work, making the history tamper-resistant.

In this implementation, the proof-of-work hash input includes five components that together create this chain:

```text
prev_hash + tx_commitment + timestamp + difficulty + nonce
```

The `prev_hash` links this block to the previous block's hash, creating the chain structure. Changing any historical block would change its hash, breaking the link and requiring all subsequent blocks to be re-mined. The `tx_commitment` is a hash (simplified Merkle root via `hash_transactions()`) that commits to the exact set of transactions in this block. Changing any transaction would change this commitment, invalidating the block's proof-of-work.

The `timestamp` records when the block was created, providing chronological ordering. It prevents miners from manipulating block ordering by using timestamps from the future, and ensures the chain reflects real-world time progression. The `difficulty` field encodes the target threshold (as `TARGET_BITS`) that determines how hard the proof-of-work must be. This is included in the hash so that changing the difficulty would require re-mining, maintaining consensus on the difficulty adjustment rules.

Finally, the `nonce` is the variable that miners increment during proof-of-work search. It's the only field that changes between hash attempts, allowing miners to search for a hash below the target without changing the block's content.

The first four components are fixed for a given candidate block, while the `nonce` is varied during mining. This means the block header commits to its position in the chain (`prev_hash`), its contents (`tx_commitment`), its creation time (`timestamp`), and the consensus rules (`difficulty`). The resulting block hash serves as both the proof-of-work result and the link that the next block will reference, creating an unbreakable chain.

### Proof-of-work (Whitepaper §4)

Proof-of-work is the mechanism that secures the blockchain by making block creation computationally expensive. The miner's task is to find a nonce value that, when combined with the block header data and hashed, produces a hash value numerically below a predetermined target threshold.

The algorithm works through brute-force search: the miner repeatedly hashes the block header with different nonce values until finding one that satisfies the difficulty requirement. Since hash functions are unpredictable, there's no shortcut—the miner must try many nonce values, making the process computationally intensive.

Here's how the proof-of-work algorithm operates (see Listing **9-6.8** for the actual implementation):

```text
function mine_block(block_header):
    target = calculate_target(TARGET_BITS)
    nonce = 0

    while nonce < MAX_NONCE:
        data = concatenate(
            block_header.prev_hash,
            block_header.tx_commitment,
            block_header.timestamp,
            block_header.difficulty_bits,
            nonce
        )
        hash = SHA256(data)
        hash_as_integer = convert_to_integer(hash)

        if hash_as_integer < target:
            return (nonce, hash)
        else:
            nonce = nonce + 1

    return error
```

The key insight is that the first four components of the data (`prev_hash`, `tx_commitment`, `timestamp`, `difficulty_bits`) are fixed for a given candidate block, while the `nonce` is the only variable that changes between attempts. This allows miners to search through nonce values without altering the block's content or position in the chain. The difficulty target determines how rare a valid hash is—lower targets mean fewer valid hashes exist, requiring more computational work on average.

The difficulty adjusts over time to maintain a consistent block production rate. As more miners join the network and computational power increases, blocks would be mined too quickly if difficulty remained constant. To compensate, the network periodically adjusts the `TARGET_BITS` value (which encodes the difficulty threshold) to make mining harder. When difficulty increases, the target value decreases, meaning miners must find rarer hashes—those with more leading zeros when interpreted as a number. This adjustment typically occurs every few blocks, ensuring that blocks continue to be produced at roughly the intended interval despite changes in total network hash rate.

### Incentives (Whitepaper §6)

Mining is rewarded by a coinbase transaction (`Transaction::new_coinbase_tx(...)`). This implementation appends coinbase to the candidate list during `prepare_mining_utxo(...)`.

### Merkle roots vs our simplification (Whitepaper §8)

Bitcoin commits to transactions via a Merkle root in the block header. This implementation uses a simplified `hash_transactions()` commitment (a hash over concatenated transaction ids). That keeps the learning focus on the byte layout and the PoW loop, but it is not a substitute for Merkle proofs.

---

## Step-by-step code walkthrough

**Goal**: trace block production and propagation in code: mempool → coinbase → PoW → persist tip → broadcast → peer acceptance path.

### Roadmap (how the steps connect)

- **Step 1**: mining trigger (`should_trigger_mining`) → Listing **9-6.1**
- **Step 2**: candidate tx list (`prepare_mining_utxo`) → Listing **9-6.2**
- **Step 3**: mine + announce (`process_mine_block`) → Listing **9-6.3**
  - **Step 3a**: mining boundary verification (`BlockchainService::mine_block`) → Listing **9-6.4**
  - **Step 3b**: block construction + tx commitment (`Block::new_block`, `hash_transactions`) → Listing **9-6.7**
  - **Step 3c**: proof-of-work engine (`ProofOfWork`) → Listing **9-6.8** (runs inside `Block::new_block`)
  - **Step 3d**: persist new tip + update derived state (`BlockchainFileSystem::mine_block`) → Listing **9-6.5**
  - **Step 3e**: UTXO state transition (`update_utxo_set`) → Listing **9-6.6**
- **Step 4**: block relay handshake (INV → GETDATA → BLOCK) → Listings **9-6.9** to **9-6.11**

**Whitepaper anchors**:
- Section 3 (Timestamp server intuition: linking via previous hash)
- Section 4 (Proof-of-Work)
- Section 5 (Network operation loop: broadcast txs/blocks; accept; build on accepted tip)
- Section 6 (Incentive mechanism: coinbase/subsidy)

### Step 1 — Decide whether to mine (`should_trigger_mining`)

Mining is only attempted when the node is configured as a miner and the mempool has reached a minimum size.

### Listing 9-6.1: mining trigger (`should_trigger_mining`)
> **Source:** `miner.rs` — Source

```rust
pub fn should_trigger_mining() -> bool {
    // Read mempool size (how many pending transactions we currently have).
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    // Configuration flag: only nodes explicitly configured as miners should
    // mine.
    let is_miner = GLOBAL_CONFIG.is_miner();
    // Teaching policy: mine only when there is "enough" work and we are a
    // miner.
    // (Production miners build templates continuously and do not wait for a
    // fixed threshold.)
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}
```

### Listing 9-6.1 explanation:

- It checks two conditions—mempool size and “am I a miner?”—and returns a boolean.
- It makes miner scheduling explicit and easy to reason about: the node mines only when there is enough pending work to justify it.

### Step 2 — Build a candidate tx list (mempool txs + coinbase) (`prepare_mining_utxo`)

This step constructs the exact list of transactions that will be included in the next block to be mined.

**In this project**: The miner takes a snapshot of the entire mempool using `GLOBAL_MEMORY_POOL.get_all()`, which returns all pending transactions without any filtering or ordering. There is no fee-based selection or size optimization—this is a simplified learning implementation that prioritizes clarity over production efficiency. Mining is triggered when the mempool contains at least 3 transactions (`TRANSACTION_THRESHOLD = 3`), and when mining occurs, all transactions in the mempool are included in the candidate block.

**In Bitcoin whitepater**: Production miners use sophisticated transaction selection algorithms to maximize revenue. Transactions are selected based on their fee rate (satoshis per byte), with higher fee-rate transactions prioritized. Miners aim to fill blocks up to the size limit (originally 1MB, now approximately 4MB with SegWit) with the most profitable transactions first. The Bitcoin whitepaper doesn't specify a particular selection algorithm, but the economic incentive is clear: "If the output value of a transaction is less than its input value, the difference is a transaction fee that is added to the incentive value of the block containing the transaction" (Whitepaper §6). This creates a fee market where users compete by offering higher fees for faster confirmation.

After selecting transactions, the miner appends a coinbase transaction, creating the candidate transaction list that will be packaged into the new block.

**Diagram — candidate assembly (mempool snapshot + coinbase)**

```text
candidate_txs =
  GLOBAL_MEMORY_POOL.get_all()
  + [Transaction::new_coinbase_tx(mining_address)]
```

### Listing 9-6.2: candidate assembly (`prepare_mining_utxo`)
> **Source:** `miner.rs` — Source

```rust
pub async fn prepare_mining_utxo(
    mining_address: &WalletAddress,
    blockchain: &BlockchainService,       // validates inputs against UTXO set
) -> Result<Vec<Transaction>> {
    let txs = GLOBAL_MEMORY_POOL.get_all()?;

    // Validate each transaction's inputs are still unspent
    let db = blockchain.get_db().await?;
    let utxo_tree = db.open_tree("chainstate")?;

    let mut valid_txs = Vec::new();
    for tx in txs {
        if tx.is_coinbase() { continue; }
        let mut inputs_valid = true;
        for input in tx.get_vin() {
            match utxo_tree.get(input.get_txid()) {
                Ok(Some(outs_bytes)) => {
                    let outputs: Vec<TXOutput> = decode(outs_bytes)?;
                    if input.get_vout() >= outputs.len() {
                        inputs_valid = false;
                        break;
                    }
                }
                _ => { inputs_valid = false; break; }
            }
        }
        if inputs_valid {
            valid_txs.push(tx);
        } else {
            remove_from_memory_pool(tx, blockchain).await;
        }
    }

    if valid_txs.is_empty() {
        return Err(BtcError::InvalidValueForMiner(
            "No valid transactions to mine".to_string(),
        ));
    }

    let coinbase_tx = create_mining_coinbase_transaction(mining_address)?;
    let mut final_txs = valid_txs;
    final_txs.push(coinbase_tx);
    Ok(final_txs)
}
```

### Listing 9-6.2 explanation:

- It snapshots the current mempool and validates each transaction's inputs against the UTXO set. This prevents mining blocks with already-spent inputs when a competing block has been accepted during the race between miners.
- Transactions with spent inputs are removed from the mempool (they were already confirmed in a competing block).
- If no valid transactions remain (all were stale), mining is aborted.
- It appends a coinbase transaction, which is the mechanism by which new coins (subsidy) are created for the miner.
- **Note:** The function is now `async` and accepts a `BlockchainService` parameter for UTXO validation. The `chainstate.rs` wrapper provides additional protection by re-validating inputs under the write lock just before calling `mine_block`.

> **Why does validation happen in `prepare` AND again in `mine_block`?**
>
> `prepare_mining_utxo` validates with a **read lock** — other operations can proceed concurrently. Between this validation and the actual `mine_block` call (which requires the **write lock**), a competing block may arrive from the network and be processed via `add_block`. That competing block spends the same transaction inputs. If we didn't re-validate under the write lock, we'd mine a block with already-spent inputs — creating a duplicate coinbase subsidy (money from nothing). The second validation in `BlockchainService::mine_block` (Section 9.2, Listing 9-10) catches this race condition.

### Step 3 — Mine, persist, and clean up mempool (`process_mine_block` → `mine_block`)

**Mining + persistence + mempool cleanup code**: `bitcoin/src/node/miner.rs`, `bitcoin/src/store/file_system_db_chain.rs`

### Listing 9-6.3: mine → mempool cleanup → announce (`process_mine_block`)
> **Source:** `miner.rs` — Source

```rust
pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    // Prevent concurrent mining — only one mining task at a time
    if MINING_IN_PROGRESS
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        let msg = "Mining already in progress";
        return Err(BtcError::InvalidValueForMiner(msg.to_string()));
    }

    // Reset cancellation flag before starting
    reset_mining_cancellation();

    let result = async {
        // Check for cancellation before mining
        // (a competing block may have arrived)
        if is_mining_cancelled() {
            let msg = "Mining cancelled";
            return Err(BtcError::InvalidValueForMiner(msg.to_string()));
        }

        let my_node_addr = GLOBAL_CONFIG.get_node_addr();
        let new_block = blockchain.mine_block(&txs).await?;

        // CRITICAL: Do NOT cancel after block creation.
        // Once mine_block() completes, the block is in our chain.
        // If we skip broadcasting, other nodes never learn about it,
        // creating a permanent fork.

        let block_hash = new_block.get_hash();
        info!(
            "New block {} is mined by node {}!",
            block_hash, my_node_addr
        );

        // Local housekeeping: remove confirmed txs from mempool
        for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }

    // Announce the new block to peers by inventory (hash only).
    // Peers will request the full block bytes via GETDATA if they need it.
    broadcast_new_block(&new_block).await?;
    Ok(new_block)
}
```

### Listing 9-6.3 explanation:

- It delegates block production to `blockchain.mine_block(&txs)` (which enforces the mining boundary and persists the resulting block).
- It removes transactions from the local mempool after they have been confirmed in the mined block (local housekeeping).
- It announces the new block to peers by inventory (INV), allowing peers to explicitly request full block bytes.

#### Step 3a — Enforce correctness at the mining boundary (`BlockchainService::mine_block`)

### Listing 9-6.4: mining boundary verification (`BlockchainService::mine_block`)

> **Source:** `chainstate.rs` — Source

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

### Listing 9-6.4 explanation:

- It verifies every transaction’s signatures before any proof-of-work effort is spent.
- It delegates the “build + mine + persist” work to the storage-backed chain only after validation passes.

#### Step 3b — Construct the block and commit to transactions (`Block::new_block`, `hash_transactions`)

### Listing 9-6.7: block construction + transaction commitment

> **Source:** `block.rs` — Source

```rust
pub fn new_block(
    pre_block_hash: String,
    transactions: &[Transaction],
    height: usize,
) -> Block {
    let header = BlockHeader {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        hash: String::new(),
        nonce: 0,
        height,
    };
    let mut block = Block {
        header,
        transactions: transactions.to_vec(),
    };
    let pow = ProofOfWork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();
    block.header.nonce = nonce;
    block.header.hash = hash;
    block
}

pub fn hash_transactions(&self) -> Vec<u8> {
    let mut txhashs = vec![];
    for transaction in &self.transactions {
        txhashs.extend(transaction.get_id());
    }
    crate::sha256_digest(txhashs.as_slice())
}
```

### Listing 9-6.7 explanation:

- It builds a header that links to the previous block and captures a timestamp and height.
- It commits to the block's transactions via `hash_transactions()` (a simplified stand-in for a Merkle root).
- It runs proof-of-work immediately and returns a block with a final `nonce` and `hash`.

#### Step 3c — Proof-of-work: byte layout and nonce search (`ProofOfWork`)

This is the proof-of-work engine that `Block::new_block(...)` calls to find a valid nonce. The algorithm works by:

1. Calculating a difficulty target based on `TARGET_BITS`
2. Repeatedly hashing a byte sequence containing the previous block hash, transaction commitment, timestamp, difficulty bits, and a varying nonce
3. Converting each hash to an integer and comparing it to the target
4. Incrementing the nonce until finding a hash below the target

For a detailed conceptual overview with pseudocode, see the **Proof-of-work (Whitepaper §4)** section earlier in this chapter.

**Diagram — the exact bytes hashed in each PoW iteration**

```text
prepare_data(nonce) =
  pre_block_hash.as_bytes()
  || hash_transactions()
  || timestamp.to_be_bytes()
  || TARGET_BITS.to_be_bytes()
  || nonce.to_be_bytes()
```

### Listing 9-6.8: proof-of-work target + data layout + search loop (`pow.rs`)

> **Source:** `pow.rs` — Source

```rust
pub struct ProofOfWork { block: Block, target: BigInt }

const TARGET_BITS: i32 = 8;

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> Self {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = vec![];
        data.extend(self.block.get_pre_block_hash().as_bytes());
        data.extend(self.block.hash_transactions());
        data.extend(self.block.get_timestamp().to_be_bytes());
        data.extend(TARGET_BITS.to_be_bytes());
        data.extend(nonce.to_be_bytes());
        data
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        while nonce < i64::MAX {
            let hash = crate::sha256_digest(
                self.prepare_data(nonce).as_slice()
            );
            let hash_int = BigInt::from_bytes_be(
                Sign::Plus,
                hash.as_slice()
            );
            if hash_int < self.target {
                return (nonce, HEXLOWER.encode(
                    hash.as_slice()
                ));
            }
            nonce += 1;
        }
        panic!("PoW failed");
    }
}
```

### Listing 9-6.8 explanation:

- It calculates a difficulty target based on `TARGET_BITS` (a 256-bit number that determines mining difficulty).
- It prepares the data to hash by concatenating: previous block hash, transaction commitment, timestamp, difficulty bits, and nonce.
- It searches for a nonce value that produces a hash below the target by incrementing the nonce and hashing repeatedly.
- When a valid nonce is found, it returns the nonce and the resulting hash.

#### Step 3d — Persist the mined block and advance derived state (`BlockchainFileSystem::mine_block`)

### Listing 9-6.5: persist mined block and update derived state (`BlockchainFileSystem::mine_block`)

> **Source:** `file_system_db_chain.rs` — Source

```rust
pub async fn mine_block(
    &self,
    transactions: &[Transaction],
) -> Result<Block> {
    let best_height = self.get_best_height().await?;
    let block = Block::new_block(
        self.get_tip_hash().await?,
        transactions,
        best_height + 1,
    );
    let block_hash = block.get_hash();

    let blocks_tree = self.blockchain.db.open_tree(
        self.get_blocks_tree_path()
    )?;
    Self::update_blocks_tree(&blocks_tree, &block)
        .await?;
    self.set_tip_hash(block_hash).await?;
    self.update_utxo_set(&block).await?;
    Ok(block)
}
```

### Listing 9-6.5 explanation:

- It constructs a new block at $height = best\_height + 1$ and runs proof-of-work inside `Block::new_block(...)`.
- It writes the block to storage and advances the tip hash to point at the new block.
- It applies the UTXO state transition for the newly accepted tip.

#### Step 3e — Apply the UTXO state transition for the connected block (`update_utxo_set`)

**Why update the UTXO set?**

The UTXO set represents the current state of spendable outputs—it answers the question "which outputs can be spent right now?" When a new block is added to the chain, transactions in that block consume some outputs (spending them) and create new outputs (making them spendable). The UTXO set must be updated to reflect these changes: spent outputs are removed (they can no longer be spent), and newly created outputs are added (they become available for future transactions). This ensures that the UTXO set always accurately reflects the spendability state at the current chain tip, preventing double-spending and enabling efficient validation of new transactions.

**Diagram — what "updating the UTXO set" means**

```text
For each non-coinbase tx in the mined block:
  spend: remove the output at index vout
    Each input contains (txid, vout) where:
    - txid points to a PREVIOUS transaction
    - vout is an index into outputs of that tx
    - Previous tx can have multiple outputs
    - Only the referenced output is removed
    - Other outputs remain in UTXO set

For every tx in the mined block (coinbase too):
  create: insert all outputs under current txid
    Every output created by the transaction is
    added to the UTXO set and can be spent by
    future transactions via (txid, vout) pair
```

### Listing 9-6.6: UTXO update on block connection (`update_utxo_set`)

> **Source:** `file_system_db_chain.rs` — Source

```rust
pub async fn update_utxo_set(
    &self,
    block: &Block,
) -> Result<()> {
    let db = self.blockchain.db.clone();
    let utxo_tree = db.open_tree("chainstate")?;

    for tx in block.get_transactions().await? {
        // Remove spent outputs (for non-coinbase txs)
        if !tx.is_coinbase() {
            for input in tx.get_vin() {
                let utxo_list: Vec<TXOutput> =
                    bincode::serde::decode_from_slice(
                        utxo_tree.get(input.get_txid())?
                            .ok_or(BtcError::UTXONotFoundError(
                                "".into()
                            ))?
                            .as_ref(),
                        bincode::config::standard(),
                    )?.0;

                let updated: Vec<_> =
                    utxo_list.iter().enumerate()
                        .filter(|(idx, _)|
                            *idx != input.get_vout()
                        )
                        .map(|(_, o)| o.clone())
                        .collect();

                if updated.is_empty() {
                    utxo_tree.remove(
                        input.get_txid()
                    )?;
                } else {
                    let encoded = bincode::serde::
                        encode_to_vec(
                            &updated,
                            bincode::config::standard()
                        )?;
                    utxo_tree.insert(
                        input.get_txid(),
                        encoded
                    )?;
                }
            }
        }
        // ... (insert new outputs)
    }
    Ok(())
}
```

After removing any spent outputs, the function inserts all newly created transaction outputs into the UTXO set, making them immediately available for future spending:

```rust
        // Insert newly created outputs
        let outs_bytes = bincode::serde::encode_to_vec(
            &tx.get_vout(),
            bincode::config::standard()
        )?;
        utxo_tree.insert(tx.get_id(), outs_bytes)?;
    }
    Ok(())
}
```

### Listing 9-6.6 explanation:

- For each non-coinbase transaction, it removes the spent output index from the referenced previous transaction’s UTXO entry.
- For every transaction, it inserts the newly created outputs, making them spendable for subsequent transactions.

### Step 4 — Broadcast and receive blocks (INV → BLOCK handling)

Once a block is mined and persisted locally, it must be propagated to other nodes in the network. This implementation uses an "inventory then request data" pattern for efficient network communication between nodes. The following describes the network protocol messages used for block propagation:

- **INV**: announce "I have object X" (by id only) — a lightweight inventory message that tells peers what blocks or transactions a node has available, without sending the full data. This allows peers to discover new content efficiently without wasting bandwidth on data they may already have.

- **GETDATA**: request the full bytes for X — when a peer receives an INV message for a block or transaction it doesn't have, it sends GETDATA to request the complete data. This two-step process (INV → GETDATA) reduces bandwidth by only transmitting full blocks when explicitly requested.

- **BLOCK**: deliver the full block bytes — the final step where the node that announced the block (via INV) sends the complete serialized block data in response to a GETDATA request. The receiving node can then validate and add the block to its chain.


**Diagram — block relay handshake**

```text
Miner/peer A                      Peer B
---------                         ------
INV(Block, hash)  ------->  (learns block id)
GETDATA(Block, hash) <---  (requests bytes)
BLOCK(block_bytes)  ------>  (receives block)
                              add_block(block)
                              remove txs from mempool
```

### Listing 9-6.9: broadcast block inventory (`broadcast_new_block`)

> **Source:** `miner.rs` — Source

```rust
pub async fn broadcast_new_block(
    block: &Block,
) -> Result<()> {
    let my_addr = GLOBAL_CONFIG.get_node_addr();
    for node in GLOBAL_NODES.get_nodes()? {
        if node.get_addr() != my_addr {
            let addr = node.get_addr();
            let hash = block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&addr, OpType::Block, &[hash])
                    .await;
            });
        }
    }
    Ok(())
}
```

### Listing 9-6.9 explanation:

- It announces the new block by hash (inventory), rather than pushing the full block to peers.
- It spawns per-peer sends so announcement does not block the mining flow.

### Listing 9-6.10: message constructors for INV / GETDATA / BLOCK (`send_inv`, `send_get_data`, `send_block`)

> **Source:** `net_processing.rs` — Source

```rust
pub async fn send_get_data(
    addr_to: &SocketAddr,
    op_type: OpType,
    id: &[u8],
) {
    // Request full bytes for object.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    )
    .await;
}

pub async fn send_inv(
    addr_to: &SocketAddr,
    op_type: OpType,
    blocks: &[Vec<u8>],
) {
    // Announce we have objects (by id only).
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Inv {
            addr_from: node_addr,
            op_type,
            items: blocks.to_vec(),
        },
    )
    .await;
}

pub async fn send_block(
    addr_to: &SocketAddr,
    block: &Block,
) {
    // Send full block bytes to peer.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize()?
        },
    )
    .await;
}
```

### Listing 9-6.11: network handler for INV / GETDATA / BLOCK (`process_stream`)

This is a long handler because it supports many message types; for the purposes of this section, focus on the `Package::{Inv,GetData,Block}` match arms and treat the other arms as contextual background.

> **Source:** `net_processing.rs` — Source

```rust
pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(&stream);
    let pkg_reader = Deserializer::from_reader(reader)
        .into_iter::<Package>();

    for pkg in pkg_reader {
        let pkg = pkg?;
        match pkg {
            Package::Block { addr_from, block } => {
                let block = Block::deserialize(block.as_slice())?;
                node_context.add_block(&block).await?;
                for tx in block.get_transactions().await? {
                    node_context.remove_from_memory_pool(tx).await;
                }
                GLOBAL_BLOCKS_IN_TRANSIT.remove(
                    block.get_hash_bytes().as_ref())?;
            }
            Package::GetBlocks { addr_from } => {
                let blocks = node_context.get_block_hashes().await?;
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
            Package::GetData { addr_from, op_type, id } =>
                match op_type {
                OpType::Block => {
                    if let Some(block) =
                        node_context.get_block(id.as_slice()).await? {
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    if let Some(tx) = GLOBAL_MEMORY_POOL.get(&txid_hex)? {
                        send_tx(&addr_from, &tx).await;
                    }
                }
            },
            // ... (other match arms)
        }
    }
```

The match statement continues with additional handlers for inventory (INV), transaction (TX), coin transmission, and version messages. The INV and TX handlers manage the synchronization of blocks and transactions across the peer network:

```rust
            Package::Inv { addr_from, op_type,
                items } => match op_type {
                OpType::Block => {
                    GLOBAL_BLOCKS_IN_TRANSIT
                        .add_blocks(items.as_slice())?;
                    if let Ok(Some(hash)) =
                        items.first() {
                        send_get_data(
                            &addr_from,
                            OpType::Block,
                            &hash
                        ).await;
                    }
                }
                OpType::Tx => {
                    if let Ok(Some(txid)) =
                        items.first() {
                        let txid_hex =
                            HEXLOWER.encode(txid);
                        if !GLOBAL_MEMORY_POOL
                            .contains(&txid_hex)? {
                            send_get_data(
                                &addr_from,
                                OpType::Tx,
                                txid
                            ).await;
                        }
                    }
                }
            },
            Package::Tx { addr_from,
                transaction } => {
                let tx = Transaction::deserialize(
                    transaction.as_slice()
                )?;
                let _ = node_context
                    .process_transaction(
                        &addr_from,
                        tx
                    ).await;
            }
            // ... (SendBitCoin and Version handlers continue)
        }
    }
```

For coin transfers and version synchronization, the handler validates addresses and checks blockchain height to trigger block sync:

```rust
            Package::SendBitCoin { addr_from,
                wlt_frm_addr, wlt_to_addr,
                amount } => {
                let from =
                    WalletAddress::validate(
                        wlt_frm_addr
                    )?;
                let to =
                    WalletAddress::validate(
                        wlt_to_addr
                    )?;
                let _ = node_context
                    .btc_transaction(
                        &from,
                        &to,
                        amount
                    ).await;
            }
            Package::Version { addr_from,
                best_height, .. } => {
                let local_height =
                    node_context
                        .get_blockchain_height()
                        .await?;
                if local_height < best_height {
                    send_get_blocks(&addr_from)
                        .await;
                } else if
                    local_height > best_height {
                    send_version(&addr_from,
                        local_height).await;
                }
                if !GLOBAL_NODES
                    .node_is_known(&addr_from)? {
                    GLOBAL_NODES.add_node(
                        addr_from
                    )?;
                }
            }
        }
    }
```

The transaction query handler iterates through all confirmed transactions and pretty-prints them with inputs and outputs. The output formatting uses box-drawing characters for readability:

```rust
AdminNodeQueryType::GetAllTransactions => {
    let txs = node_context
        .find_all_transactions().await?;
    for (idx, (txid_hex, tx_summary)) in
        txs.iter().enumerate() {
        let inputs = tx_summary.get_inputs();
        let outputs =
            tx_summary.get_outputs();
        // ... (format and log tx details)
    }
}
```

The remaining administrative queries support blockchain inspection and mining operations. These include retrieval of chain height and UTXO reindexing:

```rust
AdminNodeQueryType::GetBlockHeight => {
    let height = node_context
        .get_blockchain_height().await?;
    // ... (send height to client)
}
AdminNodeQueryType::MineEmptyBlock => {
    if GLOBAL_CONFIG.is_miner() {
        let addr =
            GLOBAL_CONFIG.get_mining_addr()?;
        node_context.mine_empty_block(&addr)
            .await?;
    }
}
AdminNodeQueryType::ReindexUtxo => {
    let blockchain =
        node_context.get_blockchain().clone();
    let utxo_set = UTXOSet::new(blockchain);
    utxo_set.reindex().await?;
}
```

### Listing 9-6.11 explanation (block relay path):

- On `Package::Inv` for a block, it requests the block by hash (`send_get_data`).
- On `Package::GetData` for a block, it serves the block bytes if present (`send_block`).
- On `Package::Block`, it deserializes the block, hands it to `node_context.add_block(&block)` (chainstate acceptance boundary), and then removes the block’s transactions from the local mempool.

## Navigation

- **Previous**: Section 9.5 (Transaction Lifecycle)
- **Next**: Section 9.7 (Consensus and Validation)

---

<div align="center">

**[← Previous: Transaction Lifecycle](05-Transaction-Lifecycle.md)** | **Block Lifecycle and Mining** | **[Next: Consensus and Validation →](07-Consensus-and-Validation.md)**

</div>

