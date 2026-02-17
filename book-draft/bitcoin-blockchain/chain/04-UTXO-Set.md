<div align="left">

<details>
<summary><b>📑 Section Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Section 1: Introduction & Overview
2. Section 1.2: Introduction to Bitcoin & Blockchain
3. Section 1.3: Bitcoin Whitepaper
4. Section 1.4: Bitcoin Whitepaper In Rust
5. Section 2.0: Rust Blockchain Project
6. Section 2.1: Primitives
7. Section 2.2: Utilities
8. Section 2.3: Cryptography
9. **Section 2.4: Blockchain (Technical Foundations)** ← *You are here*
10. Section 2.5: Storage Layer

</details>

</div>

---

# Section 2.4.4: UTXO Set — The Spendability Database

In Bitcoin, a **UTXO** (Unspent Transaction Output) is a *specific coin* you can spend: an output created by some earlier transaction that has not yet been referenced as an input by any later transaction. Your “balance” is therefore not a single stored number; it is the sum of the values of all UTXOs you can unlock with your keys.

The **UTXO set** is the node’s *current index of spendability*: the collection of all UTXOs that exist **right now** (often keyed by \(txid, vout\) → output details). It is updated when blocks are connected (remove spent outputs, add newly created outputs), and it’s what wallets and validators consult to answer “is this input spendable?” efficiently.

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
- **outpoint**: \((txid, vout)\) — a pointer to a specific previous output
- **pub_key_hash**: the “lock” on an output (who can spend it)
- **UTXO tree entry**: `txid_bytes -> Vec<TXOutput>` (serialized)

## UTXO flow (where “spendable” is computed and updated)

This subsection covers **Section 2.4 (Blockchain — From Transaction to Block Acceptance) Steps 1 and 7**:

- **Step 1 (transaction creation)**: selecting spendable outpoints (coin selection).
- **Step 7 (block connection)**: updating the UTXO set after a block is accepted.

**Reader promise**: after this section, you should be able to explain *exactly* what the UTXO code is doing from the listings shown here:

- What is stored on disk (key/value layout).
- How the wallet selects coins (and what “in mempool” means here).
- How a block mutates spendability (spend inputs, create outputs).
- How we rebuild derived state (reindex) and how we roll it back (reorg support).

### Diagram — end-to-end spendability flow in this implementation

```
Transaction creation (build & sign a new tx from spendable coins):
  Transaction::new_utxo_transaction
    -> UTXOSet::find_spendable_outputs          (coin selection: pick outpoints that cover amount)
    -> (optional) txmempool::add_to_memory_pool (mark selected inputs as “reserved” locally)
         -> UTXOSet::set_global_mem_pool_flag(true) (flip in_global_mem_pool on referenced outputs)

Block acceptance (advance spendability state to include this block):
  UTXOSet::update(block)                        (remove spent outputs, insert newly created outputs)

Chain reorg (undo a block’s UTXO effects):
  UTXOSet::rollback_block(block)                (remove outputs created by block, restore outputs it spent)
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

```
UTXO_TREE ("chainstate")
  <txid bytes> -> [TXOutput, TXOutput, ...]
  <txid bytes> -> [TXOutput, ...]
  ...
```

### The on-disk schema in one sentence

We persist **derived state** as: `txid_bytes -> bincode(Vec<TXOutput>)`, and interpret the *outpoint* \((txid, vout)\) as “the output at index `vout` inside that stored `Vec<TXOutput>`”.

`txid_bytes` means the **raw transaction ID bytes** (the 32-byte hash) rather than a human-friendly string. We use bytes as the database key because it is the **canonical representation**, more **space/time efficient**, and avoids repeated hex encoding/decoding. When you see a “transaction id” displayed as hex in logs/UI/APIs, that’s the same value, just rendered for humans.

Here, **`bincode`** is a Rust binary serialization format (built on `serde`). We use it because `sled` stores values as raw bytes (`IVec`), so our `Vec<TXOutput>` must be encoded/decoded on the way in/out. `bincode` is a good fit for this learning project because it’s **compact and fast** for local, internal persistence (as opposed to a human-readable format like JSON).

### A tiny concrete example (how a UTXO (outpoint) is represented in our DB)

If the DB contains:

```
<txid=aa..ff bytes> -> [out0, out1, out2]
```

Then:

- Outpoint \((aa..ff, 0)\) means “the first element in the stored Vec”
- Outpoint \((aa..ff, 2)\) means “the third element in the stored Vec”

### Code Listing 2.4-4.0 — The tree name and UTXO handle

```rust
// Source: bitcoin/src/chain/utxo_set.rs
const UTXO_TREE: &str = "chainstate"; // sled tree name where we persist the derived UTXO set

