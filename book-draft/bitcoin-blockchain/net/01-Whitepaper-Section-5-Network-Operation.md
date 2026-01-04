<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. **Chapter 2.7: Network Layer** ← *You are here*
13. [Chapter 2.8: Node Orchestration](../node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../../ci/kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
# Network Operation (Whitepaper §5) — Six-Step Protocol in Rust

**Chapter 8.A: Whitepaper §5 → Implementation Map (Network Operation)**

This chapter explains *how nodes coordinate* using the whitepaper’s six sequential steps, and maps each step to the concrete code paths in this repository.

> Source: [Bitcoin Whitepaper, Section 5](https://bitcoin.org/bitcoin.pdf)

---

## The Six Steps (What the Whitepaper Says)

Section 5 describes the protocol as six sequential steps:

1. **Transaction Propagation**: New transactions are broadcast to all nodes
2. **Block Assembly**: Each node collects new transactions into a block
3. **Proof-of-Work Computation**: Each node works on finding a difficult proof-of-work for its block
4. **Block Propagation**: When a node finds a proof-of-work, it broadcasts the block to all nodes
5. **Block Validation**: Nodes accept the block only if all transactions in it are valid and not already spent
6. **Chain Extension**: Nodes express their acceptance by mining the next block using the accepted block as the previous hash

In a real node, steps 1–6 are not “one function call”; they are an end-to-end pipeline spanning mempool, mining, networking, and chainstate.

---

## Step-by-Step: How This Repo Implements Each Step

### Step 1 — Transaction Propagation

- **Where**: `bitcoin/src/net/net_processing.rs`
- **What happens**:
  - A node receives `Package::Tx`.
  - The transaction is deserialized and passed into the node’s transaction processing, which typically ends by putting it into the mempool.

Conceptually, Step 1 is “mempool ingress”.

### Step 2 — Block Assembly

- **Where**: `bitcoin/src/node/miner.rs`
- **What happens**:
  - The miner pulls transactions from `GLOBAL_MEMORY_POOL`.
  - The miner appends a coinbase transaction (block subsidy) and builds the candidate block template.

Conceptually, Step 2 is “BlockAssembler”.

### Step 3 — Proof-of-Work Computation

- **Where**:
  - `bitcoin/src/primitives/block.rs` (block construction)
  - `bitcoin/src/pow.rs` (mining loop)
- **What happens**:
  - `Block::new_block(...)` runs PoW via `ProofOfWork::run()` and stores `(nonce, hash)` into the block header.

Conceptually, Step 3 is “find a nonce such that `hash < target`”.

### Step 4 — Block Propagation

- **Where**:
  - `bitcoin/src/node/miner.rs` (`broadcast_new_block`)
  - `bitcoin/src/net/net_processing.rs` (`send_inv`, `send_get_data`, `Package::Block`)
- **What happens**:
  - Miner broadcasts an inventory (`inv`) for the new block hash.
  - Peers request the block with `get_data`.
  - The full block is sent in `Package::Block`.

Conceptually, Step 4 is “announce then fetch”.

### Step 5 — Block Validation (the critical step)

This is the rule you referenced:

> “Nodes accept the block only if all transactions in it are valid and not already spent.”

**Two different things are being checked here:**

- **Transaction validity** (cryptography + structure)
  - signatures verify
  - coinbase rules (exactly one coinbase)
- **Not already spent** (UTXO / double-spend prevention)
  - every input refers to an *unspent* output in the current UTXO set
  - no two transactions in the same block spend the same `(txid, vout)`

**Where it is enforced in this repo (after this change):**

- `bitcoin/src/store/file_system_db_chain.rs`
  - `BlockchainFileSystem::validate_block_for_connect(...)`
  - called from:
    - `add_block()` when a block becomes eligible to be connected
    - `apply_chain_from_ancestor()` during reorg, per-block, in order

**Important implementation note (UTXO indices):**

Bitcoin inputs reference outputs by `vout` index. That index must remain stable.
This repo now keeps UTXO output vectors **index-stable** by marking spent outputs in-place instead of removing elements.

### Step 6 — Chain Extension (“express acceptance”)

In code, “express acceptance” means:

- we update the canonical tip
- we update the canonical UTXO set
- miners then build their next candidate block using the new tip as the `pre_block_hash`

**Where**:

- `bitcoin/src/store/file_system_db_chain.rs`
  - `add_block()` sets the tip when the block wins consensus
  - `apply_chain_from_ancestor()` advances the tip during reorg application
- `bitcoin/src/store/file_system_db_chain.rs`
  - `mine_block()` uses `self.get_tip_hash()` and builds `best_height + 1`

---

## A Detailed Rust Example: “Connect Block” (Step 5 + Step 6)

This is a simplified “connect block” pattern that matches the whitepaper intent:

```rust
/// Pseudocode-ish example that matches how the repo now behaves.
async fn accept_and_connect_block(
    chain: &mut BlockchainFileSystem,
    block: &Block,
) -> Result<()> {
    // 1) Header validity: PoW + linkage
    let parent = chain.get_tip_hash().await?;
    chain.validate_block_for_connect(block, &parent).await?;

    // 2) State transition: connect block
    chain.update_utxo_set(block).await?;
    chain.set_tip_hash(block.get_hash()).await?;

    Ok(())
}
```

**Key idea:** a block is “accepted” in the Step‑5 sense only once it is *valid* and can be *connected* without violating UTXO rules.

---

## What This Chapter Does *Not* Claim

The whitepaper describes a high-level protocol. Bitcoin Core adds many additional consensus rules (script evaluation details, coinbase maturity, locktime/sequence rules, fees, etc.). This project is intentionally simpler, but the *core Step‑5 invariant* still matters:

- **Never accept a block that double-spends**
- **Never accept a block whose transactions fail verification**

---

## Related Files (quick map)

- **Mempool**: `bitcoin/src/txmempool.rs`
- **Mining**: `bitcoin/src/node/miner.rs`
- **Networking**: `bitcoin/src/net/net_processing.rs`
- **Chain + storage**: `bitcoin/src/store/file_system_db_chain.rs`
- **UTXO view**: `bitcoin/src/chain/utxo_set.rs`
- **PoW**: `bitcoin/src/pow.rs`



