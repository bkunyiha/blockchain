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
13. **Chapter 13: UTXO Set** ← *You are here*
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

# Chapter 13: UTXO Set — The Spendability Database

In Bitcoin, a **UTXO** (Unspent Transaction Output) is a *specific coin* you can spend: an output created by some earlier transaction that has not yet been referenced as an input by any later transaction. Your “balance” is therefore not a single stored number; it is the sum of the values of all UTXOs you can unlock with your keys.

The **UTXO set** is the node’s *current index of spendability*: the collection of all UTXOs that exist **right now** (often keyed by $txid, vout$ → output details). It is updated when blocks are connected (remove spent outputs, add newly created outputs), and it’s what wallets and validators consult to answer “is this input spendable?” efficiently.

This is different from the **blockchain**, which is an append-only history of blocks and transactions. The chain records *what happened*; the UTXO set represents the *resulting state* (what remains spendable after applying that history). Nodes do **not** store the full UTXO set “in the blockchain” because it would be redundant and expensive: it can be deterministically derived by replaying the chain from genesis, it changes with every block, and different nodes may store it in different internal formats for performance. Instead, each node maintains it locally as derived state (and can always rebuild it from the chain if needed).

In this subsection, we define “what is spendable?” by walking the UTXO code paths. The primary implementation lives in:
- `bitcoin/src/chain/utxo_set.rs`
- `bitcoin/src/primitives/transaction.rs`
- `bitcoin/src/node/txmempool.rs` (mempool flagging)

### How to read this section

- **Code listings are copied from the project** and then annotated with inline comments to explain intent. If you see `debug!` / `trace!` lines, treat them as logging — they’re not core logic.
- **Read the diagrams first**, then the code. Each section is “diagram → full method listing → what it means”.
- **Keep one mental model**: the UTXO set answers one question quickly: *“is (txid, vout) spendable right now?”*

### Terminology quick-reference (used throughout)

- **txid**: transaction identifier (hash bytes; often displayed as hex for readability)
- **vout**: output index within a transaction’s outputs (0-based)
- **outpoint**: $(txid, vout)$ — a pointer to a specific previous output
- **pub_key_hash**: the “lock” on an output (who can spend it)
- **UTXO tree entry**: `txid_bytes -> Vec<TXOutput>` (serialized)

## UTXO flow (where “spendable” is computed and updated)

This chapter covers **the blockchain pipeline (Blockchain — From Transaction to Block Acceptance) Steps 1 and 7**:

- **Step 1 (transaction creation)**: selecting spendable outpoints (coin selection).
- **Step 7 (block connection)**: updating the UTXO set after a block is accepted.

**Reader promise**: after this section, you should be able to explain *exactly* what the UTXO code is doing from the listings shown here:

- What is stored on disk (key/value layout).
- How the wallet selects coins (and what “in mempool” means here).
- How a block mutates spendability (spend inputs, create outputs).
- How we rebuild derived state (reindex) and how we roll it back (reorg support).

### Diagram — end-to-end spendability flow in this implementation

```text
Transaction creation (build & sign a new tx from spendable coins):
  Transaction::new_utxo_transaction
    -> UTXOSet::find_spendable_outputs
         (coin selection: pick outpoints that cover amount)
    -> (optional) txmempool::add_to_memory_pool
         (mark selected inputs as “reserved” locally)
    -> UTXOSet::set_global_mem_pool_flag(true)
         (flip in_global_mem_pool on referenced outputs)

Block acceptance (advance spendability state):
  UTXOSet::update(block)
    (remove spent outputs, insert newly created outputs)

Chain reorg (undo a block’s UTXO effects):
  UTXOSet::rollback_block(block)
    (remove created outputs, restore spent outputs)
```

This diagram shows **where “spendability” is decided and updated** in our node. First, the wallet builds a transaction by selecting outpoints from the UTXO set; optionally, the mempool “reserves” those outpoints to reduce local double-selection; then, when a block is accepted, the UTXO set is advanced by removing spent outputs and inserting newly created ones. If a reorg disconnects a block, `rollback_block` attempts to undo that state transition.

### Step-by-step method map (what gets called at each stage)