pub struct UTXOSet {
    blockchain: BlockchainService, // handle used to access the DB and chain data needed for reindex/rollback
}
```

---

## Core data types (TXInput / TXOutput)

Before we can understand the UTXO set, we need to understand what a transaction *points at* and what an output *contains* in our codebase.

### Code Listing 2.4-4.1 — `TXInput` and how it references an outpoint

```rust
// Source: bitcoin/src/primitives/transaction.rs
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),   // bytes of the previous transaction id
            vout,                  // which output index inside that previous transaction
            signature: vec![],     // filled in later by signing
            pub_key: vec![],       // filled in later (or empty for coinbase)
        }
    }

    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_input_tx_id_hex(&self) -> String {
        HEXLOWER.encode(self.txid.as_slice())
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    pub fn get_signature(&self) -> &[u8] {
        self.signature.as_slice()
    }

    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        // "Does this input belong to / come from this pubkey-hash?"
        // We hash the raw pubkey and compare it to the pubkey-hash lock.
        let locking_hash = hash_pub_key(self.pub_key.as_slice());
        locking_hash.eq(pub_key_hash)
    }
}
```

**What this means**

- **Outpoint**: An input doesn’t store “which address” it is spending. It stores **which previous output** it is spending: \((txid, vout)\).
- **Authorization**: `signature` and `pub_key` prove the spender can unlock the referenced output (see `Transaction::sign` and `Transaction::verify`).

### Code Listing 2.4-4.2 — `TXOutput` and the “in mempool” flag

```rust
// Source: bitcoin/src/primitives/transaction.rs
#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    in_global_mem_pool: bool,
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn new(value: i32, address: &WalletAddress) -> Result<TXOutput> {
        let mut output = TXOutput {
            value,                    // coin amount carried by this output
            in_global_mem_pool: false, // local hint: reserved by pending mempool tx?
            pub_key_hash: vec![],      // "lock": who can spend this output
        };
        output.lock(address)?;
        Ok(output)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    fn lock(&mut self, address: &WalletAddress) -> Result<()> {
        // Lock the output by storing the address's pubkey-hash (P2PKH-style).
        let pub_key_hash = get_pub_key_hash(address)?;
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }

    pub fn not_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.ne(pub_key_hash)
    }

    pub fn set_in_global_mem_pool(&mut self, value: bool) {
        // Mark/unmark this output as reserved by an in-flight mempool transaction.
        self.in_global_mem_pool = value;
    }

    pub fn is_in_global_mem_pool(&self) -> bool {
        self.in_global_mem_pool
    }
    pub fn not_in_global_mem_pool(&self) -> bool {
        !self.in_global_mem_pool
    }
}
```

**What this means**

- `pub_key_hash` is the “lock” — it’s what `is_locked_with_key()` matches during coin selection.
- `in_global_mem_pool` is **not a Bitcoin consensus concept**; it’s a *local hint in this learning implementation* used to avoid selecting an output that we already referenced in a pending transaction.

**Checkpoint (you should be able to answer)**

- What exact data identifies “a coin” in this project? (Answer: \((txid, vout)\).)
- Where is “ownership” stored? (Answer: `TXOutput.pub_key_hash`.)

---

## Finding spendable outputs (coin selection)

When creating a transaction, the wallet needs to answer:

> “Which outpoints locked to me can cover \(amount\), while avoiding outputs we already used in the mempool?”

### Diagram — what coin selection reads

```
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
  txid_hex -> [vout0, vout1, ...]   // each pair (txid, vout) is a spendable UTXO
