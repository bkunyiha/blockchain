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
10. <a href="01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. **Chapter 16: Consensus and Validation** ← *You are here*
17. <a href="08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
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

# Chapter 16: Consensus and Validation

This section explains how the node decides what to **accept** (validity) and what to **build on** (fork choice). The goal is to make the implementation readable without having the repository open: we will print the relevant Rust methods in full and explain how each one participates in the acceptance boundary.

We will use three primary code locations:

- `bitcoin/src/primitives/transaction.rs` (transaction construction, signing, verification)
- `bitcoin/src/pow.rs` + `bitcoin/src/primitives/block.rs` (proof-of-work and block construction)
- `bitcoin/src/store/file_system_db_chain.rs` (block insertion, fork choice, reorg, UTXO maintenance)

## Scope (section flow)

This chapter covers the rules behind **the blockchain pipeline (Blockchain — From Transaction to Block Acceptance) Steps 3 and 8**: transaction validity checks and the block acceptance boundary (validate → connect).

## Conceptual model (what “consensus” means in this codebase)

We will use the following distinction throughout the chapter:

- **Validation (local correctness checks)**: “Is this object internally consistent and authorized?”
  Examples: transaction signatures verify; a block contains exactly one coinbase transaction; a block hash satisfies proof-of-work.

- **Consensus / fork choice (global convergence rule)**: “Given multiple competing histories, which one is canonical?”
  In Bitcoin, the rule is “the chain with the most cumulative proof-of-work wins.” In this learning implementation, fork choice is modeled as a deterministic hierarchy: **height → cumulative work → hash tie-break**.

The most important engineering rule is the Step‑5 gate from the whitepaper (§5): **do not mutate durable chain state (tip + UTXO set) unless the block is valid and its spends are not already spent**. When this boundary is violated, you can “mint” money or accept double-spends even if proof-of-work is present elsewhere.

## Diagram: acceptance boundary (“Validate → Connect”)

               ┌──────────────────────────────────────────────────────────┐
Incoming data  │  Validate (pure checks, no durable state mutation)        │
──────────────►│  - tx signatures / structure                              │
               │  - block structure (coinbase rules, etc.)                 │
               │  - proof-of-work target check (TODO in some paths)        │
               └───────────────┬──────────────────────────────────────────┘
                               │ only if VALID
                               ▼
               ┌──────────────────────────────────────────────────────────┐
               │  Connect (state mutation)                                 │
               │  - write block to DB                                      │
               │  - update tip / reorganize chain                          │
               │  - update UTXO set (spend inputs, create outputs)         │
               └──────────────────────────────────────────────────────────┘

In the current codebase, transaction signature verification is implemented and used during mining. The full Step‑5 block acceptance gate is explicitly noted as a FIXME in `BlockchainFileSystem::add_block` and is explored further in the dedicated Step‑5 chapter.

> **Important:** The current implementation does **not** consistently enforce the Bitcoin whitepaper’s Step‑5 contract (“**valid AND not already spent**”) as a hard **Validate → Connect** gate for inbound blocks before mutating durable state (tip + UTXO set).
> For the concrete “what’s missing + how to implement it” deep dive, read **Section 10 (Whitepaper Step 5: Block Acceptance)**.

## 1) Transaction Validation (signature verification)

Validation is performed in `Transaction::verify(...)`. Conceptually, this method answers:

> “Does each input prove authorization to spend the referenced previous output?”

It does this by reconstructing the signed digest for each input and then verifying the Schnorr signature against the input’s public key.

## 2) Proof-of-work (mining) and block construction

In this implementation, proof-of-work is computed when a block is constructed. Two methods matter:

- `Block::new_block(...)` (construct header, run PoW, fill in `nonce` and `hash`)
- `ProofOfWork::run()` (iterate nonces until hash < target)

These methods define *how* blocks are mined. Later, fork choice decides *which* mined blocks become canonical.

