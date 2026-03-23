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
16. <a href="07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
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

# Blockchain: Technical Foundations (Chapters 10–18)

**Part I: Foundations & Core Implementation** | **Chapters 10--18: Blockchain (Technical Foundations)**

<div align="center">

**[← Cryptography](../crypto/README.md)** | **Chapter 9: Blockchain (Technical Foundations)** | **[Block Acceptance →](10-Whitepaper-Step-5-Block-Acceptance.md)**
</div>

---

In this section, we provide an **implementation guide** for how the project turns transactions into blocks, blocks into chain state, and chain state into spendability (the UTXO set). We proceed linearly through the `chain/` implementation: we introduce an idea, then we show the corresponding Rust code as a short code listing.

> **What you will learn in this chapter:**
> - Trace the end-to-end chain pipeline from domain model through state management, storage, and UTXO tracking
> - Understand how transactions move from creation through the mempool to block inclusion
> - Explain how mining, consensus rules, and node orchestration work together
> - Follow the complete transaction-to-block flow from start to finish

> **Scope:** This chapter covers the chain pipeline for a single-node blockchain with simplified consensus. We do not cover chain reorganization (reorgs), orphan block handling, or the full Bitcoin difficulty adjustment algorithm.

In this section, we walk through the implementation across these primary modules:
- `bitcoin/src/primitives/` (Block, Transaction structs)
- `bitcoin/src/chain/` (chain facade + UTXO set)
- `bitcoin/src/store/file_system_db_chain.rs` (sled-backed persistence)
- `bitcoin/src/pow.rs` (PoW loop + target)
- `bitcoin/src/node/` and `bitcoin/src/net/net_processing.rs` (wiring + propagation)

---

## 9.0 Reading guide and vocabulary

### A short vocabulary primer (UTXO, transaction, mempool)

- **UTXO (Unspent Transaction Output)**: a spendable “coin” in Bitcoin. Conceptually, it is a specific output in a previous transaction, identified by an **outpoint** $(txid, vout)$. The UTXO set is the node’s current database of “what is unspent right now?”
- **Transaction**: a signed data structure that consumes one or more UTXOs as inputs and creates new outputs (new potential UTXOs). In Bitcoin’s model, spending is not “update an account balance”; spending is “prove you can unlock a previous output, then create new outputs.”
- **Trimmed copy**: a temporary copy of a transaction constructed for signing and verification where signature fields are cleared (and, per-input, the referenced output’s locking data is injected) so we can compute a deterministic message digest to sign. This prevents circularity (“a signature cannot sign itself”) and ensures verifiers rebuild the same bytes before calling signature verification.
- **Mempool**: the node’s in-memory (or local) holding area for valid, unconfirmed transactions that have been received but are not yet in a block. Miners select transactions from the mempool when assembling candidate blocks.

In this section, we follow a single execution trace from “a wallet constructs a spend” to “a node accepts a block”:

1. We build a transaction from UTXOs (selection + change).
2. We sign it using the project’s trimmed-copy approach.
3. We admit it to mempool and broadcast it.
4. We assemble a candidate block, run proof-of-work, and persist the result.
5. We update the UTXO set so “what is spendable?” changes deterministically.
6. We accept blocks from peers by enforcing the whitepaper’s safety gate: **valid and not already spent**.

When we quote code, we label it as **Listing 9-x.y** so it can be referenced consistently throughout the section (and later in print).

**Figure 9-1: Transaction Lifecycle**

```text
 ┌──────────┐     ┌──────────┐     ┌──────────┐
 │  Wallet  │     │  Crypto  │     │   Node   │
 │ (Create) │────▶│  (Sign)  │────▶│ (Submit) │
 └──────────┘     └──────────┘     └────┬─────┘
                                        │
                                        ▼
                                  ┌──────────┐
                                  │  Mempool  │
                                  │ (Pending) │
                                  └────┬─────┘
                                       │
                          ┌────────────┘
                          ▼
                    ┌──────────┐     ┌──────────┐
                    │  Mining  │────▶│ Validate │
                    │ (PoW)    │     │ (Rules)  │
                    └──────────┘     └────┬─────┘
                                          │
                                          ▼
                    ┌──────────┐     ┌──────────┐
                    │  Chain   │◀────│  Block   │
                    │ (Append) │     │ (Accept) │
                    └────┬─────┘     └──────────┘
                         │
                         ▼
                    ┌──────────┐
                    │   UTXO   │
                    │ (Update) │
                    └──────────┘
```

## 9.1 End-to-end execution trace: transaction → mempool → block → acceptance

In the sections that follow, we will follow the code as it runs. Each step names the primary entry points and points to the subsection where we read the full code listing in context.

### Worked example: from “create transaction” to “persisted in a block”

This section is organized around a single running path and three boundaries that matter most in the code:

- **Transaction creation boundary**: `Transaction::new_utxo_transaction(...)`
- **Mempool admission boundary**: `NodeContext::process_transaction(...)`
- **Block persistence boundary**: `BlockchainService::mine_block(...)` (local mining path) and `BlockchainFileSystem::add_block(...)` (peer acceptance path)