Use this as your “index” while reading the sections below.

1. **Storage and data model (how UTXOs live on disk)**
   - `UTXOSet` (type holding a `BlockchainService`)
   - `UTXO_TREE` (sled tree name)
   - Serialization boundary: `bincode::{encode_to_vec, decode_from_slice}`

2. **Coin selection (choose spendable outpoints)**
   - `UTXOSet::find_spendable_outputs(from_pub_key_hash, amount)`

3. **Transaction construction (inputs/outputs/change + signing)**
   - `Transaction::new_utxo_transaction(from, to, amount, utxo_set)`
   - Calls into:
     - `UTXOSet::find_spendable_outputs(...)`
     - `Transaction::hash()` (to compute txid)
     - `Transaction::sign(blockchain, private_key)`

4. **Mempool reservation (avoid selecting the same coin twice locally)**
   - `txmempool::add_to_memory_pool(tx, blockchain_service)`
   - `txmempool::remove_from_memory_pool(tx, blockchain_service)`
   - `txmempool::transaction_exists_in_pool(tx)`
   - Calls into:
     - `UTXOSet::set_global_mem_pool_flag(tx, flag)`

5. **Block connect (advance UTXO set to the new tip)**
   - `UTXOSet::update(block)`

6. **Rebuild derived state (reindex from chain history)**
   - `UTXOSet::reindex()`
   - Calls into:
     - `BlockchainService::find_utxo()` (derive the authoritative unspent map from the chain)

7. **Chain reorg support (disconnect a block)**
   - `UTXOSet::rollback_block(block)`
   - Calls into:
     - `BlockchainService::find_transaction(txid)` (to restore the spent output)

8. **Balance queries (derive “account balance” from UTXOs)**
   - `UTXOSet::find_utxo(pub_key_hash)`
   - `UTXOSet::get_balance(address)`
   - `UTXOSet::utxo_count(address)`

## Table of Contents