Block insertion and fork choice happens in `BlockchainFileSystem::add_block(...)`. The method:

- inserts the block into the sled “blocks tree”,
- rejects duplicates,
- and applies fork choice (height/work/tie-break), potentially triggering a reorganization.

Mining uses `BlockchainFileSystem::mine_block(...)`, which constructs a new block on the current tip and then updates the UTXO set for that mined block.

## 6) Diagram: fork-choice hierarchy used by `add_block`

add_block(new_block)
  ├─> if height greater: accept as tip
  ├─> else if height equal:
  │     ├─> compare get_chain_work()
  │     └─> if equal: deterministic hash tie-break
  └─> else: do not update tip

## 3) Chain Selection (fork choice)

The code implements a three-level rule:

1. **Height comparison** (longest chain).
2. **Cumulative work** (`get_chain_work(...)`).
3. **Deterministic hash tie-break**.

## 3.1) Double-spending: what the “accept only if…” gate is protecting

The double-spend problem is simple to state and easy to accidentally reintroduce in code:

- An attacker creates two different transactions that spend the **same outpoint** $(txid, vout)$.
- If different peers accept different spends (even temporarily), your node’s notion of “balance” and “ownership” becomes inconsistent.

In our implementation, the core protection is: **only mutate the canonical chain tip + UTXO set after we are sure the block is valid**.

### 3.1.1 Diagram: conflicting spends of the same outpoint

UTXO: (txA, vout=0) locked to Alice

tx1: spends (txA,0) -> pays Bob
tx2: spends (txA,0) -> pays Mallory

Both cannot be accepted into the same canonical history.

### 3.1.2 Where the protection lives in code (mental map)

- **Transaction-level check**: `Transaction::verify(...)` proves authorization (signatures).
- **State-level check**: the UTXO set determines whether an outpoint is **already spent**.
- **Block-level gate**: `BlockchainFileSystem::add_block(...)` is where “accept only if…” must hold before updating tip/UTXO.

For the whitepaper-aligned “Validate → Connect” explanation (Step 5), continue to:

- **Block Acceptance (Whitepaper §5, Step 5)**

## 4) Implementation boundaries (what is simplified or incomplete)

These whitepaper or Bitcoin Core features are intentionally simplified:

- Difficulty retargeting (TARGET_BITS is constant).
- Full Merkle tree verification.
- Full mempool policy and conflict tracking.
- Robust fork reorg logic in normal operation.

## 16) Reorgs (what they mean, and what we would add next)

In Bitcoin, nodes sometimes learn about a competing branch that becomes “better” (more cumulative work). When that happens, the node must **reorganize**:

1. Find the common ancestor of the old tip and the new tip.
2. Undo state changes from blocks that are no longer on the best chain.
3. Apply blocks on the winning branch in order.

In practical terms, a correct reorg requires the ability to **roll back and re-apply UTXO updates** deterministically. Our codebase contains a reorg path (`reorganize_chain` plus UTXO rollback/apply helpers).

**Critical rule**: rolled-back blocks are **never deleted** from the database. They remain as non-canonical entries so that `find_common_ancestor()` can always walk both branches during future reorganizations. This matches Bitcoin Core's behavior where all blocks are kept in `blk*.dat` files.

**Critical implementation details for correct reorgs:**

- **UTXO rollback for fully-spent transactions**: When `rollback_block()` restores a spent output, the original transaction's UTXO entry may have been completely removed from the database (because all its outputs were spent). The code handles this by creating a fresh vector when the entry is missing, rather than silently losing the output. Without this, coins would be permanently lost during reorganization.

- **Reverse transaction order**: During rollback, transactions are processed newest-first (reverse order). This correctly handles intra-block dependencies where a later transaction spends an earlier transaction's output within the same block.

