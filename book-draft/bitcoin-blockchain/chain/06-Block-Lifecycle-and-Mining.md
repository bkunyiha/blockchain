<div align="left">

<details>
<summary><b>📑 Section Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Section 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Section 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Section 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Section 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../Rust-Project-Index.md">Section 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Section 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Section 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Section 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. **Section 2.4: Blockchain (Technical Foundations)** ← *You are here*
10. <a href="../store/README.md">Section 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="10-Whitepaper-Step-5-Block-Acceptance.md">Section 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Section 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Section 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Section 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Section 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Section 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Section 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Section 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Section 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Section 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Section 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Section 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

## Section 2.4.6: Block Lifecycle and Mining

In the previous section (Section 2.4.5, **Transaction Lifecycle**), we followed a transaction from construction and signing, through mempool admission and propagation. That brings us to the natural next question: **what happens to those pending transactions next?**

This section picks up from that boundary and traces the rest of the path: how the node assembles a candidate block from the mempool, performs proof-of-work, persists the new tip, advances the UTXO set, and relays the new block to peers. We will walk through the code: key methods are printed, and each listing is followed by a succinct explanation of what the code is doing and why.

## Scope within Section 2.4 (section flow)

This subsection covers **Section 2.4 (Blockchain — From Transaction to Block Acceptance) Steps 5–7**: candidate assembly, proof-of-work, persistence, and the UTXO state transition after a block is connected.

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

```
Pending transactions (mempool)
  |
  |  (candidate transaction assembly)
  v
 Candidate transaction list (transactions selected from the mempool for this block + coinbase)
  |
  |  (block construction)
  v
Constructed block header (includes hash commitment to transaction list)
  |
  |  (proof-of-work)
  v
Mined block (Nonce search until hash < target)
  |
  |  (persist + state transition)
  v
Add block to the blockchain (Persist block as new tip + update UTXO set)
  |
  |  (announce to peers nodes)
  v
Relay new block to peers (INV announces hash → GETDATA requests bytes → BLOCK delivers full block)
  |
  |  (peer acceptance boundary + housekeeping)
  v
Add new block to chainstate; remove its txs from local mempool
```

In this diagram, “selected from the mempool for this block” means a fixed list of transactions the miner will try to include in the next block; those transactions are still unconfirmed until the block is mined and accepted.

### B. Diagram — code-level call sequence (method call sequence)

```
miner::should_trigger_mining                             // Check if mempool size >= threshold and node is miner
  -> miner::prepare_mining_utxo                          // Snapshot mempool + append coinbase transaction
       -> chainstate::BlockchainService::mine_block      // Mining boundary: verify tx signatures, then mine
            -> store::BlockchainFileSystem::mine_block   // Persist new block as tip + update UTXO set
                 -> Block::new_block                     // Construct block header + run proof-of-work
                      -> ProofOfWork::new_proof_of_work  // Initialize PoW engine with block data
                      -> ProofOfWork::run                // Search for nonce until hash < target
       -> miner::broadcast_new_block                     // Announce new block to network
            -> net_processing::send_inv(OpType::Block, [block_hash])// Send inventory (hash only) to peers

net_processing::process_stream                             // Network message handler (runs per-peer connection)
  Package::Inv(Block)  -> send_get_data(Block, block_hash) // On INV: request full block bytes
  Package::GetData     -> send_block(block_bytes)          // On GETDATA: send full serialized block
  Package::Block       -> node_context.add_block(&block) + mempool cleanup  // On BLOCK: add to chainstate, remove txs from mempool
```

## Whitepaper connections (what to keep in mind while reading code)

### Timestamp chaining (Whitepaper §3)

The “timestamp server” concept creates an immutable, chronological chain by linking each block to its predecessor through cryptographic hashing. The key insight is that changing any block would require redoing all subsequent proof-of-work, making the history tamper-resistant.

In this implementation, the proof-of-work hash input includes five components that together create this chain:

```
prev_hash + tx_commitment + timestamp + difficulty + nonce
```

The `prev_hash` links this block to the previous block's hash, creating the chain structure. Changing any historical block would change its hash, breaking the link and requiring all subsequent blocks to be re-mined. The `tx_commitment` is a hash (simplified Merkle root via `hash_transactions()`) that commits to the exact set of transactions in this block. Changing any transaction would change this commitment, invalidating the block's proof-of-work.

The `timestamp` records when the block was created, providing chronological ordering. It prevents miners from manipulating block ordering by using timestamps from the future, and ensures the chain reflects real-world time progression. The `difficulty` field encodes the target threshold (as `TARGET_BITS`) that determines how hard the proof-of-work must be. This is included in the hash so that changing the difficulty would require re-mining, maintaining consensus on the difficulty adjustment rules.

Finally, the `nonce` is the variable that miners increment during proof-of-work search. It's the only field that changes between hash attempts, allowing miners to search for a hash below the target without changing the block's content.

The first four components are fixed for a given candidate block, while the `nonce` is varied during mining. This means the block header commits to its position in the chain (`prev_hash`), its contents (`tx_commitment`), its creation time (`timestamp`), and the consensus rules (`difficulty`). The resulting block hash serves as both the proof-of-work result and the link that the next block will reference, creating an unbreakable chain.

### Proof-of-work (Whitepaper §4)

Proof-of-work is the mechanism that secures the blockchain by making block creation computationally expensive. The miner's task is to find a nonce value that, when combined with the block header data and hashed, produces a hash value numerically below a predetermined target threshold.