The organizing idea is to recognize where the system transitions from:

- “build data structures” → “validate” → “persist” → “update derived state”.

### Listing 9-0.1: Create a spend + submit it to the node (exact project signatures)

```rust
// 1) Wallet/client constructs a spend
// (UTXO selection → inputs/outputs → txid → sign).
// Source: bitcoin/src/primitives/transaction.rs
let tx = Transaction::new_utxo_transaction(
    &from_wallet_addr,
    &to_wallet_addr,
    amount,
    &utxo_set
)
.await?;

// 2) Node admits tx to mempool and
// triggers propagation/mining.
// Source: bitcoin/src/node/context.rs
let txid_hex = node
    .process_transaction(&remote_addr, tx)
    .await?;
```

### Listing 9-0.2: Persistence happens on two paths (local mining vs peer acceptance)

```rust
// 3a) Local mining path: validate txs,
// build block, persist, update UTXO set.
// Source: bitcoin/src/chain/chainstate.rs
let block = blockchain_service
    .mine_block(&mempool_txs)
    .await?;

// 3b) Peer acceptance path: persist
// candidate + fork-choice + connect state.
// Source: bitcoin/src/chain/chainstate.rs →
// bitcoin/src/store/file_system_db_chain.rs
blockchain_service.add_block(&peer_block).await?;
```

### Listing 9-0.3: Mempool admission boundary (what `process_transaction` actually does)

```rust
// Source: bitcoin/src/node/context.rs
pub async fn process_transaction(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<String> {
    if transaction_exists_in_pool(&utxo) {
        return Err(
            BtcError::TransactionAlreadyExistsInMemoryPool(
                utxo.get_tx_id_hex()
            )
        );
    }
    add_to_memory_pool(utxo.clone(), &self.blockchain)
        .await?;

    let context = self.clone();
    let addr_copy = *addr_from;
    let tx = utxo.clone();
    tokio::spawn(async move {
        let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
    });
    Ok(utxo.get_tx_id_hex())
}
```

### Listing 9-0.4: Mining boundary (verify first, then persist)

```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(
    &self,
    transactions: &[Transaction],
) -> Result<Block> {
    for transaction in transactions {
        let is_valid =
            transaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions)
        .await
}
```

### Listing 9-0.5: Persistence + UTXO update on the mining path

```rust
// Source: bitcoin/src/store/file_system_db_chain.rs
pub async fn mine_block(
    &self,
    transactions: &[Transaction],
) -> Result<Block> {
    let best_height =
        self.get_best_height().await?;
    let block = Block::new_block(
        self.get_tip_hash().await?,
        transactions,
        best_height + 1
    );
    let block_hash = block.get_hash();

    let blocks_tree = self.blockchain.db
        .open_tree(self.get_blocks_tree_path())
        .map_err(|e| {
            BtcError::BlockchainDBconnection(
                e.to_string()
            )
        })?;
    Self::update_blocks_tree(&blocks_tree, &block)
        .await?;
    self.set_tip_hash(block_hash).await?;

    // Derived state update after persistence.
    self.update_utxo_set(&block).await?;
    Ok(block)
}
```

### Step 0: Establish the prerequisites (data model + crypto meaning)

- A transaction input references an **outpoint** $(txid, vout)$ and proves authorization with a **signature**.
- The UTXO set answers the state question: “is this outpoint spendable right now?”
- For the supporting definitions and data layout, we rely on:
  - Section 6 (Primitives): `Transaction`, `TXInput`, `TXOutput`, `Block`
  - Section 8 (Cryptography): signing and verification primitives

### Step 1: Create a spending transaction (UTXO selection + inputs/outputs)

See Chapter 14 (Transaction Lifecycle). The primary entry point is `Transaction::new_utxo_transaction(...)`, which calls `UTXOSet::find_spendable_outputs(...)` to select spendable outpoints and then constructs inputs/outputs (including change).

### Step 2: Sign the transaction (trimmed copy → hash → signature)

See Chapter 14 (Transaction Lifecycle). The primary entry point is `Transaction::sign(...)`, which builds a trimmed copy per input, hashes it, and produces a signature via `schnorr_sign_digest(...)` (or `ecdsa_p256_sha256_sign_digest(...)` in the ECDSA path used elsewhere in the project). This is the whitepaper’s “ownership transfer” mechanism expressed as code.

### Step 3: Verify the transaction (what a node checks before accepting it)

The primary entry point is `Transaction::verify(...)`. We study the transaction-level logic in Chapter 14, then we study where that check is enforced (mining vs acceptance paths) in Chapter 16.

### Step 4: Admit the transaction to the mempool (and propagate it)

This boundary is implemented by `NodeContext::process_transaction(...)` and `txmempool::add_to_memory_pool(...)` (Chapter 14). The propagation and routing paths live in Chapter 17 (`net/net_processing.rs`, INV/GETDATA flow).

