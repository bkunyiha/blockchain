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
14. **Chapter 14: Transaction Lifecycle** ← *You are here*
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

# Chapter 14: Transaction Lifecycle — Create → Sign → Verify → Mempool

When you want to send coins to someone, what actually happens behind the scenes? This section traces a transaction through its complete lifecycle—from the moment you decide to send coins, through construction and signing, verification and mempool admission, to network propagation and eventual inclusion in a block.

We'll follow a transaction step-by-step through the exact Rust code that implements each stage, showing you how the system transforms a simple payment intent into a cryptographically secured, network-propagated transaction ready for mining.

**What you'll learn**: How transactions are built, signed, verified, stored in the mempool, and prepared for inclusion in blocks. You'll understand the complete flow from wallet to blockchain.

**Primary code files** (we'll walk through these):
- `bitcoin/src/primitives/transaction.rs` — transaction construction and signing
- `bitcoin/src/chain/utxo_set.rs` — finding spendable outputs
- `bitcoin/src/node/txmempool.rs` — mempool storage and management
- `bitcoin/src/node/context.rs` — node-level transaction processing

## Scope (section flow)

This chapter covers **the blockchain pipeline (Blockchain — From Transaction to Block Acceptance) Steps 1–4**: transaction construction, signing, verification, mempool admission, and propagation triggers.


**Important boundaries to understand**:

- **Mempool boundary**: This section follows a transaction through mempool admission. Once in the mempool, transactions are pending and waiting to be included in a block.

- **Mining boundary**: We also show where the system makes its **final "no invalid transactions" decision**. That decision happens in `BlockchainService::mine_block` (Step 7), which calls `Transaction::verify` for each transaction and aborts mining if any verification fails. This is a defensive check that ensures only valid transactions enter blocks.

- **Mining triggers**: We show the **mining trigger entry points** (Step 8) that decide "should we start mining now?" and gather candidate transactions. The detailed block-building, Proof-of-Work, and persistence pipeline is explained in the next section (Chapter 15: Block Lifecycle and Mining).


### How to read this section

To get the most out of this section, follow this approach:

- **Start with the overview diagrams** to understand the big picture before diving into code details
- **Read each step in order** — each step builds on the previous one, creating a complete picture
- **Follow the flow**: diagram → method overview → code listing → explanation → checkpoint
- **Keep the mental model simple**: A UTXO transaction is just a structured statement: "I am consuming these specific outpoints $(txid, vout)$, and I am creating these new outputs."

**Code listings** are copied from the project and annotated with inline comments to explain *what* each line does and *why* it matters.

### Terminology quick-reference (used throughout)

- **txid**: transaction identifier (32-byte hash; often displayed as hex for humans)
- **vout**: output index (0-based) inside a transaction’s output vector
- **outpoint**: $(txid, vout)$ pointer to a specific previous output
- **mempool**: local “pending set” of unconfirmed transactions (`GLOBAL_MEMORY_POOL`)
- **UTXO set**: derived state that answers “what outputs are spendable right now?”

## Overview: the transaction lifecycle at a glance

Before diving into the code, we first lay out the big picture. A transaction goes through several distinct stages, each handled by different parts of the system. Understanding this flow will make the detailed code walkthrough much easier to follow.

### Diagram — high-level lifecycle (stages: wallet → node → mempool → mining boundary)

This diagram shows the main stages a transaction passes through from creation to confirmation:

```text
Wallet/client
  builds + signs a transaction:
    Transaction::new_utxo_transaction
          |
          v
Node (mempool acceptance boundary)
  NodeContext::process_transaction
    - store pending: GLOBAL_MEMORY_POOL.add
    - reserve inputs (local conflict guard):
      UTXOSet::set_global_mem_pool_flag(true)
    - spawn background work: submit_transaction_for_mining
          |
          +--> relay tx inventory to peers: send_inv(OpType::Tx, txid)
          |
          +--> if miner && mempool_size >= TRANSACTION_THRESHOLD:
                 prepare candidate tx list (+ coinbase): prepare_mining_utxo
                 mine block (mining boundary): BlockchainService::mine_block
                   - verify each tx (defensive): Transaction::verify
                 remove confirmed txs from mempool: remove_from_memory_pool
```

This diagram shows the **separation of responsibilities** in our implementation: the wallet constructs and signs; the node accepts and stores pending transactions; the node broadcasts inventory to peers; and the **mining boundary** performs defensive verification before producing a block.

One important orientation detail: the “wallet builds + signs tx” box corresponds to `Transaction::new_utxo_transaction`, but the *human-facing* entry point you will usually call in this project is `NodeContext::btc_transaction` (Step 1), which orchestrates construction and submission.

### Diagram — code-level call sequence (who calls whom: wallet → node → network → mining boundary)

### Text diagram — transaction journey (wallet → mempool → network → block)

Reading this top-to-bottom gives a short narrative of a transaction’s journey through the system. First, a wallet constructs and signs a transaction. Next, the node accepts it into the mempool and announces it to peers by `txid` (INV). Finally, at the mining boundary, the transaction is verified again before it can be included in a block.

```text
Wallet (construct & sign a transaction):
  Transaction::new_utxo_transaction(...)
    -> UTXOSet::find_spendable_outputs(...)     // coin selection
    -> Transaction::sign(...)                   // authorize spending

Node (add transaction into mempool):
  NodeContext::process_transaction(tx)
    -> txmempool::add_to_memory_pool(tx)        // store pending
         -> UTXOSet::set_global_mem_pool_flag(true) // reserve inputs locally
    // async propagation/mining trigger
    -> spawn submit_transaction_for_mining(...)

Network (broadcast transaction to other nodes by txid):
  // advertise inventory (txid only)
  send_inv(OpType::Tx, txid)

Mining boundary (verify transaction before adding to blockchain and UTXO Set):
  BlockchainService::mine_block(txs)
    -> Transaction::verify(...)                 // defensive signature check

After confirmation transaction in the blockchain and UTXO Set:
  txmempool::remove_from_memory_pool(txs)       // remove confirmed txs
  send_inv(OpType::Block, block_hash)           // announce new block
```

**The complete flow**: construct → sign → submit → mempool → (async) broadcast/mining → verify at the mining boundary → confirmation.

After reading this section, you'll understand exactly what happens at each stage and why each step is necessary for the transaction to successfully move from your wallet to the blockchain.

---

## Step-by-step code walkthrough

Now that you understand the high-level flow, we dive into the code. We'll trace a transaction end-to-end through each stage: **build → sign → submit → mempool → broadcast/mining trigger → verify at mining boundary**.

Each step builds on the previous one, so reading them in order will give you a complete picture of how transactions work in this implementation.

**Primary files**:
- `bitcoin/src/node/context.rs`
- `bitcoin/src/primitives/transaction.rs`
- `bitcoin/src/node/txmempool.rs`
- `bitcoin/src/node/miner.rs`
- (mining boundary) `bitcoin/src/chain/chainstate.rs`

**Whitepaper anchors**:
- **§2**: transactions and signatures (“chain of ownership”)
- **§5**: transaction relay and collection into blocks

### Roadmap (how the steps connect)

Here's the path we'll follow through the code. Each step leads naturally to the next:

- **Step 1** (`NodeContext::btc_transaction`) → Listing **9-18** — The entry point: user wants to send coins
- **Step 2** (`Transaction::new_utxo_transaction`) → Listing **9-19** — Build the transaction: select inputs, create outputs, compute ID
- **Step 3** (`trimmed_copy` / `sign` / `verify`) → Listing **9-5.3** — Authorize the transaction: sign inputs to prove ownership
- **Step 4** (`NodeContext::process_transaction`) → Listing **9-5.4** — Accept into mempool: store pending transaction
- **Step 5** (`txmempool.rs` add/remove/exists) → Listing **9-5.5** — Mempool internals: how transactions are stored and tracked
- **Step 6** (`submit_transaction_for_mining` + broadcast helpers) → Listing **9-5.6** — Propagate to network: announce transaction to peers
- **Step 7** (mining boundary verification in `BlockchainService::mine_block`) → Listing **9-5.7** — Final verification: check signatures before mining
- **Step 8** (mining trigger entry points in `miner.rs`) → Listing **9-5.8** — Mining trigger: when to start mining a block

---

### Step 1 — The top-level “send payment” entry point (`NodeContext::btc_transaction`)

This is the highest-level orchestration method: it creates a transaction (wallet-side work) and then hands it to the node for mempool/network processing.

**Methods involved**
- `NodeContext::btc_transaction` → `Transaction::new_utxo_transaction` → `NodeContext::process_transaction`

**Inputs / Outputs / Side-effects**

- **Inputs**: `(from_address, to_address, amount)`
- **Outputs**: returns `Ok(txid_hex)` if accepted into the node pipeline
- **Side-effects**:
  - reads wallet keys (during construction)
  - submits the transaction into the node’s mempool/network pipeline (via `process_transaction`)
- **Why this method exists**: it is the book’s “one call” entry point that turns a user intent (“send coins”) into a constructed UTXO transaction and then hands it to the node for distribution/confirmation.

### Listing 9-18 (annotated): `NodeContext::btc_transaction`

> **Source:** `context.rs` — Source

```rust
pub async fn btc_transaction(
    &self,
    wlt_frm_addr: &WalletAddress,
    wlt_to_addr: &WalletAddress,
    amount: i32,
) -> Result<String> {
    // Create a UTXOSet handle (derived-state accessor).
    // Why: transaction construction needs coin selection; signing needs
    // previous-tx lookups via chainstate.
    let utxo_set = UTXOSet::new(self.blockchain.clone());

    // Build + sign the transaction (wallet-side responsibility).
    // Why: the node should not "invent" spends; it only accepts/relays what the
    // wallet constructed.
    let utxo = Transaction::new_utxo_transaction(
        wlt_frm_addr, wlt_to_addr, amount, &utxo_set,
    ).await?;

    // Submit to the node’s mempool/network pipeline.
    // Why: once constructed, a tx must enter the node’s pending set before it
    // can be mined/relayed.
    let addr_from = crate::GLOBAL_CONFIG.get_node_addr();
    self.process_transaction(&addr_from, utxo).await
}
```
### Listing 9-18 explanation:

- It creates a `UTXOSet` handle so the constructor can perform coin selection and later resolve previous transactions during signing.
- It constructs and signs a new spending transaction by calling `Transaction::new_utxo_transaction(...)`.
- It hands that signed transaction to the node via `process_transaction(...)`, which places it on the mempool/network pipeline and returns a `txid` string to the caller.

**Checkpoint (you should be able to explain)**
- Where the “wallet-like” part ends (transaction construction) and where the “node-like” part begins (mempool/network processing).

---

### Step 2 — Construct a spending transaction (`Transaction::new_utxo_transaction`)

Now that we have the entry point, we see how the transaction is actually built. This step is where the magic happens: the system selects which coins to spend, creates the payment and change outputs, computes the transaction ID, and signs everything to prove ownership.

**What happens here**: This is the core UTXO spend constructor that transforms a payment intent into a fully-formed transaction ready for the network.

**Methods involved**
- `Transaction::new_utxo_transaction`
- Calls into: `UTXOSet::find_spendable_outputs`, `Transaction::hash`, `Transaction::sign`

**Inputs / Outputs / Side-effects**

- **Inputs**: `(from_address, to_address, amount, utxo_set)`
- **Outputs**: returns a fully constructed `Transaction` (with `id` and per-input `signature`s)
- **Side-effects**:
  - reads wallet keys (public key + private key)
  - reads UTXO DB (via `find_spendable_outputs`)
  - reads chain history to sign (via `Transaction::sign` → `find_transaction`)
- **Why this method exists**: it is the concrete implementation of the UTXO model—spending is expressed as consuming specific outpoints and creating new outputs, with signatures authorizing that consumption.

**Diagram — outpoints and change**

### Text diagram — outpoints and change

```text
Inputs (vin): concrete outpoints spent
  - (txid_A, vout=0)
  - (txid_B, vout=1)

Outputs (vout): new coins created
  - vout=0: recipient, value = amount
  - vout=1: change back to sender, value = input_sum - amount
```

This diagram is the key UTXO mental model: **inputs consume specific previous outputs**; outputs **create new spendable coins**, including change when the selected inputs exceed the payment amount.

### Listing 9-19 (annotated): `Transaction::new_utxo_transaction`

> **Source:** `transaction.rs` — Source

```rust
pub async fn new_utxo_transaction(
    from_wlt_addr: &WalletAddress,
    to_wlt_addr: &WalletAddress,
    tx_amount: i32,
    utxo_set: &UTXOSet,
) -> Result<Transaction> {
    let wallets = WalletService::new()?;
    let from_wallet = wallets.get_wallet(from_wlt_addr)?;
    let from_public_key_hash = hash_pub_key(from_wallet.get_public_key());
    let (available_funds, valid_outputs) = utxo_set
        .find_spendable_outputs(&from_public_key_hash, tx_amount)
        .await?;

    if available_funds < tx_amount { return Err(BtcError::NotEnoughFunds); }

    let mut inputs = vec![];
    for (txid_hex, out_indexes) in valid_outputs {
        let txid = HEXLOWER.decode(txid_hex.as_bytes())?;
        for vout_idx in out_indexes {
            inputs.push(TXInput::new(&txid, vout_idx));
        }
    }

    let mut outputs = vec![TXOutput::new(tx_amount, to_wlt_addr)?];
    if available_funds > tx_amount {
        let change = available_funds - tx_amount;
        outputs.push(TXOutput::new(change, from_wlt_addr)?);
    }

    let mut tx = Transaction { id: vec![], vin: inputs, vout: outputs };
    tx.id = tx.hash()?;
    tx.sign(utxo_set.get_blockchain(), from_wallet.get_pkcs8()).await?;
    Ok(tx)
}
```
### Listing 9-19 explanation:

- It loads the sender’s keys and derives the sender’s `pub_key_hash`, which is the “lock identity” used by outputs in this implementation.
- It performs coin selection via `UTXOSet::find_spendable_outputs(...)`, producing concrete outpoints $(txid, vout)$ that cover the requested amount (or errors if funds are insufficient).
- It turns those outpoints into `TXInput`s (with empty `signature`s for now) and builds the `TXOutput`s: the payment output plus an optional change output.
- It finalizes the transaction by computing its `txid` (`tx.hash()`) and signing every input (`tx.sign(...)`) so each referenced outpoint is authorized for spending.

**Checkpoint (you should be able to explain)**
- Why change is a *new output* rather than “leftover in an account”.

---

### Step 3 — Sign and verify (the trimmed-copy pattern)

Once the transaction is constructed, it needs to be authorized. This step shows how signatures are created to prove that the sender owns the coins being spent, and how those signatures are verified to ensure the transaction is valid.

**What happens here**: Each input in the transaction is signed using a special "trimmed copy" technique that ensures signatures cannot be reused, and we trace how verification works to check those signatures.

This project uses a common Bitcoin-like trick: signatures do not “sign themselves”. Instead, we construct a copy with signatures cleared, inject the referenced output’s locking data (`pub_key_hash`) to bind the signature to the correct spend, hash the copy, and sign that digest.

**Methods involved**
- `Transaction::trimmed_copy`
- `Transaction::sign`
- `Transaction::verify`
- Calls into: `BlockchainService::find_transaction`, `schnorr_sign_digest`, `schnorr_sign_verify`

**Inputs / Outputs / Side-effects**

- **Inputs**:
  - `sign`: a mutable transaction + `private_key`
  - `verify`: a transaction + `blockchain` (for previous-tx lookup)
- **Outputs**:
  - `sign`: fills each input’s `signature`
  - `verify`: returns `true/false`
- **Side-effects**:
  - reads previous transactions from chainstate (to bind signatures to referenced output locks)
- **Why this pattern exists**: it avoids circular signing (“the signature cannot sign itself”) while still committing the signature to the exact outpoint and locking data being spent.

### Listing 9-5.3 (annotated): `trimmed_copy`, `sign`, `verify`

> **Source:** `transaction.rs` — Source

```rust
fn trimmed_copy(&self) -> Transaction {
    let inputs = self.vin.iter()
        .map(|i| TXInput::new(i.get_txid(), i.get_vout()))
        .collect();
    Transaction { id: self.id.clone(), vin: inputs, vout: self.vout.clone() }
}

async fn sign(
    &mut self,
    blockchain: &BlockchainService,
    private_key: &[u8],
) -> Result<()> {
    let mut tx_copy = self.trimmed_copy();
    for (idx, vin) in self.vin.iter_mut().enumerate() {
        let prev_tx = blockchain.find_transaction(vin.get_txid()).await??;

        tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
        tx_copy.id = tx_copy.hash()?;
        let signature = schnorr_sign_digest(private_key, tx_copy.get_id())?;
        vin.signature = signature;
        tx_copy.vin[idx].pub_key = vec![];
    }
    Ok(())
}

pub async fn verify(&self, blockchain: &BlockchainService) -> Result<bool> {
    if self.is_coinbase() { return Ok(true); }
    let mut trimmed_copy = self.trimmed_copy();
    for (idx, vin) in self.vin.iter().enumerate() {
        let prev_tx = blockchain.find_transaction(vin.get_txid()).await??;
        let pkh = prev_tx.vout[vin.vout].pub_key_hash.clone();
        trimmed_copy.vin[idx].pub_key = pkh;
        trimmed_copy.id = trimmed_copy.hash()?;
        let is_valid = schnorr_sign_verify(
            vin.get_pub_key(),
            vin.get_signature(),
            trimmed_copy.get_id(),
        );
        if !is_valid {
            return Ok(false);
        }
        trimmed_copy.vin[idx].pub_key = vec![];
    }
    Ok(true)
}
```
### Listing 9-5.3 explanation:

- `trimmed_copy()` creates a “trimmed copy” of the transaction for signing/verification: it keeps the outputs, but rewrites each input to contain only its outpoint $(txid, vout)$, with signing-related fields (like `signature`, and later the temporary `pub_key`/lock binding) cleared. This gives us a stable payload to hash: the signature is computed over the transaction *shape* and referenced outpoints, not over the signature bytes themselves.
- `sign(...)` signs each input separately: it looks up the previous transaction, temporarily injects the referenced output’s `pub_key_hash` into the copy, hashes that copy, and signs the digest; the resulting signature is written into the real input.
- `verify(...)` recomputes that same digest for each input (again using the referenced output’s `pub_key_hash`) and verifies the signature; coinbase is treated as always valid in this simplified model.

**Checkpoint (you should be able to explain)**
- Why we must look up the previous transaction during signing/verification (to bind the signature to the referenced output’s lock).

---

### Step 4 — Accept into mempool and spawn background work (`NodeContext::process_transaction`)

This is the node’s acceptance boundary: reject duplicates, add to the mempool, then spawn propagation/mining trigger in the background.

**Methods involved**
- `NodeContext::process_transaction`
- Calls into: `transaction_exists_in_pool`, `add_to_memory_pool`, `submit_transaction_for_mining`

**Inputs / Outputs / Side-effects**

- **Inputs**: `(addr_from, transaction)`
- **Outputs**: returns `Ok(txid_hex)` if accepted (or an error on duplicates)
- **Side-effects**:
  - writes to `GLOBAL_MEMORY_POOL`
  - mutates UTXO “reservation” flags (via `add_to_memory_pool`)
  - spawns an async task for propagation/mining trigger
- **Why this method exists**: it centralizes the node’s “mempool gate” so API and network submissions follow the same path.

### Listing 9-5.4 (annotated): `NodeContext::process_transaction`

> **Source:** `context.rs` — Source

```rust
pub async fn process_transaction(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<String> {
    if transaction_exists_in_pool(&utxo) {
        let txid = utxo.get_tx_id_hex();
        return Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid));
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
### Listing 9-5.4 explanation:

- It first checks whether this transaction is **already in the mempool** (a duplicate by `txid`) so the node doesn’t store, relay, or attempt to mine the same pending transaction twice.
- It writes the transaction into the mempool via `add_to_memory_pool(...)` and applies local UTXO “reservation” so the node is less likely to build conflicting pending spends concurrently.
- It spawns propagation/mining-trigger work onto a background task, returning the `txid` immediately to the caller.

**Learning simplification (important for understanding correctness)**

This method does **not** call `Transaction::verify` before storing the tx in mempool. In this implementation, signature verification is enforced at the **mining boundary** (`BlockchainService::mine_block`). That is a reasonable learning simplification, but production systems validate at mempool admission too.

**Checkpoint (you should be able to explain)**
- What is checked *synchronously* before accepting into mempool (duplicate suppression).
- What is deferred to background work (broadcast/mining trigger) and why.
- Where signature verification is enforced in this implementation (mining boundary).

---

### Step 5 — Mempool storage and UTXO “reservation” (`txmempool.rs`)

In our implementation, mempool admission “reserves” the inputs by marking their referenced outpoints as **in mempool**. This is a local concurrency guard: it helps the node avoid constructing multiple pending transactions that try to spend the same coin at the same time (it is not a consensus rule).

**Methods involved**
- `transaction_exists_in_pool`
- `add_to_memory_pool`
- `remove_from_memory_pool`
- Calls into: `GLOBAL_MEMORY_POOL`, `UTXOSet::set_global_mem_pool_flag`

**Inputs / Outputs / Side-effects**

- **Inputs**:
  - `add_to_memory_pool`: `(tx, blockchain_service)`
  - `remove_from_memory_pool`: `(tx, blockchain_service)`
- **Outputs**: none (aside from `Result<()>` for add)
- **Side-effects**:
  - mutates the mempool (`GLOBAL_MEMORY_POOL`)
  - mutates persisted UTXO flags (`set_global_mem_pool_flag`)
- **Why this exists**: the mempool is the node’s holding area for unconfirmed txs, and the “reservation flag” reduces obvious local coin-selection races while a tx is pending.

### Listing 9-5.5 (annotated): `transaction_exists_in_pool`, `add_to_memory_pool`, `remove_from_memory_pool`

> **Source:** `txmempool.rs` — Source

```rust
pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain: &BlockchainService,
) -> Result<()> {
    GLOBAL_MEMORY_POOL.add(tx.clone())?;
    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set.set_global_mem_pool_flag(&tx, true).await?;
    Ok(())
}