- **Blocks are never deleted during rollback**: The `rollback_to_block()` method only rolls back the UTXO set and moves the tip pointer — it does NOT delete blocks from the database. Rolled-back blocks must remain available for `find_common_ancestor()` to work during future reorganizations. This matches Bitcoin Core's behavior where all blocks stay in `blk*.dat` files.

## Multi-Node Consensus: Mining Protection and Block Relay

When multiple miners compete in a multi-node network, several additional mechanisms ensure correct consensus:

### Block Relay

When a node receives a block and it is **new** (not already in the database), the node relays the block's `Inv` to all its peers (except the sender). This ensures blocks propagate across the full network, not just one hop from the miner. Without relay, in a linear topology (1→2→3→4→5→6→7), a block mined by Node 4 would only reach Nodes 3 and 5 — the rest of the network would never see it.

The `block_is_new` check prevents infinite relay loops where nodes keep forwarding the same block back and forth.

### Mining Concurrency and Cancellation

When transactions arrive at a node, mining is triggered in a background task. Two guards prevent problems:

1. **`MINING_IN_PROGRESS` flag**: Prevents concurrent mining tasks from running simultaneously on the same node.
2. **`MINING_CANCELLED` flag**: Set by the `Package::Block` handler when a competing block arrives. Checked before mining starts — if a competing block was already accepted, mining is aborted.

**Critical rule**: Cancellation is NEVER checked after `mine_block()` completes. Once the block is created and added to the local chain, it MUST be broadcast. Skipping the broadcast would leave an unbroadcast block in the node's chain, creating a permanent fork where this node has a block nobody else knows about.

### Stale Mining Protection

Between `prepare_mining_utxo()` (which validates transaction inputs with a read lock) and the actual `mine_block()` call (which requires the write lock), a competing block may be accepted — spending the same transaction inputs. The `chainstate.rs` wrapper re-validates all inputs under the write lock just before mining. If any inputs are already spent, mining is aborted with a "stale mining" error. Without this check, the node would create a block with already-spent inputs, and `update_utxo_set` would silently add the coinbase transaction — creating money from nothing.

## 5) Whitepaper Connections: Network Operation and Security Intuition

### 18 Network operation steps (Whitepaper §5)

The whitepaper’s Section 5 lists the “network operation” loop. In this codebase, the closest mapping is:

1. **New transactions are broadcast**
   - Entry point: `NodeContext::process_transaction(...)`
   - Propagation primitive: `send_inv(...)` in `bitcoin/src/net/net_processing.rs`

2. **Nodes collect new transactions into a block**
   - `miner::prepare_mining_utxo(...)` pulls from `GLOBAL_MEMORY_POOL`

3. **Nodes work on proof-of-work**
   - `ProofOfWork::run()` in `bitcoin/src/pow.rs`

4. **When a node finds PoW, it broadcasts the block**
   - `send_inv(...)` announces the new block hash

5. **Nodes accept the block only if all transactions are valid and not already spent**
   - This is the Step‑5 acceptance gate. In this book, it’s covered explicitly here:
     - **Block Acceptance (Whitepaper §5, Step 5)**

6. **Nodes express acceptance by working on the next block**
   - Mining continues on the best tip selected by `add_block(...)` logic.

### 19 Security analysis (Whitepaper §11) — what we should take away as implementers

The whitepaper’s Section 11 quantifies the “attacker catching up” probability. You don’t need the exact math to understand the implementation impact:

- **More confirmed blocks → harder to rewrite history** because rewriting requires redoing PoW from the fork point forward.
- **Chain selection must be deterministic** across nodes (height/work/tie-break) or the network will diverge.
- **Step‑5 correctness is non-negotiable**: if we accept invalid spends or double-spends, PoW cannot “save” the system.

---

## Acceptance boundary walkthrough (Validate → Add To Block)

**Goal**: identify the exact code paths that enforce “valid transactions”, choose the best chain, and decide when state (tip + UTXO set) changes.

