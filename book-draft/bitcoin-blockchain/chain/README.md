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
16. <a href="../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md">Section 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui/04-Wallet-UI.md">Section 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui/05-Embedded-Database.md">Section 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Section 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Section 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Section 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Section 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

# Section 2.4: Blockchain — From Transaction to Block Acceptance

**Part I: Core Blockchain Implementation** | **Section 2.4: Blockchain (Technical Foundations)**

<div align="center">

**📚 [← Cryptography](../crypto/README.md)** | **Section 2.4 Blockchain (Technical Foundations)** | **[Storage Layer →](../store/README.md)** 📚

</div>

---

In this section, we provide an **implementation guide** for how the project turns transactions into blocks, blocks into chain state, and chain state into spendability (the UTXO set). We proceed linearly through the `chain/` implementation: we introduce an idea, then we show the corresponding Rust code as a short code listing.

In this section, we walk through the implementation across these primary modules:
- `bitcoin/src/primitives/` (Block, Transaction structs)
- `bitcoin/src/chain/` (chain facade + UTXO set)
- `bitcoin/src/store/file_system_db_chain.rs` (sled-backed persistence)
- `bitcoin/src/pow.rs` (PoW loop + target)
- `bitcoin/src/node/` and `bitcoin/src/net/net_processing.rs` (wiring + propagation)

---

## 2.4.0 Reading guide and vocabulary

### A short vocabulary primer (UTXO, transaction, mempool)

- **UTXO (Unspent Transaction Output)**: a spendable “coin” in Bitcoin. Conceptually, it is a specific output in a previous transaction, identified by an **outpoint** \((txid, vout)\). The UTXO set is the node’s current database of “what is unspent right now?”
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

When we quote code, we label it as **Code Listing 2.4-x.y** so it can be referenced consistently throughout the section (and later in print).

## 2.4.1 End-to-end execution trace: transaction → mempool → block → acceptance

In the sections that follow, we will follow the code as it runs. Each step names the primary entry points and points to the subsection where we read the full code listing in context.

### Worked example: from “create transaction” to “persisted in a block”

This section is organized around a single running path and three boundaries that matter most in the code:

- **Transaction creation boundary**: `Transaction::new_utxo_transaction(...)`
- **Mempool admission boundary**: `NodeContext::process_transaction(...)`
- **Block persistence boundary**: `BlockchainService::mine_block(...)` (local mining path) and `BlockchainFileSystem::add_block(...)` (peer acceptance path)

The organizing idea is to recognize where the system transitions from:

- “build data structures” → “validate” → “persist” → “update derived state”.

#### Code Listing 2.4-0.1: Create a spend + submit it to the node (exact project signatures)

```rust
// 1) Wallet/client constructs a spend (UTXO selection → inputs/outputs → txid → sign).
// Source: bitcoin/src/primitives/transaction.rs
let tx = Transaction::new_utxo_transaction(&from_wallet_addr, &to_wallet_addr, amount, &utxo_set)
    .await?;

// 2) Node admits the transaction to the mempool and triggers propagation/mining.
// Source: bitcoin/src/node/context.rs
let txid_hex = node.process_transaction(&remote_addr, tx).await?;
```

#### Code Listing 2.4-0.2: Persistence happens on two paths (local mining vs peer acceptance)

```rust
// 3a) Local mining path: validate txs, build block, persist, and update the UTXO set.
// Source: bitcoin/src/chain/chainstate.rs
let block = blockchain_service.mine_block(&mempool_txs).await?;

// 3b) Peer acceptance path: persist candidate + fork-choice + (eventually) connect state.
// Source: bitcoin/src/chain/chainstate.rs → bitcoin/src/store/file_system_db_chain.rs
blockchain_service.add_block(&peer_block).await?;
```

#### Code Listing 2.4-0.3: Mempool admission boundary (what `process_transaction` actually does)

```rust
// Source: bitcoin/src/node/context.rs
pub async fn process_transaction(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<String> {
    if transaction_exists_in_pool(&utxo) {
        return Err(BtcError::TransactionAlreadyExistsInMemoryPool(utxo.get_tx_id_hex()));
    }
    add_to_memory_pool(utxo.clone(), &self.blockchain).await?;

    let context = self.clone();
    let addr_copy = *addr_from;
    let tx = utxo.clone();
    tokio::spawn(async move {
        let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
    });
    Ok(utxo.get_tx_id_hex())
}
```

#### Code Listing 2.4-0.4: Mining boundary (verify first, then persist)

```rust
// Source: bitcoin/src/chain/chainstate.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    for trasaction in transactions {
        let is_valid = trasaction.verify(self).await?;
        if !is_valid {
            return Err(BtcError::InvalidTransaction);
        }
    }
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions).await
}
```

#### Code Listing 2.4-0.5: Persistence + UTXO update on the mining path

```rust
// Source: bitcoin/src/store/file_system_db_chain.rs
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    let best_height = self.get_best_height().await?;
    let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
    let block_hash = block.get_hash();

    let blocks_tree = self.blockchain.db.open_tree(self.get_blocks_tree_path())
        .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
    Self::update_blocks_tree(&blocks_tree, &block).await?;
    self.set_tip_hash(block_hash).await?;

    // Derived state update happens after persistence.
    self.update_utxo_set(&block).await?;
    Ok(block)
}
```