```

This diagram is the mental model for `find_spendable_outputs`: we **scan the persisted UTXO set**, and for each transaction’s output list we select the indices (vout) that match our ownership predicate and are not locally reserved. The result is a set of concrete outpoints \((txid, vout)\) the wallet can turn into transaction inputs.

### Code Listing 2.4-4.3 — `UTXOSet::find_spendable_outputs`

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn find_spendable_outputs(
    &self,
    from_pub_key_hash: &[u8],
    amount: i32,
) -> Result<(i32, HashMap<String, Vec<usize>>)> {
    debug!("Finding spendable outputs for amount: {}", amount);
    // Output: selected spendable outpoints grouped by txid (txid_hex -> [vout indexes]).
    let mut unspent_outputs_indexes: HashMap<String, Vec<usize>> = HashMap::new();
    // Running total of selected value; used to cover `amount`.
    let mut accmulated = 0;
    // Open the node's persistent database and the UTXO tree.
    let db = self.blockchain.get_db().await?; // open sled database
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
    // Debug counter: how many outputs we inspected during this scan.
    let mut total_checked = 0;
    for item in utxo_tree.iter() {
        // Each key/value pair is one "transaction id -> list of its unspent outputs"
        let (k, v) = item.map_err(|e| BtcError::GettingUTXOError(e.to_string()))?;
        // `k` is the raw txid bytes; we hex-encode only for readability + HashMap keying.
        let txid_hex = HEXLOWER.encode(k.to_vec().as_slice()); // for return/debug readability
        // Decode the stored Vec<TXOutput>; the "vout" for an outpoint is the Vec index.
        let (tx_out, _): (Vec<TXOutput>, usize) = bincode::serde::decode_from_slice(
            v.to_vec().as_slice(),
            bincode::config::standard(),
        )
        .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?;
        for (current_out_index, out) in tx_out.iter().enumerate() {
            // `current_out_index` is the vout index (0-based) for outpoint (txid, vout).
            total_checked += 1;
            debug!(
                "Checking output {} in tx {}: value={}, in_mempool={}, locked_with_key={}",
                current_out_index,
                txid_hex,
                out.get_value(),
                out.is_in_global_mem_pool(),
                out.is_locked_with_key(from_pub_key_hash)
            );
            // Selection predicate:
            // - output must be ours (locked to our pubkey-hash)
            // - output must not be locally reserved by mempool
            // - output must carry value
            // - stop selecting once we've covered the target amount
            if out.not_in_global_mem_pool()
                && out.get_value() > 0
                && out.is_locked_with_key(from_pub_key_hash)
                && accmulated < amount
            {
                accmulated += out.get_value();
                debug!(
                    "Adding spendable output: tx={}, idx={}, value={}, accumulated={}",
                    txid_hex,
                    current_out_index,
                    out.get_value(),
                    accmulated
                );
                if unspent_outputs_indexes.contains_key(txid_hex.as_str()) {
                    unspent_outputs_indexes
                        .get_mut(txid_hex.as_str())
                        .ok_or(BtcError::UTXONotFoundError(format!(
                            "(find_spendable_outputs) UTXO {} not found",
                            txid_hex
                        )))?
                        .push(current_out_index);
                } else {
                    unspent_outputs_indexes.insert(txid_hex.clone(), vec![current_out_index]);
                }
            }
        }
        // Note: even after `accmulated >= amount`, the current implementation keeps scanning.
        // That’s fine for a learning node; a production wallet would typically short-circuit early.
    }
    debug!(
        "find_spendable_outputs completed: checked {} outputs, accumulated={}, found {} spendable transactions",
        total_checked,
        accmulated,
        unspent_outputs_indexes.len()
    );
    Ok((accmulated, unspent_outputs_indexes))
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

- Opens `UTXO_TREE` and iterates the entire keyspace (this is \(O(N)\) over all stored outputs).
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

```
new_utxo_transaction(from, to, amount):
  pub_key_hash = hash(from_public_key)
  (available, outpoints) = utxo_set.find_spendable_outputs(pub_key_hash, amount)
  inputs  = [ (txid, vout, pub_key=from_pub_key, signature=empty) ... ]
  outputs = [ to:amount, (optional) change:(available-amount) back to from ]
  tx.id   = sha256(serialize(tx with id empty))
  tx.sign(blockchain, from_private_key)