**Code walkthrough**:
- `bitcoin/src/primitives/transaction.rs`
- `bitcoin/src/chain/chainstate.rs`
- `bitcoin/src/store/file_system_db_chain.rs`

**Whitepaper anchors**:
- Section 2 (Signature-based ownership transfer)
- Section 4 (PoW as the resource that makes rewriting expensive)
- Section 5 (Step 5: “accept only if…”, plus fork-choice intuition)
- Section 11 (Security analysis: attacker catching up)

### Step 1 — Transaction validity = signature checks (`Transaction::verify`)

**Transaction validity (signature verification) code**: `bitcoin/src/primitives/transaction.rs`

Transaction-level validity here primarily means: “do signatures verify against the reconstructed digest?”

### Listing 9-8.1: Transaction validity (signature verification) (`Transaction::verify`)
> **Source:** `transaction.rs` — Source

```rust
pub async fn verify(&self, blockchain: &BlockchainService) -> Result<bool> {
    if self.is_coinbase() {
        return Ok(true);
    }
    let mut trimmed_self_copy = self.trimmed_copy();
    for (idx, vin) in self.vin.iter().enumerate() {
        let current_vin_tx = blockchain
            .find_transaction(vin.get_txid())
            .await??;
        // ... (set up trimmed copy with
        // previous output's pub_key_hash)
        trimmed_self_copy.id =
            trimmed_self_copy.hash()?;
        // Verify Schnorr signature against
        // reconstructed digest
        let verify = schnorr_sign_verify(
            vin.get_pub_key(),
            vin.get_signature(),
            trimmed_self_copy.get_id(),
        );
        if !verify {
            return Ok(false);
        }
    }
    Ok(true)
}
```

- **What to notice**
  - For each input, the code looks up the referenced previous transaction to pull the locking data.
  - The digest being verified is a “trimmed copy” digest (signatures don’t sign themselves).
  - Coinbase transactions short-circuit verification.
- **Whitepaper mapping**
  - **§2**: signatures enforce the chain of ownership for spent outputs.

### Step 2 — Mining-time transaction gate and block construction

Mining is implemented at two layers:

- `BlockchainService::mine_block(...)` performs a simple “do not mine invalid txs” gate.
- `BlockchainFileSystem::mine_block(...)` constructs the block on the current tip and updates the UTXO set.

In addition, block construction itself (`Block::new_block`) runs proof-of-work using `ProofOfWork::run`.

**Mining-time validation gate code**: `bitcoin/src/chain/chainstate.rs`

Before a mined block is constructed, `BlockchainService::mine_block` verifies each transaction:

### Listing 9-8.2: Mining-time validation gate (`BlockchainService::mine_block`)
> **Source:** `chainstate.rs` — Source

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    for tx in transactions {
        let is_valid = tx.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }
    // ... (locking mechanism)
    self.0.write().await.mine_block(transactions).await
}
```

### Listing 9-8.3: Block mining (construct block + update state) (`BlockchainFileSystem::mine_block`)

> **Source:** `file_system_db_chain.rs` — Source

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    let best_height = self.get_best_height().await?;
    let block = Block::new_block(
        self.get_tip_hash().await?,
        transactions,
        best_height + 1
    );
    let block_hash = block.get_hash();
    // ... (persist to sled, update tip hash, UTXO set)
    Ok(block)
}
```