### Step 0: Establish the prerequisites (data model + crypto meaning)

- A transaction input references an **outpoint** \((txid, vout)\) and proves authorization with a **signature**.
- The UTXO set answers the state question: “is this outpoint spendable right now?”
- For the supporting definitions and data layout, we rely on:
  - Section 2.1 (Primitives): `Transaction`, `TXInput`, `TXOutput`, `Block`
  - Section 2.3 (Cryptography): signing and verification primitives

### Step 1: Create a spending transaction (UTXO selection + inputs/outputs)

See Section 2.4.5 (Transaction Lifecycle). The primary entry point is `Transaction::new_utxo_transaction(...)`, which calls `UTXOSet::find_spendable_outputs(...)` to select spendable outpoints and then constructs inputs/outputs (including change).

### Step 2: Sign the transaction (trimmed copy → hash → signature)

See Section 2.4.5 (Transaction Lifecycle). The primary entry point is `Transaction::sign(...)`, which builds a trimmed copy per input, hashes it, and produces a signature via `schnorr_sign_digest(...)` (or `ecdsa_p256_sha256_sign_digest(...)` in the ECDSA path used elsewhere in the project). This is the whitepaper’s “ownership transfer” mechanism expressed as code.

### Step 3: Verify the transaction (what a node checks before accepting it)

The primary entry point is `Transaction::verify(...)`. We study the transaction-level logic in Section 2.4.5, then we study where that check is enforced (mining vs acceptance paths) in Section 2.4.7.

### Step 4: Admit the transaction to the mempool (and propagate it)

This boundary is implemented by `NodeContext::process_transaction(...)` and `txmempool::add_to_memory_pool(...)` (Section 2.4.5). The propagation and routing paths live in Section 2.4.8 (`net/net_processing.rs`, INV/GETDATA flow).

### Step 5: Assemble a candidate block from mempool transactions (+ coinbase)

See Section 2.4.6 (Block Lifecycle and Mining). The miner collects mempool transactions, adds a coinbase via `Transaction::new_coinbase_tx(...)`, and invokes `BlockchainService::mine_block(...)`.

### Step 6: Mine the block (PoW loop) and persist it as chain state

Mining is implemented as `Block::new_block(...)` → `ProofOfWork::run(...)` (Section 2.4.6), followed by persistence and tip updates in `bitcoin/src/store/file_system_db_chain.rs` (Section 2.4.3).

### Step 7: Update “what is spendable” (UTXO set mutation)

See Section 2.4.4 (UTXO Set). The primary entry point is `UTXOSet::update(...)`. This is the point where history (blocks) becomes state (spendability), which is what prevents double spends.

### Step 8: Accept the block from the network (validate → connect)

The acceptance boundary is `BlockchainFileSystem::add_block(...)`. The capstone in Section 2.6 (Block Acceptance, Whitepaper §5 Step 5) maps the whitepaper’s Step 5 gate to concrete code boundaries: accept only if all transactions are valid **and not already spent**.

### Figure: End-to-end control flow (Steps 0–9)
```
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

## Recommended reading order (within module `chain/`)

We read this section in the following order (each item is a deep-dive into a segment of the execution trace above):

1. **Section 2.4 (overview)** — you are here
2. **Section 2.4.1: Domain Model — Blocks, Transactions, and the UTXO Worldview** — the meaning of blocks, transactions, and outpoints in this codebase
3. **Section 2.4.2: Blockchain State Management** — the chain façade (`BlockchainService`), lock boundaries, and derived-state delegation
4. **Section 2.4.3: Chain State and Storage — How Blocks Become Persistent State** — persistence, tip updates, and the mining vs network insertion write paths
5. **Section 2.4.4: UTXO Set — The Spendability Database** — spendability, update rules, and the state transition that prevents double spends
6. **Section 2.4.5: Transaction Lifecycle — Create → Sign → Verify → Mempool** — build → sign (trimmed copy) → verify → mempool admission
7. **Section 2.4.6: Block Lifecycle and Mining** — candidate assembly, proof-of-work, persistence, and propagation
8. **Section 2.4.7: Consensus and Validation** — chain selection and where validity is enforced
9. **Section 2.4.8: Node Orchestration and Runtime Wiring** — runtime routing of network messages into chainstate/mempool/mining
10. **Section 2.4.9: Transaction to Block (End-to-End Runtime Walkthrough)** — follow `main` to block creation across the runtime

After you complete Chapter 2.4, continue in book order with:

- **Chapter 2.5: Storage Layer** — persistence and storage primitives
- **Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)** — the capstone “validate → connect” contract

---

---

## References

- Nakamoto, S. (2008). **[Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf)**.

---

<div align="center">

**📚 [← Cryptography](../crypto/README.md)** | **Blockchain (Technical Foundations)** | **[Next: Domain Model →](01-Domain-Model.md)** 📚

</div>