```

This diagram shows the **mechanical steps** of building a UTXO transaction: selected outpoints become `TXInput`s, value is re-expressed as one or two `TXOutput`s (payment + optional change), then we compute a txid and sign. The key “why” is that spending consumes entire outputs, so any leftover value must be recreated as a new change output.

### Code Listing 2.4-4.4 — `Transaction::new_utxo_transaction`

```rust
// Source: bitcoin/src/primitives/transaction.rs
pub async fn new_utxo_transaction(
    from_wlt_addr: &WalletAddress,
    to_wlt_addr: &WalletAddress,
    tx_amount: i32,
    utxo_set: &UTXOSet,
) -> Result<Transaction> {
    // Load the local wallet store so we can access the sender's keys.
    // Why: building inputs requires the sender's pubkey, and signing requires the private key.
    let wallets = WalletService::new()?;
    let from_wallet = wallets
        .get_wallet(from_wlt_addr)
        .ok_or_else(|| BtcError::UTXONotFoundError(from_wlt_addr.as_string()))?;
    // Compute the pubkey-hash used to identify "coins locked to this wallet".
    // Why: our UTXO set indexes ownership by output.pub_key_hash.
    let from_public_key_hash = hash_pub_key(from_wallet.get_public_key()); // "who are we spending as?"

    let (available_funds, valid_outputs) = utxo_set
        .find_spendable_outputs(from_public_key_hash.as_slice(), tx_amount)
        .await?;

    debug!(
        "Transaction creation: from={}, to={}, amount={}",
        from_wlt_addr.as_str(),
        to_wlt_addr.as_str(),
        tx_amount
    );
    debug!(
        "Found spendable outputs: accumulated={}, valid_outputs={:?}",
        available_funds, valid_outputs
    );

    // Hard check: you can't spend more than your selected UTXOs cover.
    // Why: this is the UTXO equivalent of "insufficient funds".
    if available_funds < tx_amount {
        return Err(BtcError::NotEnoughFunds);
    }

    let mut inputs = vec![];
    for (txid_hex, out_indexes) in valid_outputs {
        // Convert the selected txid back from hex into raw bytes for TXInput.
        // Why: DB/logs often use hex strings, but the canonical txid representation is bytes.
        let txid = HEXLOWER
            .decode(txid_hex.as_bytes())
            .map_err(|e| BtcError::TransactionIdHexDecodingError(e.to_string()))?;
        for current_out_index in out_indexes {
            let input = TXInput {
                txid: txid.clone(), // txid is the hash of the previous transaction or transaction that contains the output that is being spent
                vout: current_out_index, // vout is the index of the output that is being spent in the previous transaction or transaction that contains the output that is being spent
                signature: vec![], // filled in by tx.sign(...)
                pub_key: from_wallet.get_public_key().to_vec(), // why: verifiers must know which pubkey claims to unlock the referenced output
            };
            inputs.push(input);
        }
    }

    // Primary payment output (recipient).
    let mut outputs = vec![TXOutput::new(tx_amount, to_wlt_addr)?];

    if available_funds > tx_amount {
        let change = available_funds - tx_amount;
        debug!(
            "Creating change output: {} to {}",
            change,
            from_wlt_addr.as_str()
        );
        // Change output back to the sender.
        // Why: UTXO spends consume whole outputs; any remainder must be re-created as a new output.
        outputs.push(TXOutput::new(change, from_wlt_addr)?);
    }

    // Create a new transaction with the spent inputs and unspent outputs
    let mut tx = Transaction {
        id: vec![],
        vin: inputs,
        vout: outputs,
    };
    // Compute txid (hash of serialized tx with id empty in hash()).
    // Why: the txid uniquely identifies this transaction and is referenced by future inputs.
    tx.id = tx.hash()?;
    debug!(
        "Created transaction with {} inputs and {} outputs",
        tx.get_vin().len(),
        tx.get_vout().len()
    );
    // Sign each input against the referenced previous output's pubkey-hash.
    // Why: without signatures, anyone could claim someone else's UTXOs.
    tx.sign(utxo_set.get_blockchain(), from_wallet.get_pkcs8())
        .await?;
    Ok(tx)
}
```

**What to notice**

- Inputs reference previous outputs by \((txid, vout)\) and carry the sender’s `pub_key`.
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

```
tx arrives -> add_to_memory_pool(tx)
  for each input (txid, vout):
    load Vec<TXOutput> for txid
    set Vec[vout].in_global_mem_pool = true
    write Vec back