The algorithm works through brute-force search: the miner repeatedly hashes the block header with different nonce values until finding one that satisfies the difficulty requirement. Since hash functions are unpredictable, there's no shortcut—the miner must try many nonce values, making the process computationally intensive.

Here's how the proof-of-work algorithm operates (see Listing **2.4-6.8** for the actual implementation):

```
function mine_block(block_header):
    // Calculate the difficulty target: a 256-bit number that determines mining difficulty
    // Lower target = harder to find valid hash = more computational work required
    target = calculate_target(TARGET_BITS)  // Target = 2^(256 - TARGET_BITS)
    
    // Start searching from nonce 0 and increment until we find a valid hash
    nonce = 0
    
    while nonce < MAX_NONCE:
        // Prepare the data to hash: concatenate all fixed header fields + varying nonce
        // The nonce is the only field that changes between attempts, allowing brute-force search
        data = concatenate(
            block_header.prev_hash,      // Links to previous block (fixed)
            block_header.tx_commitment,  // Hash of transactions (fixed)
            block_header.timestamp,      // Block creation time (fixed)
            block_header.difficulty_bits, // Difficulty threshold (fixed)
            nonce                        // Only variable field (changes each iteration)
        )
        
        // Hash the prepared data using SHA-256
        // This produces a 256-bit (32-byte) hash value
        hash = SHA256(data)
        
        // Convert hash bytes to integer for numerical comparison
        // We need to compare the hash as a number to check if it's below the target threshold
        // The hash is interpreted as a big-endian integer (most significant byte first)
        hash_as_integer = convert_to_integer(hash)
        
        // Check if we found a valid proof-of-work
        // Valid proof-of-work means the hash, interpreted as a number, is less than the target
        // This is probabilistically rare, requiring many hash attempts on average
        if hash_as_integer < target:
            return (nonce, hash)  // Success! Found valid nonce that produces hash < target
        else:
            nonce = nonce + 1      // Hash too large, try next nonce value
    
    // If we exhaust all possible nonces, mining failed (extremely unlikely in practice)
    // This would only happen if the difficulty is impossibly high or there's a bug
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

- **Step 1**: mining trigger (`should_trigger_mining`) → Listing **2.4-6.1**
- **Step 2**: candidate tx list (`prepare_mining_utxo`) → Listing **2.4-6.2**
- **Step 3**: mine + announce (`process_mine_block`) → Listing **2.4-6.3**
  - **Step 3a**: mining boundary verification (`BlockchainService::mine_block`) → Listing **2.4-6.4**
  - **Step 3b**: block construction + tx commitment (`Block::new_block`, `hash_transactions`) → Listing **2.4-6.7**
  - **Step 3c**: proof-of-work engine (`ProofOfWork`) → Listing **2.4-6.8** (runs inside `Block::new_block`)
  - **Step 3d**: persist new tip + update derived state (`BlockchainFileSystem::mine_block`) → Listing **2.4-6.5**
  - **Step 3e**: UTXO state transition (`update_utxo_set`) → Listing **2.4-6.6**
- **Step 4**: block relay handshake (INV → GETDATA → BLOCK) → Listings **2.4-6.9** to **2.4-6.11**

**Whitepaper anchors**:
- Section 3 (Timestamp server intuition: linking via previous hash)
- Section 4 (Proof-of-Work)
- Section 5 (Network operation loop: broadcast txs/blocks; accept; build on accepted tip)
- Section 6 (Incentive mechanism: coinbase/subsidy)

### Step 1 — Decide whether to mine (`should_trigger_mining`)

Mining is only attempted when the node is configured as a miner and the mempool has reached a minimum size.

**Code Listing 2.4-6.1**: mining trigger (`should_trigger_mining`)
```rust
// Source: bitcoin/src/node/miner.rs
pub fn should_trigger_mining() -> bool {
    // Read mempool size (how many pending transactions we currently have).
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    // Configuration flag: only nodes explicitly configured as miners should mine.
    let is_miner = GLOBAL_CONFIG.is_miner();
    // Teaching policy: mine only when there is "enough" work and we are a miner.
    // (Production miners build templates continuously and do not wait for a fixed threshold.)
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}
```

**Listing 2.4-6.1 explanation**:

- It checks two conditions—mempool size and “am I a miner?”—and returns a boolean.
- It makes miner scheduling explicit and easy to reason about: the node mines only when there is enough pending work to justify it.

### Step 2 — Build a candidate tx list (mempool txs + coinbase) (`prepare_mining_utxo`)

This step constructs the exact list of transactions that will be included in the next block to be mined. 

**In this project**: The miner takes a snapshot of the entire mempool using `GLOBAL_MEMORY_POOL.get_all()`, which returns all pending transactions without any filtering or ordering. There is no fee-based selection or size optimization—this is a simplified learning implementation that prioritizes clarity over production efficiency. Mining is triggered when the mempool contains at least 3 transactions (`TRANSACTION_THRESHOLD = 3`), and when mining occurs, all transactions in the mempool are included in the candidate block.

**In Bitcoin whitepater**: Production miners use sophisticated transaction selection algorithms to maximize revenue. Transactions are selected based on their fee rate (satoshis per byte), with higher fee-rate transactions prioritized. Miners aim to fill blocks up to the size limit (originally 1MB, now approximately 4MB with SegWit) with the most profitable transactions first. The Bitcoin whitepaper doesn't specify a particular selection algorithm, but the economic incentive is clear: "If the output value of a transaction is less than its input value, the difference is a transaction fee that is added to the incentive value of the block containing the transaction" (Whitepaper §6). This creates a fee market where users compete by offering higher fees for faster confirmation.

After selecting transactions, the miner appends a coinbase transaction, creating the candidate transaction list that will be packaged into the new block.

**Diagram — candidate assembly (mempool snapshot + coinbase)**

```
candidate_txs =
  GLOBAL_MEMORY_POOL.get_all()
  + [Transaction::new_coinbase_tx(mining_address)]