1. [Storage layout (sled DB)](#storage-layout-sled-db)
2. [Core data types (TXInput / TXOutput)](#core-data-types-txinput--txoutput)
3. [Finding spendable outputs (coin selection)](#finding-spendable-outputs-coin-selection)
4. [Constructing a transaction (inputs, outputs, change)](#constructing-a-transaction-inputs-outputs-change)
5. [Mempool flags (conflict mitigation)](#mempool-flags-conflict-mitigation)
6. [Updating after a block (connect)](#updating-after-a-block-connect)
7. [Reindexing (rebuild from chain)](#reindexing-rebuild-from-chain)
8. [Rollback (reorg support)](#rollback-reorg-support)
9. [Balance and UTXO counts](#balance-and-utxo-counts)
10. [Important caveats in this implementation](#important-caveats-in-this-implementation)

---

## Storage layout (sled DB)

`sled` is an **embedded key–value database** written in Rust. We use it in this project because it provides **fast local persistence** without running an external database service: the node can store chain/UTXO state on disk, restart, and continue from the same data directory.

There is no `UTXO` struct. The UTXO set is stored as a sled tree:

- Tree name: `chainstate` (constant `UTXO_TREE`)
- Key: `txid` (raw bytes)
- Value: `Vec<TXOutput>` serialized via bincode

Diagram:

```text
UTXO_TREE ("chainstate")
  <txid bytes> -> [TXOutput, TXOutput, ...]
  <txid bytes> -> [TXOutput, ...]
  ...
```

### The on-disk schema in one sentence

We persist **derived state** as: `txid_bytes -> bincode(Vec<TXOutput>)`, and interpret the *outpoint* $(txid, vout)$ as “the output at index `vout` inside that stored `Vec<TXOutput>`”.

`txid_bytes` means the **raw transaction ID bytes** (the 32-byte hash) rather than a human-friendly string. We use bytes as the database key because it is the **canonical representation**, more **space/time efficient**, and avoids repeated hex encoding/decoding. When you see a “transaction id” displayed as hex in logs/UI/APIs, that’s the same value, just rendered for humans.

Here, **`bincode`** is a Rust binary serialization format (built on `serde`). We use it because `sled` stores values as raw bytes (`IVec`), so our `Vec<TXOutput>` must be encoded/decoded on the way in/out. `bincode` is a good fit for this learning project because it’s **compact and fast** for local, internal persistence (as opposed to a human-readable format like JSON).

### A tiny concrete example (how a UTXO (outpoint) is represented in our DB)

If the DB contains:

```text
<txid=aa..ff bytes> -> [out0, out1, out2]
```

Then:

- Outpoint $(aa..ff, 0)$ means “the first element in the stored Vec”
- Outpoint $(aa..ff, 2)$ means “the third element in the stored Vec”

### Listing 9-4.0 — The tree name and UTXO handle

> **Source:** `utxo_set.rs` — Source

```rust
// sled tree name where we persist the derived UTXO set
const UTXO_TREE: &str = "chainstate";

pub struct UTXOSet {
    // handle used to access the DB and chain data needed for reindex/rollback
    blockchain: BlockchainService,
}
```

---

## Core data types (TXInput / TXOutput)

Before we can understand the UTXO set, we need to understand what a transaction *points at* and what an output *contains* in our codebase.

### Listing 9-16 — `TXInput` and how it references an outpoint

> **Source:** `transaction.rs` — Source

```rust
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,                  // previous transaction hash
    vout: usize,                    // output index in that transaction
    signature: Vec<u8>,             // proof of authorization
    pub_key: Vec<u8>,               // public key for verification
}

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    // getter methods: get_txid, get_vout, get_pub_key, get_signature, etc.
    // ...

    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let locking_hash = hash_pub_key(self.pub_key.as_slice());
        locking_hash.eq(pub_key_hash)
    }
}
```

**What this means**

- **Outpoint**: An input doesn’t store “which address” it is spending. It stores **which previous output** it is spending: $(txid, vout)$.
- **Authorization**: `signature` and `pub_key` prove the spender can unlock the referenced output (see `Transaction::sign` and `Transaction::verify`).

### Listing 9-17 — `TXOutput` and the “in mempool” flag

> **Source:** `transaction.rs` — Source

```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,                       // amount locked by this output
    in_global_mem_pool: bool,         // local hint: reserved in mempool?
    pub_key_hash: Vec<u8>,            // "lock": who can spend this
}

impl TXOutput {
    pub fn new(value: i32, address: &WalletAddress) -> Result<TXOutput> {
        let mut output = TXOutput {
            value,
            in_global_mem_pool: false,
            pub_key_hash: vec![],
        };
        output.lock(address)?;
        Ok(output)
    }

    fn lock(&mut self, address: &WalletAddress) -> Result<()> {
        self.pub_key_hash = get_pub_key_hash(address)?;
        Ok(())
    }

    // ... (getters: get_value, get_pub_key_hash, is_locked_with_key, etc.)

    pub fn set_in_global_mem_pool(&mut self, value: bool) {
        self.in_global_mem_pool = value;
    }
}
```

**What this means**

- `pub_key_hash` is the “lock” — it’s what `is_locked_with_key()` matches during coin selection.
- `in_global_mem_pool` is **not a Bitcoin consensus concept**; it’s a *local hint in this learning implementation* used to avoid selecting an output that we already referenced in a pending transaction.

**Checkpoint (you should be able to answer)**

- What exact data identifies “a coin” in this project? (Answer: $(txid, vout)$.)
- Where is “ownership” stored? (Answer: `TXOutput.pub_key_hash`.)

---

## Finding spendable outputs (coin selection)

When creating a transaction, the wallet needs to answer:

> “Which outpoints locked to me can cover $amount$, while avoiding outputs we already used in the mempool?”

### Diagram — what coin selection reads

```text
sled UTXO_TREE (the on-disk UTXO set)
  key = txid bytes (32-byte transaction hash)
  val = Vec<TXOutput> (that tx’s currently-unspent outputs)

scan all entries:
  for each (txid, outputs):
    select output indexes (vout) where:
      - owned-by-me: output.is_locked_with_key(my_pub_key_hash)
      - not-reserved: output.not_in_global_mem_pool()
      - spendable: output.value > 0

return (the selected outpoints, grouped by txid):
  // each pair (txid, vout) is a spendable UTXO
  txid_hex -> [vout0, vout1, ...]
```

This diagram is the mental model for `find_spendable_outputs`: we **scan the persisted UTXO set**, and for each transaction’s output list we select the indices (vout) that match our ownership predicate and are not locally reserved. The result is a set of concrete outpoints $(txid, vout)$ the wallet can turn into transaction inputs.

### Listing 9-4.3 — `UTXOSet::find_spendable_outputs`

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn find_spendable_outputs(
    &self,
    from_pub_key_hash: &[u8],
    amount: i32,
) -> Result<(i32, HashMap<String, Vec<usize>>)> {
    let mut unspent_outputs_indexes = HashMap::new();
    let mut accumulated = 0;
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db.open_tree(UTXO_TREE)?;

    for item in utxo_tree.iter() {
        let (k, v) = item?;
        let txid_hex = HEXLOWER.encode(k.as_ref());
        let (tx_out, _) = bincode::serde::decode_from_slice(
            v.as_ref(),
            bincode::config::standard(),
        )?;

        for (vout_idx, out) in tx_out.iter().enumerate() {
            if out.not_in_global_mem_pool()
                && out.is_locked_with_key(from_pub_key_hash)
                && accumulated < amount
            {
                accumulated += out.get_value();
                unspent_outputs_indexes
                    .entry(txid_hex.clone())
                    .or_insert_with(Vec::new)
                    .push(vout_idx);
            }
        }
    }
    Ok((accumulated, unspent_outputs_indexes))
}
```

**What this does (step-by-step)**

- **Methods involved (so you can map this to the code)**
  - Entry point: `UTXOSet::find_spendable_outputs(from_pub_key_hash, amount)`
  - Calls into:
    - `BlockchainService::get_db()` (open DB handle)
    - `sled::Db::open_tree(UTXO_TREE)` and `Tree::iter()` (scan persisted UTXOs)
    - `bincode::serde::decode_from_slice(...)` (deserialize `Vec<TXOutput>`)
    - `TXOutput::{is_locked_with_key, not_in_global_mem_pool, get_value}` (selection predicate)

- Opens `UTXO_TREE` and iterates the entire keyspace (this is $O(N)$ over all stored outputs).
- Deserializes each `Vec<TXOutput>` and checks each output:
  - **Ownership**: `out.is_locked_with_key(from_pub_key_hash)`
  - **Not reserved**: `out.not_in_global_mem_pool()`
  - **Non-zero**: `out.get_value() > 0`
- Keeps adding outputs until `accmulated >= amount`, then stops selecting (but it still scans all entries in the current code).
- Returns:
  - `available_funds` (the accumulated sum)
  - `HashMap<String, Vec<usize>>` mapping `txid_hex -> [vout indexes]`

**Checkpoint (you should be able to explain)**

- Why coin selection returns `txid_hex -> [vout]` instead of a flat list.
- What “in mempool” means in this project (a local reservation flag on `TXOutput`).

---

## Constructing a transaction (inputs, outputs, change)

Coin selection is used by `Transaction::new_utxo_transaction` to build the actual transaction:

1. Use the selected outpoints to build `TXInput`s.
2. Create a recipient `TXOutput`.
3. Create an optional change `TXOutput`.
4. Hash the transaction to get an ID.
5. Sign each input.

### Diagram — transaction construction in this implementation

```text
new_utxo_transaction(from, to, amount):
  pub_key_hash = hash(from_public_key)
  (available, outpoints) = utxo_set.find_spendable_outputs(pub_key_hash, amount)
  inputs  = [ (txid, vout, pub_key=from_pub_key, signature=empty) ... ]
  outputs = [ to:amount, (optional) change:(available-amount) back to from ]
  tx.id   = sha256(serialize(tx with id empty))
  tx.sign(blockchain, from_private_key)
```

This diagram shows the **mechanical steps** of building a UTXO transaction: selected outpoints become `TXInput`s, value is re-expressed as one or two `TXOutput`s (payment + optional change), then we compute a txid and sign. The key “why” is that spending consumes entire outputs, so any leftover value must be recreated as a new change output.

### Listing 9-4.4 — `Transaction::new_utxo_transaction`

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
    let from_pub_key_hash = hash_pub_key(from_wallet.get_public_key());
    let (available_funds, valid_outputs) = utxo_set
        .find_spendable_outputs(&from_pub_key_hash, tx_amount)
        .await?;

    if available_funds < tx_amount {
        return Err(BtcError::NotEnoughFunds);
    }

    let mut inputs = vec![];
    for (txid_hex, out_indexes) in valid_outputs {
        let txid = HEXLOWER.decode(txid_hex.as_bytes())?;
        for vout_idx in out_indexes {
            inputs.push(TXInput::new(&txid, vout_idx));
        }
    }

    let mut outputs = vec![TXOutput::new(tx_amount, to_wlt_addr)?];
    if available_funds > tx_amount {
        outputs.push(TXOutput::new(
            available_funds - tx_amount,
            from_wlt_addr,
        )?);
    }

    let mut tx = Transaction {
        id: vec![],
        vin: inputs,
        vout: outputs,
    };
    tx.id = tx.hash()?;
    tx.sign(utxo_set.get_blockchain(), from_wallet.get_pkcs8())
        .await?;
    Ok(tx)
}
```

**What to notice**

- Inputs reference previous outputs by $(txid, vout)$ and carry the sender’s `pub_key`.
- Outputs are just “value + lock(pub_key_hash)”.
- Change is explicitly created by returning `available_funds - tx_amount` back to `from`.

**Checkpoint (you should be able to explain)**

- How many outputs are created for a normal payment and why a change output exists.
- At what point the transaction ID is computed and when signatures are attached.

---

## Mempool flags (conflict mitigation)

In full Bitcoin, mempool is separate from the UTXO set; here we intentionally make a simplified learning trade-off:

> When a transaction is added to the mempool, we mark the referenced UTXOs as “in mempool” so coin selection won’t pick them again while the transaction is pending.

### Diagram — “reserve” an output while it’s pending

```text
tx arrives -> add_to_memory_pool(tx)
  for each input (txid, vout):
    load Vec<TXOutput> for txid
    set Vec[vout].in_global_mem_pool = true
    write Vec back
```

This diagram explains our simplified mempool integration: when a transaction is pending, we **mark the exact referenced outpoints as “reserved”** by flipping `in_global_mem_pool` on the corresponding `TXOutput`. The “why” is to reduce local races where two concurrent transaction builders pick the same coin; confirmation still happens only when a block is connected.

### Listing 9-4.5 — `UTXOSet::set_global_mem_pool_flag`

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn set_global_mem_pool_flag(
    &self,
    tx: &Transaction,
    flag: bool,
) -> Result<()> {
    if tx.is_coinbase() {
        return Ok(());
    }
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db.open_tree(UTXO_TREE)?;

    for input in tx.get_vin() {
        if let Some(utxo_ivec) = utxo_tree.get(input.get_txid())? {
            let mut utxo_list: Vec<TXOutput> = bincode::serde
                ::decode_from_slice(
                    utxo_ivec.as_ref(),
                    bincode::config::standard(),
                )?
                .0;
            if let Some(utxo) = utxo_list.get_mut(input.get_vout()) {
                utxo.set_in_global_mem_pool(flag);
            }
            let outs_bytes = bincode::serde::encode_to_vec(
                &utxo_list,
                bincode::config::standard(),
            )?;
            utxo_tree.insert(input.get_txid(), outs_bytes)?;
        }
    }
    Ok(())
}
```

### Listing 9-4.6 — Mempool call sites (`add_to_memory_pool` / `remove_from_memory_pool`)

> **Source:** `txmempool.rs` — Source

```rust
pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain_service: &BlockchainService,
) -> Result<()> {
    GLOBAL_MEMORY_POOL.add(tx.clone())?;
    let utxo_set = UTXOSet::new(blockchain_service.clone());
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
```

### Listing 9-4.6b — Mempool existence check (`transaction_exists_in_pool`)

> **Source:** `txmempool.rs` — Source

```rust
pub fn transaction_exists_in_pool(tx: &Transaction) -> bool {
    // Convenience helper: "is this tx already pending?"
    GLOBAL_MEMORY_POOL.contains_transaction(tx).unwrap_or(false)
}
```

**What to notice**

- This section’s “mempool flag” is a **local concurrency guard**, not a consensus rule.
- We mutate the UTXO database on mempool admission/removal, which is not how Bitcoin Core structures it (but is useful for learning).

**Checkpoint**

- What changes in the DB when a tx enters the mempool (a boolean flips on referenced outputs).
- What *doesn’t* happen yet (we are not removing UTXOs on mempool admission; confirmation happens at block connect).

---

## Updating after a block (connect)

When a block is accepted, we must move “what is spendable” forward:

- **Spend**: remove the specific referenced outputs for each non-coinbase input.
- **Create**: insert the outputs of each transaction as newly unspent outputs.

### Diagram — the connect operation

```text
for each tx in block:
  if not coinbase:
    for each input (txid, vout):
      load Vec<TXOutput> for txid
      remove the output at index vout (by filtering it out)
      write back Vec, or delete key if empty
  insert tx's own outputs under key = tx.id
```

This diagram is the core state transition: connecting a block **spends** previously-unspent outputs (remove referenced outpoints) and **creates** new unspent outputs (insert each tx’s outputs under its txid). This is why the UTXO set is a “current state” database: it’s updated incrementally as blocks are accepted.

### Listing 9-4.7 — `UTXOSet::update`

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn update(&self, block: &Block) -> Result<()> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db.open_tree(UTXO_TREE)?;

    for tx in block.get_transactions().await? {
        if !tx.is_coinbase() {
            for input in tx.get_vin() {
                let utxo_list: Vec<TXOutput> = bincode::serde
                    ::decode_from_slice(
                        utxo_tree
                            .get(input.get_txid())?
                            .ok_or(BtcError::UTXONotFoundError(
                                "UTXO not found".into(),
                            ))?
                            .as_ref(),
                        bincode::config::standard(),
                    )?
                    .0;
                let updated_outs: Vec<TXOutput> = utxo_list
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != input.get_vout())
                    .map(|(_, o)| o.clone())
                    .collect();
                if updated_outs.is_empty() {
                    utxo_tree.remove(input.get_txid())?;
                } else {
                    let encoded = bincode::serde::encode_to_vec(
                        &updated_outs,
                        bincode::config::standard(),
                    )?;
                    utxo_tree.insert(input.get_txid(), encoded)?;
                }
            }
        }
        let bytes = bincode::serde::encode_to_vec(
            &tx.get_vout(),
            bincode::config::standard(),
        )?;
        utxo_tree.insert(tx.get_id(), bytes)?;
    }
    Ok(())
}
```

**Checkpoint**

- After `update(block)`, where do the new UTXOs live? (Answer: under key = each transaction’s txid, value = its outputs.)
- What causes a txid key to be deleted from the UTXO tree? (Answer: when all its outputs are spent and the filtered Vec becomes empty.)

---

## Reindexing (rebuild from chain)

The UTXO set is **derived state**: it’s a performance cache of “what is spendable right now” that can always be recomputed by replaying the blockchain from genesis. A rebuild (reindex) is needed whenever that cached state can’t be trusted or no longer matches the current rules/chain tip.

You typically run `reindex()` in these situations:

- **First startup / new node**: you have the chain data, but no UTXO database yet.
- **Corruption or partial writes**: the UTXO tree may be inconsistent after a crash or disk issue.
- **Schema/format changes**: you changed how UTXOs are stored (keys/values/serialization) during development.
- **Rule/logic changes**: you updated validation/state-transition logic and want the derived state to reflect the new behavior.
- **Recovery after chain changes**: after large reorganizations or when you want a “ground truth” rebuild instead of incremental repair.

`reindex()` is the “rebuild derived state” tool:

1. Clear the UTXO tree.
2. Scan the chain and re-derive all unspent outputs (`blockchain.find_utxo()`).
3. Insert them into the UTXO tree.

### Listing 9-4.8 — `UTXOSet::reindex`

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn reindex(&self) -> Result<()> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db.open_tree(UTXO_TREE)?;
    utxo_tree.clear()?;

    let utxo_map = self.blockchain.find_utxo().await?;
    for (txid_hex, outs) in &utxo_map {
        let txid = HEXLOWER.decode(txid_hex.as_bytes())?;
        let value = bincode::serde::encode_to_vec(
            outs,
            bincode::config::standard(),
        )?;
        utxo_tree.insert(txid.as_slice(), value)?;
    }
    Ok(())
}
```

**Checkpoint**

- Why `reindex()` is safe conceptually (derived state rebuilt from the canonical chain history).
- When you would use it as a developer (schema changes, corruption recovery, rule changes in a learning project).

---

## Rollback (reorg support)

When we “disconnect” a block (e.g., during a chain reorganization), we need to undo the UTXO changes:

1. Remove outputs created by the block’s transactions.
2. Restore outputs that were spent by those transactions (by looking them up from the referenced previous transactions).

**Important: Transaction processing order** — Transactions are now processed in **reverse order** (newest first). This correctly handles intra-block dependencies where a later transaction spends an output created by an earlier transaction in the same block. Without reverse order, the earlier transaction’s outputs would be removed before the later transaction’s inputs could be restored.

### Listing 9-4.9 — `UTXOSet::rollback_block`

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn rollback_block(&self, block: &Block) -> Result<()> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db.open_tree(UTXO_TREE)?;

    // Process transactions in REVERSE order (newest first) to correctly
    // handle intra-block dependencies where a later tx spends an earlier
    // transaction’s output
    let transactions = block.get_transactions().await?;
    let reversed: Vec<_> = transactions.into_iter().rev().collect();

    for tx in reversed {
        // Step 1: Remove this transaction’s outputs from UTXO set
        utxo_tree.remove(tx.get_id())?;

        // Step 2: Restore spent inputs (skip for coinbase — it has no inputs)
        if !tx.is_coinbase() {
            for input in tx.get_vin() {
                if let Some(input_tx) =
                    self.blockchain.find_transaction(input.get_txid()).await?
                {
                    if let Some(output) =
                        input_tx.get_vout().get(input.get_vout())
                    {
                        // Load existing outputs OR start fresh if fully spent.
                        // The else branch fixes the primary consensus bug.
                        let mut outs = if let Some(existing_bytes) =
                            utxo_tree.get(input.get_txid())?
                        {
                            bincode::serde::decode_from_slice(
                                existing_bytes.as_ref(),
                                bincode::config::standard(),
                            )?.0
                        } else {
                            // Transaction fully spent — start fresh
                            vec![]
                        };

                        // Insert restored output at correct vout position
                        let vout_idx = input.get_vout();
                        if vout_idx <= outs.len() {
                            outs.insert(vout_idx, output.clone());
                        } else {
                            while outs.len() < vout_idx {
                                outs.push(output.clone());
                            }
                            outs.push(output.clone());
                        }

                        let bytes = bincode::serde::encode_to_vec(
                            &outs, bincode::config::standard(),
                        )?;
                        utxo_tree.insert(input.get_txid(), bytes)?;
                    }
                }
            }
        }
    }
    Ok(())
}
```

> **Why this code matters**: This is the function that caused the original consensus bug. When a chain reorganization rolled back a block, any transaction that had spent the LAST output of a previous transaction would fail to restore that output — because the UTXO entry had been fully removed from the tree. The `else` branch (creating a fresh `vec![]`) fixes this by always restoring the output regardless of whether the UTXO entry still exists. This matches Bitcoin Core’s `DisconnectBlock` which uses stored "undo data" (`rev*.dat` files) to reliably reverse UTXO changes.

**Checkpoint**

- What rollback removes (outputs created by the block).
- What rollback restores (the previously-spent outputs referenced by the block’s inputs).
- The `else` branch handles the case where ALL outputs of a transaction were spent (entry fully removed from UTXO tree). This was the primary cause of the consensus bug.

---

## Balance and UTXO counts

Balances are derived from UTXOs. In our codebase, balance queries are “scan the UTXO tree for outputs locked to this address and sum them”.

It’s worth pausing on *why* we read balance from the **UTXO set** rather than the **blockchain**. The blockchain is the permanent diary of *everything that ever happened*; it’s not optimized for answering “what is true right now?” To compute a balance from the chain directly, you would have to replay transactions and keep track of which outputs have already been spent—every time you ask the question.

The UTXO set exists so the node can keep a **ready-to-use view of current spendability**: it is updated once per connected block, and then used to answer balance and input-selection queries cheaply. And because it is derived from the chain, it can always be rebuilt when needed.

### Listing 9-4.10 — `UTXOSet::find_utxo` (fetch all spendable outputs for a key-hash)

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn find_utxo(
    &self,
    pub_key_hash: &[u8],
) -> Result<Vec<TXOutput>> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db.open_tree(UTXO_TREE)?;
    let mut utxos = vec![];

    for item in utxo_tree.iter() {
        let (_, v) = item?;
        let outs: Vec<TXOutput> = bincode::serde
            ::decode_from_slice(
                v.to_vec().as_slice(),
                bincode::config::standard(),
            )?
            .0;

        for out in outs.iter() {
            if out.is_locked_with_key(pub_key_hash) {
                utxos.push(out.clone())
            }
        }
    }
    Ok(utxos)
}
```

### Listing 9-4.11 — `UTXOSet::get_balance` and `UTXOSet::utxo_count`

> **Source:** `utxo_set.rs` — Source

```rust
pub async fn get_balance(
    &self,
    wlt_address: &WalletAddress,
) -> Result<i32> {
    let pub_key_hash = get_pub_key_hash(wlt_address)?;
    let utxos = self.find_utxo(pub_key_hash.as_slice()).await?;
    let balance = utxos.iter().map(|u| u.get_value()).sum();
    Ok(balance)
}

pub async fn utxo_count(
    &self,
    wlt_address: &WalletAddress,
) -> Result<usize> {
    let pub_key_hash = get_pub_key_hash(wlt_address)?;
    Ok(self.find_utxo(pub_key_hash.as_slice()).await?.len())
}
```

**Checkpoint**

- How “balance” is computed (sum of values of outputs locked to your pubkey-hash).
- Why this implementation is $O(N)$ over the UTXO DB (full scan), and what an optimization would look like (secondary indexes).

---

## Important caveats in this implementation

This section intentionally explains the code **as written**, but a few details are important for a Rust developer to understand:

1. **Outpoint stability vs `Vec<TXOutput>`**
   - Our storage model uses `Vec<TXOutput>` and refers to outputs by **index**.
   - `update()` removes a spent output by filtering it out, which can **shift indices** of later outputs.
   - In Bitcoin, `(txid, vout)` is a stable identifier; in production designs you’d typically preserve indices (e.g., store a sparse map, keep tombstones, or store per-outpoint keys).

2. **Mempool flagging is a learning shortcut**
   - `in_global_mem_pool` is a local boolean on `TXOutput` and is not part of Bitcoin’s consensus.
   - It is useful here to reduce obvious “pick the same coin twice” behavior during concurrent transaction construction.

3. **Rollback for fully-spent transactions**
   - `rollback_block()` restores spent outputs by fetching the *previous transaction* from the chain and reinserting the referenced output.
   - When **no existing `Vec<TXOutput>` for that `txid`** remains in the UTXO tree (because all outputs were spent), the `else` branch creates a fresh `vec![]` and inserts the restored output at the correct `vout` position. This ensures outputs are never silently lost during reorganization — the fix for the primary consensus bug discovered during multi-node testing.
   - Transactions are processed in **reverse order** (newest first) to correctly handle intra-block dependencies.

4. **Performance characteristics**
   - `find_spendable_outputs()` and `find_utxo()` scan the entire UTXO tree. This is fine for a learning project, but real wallets/indexers maintain additional indexes for efficient lookup.

---

## Navigation

- **Previous**: Chapter 12 (Chain State and Storage)
- **Next**: Chapter 14 (Transaction Lifecycle)

---

<div align="center">

**[← Previous: Chain State and Storage](03-Chain-State-and-Storage.md)** | **UTXO Set** | **[Next: Transaction Lifecycle →](05-Transaction-Lifecycle.md)**

</div>