```

This diagram explains our simplified mempool integration: when a transaction is pending, we **mark the exact referenced outpoints as “reserved”** by flipping `in_global_mem_pool` on the corresponding `TXOutput`. The “why” is to reduce local races where two concurrent transaction builders pick the same coin; confirmation still happens only when a block is connected.

### Code Listing 2.4-4.5 — `UTXOSet::set_global_mem_pool_flag`

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn set_global_mem_pool_flag(&self, tx: &Transaction, flag: bool) -> Result<()> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

    if !tx.is_coinbase() {
        // Coinbase transactions dont have inputs.
        // Why: there are no referenced outpoints to reserve/unreserve.
        for curr_tx_inpt in tx.get_vin() {
            // Find the stored Vec<TXOutput> for the txid this input spends from.
            if let Some(curr_tx_inpt_utxo_ivec) = utxo_tree
                .get(curr_tx_inpt.get_txid())
                .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
            {
                let mut curr_tx_inpt_utxo_list: Vec<TXOutput> =
                    bincode::serde::decode_from_slice(
                        curr_tx_inpt_utxo_ivec.as_ref(),
                        bincode::config::standard(),
                    )
                    .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
                    .0;
                for (utxo_curr_utxo_idx, db_curr_utxo) in
                    curr_tx_inpt_utxo_list.iter_mut().enumerate()
                {
                    if utxo_curr_utxo_idx == curr_tx_inpt.get_vout() {
                        // Flip the "reserved" flag for this exact outpoint.
                        // Why: a learning-project guard against selecting the same coin twice while a tx is pending.
                        db_curr_utxo.set_in_global_mem_pool(flag);
                        trace!("\n");
                        trace!("------------------------------------------------------");
                        debug!("Set TXOUT to Intransit");
                        trace!("utxo_curr_utxo_idx: {:?}", utxo_curr_utxo_idx);
                        trace!("db_curr_utxo.get_value(): {:?}", db_curr_utxo.get_value());
                        for tx_out in tx.get_vout() {
                            trace!("tx_out.get_value(): {:?}", tx_out.get_value());
                        }
                        trace!("------------------------------------------------------");
                    }
                }
                trace!("Update UTXO in DB");
                let outs_bytes = bincode::serde::encode_to_vec(
                    &curr_tx_inpt_utxo_list,
                    bincode::config::standard(),
                )
                .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
                // Write the mutated Vec<TXOutput> back under the same txid key.
                // Why: `sled` stores bytes; we must persist the updated reservation state.
                utxo_tree
                    .insert(curr_tx_inpt.get_txid(), outs_bytes)
                    .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
            } else {
                debug!("TXOUT not found in DB");
            }
        }
    }
    Ok(())
}
```

### Code Listing 2.4-4.6 — Mempool call sites (`add_to_memory_pool` / `remove_from_memory_pool`)

```rust
// Source: bitcoin/src/node/txmempool.rs
pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain_service: &BlockchainService,
) -> Result<()> {
    debug!("\n");
    debug!(
        "******************************************************************************************************"
    );
    debug!(
        "Adding transaction to memory pool: {:?}",
        tx.get_tx_id_hex()
    );
    debug!(
        "******************************************************************************************************\n"
    );
    GLOBAL_MEMORY_POOL
        .add(tx.clone())
        .expect("Memory pool add error");

    let utxo_set = UTXOSet::new(blockchain_service.clone());
    // Reserve all referenced outpoints so our coin selection won't pick them again.
    utxo_set.set_global_mem_pool_flag(&tx.clone(), true).await?;

    Ok(())
}

pub async fn remove_from_memory_pool(tx: Transaction, blockchain: &BlockchainService) {
    GLOBAL_MEMORY_POOL
        .remove(tx.clone())
        .expect("Memory pool remove error");

    let utxo_set = UTXOSet::new(blockchain.clone());
    // Un-reserve outpoints when the tx leaves the mempool.
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), false)
        .await
        .expect("Failed to get blockchain");
}
```