### Step 5: Assemble a candidate block from mempool transactions (+ coinbase)

See Chapter 15 (Block Lifecycle and Mining). The miner collects mempool transactions, adds a coinbase via `Transaction::new_coinbase_tx(...)`, and invokes `BlockchainService::mine_block(...)`.

### Step 6: Mine the block (PoW loop) and persist it as chain state

Mining is implemented as `Block::new_block(...)` → `ProofOfWork::run(...)` (Chapter 15), followed by persistence and tip updates in `bitcoin/src/store/file_system_db_chain.rs` (Chapter 12).

### Step 7: Update “what is spendable” (UTXO set mutation)

See Chapter 13 (UTXO Set). The primary entry point is `UTXOSet::update(...)`. This is the point where history (blocks) becomes state (spendability), which is what prevents double spends.

> **Warning:** The UTXO set must be updated atomically with block acceptance. If a block is accepted but the UTXO update fails partway through, the system could enter an inconsistent state where spent outputs appear unspent or vice versa.

### Step 8: Accept the block from the network (validate → connect)

The acceptance boundary is `BlockchainFileSystem::add_block(...)`. The capstone in Section 10 (Block Acceptance, Whitepaper §5 Step 5) maps the whitepaper’s Step 5 gate to concrete code boundaries: accept only if all transactions are valid **and not already spent**.

### Figure: End-to-end control flow (Steps 0–23)

```text
Wallet/API creates tx
  └─> Transaction::new_utxo_transaction
       ├─> UTXOSet::find_spendable_outputs
       └─> Transaction::sign
            └─> schnorr_sign_digest / ecdsa_*_sign_digest

Node receives tx
  └─> NodeContext::process_transaction
       └─> txmempool::add_to_memory_pool

Miner builds candidate
  └─> miner::prepare_mining_utxo (+ coinbase)
       └─> BlockchainService::mine_block
            ├─> Block::new_block
            │    └─> ProofOfWork::run
            ├─> persist block (file_system_db_chain.rs)
            └─> UTXOSet::update

Peer receives block
  └─> BlockchainFileSystem::add_block
       ├─> validate txs (Transaction::verify)
       └─> connect (update tip + UTXO set)
```

---

## Summary

- We traced the complete chain pipeline from domain model through mempool, mining, and consensus to persistent chain state.
- We explained how transactions are created, signed, verified, and admitted to the mempool before block inclusion.
- We examined the UTXO set as the definitive record of spendability and how it updates on each block.
- We walked through the mining process, block assembly, and the two acceptance paths (local mining vs network propagation).

In the next chapter, we focus on the single most important safety rule: the whitepaper's Step 5 gate that accepts blocks only if all transactions are valid and not already spent.

---

## Exercises

1. **Trace a Transaction End-to-End** — Starting with a wallet that holds 50 coins, create a transaction that sends 30 coins to another address. Trace the transaction through each stage: creation, signing, mempool insertion, block mining, and UTXO set update. Draw the UTXO set state before and after the transaction is confirmed.

2. **State Diagram Exercise** — Draw a state machine diagram showing the lifecycle of a transaction from creation to confirmation. Label each transition with the function or module responsible. Include the error states (invalid signature, insufficient funds, double spend attempt).

---

## Recommended reading order (within module `chain/`)

We read this section in the following order (each item is a deep-dive into a segment of the execution trace above):

1. **Chapter 9 (overview)** — you are here
2. **Chapter 10: Domain Model — Blocks, Transactions, and the UTXO Worldview** — the meaning of blocks, transactions, and outpoints in this codebase
3. **Chapter 11: Blockchain State Management** — the chain façade (`BlockchainService`), lock boundaries, and derived-state delegation
4. **Chapter 12: Chain State and Storage — How Blocks Become Persistent State** — persistence, tip updates, and the mining vs network insertion write paths
5. **Chapter 13: UTXO Set — The Spendability Database** — spendability, update rules, and the state transition that prevents double spends
6. **Chapter 14: Transaction Lifecycle — Create → Sign → Verify → Mempool** — build → sign (trimmed copy) → verify → mempool admission
7. **Chapter 15: Block Lifecycle and Mining** — candidate assembly, proof-of-work, persistence, and propagation
8. **Chapter 16: Consensus and Validation** — chain selection and where validity is enforced
9. **Chapter 17: Node Orchestration and Runtime Wiring** — runtime routing of network messages into chainstate/mempool/mining
10. **Chapter 18: Transaction to Block (End-to-End Runtime Walkthrough)** — follow `main` to block creation across the runtime

After you complete Chapter 9, continue in book order with:

- **Chapter 19: Block Acceptance (Whitepaper §5, Step 5)** — the capstone “validate → connect” contract
- **Chapter 20: Storage Layer** — persistence and storage primitives

---

## References

- Nakamoto, S. (2008). **[Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf)**.

---

<div align="center">

**[← Cryptography](../crypto/README.md)** | **Blockchain (Technical Foundations)** | **[Next: Domain Model →](01-Domain-Model.md)**
</div>