pub async fn remove_from_memory_pool(
    tx: Transaction,
    blockchain: &BlockchainService,
) {
    GLOBAL_MEMORY_POOL.remove(tx.clone()).ok();
    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set.set_global_mem_pool_flag(&tx, false).await.ok();
}

pub fn transaction_exists_in_pool(tx: &Transaction) -> bool {
    GLOBAL_MEMORY_POOL.contains_transaction(tx).unwrap_or(false)
}
```
### Listing 9-5.5 explanation:

- `add_to_memory_pool(...)` adds the tx to the in-memory pending set, then marks its referenced outpoints as “in mempool” in the UTXO store to reduce obvious local double-selection.
- `remove_from_memory_pool(...)` removes the tx and clears those reservation flags, making those outpoints available for selection again.
- `transaction_exists_in_pool(...)` is a fast membership check used by the node’s admission path.

**Checkpoint (you should be able to explain)**
- What problem the reservation flag is trying to mitigate (local double-selection), and what it is *not* (a full consensus double-spend solution).

---

### Step 6 — Background propagation and mining trigger (`submit_transaction_for_mining`)

After mempool admission, the node may broadcast the transaction to peers and may trigger mining if the node is configured as a miner and the mempool has enough transactions.

**Methods involved**
- `NodeContext::submit_transaction_for_mining`
- `NodeContext::get_nodes_excluding_sender`
- `NodeContext::broadcast_transaction_to_nodes`
- Calls into: `send_inv`, `miner::{should_trigger_mining, prepare_mining_utxo, process_mine_block}`

**Inputs / Outputs / Side-effects**

- **Inputs**: `(addr_from, transaction)`
- **Outputs**: `Result<()>` (but note: it is typically run in a spawned task)
- **Side-effects**:
  - network side effects: sends INV messages (`send_inv`) to peers
  - may trigger mining and block broadcast if threshold is met
- **Why this method exists**: it keeps mempool admission fast while moving propagation and mining triggers into background work.

### Listing 9-5.6 (annotated): broadcast + mining trigger helpers

> **Source:** `context.rs` — Source

```rust
async fn submit_transaction_for_mining(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<()> {
    if GLOBAL_CONFIG.get_node_addr() == CENTERAL_NODE {
        let nodes = self.get_nodes_excluding_sender(addr_from).await?;
        self.broadcast_transaction_to_nodes(&nodes, utxo.get_id_bytes()).await;
    }

    if should_trigger_mining() {
        if let Some(mining_addr) = GLOBAL_CONFIG.get_mining_addr() {
            let txs = prepare_mining_utxo(&mining_addr)?;
            if !txs.is_empty() {
                process_mine_block(txs, &self.blockchain).await?;
            }
        }
    }
    Ok(())
}

async fn get_nodes_excluding_sender(
    &self,
    addr_from: &std::net::SocketAddr,
) -> Result<Vec<Node>> {
    let my_addr = GLOBAL_CONFIG.get_node_addr();
    let nodes = GLOBAL_NODES.get_nodes()?
        .into_iter()
        .filter(|n| n.get_addr() != *addr_from && n.get_addr() != my_addr)
        .collect();
    Ok(nodes)
}

async fn broadcast_transaction_to_nodes(&self, nodes: &[Node], txid: Vec<u8>) {
    for node in nodes {
        let addr = node.get_addr();
        let txid = txid.clone();
        tokio::spawn(async move {
            send_inv(&addr, OpType::Tx, &[txid]).await;
        });
    }
}
```
### Listing 9-5.6 explanation:

- It optionally broadcasts an INV message (tx inventory) to peers, sending only the `txid` so peers can request the full transaction later if needed.
- It checks whether mining should trigger, and if so prepares a mempool snapshot for mining and hands it to the mining pipeline; errors may lead to cleanup of invalid pending transactions.
- It deliberately uses spawned tasks for per-peer sends so propagation does not block the caller or the mempool admission path.

**Checkpoint (you should be able to explain)**
- Why the node broadcasts an INV (`send_inv`) with a txid instead of sending the full transaction immediately.
- What two conditions must be true for mining to trigger in this implementation (mempool size threshold + node is a miner).

---

### Step 7 — Verification at the mining boundary (`BlockchainService::mine_block`)

This is where the implementation enforces transaction signature correctness before a block is created.

**Methods involved**
- `BlockchainService::mine_block`
- Calls into: `Transaction::verify`

**Inputs / Outputs / Side-effects**

- **Inputs**: `&[Transaction]` (candidate transactions)
- **Outputs**: returns a newly mined `Block` (or errors if any tx is invalid)
- **Side-effects**:
  - performs defensive signature validation before mining
  - then calls into the underlying block-mining implementation
- **Why this exists**: it establishes a hard correctness boundary: the node must not mine a block containing invalid signatures.

### Listing 9-5.7 (annotated): `BlockchainService::mine_block`

> **Source:** `chainstate.rs` — Source

```rust
pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
    // Defensive verification: ensure all txs are valid before mining
    for tx in transactions {
        if !tx.verify(self).await? {
            return Err(BtcError::InvalidTransaction);
        }
    }
    let blockchain_guard = self.0.write().await;
    blockchain_guard.mine_block(transactions).await
}
```
### Listing 9-5.7 explanation:

- It treats block production as a hard correctness boundary: every candidate transaction must pass `Transaction::verify(...)` before any proof-of-work effort is spent.
- Only after validation does it delegate to the underlying mining implementation to build and mine a block from the provided transaction list.

**Checkpoint (you should be able to explain)**
- Exactly where signature verification is enforced for block production in this implementation (`BlockchainService::mine_block` → `Transaction::verify`).

---

### Step 8 — Mining pipeline entry points (`miner.rs`)

This section’s goal is the transaction lifecycle, but it’s useful to see the immediate “next step” after mempool admission: mining consumes the mempool, adds a coinbase transaction, mines a block, removes mined txs from the mempool, and broadcasts the new block inventory.

**Methods involved**
- `miner::should_trigger_mining`
- `miner::prepare_mining_utxo`
- `miner::process_mine_block`
- `miner::broadcast_new_block`

**Inputs / Outputs / Side-effects**

- **Inputs**:
  - `should_trigger_mining`: implicit (global config + mempool size)
  - `prepare_mining_utxo`: `(mining_address)`
  - `process_mine_block`: `(txs, blockchain)`
- **Outputs**:
  - `prepare_mining_utxo`: returns tx list + a coinbase tx appended
  - `process_mine_block`: returns the mined block
- **Side-effects**:
  - removes mined txs from mempool and un-reserves outpoints
  - broadcasts new block inventory to peers
- **Why this is shown here**: it demonstrates the immediate “next step” after mempool admission and clarifies where transactions transition from “pending” to “confirmed”.

### Listing 9-5.8 (annotated): mining trigger and pipeline

> **Source:** `miner.rs` — Source

```rust
const TRANSACTION_THRESHOLD: usize = 3;