### Code Listing 2.4-4.6b — Mempool existence check (`transaction_exists_in_pool`)

```rust
// Source: bitcoin/src/node/txmempool.rs
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

```
for each tx in block:
  if not coinbase:
    for each input (txid, vout):
      load Vec<TXOutput> for txid
      remove the output at index vout (by filtering it out)
      write back Vec, or delete key if empty
  insert tx's own outputs under key = tx.id
```

This diagram is the core state transition: connecting a block **spends** previously-unspent outputs (remove referenced outpoints) and **creates** new unspent outputs (insert each tx’s outputs under its txid). This is why the UTXO set is a “current state” database: it’s updated incrementally as blocks are accepted.

### Code Listing 2.4-4.7 — `UTXOSet::update`

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn update(&self, block: &Block) -> Result<()> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
    for curr_block_tx in block.get_transactions().await? {
        // Coinbase transactions dont have inputs
        if !curr_block_tx.is_coinbase() {
            for curr_blc_tx_inpt in curr_block_tx.get_vin() {
                let mut updated_outs = vec![];
                // Load the current unspent outputs for the txid this input spends from.
                let curr_blc_tx_inpt_utxo_ivec = utxo_tree
                    .get(curr_blc_tx_inpt.get_txid())
                    .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                    .ok_or(BtcError::UTXONotFoundError(format!(
                        "(update) UTXO {} not found",
                        curr_blc_tx_inpt.get_input_tx_id_hex()
                    )))?;
                // Decode the full output list for that txid so we can remove the spent vout.
                let curr_blc_tx_inpt_utxo_list: Vec<TXOutput> =
                    bincode::serde::decode_from_slice(
                        curr_blc_tx_inpt_utxo_ivec.as_ref(),
                        bincode::config::standard(),
                    )
                    .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
                    .0;
                for (utxo_curr_utxo_idx, db_curr_utxo) in
                    curr_blc_tx_inpt_utxo_list.iter().enumerate()
                {
                    if utxo_curr_utxo_idx != curr_blc_tx_inpt.get_vout() {
                        updated_outs.push(db_curr_utxo.clone())
                    }
                }
                if updated_outs.is_empty() {
                    // If that transaction has no remaining unspent outputs, delete the key.
                    // Why: no point storing an empty Vec; deleting also makes "not found" a clear signal.
                    utxo_tree
                        .remove(curr_blc_tx_inpt.get_txid())
                        .map_err(|e| BtcError::RemovingUTXOError(e.to_string()))?;
                } else {
                    // Otherwise, write back the filtered Vec<TXOutput>.
                    // Why: this is the "spend" part of state transition for this input.
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
        // Insert the newly created outputs of this transaction as unspent.
        // Why: new outputs become future spendable coins once the block is accepted.
        let mut new_outputs = vec![];
        for curr_tx_out in curr_block_tx.get_vout() {
            new_outputs.push(curr_tx_out.clone())
        }
        let outs_bytes =
            bincode::serde::encode_to_vec(&new_outputs, bincode::config::standard())
                .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
        let _ = utxo_tree
            .insert(curr_block_tx.get_id(), outs_bytes) // key = current txid, value = its outputs
            .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
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

### Code Listing 2.4-4.8 — `UTXOSet::reindex`

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn reindex(&self) -> Result<()> {
    debug!("Starting UTXOSet reindex...");
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
    utxo_tree
        .clear()
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

    // Re-derive the authoritative "unspent outputs map" from the chain history.
    // Why: the chain is the source of truth; the UTXO set is cached derived state.
    let utxo_map = self.blockchain.find_utxo().await?;
    debug!("Found {} transactions with UTXOs", utxo_map.len());

    for (txid_hex, outs) in &utxo_map {
        debug!(
            "Processing transaction {} with {} outputs",
            txid_hex,
            outs.len()
        );
        // Convert txid from hex string back into raw bytes for the sled key.
        let txid = HEXLOWER
            .decode(txid_hex.as_bytes())
            .map_err(|e| BtcError::TransactionIdHexDecodingError(e.to_string()))?;
        // Serialize Vec<TXOutput> as the sled value.
        let value = bincode::serde::encode_to_vec(outs, bincode::config::standard())
            .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
        let _ = utxo_tree
            .insert(txid.as_slice(), value)
            .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
    }
    debug!("UTXOSet reindex completed");
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

**Note about transaction order**: the method’s internal comment says “reverse order”, but the actual order depends on what `block.get_transactions()` returns. The important takeaway is *what it removes/restores*, not the precise iteration order in this learning implementation.

### Code Listing 2.4-4.9 — `UTXOSet::rollback_block`

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn rollback_block(&self, block: &Block) -> Result<()> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

    // Process transactions in reverse order (newest first)
    for curr_block_tx in block.get_transactions().await? {
        // Step 1: Remove this transaction's outputs from UTXO set
        // Why: we're undoing the "create outputs" part of `update(block)`.
        utxo_tree
            .remove(curr_block_tx.get_id())
            .map_err(|e| BtcError::RemovingUTXOError(e.to_string()))?;

        // Step 2: Restore the inputs that this transaction spent (unless coinbase)
        // When a transaction is processed, its inputs are removed from UTXO set.
        // When rolling back, we need to restore those inputs as unspent outputs.
        if !curr_block_tx.is_coinbase() {
            for curr_blc_tx_inpt in curr_block_tx.get_vin() {
                // Get the transaction that this input references
                if let Some(input_tx) = self
                    .blockchain
                    .find_transaction(curr_blc_tx_inpt.get_txid())
                    .await?
                {
                    // Find the specific output that was spent and restore it
                    if let Some(output) = input_tx.get_vout().get(curr_blc_tx_inpt.get_vout()) {
                        // Prepare to restore this output as a UTXO
                        let mut outs_to_restore = vec![];

                        // Check if this transaction already has other unspent outputs
                        // If so, we need to merge the `outs_to_restore` with existing ones
                        // This is because when a transaction is processed, its outputs are removed from UTXO set.
                        // When rolling back, we need to restore those outputs as unspent outputs.
                        // If the transaction already has other unspent outputs, we need to merge the restored output with existing ones.
                        // This is because the restored output is the same as the existing output.
                        if let Some(existing_outs_bytes) = utxo_tree
                            .get(curr_blc_tx_inpt.get_txid())
                            .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                        {
                            // Deserialize existing outputs for this transaction
                            let mut existing_outs: Vec<TXOutput> =
                                bincode::serde::decode_from_slice(
                                    existing_outs_bytes.as_ref(),
                                    bincode::config::standard(),
                                )
                                .map_err(|e| {
                                    BtcError::TransactionDeserializationError(e.to_string())
                                })?
                                .0;

                            // Insert the restored output at the correct position (vout index)
                            existing_outs.insert(curr_blc_tx_inpt.get_vout(), output.clone());
                            outs_to_restore = existing_outs;
                        }

                        // Save the restored UTXOs back to the database
                        let outs_bytes = bincode::serde::encode_to_vec(
                            &outs_to_restore,
                            bincode::config::standard(),
                        )
                        .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;

                        utxo_tree
                            .insert(curr_blc_tx_inpt.get_txid(), outs_bytes)
                            .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                    }
                }
            }
        }
    }

    Ok(())
}
```