### Listing 9-8.4: Block construction + proof-of-work (`Block::new_block`)

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
    // Run PoW to find nonce such that
    // hash < target difficulty
    let pow = ProofOfWork::new_proof_of_work(
        block.clone()
    );
    let (nonce, hash) = pow.run();
    block.header.nonce = nonce;
    block.header.hash = hash;
    block
}
```

### Listing 9-8.5: Proof-of-work loop (`ProofOfWork::run`)

> **Source:** `pow.rs` — Source

```rust
pub fn run(&self) -> (i64, String) {
    let mut nonce = 0;
    let mut hash = Vec::new();
    while nonce < MAX_NONCE {
        let data = self.prepare_data(nonce);
        hash = crate::sha256_digest(data.as_slice());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
        if hash_int.lt(self.target.borrow()) {
            // Found valid hash < target difficulty
            break;
        } else {
            nonce += 1;
        }
    }
    (nonce, HEXLOWER.encode(hash.as_slice()))
}
```

- **What to notice**
  - Mining path is defensive: it refuses to mine invalid transactions.
  - This is not a substitute for Step‑5 acceptance on inbound blocks (network blocks still need a hard gate).
- **Whitepaper mapping**
  - **§5 steps 2–4**: nodes collect transactions and work on PoW; mining should build on valid transactions.

### Step 3 — Block acceptance and fork choice (`BlockchainFileSystem::add_block`)

**Fork-choice and block acceptance code**: `bitcoin/src/store/file_system_db_chain.rs`

The acceptance point for inbound blocks is the storage layer’s `add_block(...)` function. This is where:

- the block is inserted into sled,
- the tip may or may not update,
- consensus/tie-breaking and reorg logic runs.

Because this chapter is intended to stand alone, we print the method in full. While reading it, pay special attention to two boundaries:

- **DB insertion vs. consensus decision**: the sled transaction closure is synchronous, while the fork-choice logic is async and runs afterwards.
- **Validation gate (FIXME)**: the method currently contains a whitepaper Step‑5 FIXME (“accept only if all txs valid and not already spent”) that must be enforced before state mutation in a production node.

### Listing 9-8.6: Block insertion + fork choice (`BlockchainFileSystem::add_block`)

> **Source:** `file_system_db_chain.rs` — Source
> **Note:** This is a simplified version focused on the fork-choice logic. For the full implementation including the sled atomic transaction that persists the block before fork-choice runs, see Chapter 11 (Storage Layer Code Walkthrough), Listing 11-6.

```rust
pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
    let block_tree = self.blockchain.db.open_tree(self.get_blocks_tree_path())?;

    // 1) Empty chain: first block always becomes tip
    if self.is_empty() { /* ... accept and update UTXO ... */ }

    // 2) Dedup: skip if already present
    if block_tree.get(new_block.get_hash())?.is_some() { return Ok(()); }

    // 3) Persist inside sled transaction (block is now in DB
    // regardless of fork choice)
    block_tree.transaction(|tx| {
        tx.insert(new_block.get_hash(), new_block.serialize()?)
    })?;

    // 4) Fork-choice: three-level consensus hierarchy
    let current_tip = self.get_tip_hash().await?;
    let current_height = self.get_best_height().await?;

    match new_block.get_height().cmp(&current_height) {
        Ordering::Greater => {
            if new_block.get_pre_block_hash() == current_tip {
                // Normal case: extends our current chain
                self.set_tip_hash(new_block.get_hash()).await?;
                self.update_utxo_set(new_block).await?;
            } else {
                // FORK: reorganize to rollback old branch's UTXO
                self.reorganize_chain(new_block.get_hash()).await?;
            }
        }
        Ordering::Equal => {
            if new_block.get_pre_block_hash() == current_tip {
                return Ok(()); // Extends our tip naturally
            }
            // Competing tips: compare cumulative work, then tie-break
            let current_work = self.get_chain_work(&current_tip).await?;
            let new_work = self.get_chain_work(new_block.get_hash()).await?;
            match new_work.cmp(&current_work) {
                Ordering::Greater => {
                    self.reorganize_chain(new_block.get_hash()).await?;
                }
                Ordering::Equal => {
                    // Deterministic tie-break (lexicographic hash comparison)
                    if self
                        .accept_new_block_tie_break(new_block, &current_tip)
                        .await?
                    {
                        self.reorganize_chain(new_block.get_hash()).await?;
                    }
                }
                Ordering::Less => { /* weaker chain, keep ours */ }
            }
        }
        Ordering::Less => {
            // Block stored but not accepted as tip
        }
    }
    Ok(())
}
```

> **Key design decisions in `add_block`:**
>
> - **Step 3 inserts the block BEFORE fork-choice runs.** This means ALL received blocks end up in the database, even if they're rejected by consensus. This is intentional — `find_common_ancestor()` needs to walk both branches during future reorganizations, and it can only do so if the blocks are in the DB.
>
> - **`Ordering::Greater` checks for forks.** A higher-height block might be on a DIFFERENT branch (its parent doesn't match our current tip). This can happen when block relay delivers blocks out of order. Without the fork check, the old branch's UTXO (including coinbase subsidies) would never be rolled back — creating money from nothing.
>
> - **`Ordering::Less` blocks are retained, not deleted.** During `rollback_to_block()`, blocks are also NOT deleted from the database. This ensures `find_common_ancestor()` always works, matching Bitcoin Core's behavior where all blocks are kept in `blk*.dat` files regardless of whether they're on the active chain.

- **What to notice**
  - This is the canonical “fork-choice” boundary in this codebase: it may update tip and may reorganize.
  - UTXO updates happen when the code decides the new block/branch is canonical.
  - **`Ordering::Greater` now detects forks**: If a higher-height block's parent does NOT match our current tip, it is on a different branch. The code calls `reorganize_chain()` instead of directly updating the tip. Without this, the old branch's UTXO (including coinbase subsidies) would remain, creating money from nothing.
  - **`Ordering::Less` blocks are retained**: Blocks at lower heights are stored in the DB (by the Sled transaction) but not accepted as the tip. They remain available for `find_common_ancestor()` during future reorganizations. Rolled-back blocks are also never deleted from the database.
  - **Block relay**: When the `Package::Block` handler in `net_processing.rs` processes a new block, it relays the block's `Inv` to all peers (except the sender). This ensures blocks propagate across multi-hop network topologies, not just to the miner's direct neighbors.
- **Whitepaper mapping**
  - **§5 (Step 6)**: nodes express acceptance by working on the next block (i.e., building on the chosen tip).
  - **§11**: the whole explains why “more work” makes history harder to rewrite (fork-choice must be deterministic).

#### Step 3.1 — Cumulative work (`get_chain_work`) and deterministic tie-break

Fork choice needs a notion of “how strong is this branch?” In Bitcoin this is “cumulative proof-of-work”. In this codebase, that is computed by walking parent links back to genesis and summing each block’s `get_work()` value.

### Listing 9-8.7: Cumulative chain work (`BlockchainFileSystem::get_chain_work`)

> **Source:** `file_system_db_chain.rs` — Source

```rust
pub async fn get_chain_work(&self, block_hash: &str) -> Result<u64> {
    let mut work = 0u64;
    let mut current_hash = block_hash.to_string();
    // Walk backwards through parent links, summing each block's work value
    while let Some(block) = self.get_block(current_hash.as_bytes()).await? {
        work += block.get_work();
        current_hash = block.get_pre_block_hash();
        // ... (break at genesis)
    }
    Ok(work)
}
```

When two blocks have the same height but are
*not* direct parent/child, the code treats them as
a fork and compares chains.

### Listing 9-8.4: Equal-height
competitors (compute work, then reorganize or
reject)
> **Source:** `file_system_db_chain.rs` — Source

```rust
Ordering::Equal => {
    if new_block.get_pre_block_hash()
        == current_tip
    {
        return Ok(());
    }
    let current_work = self.get_chain_work(
        &current_tip
    ).await?;
    // ... (ensure block in DB for traversal)
    let new_work = self.get_chain_work(
        new_block.get_hash()
    ).await?;
    match new_work.cmp(&current_work) {
        Ordering::Greater => {
            self.reorganize_chain(
                new_block.get_hash()
            ).await?;
        }
        Ordering::Equal if self
            .accept_new_block_tie_break(
                new_block,
                &current_tip
            ).await? => {
            self.reorganize_chain(
                new_block.get_hash()
            ).await?;
        }
        _ => { /* reject or clean up */ }
    }
}
```

- **What to notice**
  - Height alone is not enough when forks happen at the same depth; the code explicitly computes “cumulative work”.
  - The project temporarily inserts blocks so
    `get_chain_work(...)` can traverse a chain
    that exists in the DB (this is a pragmatic
    learning-implementation trick, not a
    production approach).

#### Step 3.2 — Deterministic tie-break is lexicographic hash ordering

When work is equal, the project resolves the fork deterministically by comparing hashes:

### Listing 9-8.8: Deterministic tie-break (lexicographic hash ordering)
> **Source:** `file_system_db_chain.rs` — Source

```rust
async fn accept_new_block_tie_break(
    &self,
    new_block: &Block,
    current_tip: &str,
) -> Result<bool> {
    let current_block = self.get_block(current_tip.as_bytes()).await??;
    // Deterministic tie-break: lexicographic hash ordering
    Ok(new_block.get_hash_string() > current_block.get_hash_string())
}
```

- **What to notice**
  - This is not “Bitcoin’s exact rule”; it is a deterministic rule chosen so all nodes converge the same way in this teaching implementation.

#### Step 3.3 — Reorg is “rollback to common ancestor, then apply new branch”

If a stronger branch wins (higher work or tie-break), the project reorganizes:

### Listing 9-8.9: Reorganization contract (rollback to common ancestor, then apply new branch)
> **Source:** `file_system_db_chain.rs` — Source

```rust
pub async fn reorganize_chain(&mut self, new_tip_hash: &str) -> Result<()> {
    let current_tip = self.get_tip_hash().await?;
    let ancestor = self
        .find_common_ancestor(&current_tip, new_tip_hash)
        .await??;
    // Rollback UTXO effects from old tip back to ancestor
    self.rollback_to_block(&ancestor).await?;
    self.apply_chain_from_ancestor(&ancestor, new_tip_hash)
        .await?;
    Ok(())
}
```

Because this chapter is intended to be repo-independent, we also include the two stateful helpers that make reorgs correct:

- `rollback_to_block(...)` (remove blocks from the old tip back to the ancestor and rollback UTXO effects)
- `update_utxo_set(...)` (incrementally apply UTXO changes for blocks on the winning branch)

In production systems these functions must be carefully audited: any mismatch between “block history” and “UTXO state” is a consensus failure.

- **What to notice**
  - “Fork-choice” is not just “pick a tip hash”; it requires derived state (UTXO set) to be rolled back and re-applied to remain consistent.
  - This is the concrete embodiment of the whitepaper’s claim that nodes follow the strongest chain (and that later blocks can cause reorganization).

### Step 4 — Step‑5 “valid and not already spent” is the missing hard gate

**Whitepaper Step‑5 acceptance boundary (FIXME) code**: `bitcoin/src/store/file_system_db_chain.rs`

The current `add_block(...)` implementation includes an explicit FIXME reminder that it must implement the whitepaper gate. The dedicated deep dive is:

- Section 10 (Block Acceptance, Whitepaper §5 Step 5)

- **What to notice**
  - The current implementation explicitly acknowledges the Step‑5 requirement (FIXME) but does not fully enforce it before state mutation.
- **Whitepaper mapping**
  - **§5 (Step 5)**: “accept only if all transactions are valid and not already spent” must be a hard validation gate before connecting state.

## Navigation

- **Previous**: Chapter 15 (Block Lifecycle and Mining)
- **Next**: Chapter 17 (Node Orchestration and Runtime Wiring)

---

<div align="center">

**[← Previous: Block Lifecycle and Mining](06-Block-Lifecycle-and-Mining.md)** | **Consensus and Validation** | **[Next: Node Orchestration →](08-Node-Orchestration.md)**

</div>