pub fn should_trigger_mining() -> bool {
    let pool_size = GLOBAL_MEMORY_POOL.len().unwrap_or(0);
    pool_size >= TRANSACTION_THRESHOLD && GLOBAL_CONFIG.is_miner()
}

pub fn prepare_mining_utxo(
    mining_address: &WalletAddress,
) -> Result<Vec<Transaction>> {
    let mut txs = GLOBAL_MEMORY_POOL.get_all()?;
    let coinbase = Transaction::new_coinbase_tx(mining_address)?;
    txs.push(coinbase);
    Ok(txs)
}

pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    let new_block = blockchain.mine_block(&txs).await?;
    for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }
    broadcast_new_block(&new_block).await?;
    Ok(new_block)
}

pub async fn broadcast_new_block(block: &Block) -> Result<()> {
    let my_addr = GLOBAL_CONFIG.get_node_addr();
    let nodes = GLOBAL_NODES.get_nodes()?;
    for node in nodes {
        if node.get_addr() != my_addr {
            let addr = node.get_addr();
            let hash = block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&addr, OpType::Block, &[hash]).await;
            });
        }
    }
    Ok(())
}
```
### Listing 9-5.8 explanation:

- `should_trigger_mining()` gates mining on “am I a miner?” and “is the mempool large enough?”.
- `prepare_mining_utxo(...)` snapshots the mempool and appends a coinbase transaction so the mined block includes a reward.
- `process_mine_block(...)` mines a new block (which re-verifies txs at the mining boundary), then removes those transactions from the mempool and clears reservation flags.
- `broadcast_new_block(...)` announces the new block by inventory (hash only), allowing peers to fetch the block data afterward.

**Checkpoint (you should be able to explain)**
- Why `prepare_mining_utxo` adds a coinbase tx (block subsidy) and why `process_mine_block` removes mined transactions from the mempool afterwards.

---

## Summary (what you should be able to narrate)

- A wallet constructs a spend by selecting outpoints from the UTXO set, creating outputs (including change), computing a txid, and signing inputs.
- The node accepts the transaction into its mempool, reserves referenced outpoints locally, and spawns background propagation/mining trigger work.
- The mining boundary verifies signatures (`Transaction::verify`) before producing a block.
- After mining, the node removes the mined transactions from mempool and broadcasts the new block inventory.

## Navigation

- **Previous**: Chapter 13 (UTXO Set)
- **Next**: Chapter 15 (Block Lifecycle and Mining)

---

<div align="center">

**[← Previous: UTXO Set](04-UTXO-Set.md)** | **Transaction Lifecycle** | **[Next: Block Lifecycle & Mining →](06-Block-Lifecycle-and-Mining.md)**

</div>