**Checkpoint**

- What rollback removes (outputs created by the block).
- What rollback tries to restore (the previously-spent outputs referenced by the block’s inputs).

---

## Balance and UTXO counts

Balances are derived from UTXOs. In our codebase, balance queries are “scan the UTXO tree for outputs locked to this address and sum them”.

It’s worth pausing on *why* we read balance from the **UTXO set** rather than the **blockchain**. The blockchain is the permanent diary of *everything that ever happened*; it’s not optimized for answering “what is true right now?” To compute a balance from the chain directly, you would have to replay transactions and keep track of which outputs have already been spent—every time you ask the question.

The UTXO set exists so the node can keep a **ready-to-use view of current spendability**: it is updated once per connected block, and then used to answer balance and input-selection queries cheaply. And because it is derived from the chain, it can always be rebuilt when needed.

### Code Listing 2.4-4.10 — `UTXOSet::find_utxo` (fetch all spendable outputs for a key-hash)

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn find_utxo(&self, pub_key_hash: &[u8]) -> Result<Vec<TXOutput>> {
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
    let mut utxos = vec![];
    let mut total_items = 0;

    for item in utxo_tree.iter() {
        let (k, v) = item.map_err(|e| BtcError::GettingUTXOError(e.to_string()))?;
        total_items += 1;
        let txid_hex = HEXLOWER.encode(&k);
        debug!("Checking UTXO tree item: {}", txid_hex);

        // Each value is a Vec<TXOutput> for a given txid.
        let outs: Vec<TXOutput> = bincode::serde::decode_from_slice(
            v.to_vec().as_slice(),
            bincode::config::standard(),
        )
        .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?
        .0;

        debug!("Transaction {} has {} outputs", txid_hex, outs.len());
        for (idx, out) in outs.iter().enumerate() {
            debug!(
                "Output {}: value = {}, checking if locked with key",
                idx,
                out.get_value()
            );
            if out.is_locked_with_key(pub_key_hash) {
                debug!("Found matching UTXO: value = {}", out.get_value());
                utxos.push(out.clone())
            }
        }
    }
    debug!(
        "UTXO tree has {} total items, found {} matching UTXOs",
        total_items,
        utxos.len()
    );
    Ok(utxos)
}
```

### Code Listing 2.4-4.11 — `UTXOSet::get_balance` and `UTXOSet::utxo_count`

```rust
// Source: bitcoin/src/chain/utxo_set.rs
pub async fn get_balance(&self, wlt_address: &WalletAddress) -> Result<i32> {
    // Convert a user-facing address into the pubkey-hash our UTXO outputs are locked to.
    // Why: addresses are for humans; `pub_key_hash` is what the UTXO set can match.
    let pub_key_hash = get_pub_key_hash(wlt_address)?;
    debug!("Getting balance for address: {}", wlt_address.as_str());
    debug!("Public key hash: {:?}", pub_key_hash);

    // Gather all outputs locked to this key-hash and sum them.
    let utxos = self.find_utxo(pub_key_hash.as_slice()).await?;
    debug!(
        "Found {} UTXOs for address {}",
        utxos.len(),
        wlt_address.as_str()
    );

    let mut balance = 0;
    for (idx, utxo) in utxos.iter().enumerate() {
        debug!("UTXO {}: value = {}", idx, utxo.get_value());
        balance += utxo.get_value();
    }
    debug!("Total balance for {}: {}", wlt_address.as_str(), balance);
    Ok(balance)
}