```

**Code Listing 2.4-6.2**: candidate assembly (`prepare_mining_utxo`)
```rust
// Source: bitcoin/src/node/miner.rs
pub fn prepare_mining_utxo(mining_address: &WalletAddress) -> Result<Vec<Transaction>> {
    // Snapshot the mempool at a point in time.
    // This gives the miner a deterministic candidate list to work with.
    let txs = GLOBAL_MEMORY_POOL.get_all()?;

    // Construct the coinbase transaction (subsidy) paying to the miner.
    let coinbase_tx = create_mining_coinbase_transaction(mining_address)?;
    // Candidate ordering in this implementation: mempool txs, then coinbase appended.
    let mut final_txs = txs;
    final_txs.push(coinbase_tx);

    Ok(final_txs)
}
```

**Listing 2.4-6.2 explanation**:

- It snapshots the current mempool and uses that as the candidate transaction set (no fee or weight selection in this learning implementation).
- It appends a coinbase transaction, which is the mechanism by which new coins (subsidy) are created for the miner.

### Step 3 — Mine, persist, and clean up mempool (`process_mine_block` → `mine_block`)

**Mining + persistence + mempool cleanup code**: `bitcoin/src/node/miner.rs`, `bitcoin/src/store/file_system_db_chain.rs`

**Code Listing 2.4-6.3**: mine → mempool cleanup → announce (`process_mine_block`)
```rust
// Source: bitcoin/src/node/miner.rs
pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    // Used only for logging/identification.
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // Ask the chainstate to mine a block from this candidate list.
    // Important: the chainstate enforces signature verification at the mining boundary.
    let new_block = blockchain.mine_block(&txs).await?;

    // Log the mined block id (hash) and which node mined it.
    info!(
        "New block {} is mined by node {}!",
        new_block.get_hash(),
        my_node_addr
    );

    // Local housekeeping: once the block is mined, those transactions are no longer pending locally.
    for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }

    // Announce the new block to peers by inventory (hash only).
    // Peers will request the full block bytes via GETDATA if they need it.
    broadcast_new_block(&new_block).await?;
    Ok(new_block)
}
```

**Listing 2.4-6.3 explanation**:

- It delegates block production to `blockchain.mine_block(&txs)` (which enforces the mining boundary and persists the resulting block).
- It removes transactions from the local mempool after they have been confirmed in the mined block (local housekeeping).
- It announces the new block to peers by inventory (INV), allowing peers to explicitly request full block bytes.

#### Step 3a — Enforce correctness at the mining boundary (`BlockchainService::mine_block`)

**Code Listing 2.4-6.4**: mining boundary verification (`BlockchainService::mine_block`)

```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Mining boundary: refuse to mine a block that contains invalid signatures.
    for trasaction in transactions {
        let is_valid = trasaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }
    // After validation, delegate to the storage-backed chain to actually construct, PoW-mine,
    // persist the block, and advance derived state.
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions).await
}
```

**Listing 2.4-6.4 explanation**:

- It verifies every transaction’s signatures before any proof-of-work effort is spent.
- It delegates the “build + mine + persist” work to the storage-backed chain only after validation passes.

#### Step 3b — Construct the block and commit to transactions (`Block::new_block`, `hash_transactions`)

**Code Listing 2.4-6.7**: block construction + transaction commitment

```rust
// Source: bitcoin/src/primitives/block.rs
pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
    // Header is created first; `hash` will be filled only after proof-of-work completes.
    let header = BlockHeader {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        hash: String::new(), // to be filled in the next step
        nonce: 0,
        height,
    };
    let mut block = Block {
        header,
        // Snapshot the candidate transaction list into the block.
        transactions: transactions.to_vec(),
    };
    // Proof-of-work commits to the previous hash link + tx commitment + timestamp + difficulty + nonce.
    let pow = ProofOfWork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();
    // Persist PoW results into the header.
    block.header.nonce = nonce;
    block.header.hash = hash;
    block
}

pub fn hash_transactions(&self) -> Vec<u8> {
    let mut txhashs = vec![];
    for transaction in &self.transactions {
        // This implementation commits to the list by concatenating txids and hashing.
        // (In Bitcoin this would be a Merkle root.)
        txhashs.extend(transaction.get_id());
    }
    crate::sha256_digest(txhashs.as_slice())
}
```

**Listing 2.4-6.7 explanation**:

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

```
prepare_data(nonce) =
  pre_block_hash.as_bytes()
  || hash_transactions()
  || timestamp.to_be_bytes()
  || TARGET_BITS.to_be_bytes()
  || nonce.to_be_bytes()
```

**Code Listing 2.4-6.8**: proof-of-work target + data layout + search loop (`pow.rs`)

```rust
// Source: bitcoin/src/pow.rs
use super::block::Block;        // Block type containing header and transactions
use data_encoding::HEXLOWER;    // Hex encoding for converting hash bytes to hex string
use num_bigint::{BigInt, Sign}; // Big integer type for comparing hash (256-bit) to target
use std::borrow::Borrow;        // Trait for borrowing references (used in hash comparison)
use std::ops::ShlAssign;        // Left-shift assignment operator (<<=) for target calculation
use tracing::debug;             // Debug logging macro for mining progress

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

