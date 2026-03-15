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

# Section 2.4.1: Domain Model — Blocks, Transactions, and the UTXO Worldview

In this section, we will walk through the project’s Rust “model classes” (Rust types/structs) that represent Bitcoin’s core domain objects. Our goal is to confidently answer “what does this type *mean* in whitepaper terms?” and “where is it used in the implementation?”

As we walk through the code in this section, use these files as our primary code walkthrough:
- `bitcoin/src/primitives/block.rs`
- `bitcoin/src/primitives/transaction.rs`
- `bitcoin/src/chain/utxo_set.rs` (for how outputs become “spendable”)

## Table of Contents

1. [Whitepaper Term → Rust Type Map](#whitepaper-term-rust-type-map)
2. [Domain object diagrams](#domain-object-diagrams)
3. [Key code reading checklist](#key-code-reading-checklist)
4. [Step-by-step code walkthrough](#step-by-step-code-walkthrough)

---

<a id="whitepaper-term-rust-type-map"></a>
## Whitepaper Term → Rust Type Map

```
Whitepaper Concept             Rust Type / Field
--------------------------------------------------------------
Block header                   BlockHeader
Previous block hash            BlockHeader.pre_block_hash
Nonce                          BlockHeader.nonce
Block hash                     BlockHeader.hash
Transaction                    Transaction
Transaction ID                 Transaction.id
Input (spend)                  TXInput
Output (new coin / change)     TXOutput
Locking condition              TXOutput.pub_key_hash
Signature (authorization)      TXInput.signature
```

<a id="domain-object-diagrams"></a>
## Domain object diagrams

The diagram below shows the *runtime relationship* between the main “state buckets”:

- the **blockchain** (append-only history),
- the **UTXO set** (derived “what is spendable right now” state),
- and **transactions** (which consume old outputs and create new ones).

```
┌──────────────────────────────────────────────────────────┐
│                      Blockchain                          │
│  ┌─────────────────────────────────────────────────────┐ │
│  │  Block 0 (Genesis)                                  │ │
│  │    └─> Block 1                                      │ │
│  │         └─> Block 2                                 │ │
│  │              └─> Block N (Tip)                      │ │
│  └─────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
                            │
                            │ maintains derived state
                            ▼
                    ┌───────────────┐
                    │    UTXOSet    │
                    │               │
                    │  (txid,vout)  │ -> TXOutput
                    │  (txid,vout)  │ -> TXOutput
                    │  ...          │
                    └───────────────┘

┌───────────────────────────────────────────────────────────┐
│                         Block                             │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  BlockHeader                                        │  │
│  │    - Previous Hash                                  │  │
│  │    - Timestamp                                      │  │
│  │    - Nonce                                          │  │
│  │    - Height                                         │  │
│  │    - (Bitcoin: Merkle root; this implementation:    │  │
│  │       `Block::hash_transactions()` simplification)  │  │
│  └─────────────────────────────────────────────────────┘  │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  Transactions                                       │  │
│  │    ├─> Transaction 1                                │  │
│  │    ├─> Transaction 2                                │  │
│  │    └─> Transaction N                                │  │
│  └─────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│                      Transaction                         │
│  ┌────────────────────────────────────────────────────┐  │
│  │  TXInputs (spends previous outputs)                │  │
│  │    ├─> Input 1: references (prev_txid, vout)       │  │
│  │    └─> Input 2: references (prev_txid, vout)       │  │
│  └────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────┐  │
│  │  TXOutputs (creates new outputs)                   │  │
│  │    ├─> Output 1: becomes a future UTXO             │  │
│  │    └─> Output 2: becomes a future UTXO             │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

<a id="key-code-reading-checklist"></a>
## Key code reading checklist

As we follow the Rust code, we should be able to answer these questions from the structs alone:

- Where does the txid come from? (Search for `Transaction::hash()`.)
- Where does the signature live? (Look at `TXInput.signature`.)
- What does an output lock to? (`TXOutput.pub_key_hash`.)
- Where is “unspent vs spent” tracked? (Not on the output — in `UTXOSet`.)

---

<a id="step-by-step-code-walkthrough"></a>
## Step-by-step code walkthrough

**Goal**: build a correct mental model of what the core structs *mean*, and how the bytes flow between modules.

**Code walkthrough**:
- `bitcoin/src/primitives/block.rs`
- `bitcoin/src/primitives/transaction.rs`
- `bitcoin/src/chain/utxo_set.rs`

**Whitepaper anchors**:
- Section 2 (Transactions)
- Section 4 (Proof-of-Work)
- Section 5 (Network operation, “accept only if…”)

### Step 1(Block and BlockHeader) — Identify what makes a “block header” in our implementation (`Block::new_block`)

**Block domain model code**: `bitcoin/src/primitives/block.rs`

In the Bitcoin whitepaper, a block is a **timestamped commitment** to two things:

- **The previous block** (so the history is tamper-evident by hash chaining).
- **A set of transactions** (so the block “anchors” ownership transfers into the history).

In our implementation, `BlockHeader` is the smallest header we need to make those ideas concrete in code:

In practice, the header matters because it defines the **proof-of-work input**: mining repeatedly hashes a byte sequence derived from the header (plus a transaction commitment) until it finds a valid `nonce`.

Those bytes are assembled from the header fields (plus a transaction commitment) in `ProofOfWork::prepare_data(...)`, which is why `BlockHeader` contains exactly the fields it does. We cover the proof-of-work loop and the exact hashed byte layout in Section 2.4.6 (Block Lifecycle and Mining).

**Struct layout (simplified):**

```rust
pub struct BlockHeader {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    nonce: i64,
    height: usize,
}

pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}
```

**What the fields mean**:
- **`pre_block_hash`**: link to the previous block (whitepaper Section 3/4).
- **`nonce` + `hash`**: PoW search result (whitepaper Section 4).
- **`timestamp`**: ordering signal; also part of the PoW header bytes.
- **`height`**: convenience for iteration/selection; not a Bitcoin header field.

**Project-specific note (important)**:
- In our project implementation we do not use a **Merkle root** field. We instead concatenate tx IDs and hash once using `Block::hash_transactions()`. This keeps the code readable but does not support Merkle proofs / SPV.

**Creating a new block**:
In our implementation, `Block::new_block(pre_block_hash, transactions, height)` is the entry point that *defines* the header boundary: it takes the parent link (`pre_block_hash`) and the transaction list (`transactions`) as inputs, sets derived header fields such as `timestamp`, and then finalizes `nonce` and `hash` by running proof-of-work.

```rust
pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
    let header = BlockHeader {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        hash: String::new(), // to be filled in the next step
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
```

- **What to notice**
  - `pre_block_hash` is the chain link (what we “commit to” from the previous block).
  - `nonce` and `hash` are filled by PoW (`ProofOfWork::run()`), not supplied by the caller.
  - `height` lives in the header as a convenience (not a Bitcoin header field).
- **Whitepaper mapping**
  - **§3/§4**: blocks link to previous hashes and are mined by searching for a PoW solution.
  - **§5**: once mined/received, a block is the unit that may become part of the canonical history.

### Step 2(Transaction, Inputs, and Outputs) — Define the transaction shape (the UTXO worldview is encoded in the fields)

Bitcoin is not an “account system.” It is a system of **outputs**, and spending means consuming previous outputs as inputs.

In this step, we do two things:

1. We define what a transaction *is* by reading the struct fields.
2. We show how a spending transaction is *constructed* in code (UTXO selection → inputs/outputs → txid → sign).

**Conceptual intent**:
- `TXInput.(txid, vout)` points to a specific previous output (which output are we spending?).
- `TXInput.signature` proves authorization to spend that referenced output.
- `TXInput.pub_key` is the public key used during signature verification.
- `TXOutput.pub_key_hash` is the “locking condition” in this simplified design (who can spend it).

**Project-specific note**:
- Signatures are stored **per input** (`TXInput.signature`), not on the transaction struct.
- “Scripts” are not implemented; the output is locked by `pub_key_hash` only.

**Transaction domain model code**: `bitcoin/src/primitives/transaction.rs`

In this section, “surface area” simply means: **the set of fields exposed by the `Transaction`, `TXInput`, and `TXOutput` types**. Those fields are the contract the rest of the system relies on.

They also reveal the project’s UTXO worldview: a transaction spends prior outputs by referencing an outpoint \((txid, vout)\) in `TXInput`, and it creates new spendable outputs in `TXOutput`.

These structs define what a transaction *is* in this codebase.

```rust
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    in_global_mem_pool: bool,
    pub_key_hash: Vec<u8>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}
```

- **What to notice**
  - A spend is identified by an outpoint \((txid, vout)\): `TXInput.{txid, vout}`.
  - Authorization is per-input: `TXInput.signature` + `TXInput.pub_key`.
  - Outputs are locked by a pubkey-hash: `TXOutput.pub_key_hash` (no script engine here).
- **Whitepaper mapping**
  - **§2**: transactions spend prior outputs and create new outputs; signatures enforce ownership transfer.

**How a spending transaction is constructed in this codebase (high-level)**:

1. **Select spendable outputs**: query the UTXO set for outpoints locked to the sender and accumulate value until we cover the target amount.
2. **Create inputs**: for each selected outpoint \((txid, vout)\), create `TXInput { txid, vout, signature: [], pub_key: sender_pub_key }`.
3. **Create outputs**: create a payment output to the recipient, and (if needed) a **change** output back to the sender.
4. **Compute the txid**: hash a serialized copy with `id = []` (so the definition is not circular), then store the digest into `Transaction.id`.
5. **Sign**: produce per-input signatures over a trimmed copy and store them into `TXInput.signature`.

In the project, the concrete entry point for this flow is `Transaction::new_utxo_transaction(...)` (we read the full implementation in Section 2.4.5: Transaction Lifecycle).

**Code Listing 2.4-1.2: Constructing a UTXO spending transaction (selection → change → txid → sign)**  
Source: `bitcoin/src/primitives/transaction.rs`

```rust
pub async fn new_utxo_transaction(
    from_wlt_addr: &WalletAddress,
    to_wlt_addr: &WalletAddress,
    tx_amount: i32,
    utxo_set: &UTXOSet,
) -> Result<Transaction> {
    // 0) Load the sender’s wallet material (pubkey, private key bytes)
    let wallets = WalletService::new()?;
    let from_wallet = wallets
        .get_wallet(from_wlt_addr)
        .ok_or_else(|| BtcError::UTXONotFoundError(from_wlt_addr.as_string()))?;

    // 1) Select spendable outputs (outpoints) from the UTXO set for this sender
    let from_public_key_hash = hash_pub_key(from_wallet.get_public_key());
    let (available_funds, valid_outputs) = utxo_set
        .find_spendable_outputs(from_public_key_hash.as_slice(), tx_amount)
        .await?;
    if available_funds < tx_amount {
        return Err(BtcError::NotEnoughFunds);
    }

    // 2) Create inputs from selected outpoints (txid, vout)
    let mut inputs = vec![];
    for (txid_hex, out_indexes) in valid_outputs {
        let txid = HEXLOWER
            .decode(txid_hex.as_bytes())
            .map_err(|e| BtcError::TransactionIdHexDecodingError(e.to_string()))?;
        for current_out_index in out_indexes {
            inputs.push(TXInput {
                txid: txid.clone(),
                vout: current_out_index,
                signature: vec![], // filled after we sign
                pub_key: from_wallet.get_public_key().to_vec(),
            });
        }
    }

    // 3) Create outputs: payment + (optional) change back to the sender
    let mut outputs = vec![TXOutput::new(tx_amount, to_wlt_addr)?];
    if available_funds > tx_amount {
        let change = available_funds - tx_amount;
        outputs.push(TXOutput::new(change, from_wlt_addr)?);
    }

    // 4) Compute the transaction ID (txid) from a copy with id = [] and store it
    let mut tx = Transaction {
        id: vec![],
        vin: inputs,
        vout: outputs,
    };
    tx.id = tx.hash()?;

    // 5) Sign: produce per-input signatures (trimmed copy → hash → signature) and store them
    tx.sign(utxo_set.get_blockchain(), from_wallet.get_pkcs8()).await?;
    Ok(tx)
}
```

**Code Listing 2.4-1.3: Computing the txid (hash a copy with `id = []`)**  
Source: `bitcoin/src/primitives/transaction.rs`

```rust
fn hash(&mut self) -> Result<Vec<u8>> {
    // IMPORTANT: do not include the transaction’s own id in the bytes we hash,
    // otherwise the definition becomes circular (“the id depends on itself”).
    let tx_copy = Transaction {
        id: vec![],
        vin: self.vin.clone(),
        vout: self.vout.clone(),
    };
    Ok(sha256_digest(tx_copy.serialize()?.as_slice()))
}
```

### Step 3 — Separate “history” (blocks) from “derived state” (UTXO set)

**UTXO (Unspent Transaction Output)**: a UTXO is a specific transaction output that is currently spendable. In other words, it is an output identified by an outpoint \((txid, vout)\) that has **not** been consumed by any later transaction input.

**How this differs from the blockchain (and why it is separate)**:

- The **blockchain** is the append-only history (blocks and the transactions they contain).
- The **UTXO set** is derived state: a compact, query-friendly view of “what is spendable right now?” computed from that history.

We keep the UTXO set separate so the node can answer spendability questions efficiently (and enforce “not already spent”) without rescanning the entire chain every time it validates a transaction or connects a block.

**UTXO set domain model code**: `bitcoin/src/chain/utxo_set.rs`

Blocks are the append-only log. “Is this spendable right now?” is derived state stored separately.

The UTXO set is stored in a sled tree called `"chainstate"`:

```rust
const UTXO_TREE: &str = "chainstate";

pub struct UTXOSet {
    blockchain: BlockchainService,
}

impl UTXOSet {
    pub fn new(blockchain: BlockchainService) -> UTXOSet {
        UTXOSet { blockchain }
    }
}
```

- **What to notice**
  - Spendability is not a field on `TXOutput`; it’s tracked by membership in the UTXO DB.
- **Whitepaper mapping**
  - **§5 (Step 5)**: “not already spent” is a stateful check (we need a UTXO view to answer it).

### Step 4 — Trace one meaningful query end-to-end: selecting spendable outputs

**UTXO selection code**: `bitcoin/src/chain/utxo_set.rs`

This function turns the abstract model (“I own some outputs”) into concrete outpoints to spend.

```rust
pub async fn find_spendable_outputs(
    &self,
    from_pub_key_hash: &[u8],
    amount: i32,
) -> Result<(i32, HashMap<String, Vec<usize>>)> {
    debug!("Finding spendable outputs for amount: {}", amount);

    // We return two things:
    // 1) the total value we managed to accumulate (so the caller can decide if it needs “change”)
    // 2) the concrete outpoints to spend, encoded as: txid_hex -> [vout indexes]
    let mut unspent_outputs_indexes: HashMap<String, Vec<usize>> = HashMap::new();
    let mut accmulated = 0;

    // Open the embedded DB handle behind the chain façade, then open the UTXO tree.
    // Each key is a transaction id (txid) and each value encodes that transaction’s unspent outputs.
    let db = self.blockchain.get_db().await?;
    let utxo_tree = db
        .open_tree(UTXO_TREE)
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
    let mut total_checked = 0;

    // Scan the UTXO set and greedily accumulate spendable outputs until we reach `amount`.
    for item in utxo_tree.iter() {
        let (k, v) = item.map_err(|e| BtcError::GettingUTXOError(e.to_string()))?;

        // `k` is the txid (binary); we convert it to hex because the output map uses String keys.
        let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());

        // `v` stores a serialized Vec<TXOutput> for that txid (plus some metadata we ignore here).
        // We deserialize it so we can inspect each candidate output.
        let (tx_out, _): (Vec<TXOutput>, usize) = bincode::serde::decode_from_slice(
            v.to_vec().as_slice(),
            bincode::config::standard(),
        )
        .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?;

        for (current_out_index, out) in tx_out.iter().enumerate() {
            total_checked += 1;
            debug!(
                "Checking output {} in tx {}: value={}, in_mempool={}, locked_with_key={}",
                current_out_index,
                txid_hex,
                out.get_value(),
                out.is_in_global_mem_pool(),
                out.is_locked_with_key(from_pub_key_hash)
            );
            // Selection predicate (our simplified “spendability” rule):
            // - not already being spent by a pending mempool transaction (project-specific safety)
            // - positive value
            // - locked to `from_pub_key_hash` (i.e., “owned” by the sender)
            // - keep accumulating until we reach the target amount
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
                // Record the outpoint index (vout) under the txid key.
                // Later, `Transaction::new_utxo_transaction(...)` turns these entries into TXInputs.
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
    }

    debug!(
        "find_spendable_outputs completed: checked {} outputs, accumulated={}, found {} spendable transactions",
        total_checked,
        accmulated,
        unspent_outputs_indexes.len()
    );

    // The caller uses:
    // - `accmulated` to decide if it must add a change output
    // - `unspent_outputs_indexes` to build TXInputs that reference (txid, vout)
    Ok((accmulated, unspent_outputs_indexes))
}
```

- **What to notice**
  - The selection predicate is explicit: owned-by-hash, not-in-mempool, positive value.
  - The return type (`txid_hex -> [vout indexes]`) is exactly what `Transaction::new_utxo_transaction(...)` consumes.
- **Whitepaper mapping**
  - **§2**: inputs must reference specific previous outputs.
  - **§5 (Step 5)**: “not already spent” requires an authoritative spendability view (the UTXO set).

### Step 5 — How txids are computed (what bytes become “the ID”?)

**Transaction ID (txid) code**: `bitcoin/src/primitives/transaction.rs`

In this implementation, a transaction’s “ID” is computed by hashing a serialized copy where `id` is empty (see **Code Listing 2.4-1.3** above). The important rule is: **the txid commits to `vin` and `vout`, not to the already-stored `id` field**, so the definition is not circular.
```rust
fn hash(&mut self) -> Result<Vec<u8>> {
    let tx_copy = Transaction {
        id: vec![],
        vin: self.vin.clone(),
        vout: self.vout.clone(),
    };
    Ok(sha256_digest(tx_copy.serialize()?.as_slice()))
}
```
- **What to notice**
  - The txid depends on the serialized `vin` and `vout` (but not on the existing `id` field).
  - This gives a stable internal identifier for this codebase, but it is not byte-for-byte identical to Bitcoin Core’s txid/wtxid rules.
- **Whitepaper mapping**
  - **§2**: the whitepaper treats transactions as signed “messages” that are chained by reference; in production Bitcoin the exact txid definition is consensus-critical.

### Step 6 — Understand the Merkle simplification (what commits the block to its transactions?)

**Block transaction commitment code**: `bitcoin/src/primitives/block.rs`

Bitcoin commits to transactions via a Merkle root in the header. In our implementation, the block “transaction commitment” is simplified to a single hash of concatenated txids:

```rust
pub fn hash_transactions(&self) -> Vec<u8> {
    let mut txhashs = vec![];
    for transaction in &self.transactions {
        txhashs.extend(transaction.get_id());
    }
    crate::sha256_digest(txhashs.as_slice())
}
```

- **What to notice**
  - This is not a Merkle tree: it is a “Merkle-ish” commitment that trades correctness/features for readability.
  - It cannot produce Merkle branches, so it cannot support SPV-style inclusion proofs.
- **Whitepaper mapping**
  - **§3**: blocks are chained by hashing data that includes the previous hash.
  - **§7–§8**: Bitcoin’s Merkle root enables efficient proofs; this implementation simplifies that away.

### Step 7 — See where coinbase fits the domain model (`Transaction::new_coinbase_tx`)

**Coinbase transaction code**: `bitcoin/src/primitives/transaction.rs`

Coinbase is the “special first transaction” concept from the whitepaper. Here it is created as a transaction with a special input and a fixed subsidy output:

```rust
pub fn new_coinbase_tx(to: &WalletAddress) -> Result<Transaction> {
    let txout = TXOutput::new(SUBSIDY, to)?;
    let tx_input = TXInput {
        signature: Uuid::new_v4().as_bytes().to_vec(),
        ..Default::default()
    };

    let mut tx = Transaction {
        id: vec![],
        vin: vec![tx_input],
        vout: vec![txout],
    };

    tx.id = tx.hash()?;
    Ok(tx)
}
```

- **What to notice**
  - Coinbase is identified structurally (see `Transaction::is_coinbase()` in code): it does not spend a real previous outpoint.
  - The output uses the same locking mechanism (`TXOutput::new(...)` / `pub_key_hash`) as normal outputs.
- **Whitepaper mapping**
  - **§6**: incentive mechanism (coinbase) is why miners invest work; it is also why chain state grows even if fees are ignored in a teaching implementation.

---

## Navigation

- **Previous**: Section 2.4 (Technical Foundations overview)
- **Next**: Section 2.4.2 (Blockchain State Management)

---

<div align="center">

**📚 [← Previous: Technical Foundations](README.md)** | **Domain Model** | **[Next: Blockchain State Management →](02-Blockchain-State-Management.md)** 📚

</div>