pub async fn utxo_count(&self, wlt_address: &WalletAddress) -> Result<usize> {
    let pub_key_hash = get_pub_key_hash(wlt_address)?;
    debug!("Getting balance for address: {}", wlt_address.as_str());
    debug!("Public key hash: {:?}", pub_key_hash);

    // Count UTXOs (useful for UX/debugging; balance alone hides UTXO fragmentation).
    let count = self.find_utxo(pub_key_hash.as_slice()).await?.len();

    debug!("Total count for {}: {}", wlt_address.as_str(), count);
    Ok(count)
}
```

**Checkpoint**

- How “balance” is computed (sum of values of outputs locked to your pubkey-hash).
- Why this implementation is \(O(N)\) over the UTXO DB (full scan), and what an optimization would look like (secondary indexes).

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

3. **Rollback edge cases**
   - `rollback_block()` restores spent outputs by fetching the *previous transaction* from the chain and reinserting the referenced output.
   - As currently written, if there is **no existing `Vec<TXOutput>` for that `txid`** in the UTXO tree, the function does not populate `outs_to_restore` before writing it back. In other words: it’s a helpful learning scaffold, but not a full reorg-safe implementation yet.

4. **Performance characteristics**
   - `find_spendable_outputs()` and `find_utxo()` scan the entire UTXO tree. This is fine for a learning project, but real wallets/indexers maintain additional indexes for efficient lookup.

---

## Navigation

- **Previous**: Section 2.4.3 (Chain State and Storage)
- **Next**: Section 2.4.5 (Transaction Lifecycle)

---

<div align="center">

**📚 [← Previous: Chain State and Storage](03-Chain-State-and-Storage.md)** | **UTXO Set** | **[Next: Transaction Lifecycle →](05-Transaction-Lifecycle.md)** 📚

</div>