const TARGET_BITS: i32 = 8;         // Difficulty parameter: determines mining difficulty (lower = easier, higher = harder)
                                    // Target = 2^(256 - TARGET_BITS), so TARGET_BITS=8 means target = 2^248

const MAX_NONCE: i64 = i64::MAX;    // Maximum nonce value to try before giving up (2^63 - 1)
                                    // In practice, a valid nonce is almost always found well before this limit

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        // Calculate the difficulty target: a 256-bit number that determines mining difficulty.
        // Lower target = harder to find valid hash = more computational work required.
        // Formula: target = 2^(256 - TARGET_BITS)
        // Example: if TARGET_BITS = 8, target = 2^248 (a very large number with many leading zeros)
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);  // Left-shift by (256 - TARGET_BITS) bits
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        // Prepare the exact byte sequence that will be hashed for this nonce attempt.
        // The order and format of these bytes must match exactly across all nodes for consensus.
        
        // Get the previous block's hash (links this block to the chain)
        let pre_block_hash = self.block.get_pre_block_hash();
        // Get the hash commitment to all transactions in this block (simplified Merkle root)
        let transactions_hash = self.block.hash_transactions();
        // Get the timestamp when this block was created
        let timestamp = self.block.get_timestamp();
        
        // Concatenate all fixed header fields + the varying nonce into a single byte vector
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());      // Previous block hash (fixed)
        data_bytes.extend(transactions_hash);              // Transaction commitment (fixed)
        data_bytes.extend(timestamp.to_be_bytes());        // Timestamp (fixed)
        data_bytes.extend(TARGET_BITS.to_be_bytes());      // Difficulty bits (fixed)
        data_bytes.extend(nonce.to_be_bytes());            // Nonce (varies each iteration)
        data_bytes
    }

    pub fn run(&self) -> (i64, String) {
        // Brute-force search: try nonce values starting from 0 until we find a valid hash.
        // The nonce is the only field that changes between attempts, allowing us to search
        // for a hash below the target without altering the block's content.
        let mut nonce = 0;
        while nonce < MAX_NONCE {
            // Prepare the byte sequence for this nonce attempt
            let data = self.prepare_data(nonce);
            // Hash the prepared data using SHA-256 (produces a 256-bit / 32-byte hash)
            let hash = crate::sha256_digest(data.as_slice());
            // Convert the hash bytes to a big integer for numerical comparison
            // Big-endian interpretation: most significant byte first
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            // Check if we found a valid proof-of-work
            // Valid means the hash, interpreted as a number, is less than the target threshold
            // This is probabilistically rare, requiring many hash attempts on average
            if hash_int < self.target {
                // Success! Encode the hash as hex string for storage and return both nonce and hash
                let hash_hex = HEXLOWER.encode(hash.as_slice());
                debug!("Found valid nonce: {} with hash: {}", nonce, hash_hex);
                return (nonce, hash_hex);
            }
            // Hash too large, try next nonce value
            nonce += 1;
        }
        // If we exhaust all possible nonces, mining failed (extremely unlikely in practice)
        // This would only happen if the difficulty is impossibly high or there's a bug
        panic!("Proof-of-work failed: could not find valid nonce");
    }
}
```

**Listing 2.4-6.8 explanation**:

- It calculates a difficulty target based on `TARGET_BITS` (a 256-bit number that determines mining difficulty).
- It prepares the data to hash by concatenating: previous block hash, transaction commitment, timestamp, difficulty bits, and nonce.
- It searches for a nonce value that produces a hash below the target by incrementing the nonce and hashing repeatedly.
- When a valid nonce is found, it returns the nonce and the resulting hash.

#### Step 3d — Persist the mined block and advance derived state (`BlockchainFileSystem::mine_block`)

**Code Listing 2.4-6.5**: persist mined block and update derived state (`BlockchainFileSystem::mine_block`)

```rust
// Source: bitcoin/src/store/file_system_db_chain.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Determine the next block height from the current best chain tip.
    let best_height = self.get_best_height().await?;

    // Construct the new block, linking to our current tip hash.
    // Note: `Block::new_block` runs proof-of-work immediately and fills `nonce` + `hash`.
    let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
    let block_hash = block.get_hash();

    // Open the blocks tree in the local database (sled).
    let blocks_tree = self
        .blockchain
        .db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;

    // Persist the block and update the stored tip hash inside the blocks tree.
    Self::update_blocks_tree(&blocks_tree, &block).await?;
    // Update the in-memory tip hash cache so subsequent operations see the new tip.
    self.set_tip_hash(block_hash).await?;

    // Advance derived state: apply the UTXO transition for this block.
    self.update_utxo_set(&block).await?;

    Ok(block)
}
```

**Listing 2.4-6.5 explanation**:

- It constructs a new block at \(height = best\_height + 1\) and runs proof-of-work inside `Block::new_block(...)`.
- It writes the block to storage and advances the tip hash to point at the new block.
- It applies the UTXO state transition for the newly accepted tip.

#### Step 3e — Apply the UTXO state transition for the connected block (`update_utxo_set`)

**Why update the UTXO set?**

The UTXO set represents the current state of spendable outputs—it answers the question "which outputs can be spent right now?" When a new block is added to the chain, transactions in that block consume some outputs (spending them) and create new outputs (making them spendable). The UTXO set must be updated to reflect these changes: spent outputs are removed (they can no longer be spent), and newly created outputs are added (they become available for future transactions). This ensures that the UTXO set always accurately reflects the spendability state at the current chain tip, preventing double-spending and enabling efficient validation of new transactions.

**Diagram — what "updating the UTXO set" means**

```
For each non-coinbase tx in the newly mined block:
  spend: remove the output at index vout from the previous transaction identified by txid
         (each input contains (txid, vout) where txid points to a PREVIOUS transaction 
          and vout is an index into that previous transaction's outputs;
          the previous transaction (identified by txid) can have multiple outputs 
          (e.g., outputs at indices 0, 1, 2);
          when an input references (txid, vout=1), only output 1 from that previous 
          transaction is removed;
          outputs 0 and 2 from that previous transaction remain in the UTXO set and 
          can only be spent when a future transaction references them via their own (txid, vout) pair;
          note: vout is NOT an index into the current transaction's outputs)

For every tx in the newly mined block (including coinbase):
  create: insert all outputs from the transaction under current txid
          (every output created by the transaction is added to the UTXO set;
           all outputs are unspent and available to be referenced by future inputs via (txid, vout))
```

**Code Listing 2.4-6.6**: UTXO update on block connection (`update_utxo_set`)

```rust
// Source: bitcoin/src/store/file_system_db_chain.rs
pub async fn update_utxo_set(&self, block: &Block) -> Result<()> {
    // Derived-state store: "chainstate" tree holds the spendable outputs map (UTXO set).
    let db = self.blockchain.db.clone();
    let utxo_tree = db
        .open_tree("chainstate")
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

    // Apply each transaction to the UTXO set in block order.
    for curr_block_tx in block.get_transactions().await? {
        // Coinbase has no real inputs; it only creates new outputs.
        if !curr_block_tx.is_coinbase() {
            // For each input, remove (spend) the referenced output from the UTXO set.
            for curr_blc_tx_inpt in curr_block_tx.get_vin() {
                // Look up the referenced previous transaction's unspent outputs list.
                let curr_blc_tx_inpt_utxo_ivec = utxo_tree
                    .get(curr_blc_tx_inpt.get_txid())
                    .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                    .ok_or(BtcError::UTXONotFoundError(format!(
                        "(update) UTXO {} not found",
                        curr_blc_tx_inpt.get_input_tx_id_hex()
                    )))?;

                // Deserialize the stored Vec<TXOutput> so we can remove the spent index.
                let curr_blc_tx_inpt_utxo_list: Vec<TXOutput> = bincode::serde::decode_from_slice(
                    curr_blc_tx_inpt_utxo_ivec.as_ref(),
                    bincode::config::standard(),
                )
                .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
                .0;

                // Build a new list with the spent output removed.
                let mut updated_outs = vec![];
                for (utxo_curr_utxo_idx, db_curr_utxo) in
                    curr_blc_tx_inpt_utxo_list.iter().enumerate()
                {
                    // Keep all outputs except the one referenced by this input's vout.
                    if utxo_curr_utxo_idx != curr_blc_tx_inpt.get_vout() {
                        updated_outs.push(db_curr_utxo.clone())
                    }
                }

                // Persist the updated list (or remove the key if nothing remains unspent).
                if updated_outs.is_empty() {
                    // No outputs remain: delete the entire UTXO entry.
                    utxo_tree
                        .remove(curr_blc_tx_inpt.get_txid())
                        .map_err(|e| BtcError::RemovingUTXOError(e.to_string()))?;
                } else {
                    // Outputs remain: overwrite the entry with the remaining unspent outputs.
                    let outs_bytes = bincode::serde::encode_to_vec(
                        &updated_outs,
                        bincode::config::standard(),
                    )
                    .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
                    utxo_tree
                        .insert(curr_blc_tx_inpt.get_txid(), outs_bytes)
                        .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                }
            }
        }

        // Insert the new outputs created by this transaction as fresh UTXOs.
        let mut new_outputs = vec![];
        for curr_tx_out in curr_block_tx.get_vout() {
            new_outputs.push(curr_tx_out.clone())
        }

        // Store the outputs under the current transaction id.
        let outs_bytes = bincode::serde::encode_to_vec(&new_outputs, bincode::config::standard())
            .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
        let _ = utxo_tree
            .insert(curr_block_tx.get_id(), outs_bytes)
            .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
    }
    Ok(())
}
```

**Listing 2.4-6.6 explanation**:

- For each non-coinbase transaction, it removes the spent output index from the referenced previous transaction’s UTXO entry.
- For every transaction, it inserts the newly created outputs, making them spendable for subsequent transactions.

### Step 4 — Broadcast and receive blocks (INV → BLOCK handling)

Once a block is mined and persisted locally, it must be propagated to other nodes in the network. This implementation uses an "inventory then request data" pattern for efficient network communication between nodes. The following describes the network protocol messages used for block propagation:

- **INV**: announce "I have object X" (by id only) — a lightweight inventory message that tells peers what blocks or transactions a node has available, without sending the full data. This allows peers to discover new content efficiently without wasting bandwidth on data they may already have.

- **GETDATA**: request the full bytes for X — when a peer receives an INV message for a block or transaction it doesn't have, it sends GETDATA to request the complete data. This two-step process (INV → GETDATA) reduces bandwidth by only transmitting full blocks when explicitly requested.

- **BLOCK**: deliver the full block bytes — the final step where the node that announced the block (via INV) sends the complete serialized block data in response to a GETDATA request. The receiving node can then validate and add the block to its chain.


**Diagram — block relay handshake**

```
Miner/peer A                              Peer B
---------                                 ------
INV(Block, block_hash)  ----------------->  (learns block id)
GETDATA(Block, block_hash) <-------------  (requests full bytes)
BLOCK(block_bytes)       ---------------->  (receives full block)
                                            add_block(block)
                                            remove txs from mempool (housekeeping)
```

**Code Listing 2.4-6.9**: broadcast block inventory (`broadcast_new_block`)

```rust
// Source: bitcoin/src/node/miner.rs
pub async fn broadcast_new_block(block: &Block) -> Result<()> {
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();
    // Broadcast to all known peers except ourselves.
    let nodes = GLOBAL_NODES.get_nodes().expect("Global nodes get error");
    nodes
        .iter()
        .filter(|node| !my_node_addr.eq(&node.get_addr()))
        .for_each(|node| {
            let node_addr = node.get_addr();
            // Only the block hash is sent in INV; full bytes come later via GETDATA/BLOCK.
            let block_hash = block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Block, &[block_hash]).await;
            });
        });
    Ok(())
}
```

**Listing 2.4-6.9 explanation**:

- It announces the new block by hash (inventory), rather than pushing the full block to peers.
- It spawns per-peer sends so announcement does not block the mining flow.

**Code Listing 2.4-6.10**: message constructors for INV / GETDATA / BLOCK (`send_inv`, `send_get_data`, `send_block`)

```rust
// Source: bitcoin/src/net/net_processing.rs
pub async fn send_get_data(addr_to: &SocketAddr, op_type: OpType, id: &[u8]) {
    // Request the full bytes for an object previously announced by INV.
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

pub async fn send_inv(addr_to: &SocketAddr, op_type: OpType, blocks: &[Vec<u8>]) {
    // Inventory: announce that we have one or more objects (by id only).
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

pub async fn send_block(addr_to: &SocketAddr, block: &Block) {
    // Respond to GETDATA by sending the full serialized block payload.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize().expect("Block serialization error"),
        },
    )
    .await;
}
```

**Code Listing 2.4-6.11**: network handler for INV / GETDATA / BLOCK (`process_stream`)

This is a long handler because it supports many message types; for the purposes of this section, focus on the `Package::{Inv,GetData,Block}` match arms and treat the other arms as contextual background.

```rust
// Source: bitcoin/src/net/net_processing.rs
#[instrument(skip(node_context, stream))]
pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    // Extract the peer's network address from the TCP stream
    // This identifies which node is communicating with us on this connection
    let peer_addr = stream.peer_addr()?;
    // Wrap the stream in a buffered reader for efficient reading of network data
    let reader = BufReader::new(&stream);
    // Create a deserializer that converts incoming bytes into Package enum variants
    // This handles the network protocol parsing (INV, GETDATA, BLOCK, etc.)
    let pkg_reader = Deserializer::from_reader(reader).into_iter::<Package>();

    // Process each incoming network message sequentially
    // Each TCP message is deserialized into a Package enum and handled by the match statement below
    for pkg in pkg_reader {
        let pkg = pkg?;
        info!("Receive request from {}: {:?}", peer_addr, pkg);

        match pkg {
            Package::Block { addr_from, block } => {
                // BLOCK: we received the full block bytes. This is the acceptance boundary.
                // Deserialize the raw bytes into a Block struct
                let block =
                    Block::deserialize(block.as_slice()).expect("Block deserialization error");
                // Add the block to our local blockchain (this performs validation and updates chainstate)
                // This is the critical moment where a block becomes part of our chain
                node_context
                    .add_block(&block)
                    .await
                    .expect("Blockchain write error");
                let added_block_hash = block.get_hash_bytes();
                info!("Added block {:?}", added_block_hash.as_slice());

                // Local housekeeping: once a block is accepted, its transactions are no longer pending here.
                // Remove all transactions from the mempool since they're now confirmed in a block
                // This prevents duplicate processing and keeps the mempool clean
                for tx in block.get_transactions().await? {
                    node_context.remove_from_memory_pool(tx.clone()).await;
                }

                // Maintain the "blocks in transit" queue: once we received one, remove it.
                // This tracks which blocks we've requested but haven't received yet
                // Removing it marks this block as successfully received
                let removed_block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                    .remove(added_block_hash.as_ref())
                    .expect("Block removal error");
                if removed_block_hash.is_some() {
                    info!(
                        "Removed block {:?} FROM GLOBAL_BLOCKS_IN_TRANSIT",
                        removed_block_hash.expect("Block removal error").as_slice()
                    );
                }

                // If we are still synchronizing blocks, request the next one.
                // Continue the block download process by requesting the next block in the queue
                // This enables efficient chain synchronization when catching up with the network
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
            Package::GetBlocks { addr_from } => {
                // GETBLOCKS: peer asks for our inventory of block hashes.
                // Retrieve all block hashes we have in our local blockchain
                // This is used during initial synchronization when a peer wants to know what blocks we have
                let blocks = node_context
                    .get_block_hashes()
                    .await
                    .expect("Blockchain read error");
                // Send an inv message with a list of hashes to the requesting peer.
                // The peer can then request specific blocks it doesn't have via GETDATA
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
            Package::GetData {
                addr_from,
                op_type,
                id,
            } => match op_type {
                OpType::Block => {
                    // GETDATA(Block): peer requests the full bytes for a block hash.
                    // Look up the block in our local blockchain storage
                    if let Some(block) = node_context
                        .get_block(id.as_slice())
                        .await
                        .expect("Blockchain read error")
                    {
                        // Send the complete serialized block data to the requesting peer
                        // This completes the INV → GETDATA → BLOCK handshake
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    // GETDATA(Tx): peer requests the full bytes for a transaction id.
                    // Convert the transaction ID from bytes to hex string for mempool lookup
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    // Check if we have this transaction in our mempool
                    if let Some(tx) = GLOBAL_MEMORY_POOL
                        .get(txid_hex.as_str())
                        .expect("Memory pool get error")
                    {
                        // Send the complete transaction data to the requesting peer
                        send_tx(&addr_from, &tx).await;
                    } else {
                        // Transaction not found - it may have been mined into a block already
                        // This is normal and indicates the transaction is confirmed
                        info!(
                            "Received request to forward a Transaction that is not found in memory pool. 
                        Most likely it has been mined!!!: {:?}",
                            txid_hex
                        );
                    }
                }
            },
            Package::Inv {
                addr_from,
                op_type,
                items,
            } => match op_type {
                OpType::Block => {
                    // INV(Block): peer announces one or more block hashes; request the first immediately.
                    // Add these block hashes to our "blocks in transit" queue
                    // This tracks which blocks we've been notified about but haven't received yet
                    GLOBAL_BLOCKS_IN_TRANSIT
                        .add_blocks(items.as_slice())
                        .expect("Blocks in transit add error");

                    // Request the first block immediately to start the download process
                    // We'll request subsequent blocks after receiving each one (see Package::Block handler above)
                    let block_hash = items.first().expect("Blocks in transit add error");
                    send_get_data(&addr_from, OpType::Block, block_hash).await;
                }
                OpType::Tx => {
                    // INV(Tx): peer announces a txid; request it if we don't already have it pending.
                    // Extract the first transaction ID from the inventory message
                    let txid = items.first().expect("Blocks in transit add error");
                    let txid_hex = HEXLOWER.encode(txid);

                    // Only request the transaction if we don't already have it in our mempool
                    // This avoids redundant network traffic for transactions we've already seen
                    if !GLOBAL_MEMORY_POOL
                        .contains(txid_hex.as_str())
                        .expect("Memory pool contains error")
                    {
                        // Request the full transaction data via GETDATA
                        send_get_data(&addr_from, OpType::Tx, txid).await;
                    }
                }
            },
            Package::Tx {
                addr_from,
                transaction,
            } => {
                // TX: peer sent the full transaction bytes; hand it to the node’s mempool admission path.
                let tx = Transaction::deserialize(transaction.as_slice())
                    .expect("Transaction deserialization error");
                // Process the transaction through our validation and mempool admission logic
                // This includes signature verification, UTXO checks, and mempool insertion
                match node_context.process_transaction(&addr_from, tx).await {
                    Ok(_) => (),  // Transaction successfully added to mempool
                    Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid)) => {
                        // Transaction is a duplicate - send error message back to peer
                        // This helps peers avoid resending transactions we already have
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            format!("Transaction: {} already exists", txid),
                        )
                        .await;
                    }
                    Err(e) => Err(e)?,  // Propagate other errors (validation failures, etc.)
                }
            }

            Package::SendBitCoin {
                addr_from,
                wlt_frm_addr,
                wlt_to_addr,
                amount,
            } => {
                let validated_wlt_frm_addr = WalletAddress::validate(wlt_frm_addr);
                let validated_wlt_to_addr = WalletAddress::validate(wlt_to_addr);

                match (validated_wlt_frm_addr, validated_wlt_to_addr) {
                    (Ok(_), Err(_)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_to: ${wlt_to_addr}".to_string(),
                        )
                        .await;
                    }
                    (Err(_), Ok(_)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                        )
                        .await;
                    }
                    (Err(_), Err(_)) => {
                        let send_message_invalid_to = send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_to: ${wlt_to_addr}".to_string(),
                        );
                        let send_message_invalid_from = send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                        );
                        tokio::join!(send_message_invalid_to, send_message_invalid_from);
                    }
                    (Ok(from), Ok(to)) => {
                        let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());

                        match node_context.btc_transaction(&from, &to, amount).await {
                            Ok(_) => (),
                            Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid)) => {
                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!("Transaction: {} already exists", txid),
                                )
                                .await;
                            }
                            Err(BtcError::NotEnoughFunds) => {
                                let current_balance =
                                    utxo_set.get_balance(&from).await.unwrap_or(0);

                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!(
                                        "Insufficient funds: cannot send {} bitcoin. Current balance: {} bitcoin",
                                        amount, current_balance
                                    ),
                                )
                                .await;

                                error!(
                                    "Transaction rejected: insufficient funds. From: {}, To: {}, Amount: {}, Balance: {}",
                                    from.as_str(),
                                    to.as_str(),
                                    amount,
                                    current_balance
                                );
                            }
                            Err(e) => {
                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!("Transaction creation failed: {}", e),
                                )
                                .await;

                                error!("Transaction creation failed: {}", e);
                            }
                        }
                    }
                }
            }
            Package::Version {
                addr_from,
                version,
                best_height,
            } => {
                debug!("version = {}, best_height = {}", version, best_height);
                let local_best_height = node_context
                    .get_blockchain_height()
                    .await
                    .expect("Blockchain read error");
                if local_best_height < best_height {
                    send_get_blocks(&addr_from).await;
                }
                if local_best_height > best_height {
                    send_version(
                        &addr_from,
                        node_context
                            .get_blockchain_height()
                            .await
                            .expect("Blockchain read error"),
                    )
                    .await;
                }

                if !GLOBAL_NODES
                    .node_is_known(&addr_from)
                    .expect("Node is known error")
                {
                    GLOBAL_NODES.add_node(addr_from).expect("Node add error");
                }
            }
            Package::KnownNodes { addr_from, nodes } => {
                process_known_nodes(node_context.clone(), &addr_from, nodes).await;
            }
            Package::Message {
                addr_from,
                message_type,
                message,
            } => match message_type {
                MessageType::Error => {
                    error!("{} sent error: {}", addr_from, message);
                }
                MessageType::Warning => {
                    warn!("{} sent warning: {}", addr_from, message);
                }
                MessageType::Info => {
                    debug!("{} sent info: {}", addr_from, message);
                }
                MessageType::Success => {
                    debug!("{} sent success: {}", addr_from, message);
                }
                MessageType::Ack => {
                    debug!("{} sent ack: {}", addr_from, message);
                }
            },
            Package::AdminNodeQuery {
                addr_from,
                query_type,
            } => match query_type {
                AdminNodeQueryType::GetBalance { wlt_address } => {
                    let address_valid = WalletAddress::validate(wlt_address)?;

                    let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());
                    let balance = utxo_set
                        .get_balance(&address_valid)
                        .await
                        .expect("UTXO set get balance error");
                    debug!("Balance of {}: {}", addr_from, balance);
                }
                AdminNodeQueryType::GetAllTransactions => {
                    let transactions_summary = node_context
                        .find_all_transactions()
                        .await
                        .expect("Blockchain find all transactions error");

                    info!("═══════════════════════════════════════════════════════════════");
                    info!("                    BLOCKCHAIN TRANSACTIONS");
                    info!("═══════════════════════════════════════════════════════════════");

                    for (idx, (cur_txid_hex, tx_summary)) in transactions_summary.iter().enumerate()
                    {
                        let mut tx_summary_input = tx_summary.clone();
                        let mut tx_summary_output = tx_summary.clone();
                        let tx_summary_inputs = tx_summary_input.get_inputs();
                        let tx_summary_outputs = tx_summary_output.get_outputs();
                        info!("");
                        info!("┌─ Transaction #{}", idx + 1);
                        info!("│  ID: {}", cur_txid_hex);
                        info!(
                            "│  Type: {}",
                            if tx_summary_inputs.is_empty() {
                                "Coinbase"
                            } else {
                                "Regular"
                            }
                        );

                        if !tx_summary_inputs.is_empty() {
                            info!("│  ┌─ Inputs ({}):", tx_summary_inputs.len());
                            for (input_idx, input_summary) in tx_summary_inputs.iter().enumerate() {
                                info!(
                                    "│  │  {} └─ From: {} (txid: {}, vout: {})",
                                    if input_idx == tx_summary_inputs.len() - 1 {
                                        "└"
                                    } else {
                                        "├"
                                    },
                                    input_summary.get_wlt_addr().as_str(),
                                    input_summary.get_txid_hex(),
                                    input_summary.get_output_idx()
                                );
                            }
                        }

                        info!("│  ┌─ Outputs ({}):", tx_summary_outputs.len());
                        for (output_idx, output_summary) in tx_summary_outputs.iter().enumerate() {
                            info!(
                                "│  │  {} └─ To: {} (value: {} BTC)",
                                if output_idx == tx_summary_outputs.len() - 1 {
                                    "└"
                                } else {
                                    "├"
                                },
                                output_summary.get_wlt_addr().as_str(),
                                output_summary.get_value()
                            );
                        }
                        info!("└─────────────────────────────────────────────────────────────");
                    }

                    info!("");
                    info!("═══════════════════════════════════════════════════════════════");
                    info!("Total Transactions: {}", transactions_summary.len());
                    info!("═══════════════════════════════════════════════════════════════");
                }
                AdminNodeQueryType::GetBlockHeight => {
                    let height = node_context
                        .get_blockchain_height()
                        .await
                        .expect("Blockchain read error");
                    trace!("Block height: {}", height);
                }
                AdminNodeQueryType::MineEmptyBlock => {
                    if GLOBAL_CONFIG.is_miner() {
                        let mining_address =
                            GLOBAL_CONFIG.get_mining_addr().ok_or(BtcError::NotAMiner)?;
                        node_context.mine_empty_block(&mining_address).await.map(|_| ())?
                    } else {
                        trace!("Not a miner");
                    }
                    trace!("Mining empty block");
                }
                AdminNodeQueryType::ReindexUtxo => {
                    let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());
                    utxo_set.reindex().await.expect("UTXO set reindex error");
                    let count = utxo_set
                        .count_transactions()
                        .await
                        .expect("UTXO set count error");
                    trace!(
                        "Reindexed UTXO set. There are {} transactions in the UTXO set.",
                        count
                    );
                }
            },
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}
```

**Listing 2.4-6.11 explanation (block relay path)**:

- On `Package::Inv` for a block, it requests the block by hash (`send_get_data`).
- On `Package::GetData` for a block, it serves the block bytes if present (`send_block`).
- On `Package::Block`, it deserializes the block, hands it to `node_context.add_block(&block)` (chainstate acceptance boundary), and then removes the block’s transactions from the local mempool.

## Navigation

- **Previous**: Section 2.4.5 (Transaction Lifecycle)
- **Next**: Section 2.4.7 (Consensus and Validation)

---

<div align="center">

**📚 [← Previous: Transaction Lifecycle](05-Transaction-Lifecycle.md)** | **Block Lifecycle and Mining** | **[Next: Consensus and Validation →](07-Consensus-and-Validation.md)** 📚

</div>

