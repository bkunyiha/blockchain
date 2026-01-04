<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../net/README.md) - Peer-to-peer networking and protocol
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
# Business objects (Rust implementation map)

## 0.2. Core objects (the “business types”)

These are the minimum objects implied by the whitepaper’s model, expressed in a way that is **closer to real Bitcoin** (fields you’ll see in Bitcoin Core and other implementations).
Think of them as “the on-wire business objects”: these are the shapes you serialize, hash, sign, verify, and store.

As we read the structs below, keep one mental model in mind: **a transaction doesn’t “move coins”**. It **consumes** previously created outputs and **creates** new outputs. Everything else (txid, Merkle root, block hash, PoW) exists to make that rule globally auditable.

Before we zoom into transactions, we’ll start with the container they live in. A **blockchain** is a sequence of **interconnected blocks**, where each block points to the previous one by hash. A **block** is therefore the fundamental building block of the chain: it packages transactions and provides the header that links history and carries proof-of-work.



---
**`Block`**
```rust
#[derive(Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}
```
- **Block**: a single “ledger step” (or unit of a blockchain) we relay and validate as a unit.
  - **What it does**: stores a `Vec<Transaction>` (`txs`) plus a `BlockHeader` (`header`). In other words: the list of state transitions, plus the metadata we hash for chaining and proof-of-work.
  - **How it connects**:
    - `header.merkle_root` commits to `txs` (change any tx → Merkle root changes → header hash changes).
    - `header.prev_hash` links this block to the previous one (the chain).
  - **Why it matters**: accepting a block is accepting a state transition (UTXO updates) *anchored* by PoW.

---

**`BlockHeader`**
```rust
#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub version: i32,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,        // nTime
    pub difficulty_bits: u32,  // nBits (compact target encoding)
    pub nonce: u32,
}
```
- **BlockHeader**: the consensus-critical commitment we hash for PoW.
  - **What it does**: is the fixed-size “hash input” for consensus. We serialize these fields deterministically, hash them, and check the hash against the target implied by `difficulty_bits`.
  - **How it connects**:
    - `prev_hash` links to the previous header hash (timestamp chain).
    - `merkle_root` commits to the block’s transactions (via their txids).
  - **What we validate**:
    - header hash meets the target implied by `difficulty_bits`
    - `nonce` is the searched value that makes that predicate true

---

**`Transaction`**
```rust
#[derive(Clone, Debug)]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}
```
- **Transaction**: the atomic ledger transition inside a block.
  - **What it does**: stores a `Vec<TxIn>` and `Vec<TxOut>`. At validation time, the inputs reference existing UTXOs (by `OutPoint`) and the outputs become newly created UTXOs after we apply the transaction.
  - **How it connects**:
    - `inputs: Vec<TxIn>` reference existing outputs through `TxIn.previous_output: OutPoint`.
    - `outputs: Vec<TxOut>` become new UTXOs keyed by this transaction’s `txid` + output index.
  - **What we validate**:
    - each input is authorized against the referenced output’s lock
    - each referenced outpoint is unspent in our UTXO view (“not already spent”)
    - values conserve (inputs ≥ outputs; fees are the difference)

---

**`TxOut`**
```rust
#[derive(Clone, Debug)]
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Vec<u8>, // “public key of the next owner” is represented by a locking script
}
```
- **TxOut**: spendable value plus a lock (“who may spend this?”).
  - **What it does**: stores the amount in smallest units (`value: u64`) and the locking program (`script_pubkey: Vec<u8>`) that defines the spending condition.
  - **How it connects**:
    - created by a `Transaction` as part of its outputs
    - later spent by a `TxIn` that references it via `OutPoint(prev_txid, vout)`

---

**`TxIn`**
```rust
#[derive(Clone, Debug)]
pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>, // legacy input script (P2PKH-style)
    pub sequence: u32,
}
```
- **TxIn**: a claim on an existing UTXO (which prior output we’re spending) plus unlocking data.
  - **What it does**:
    - names the spend target (`previous_output: OutPoint`)
    - stores the unlocking program (`script_sig: Vec<u8>`) that must satisfy the referenced output’s `script_pubkey`
  - **How it connects**:
    - `previous_output` selects the exact `TxOut` being spent
    - the selected `TxOut.script_pubkey` defines the verification rules for this input

---

**`OutPoint`**

An `OutPoint` is the canonical way Bitcoin names a specific spendable output: **(previous transaction id, output index)**.
```rust
#[derive(Clone, Debug)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}
```
- **OutPoint**: the portable identifier for a specific output (`txid` + output index).
  - **What it does**: is the exact “pointer” a `TxIn` uses to refer to a previous `TxOut`.
    - `txid: [u8; 32]` identifies *which* transaction created the output.
    - `vout: u32` selects *which output* inside that transaction.
  - **How it connects inputs to outputs (spends/payments)**:
    - Every non-coinbase `TxIn` spends value by setting `previous_output: OutPoint`.
    - During validation we use the OutPoint to look up the referenced `TxOut` in the UTXO view:
      - `OutPoint -> TxOut(value, script_pubkey)`
      - that `TxOut` provides the spend context: the amount and the locking condition the input must satisfy.
    - When the spend is accepted, we **remove** the `OutPoint` from the UTXO set (so it can’t be spent again), and the transaction’s new outputs become new spendable OutPoints (`(this_txid, 0..n)`).
  - **Key rule it enables**: “not already spent” becomes a simple check—if the `OutPoint` is not present in the UTXO view at this tip, the input is invalid (already spent or unknown).

---

**`UTXO`**

 A `UTXO` (“Unspent Transaction Output”) is a **currently spendable output** in our local view of the chain tip. In practice, we store the UTXO set as a **separate database/index** (derived state) rather than as part of the blockchain’s append-only block data.

```rust
use std::collections::HashMap;

pub type UtxoSet = HashMap<OutPoint, TxOut>;
```

- **UTXO / UTXO set**: the spendable-state database for validation.
  - **What it is (and why it’s separate from the blockchain)**:
    - The **blockchain** is the append-only history (blocks + transactions).
    - The **UTXO set** is *derived state*: a fast index of “what remains spendable” after applying that history up to a specific chain tip.
    - We keep it separate because scanning the entire chain for every input would be far too slow; with a UTXO set, validating “is this spendable?” is a direct lookup.
  - **When it changes (and why)**:
    - It changes whenever our selected chain tip changes, because it represents “what is spendable *at this tip*”.
    - **On block connect**: we remove spent `OutPoint`s (inputs) and insert new `OutPoint -> TxOut` entries (outputs) from the block’s transactions.
    - **On reorg/rollback**: we undo those changes for blocks that are disconnected and apply the new branch, so the UTXO set matches the new best chain.
  - **What it does**:
    - answers “what can be spent right now?” by mapping `OutPoint -> TxOut`
    - provides the spend context for validation: `TxOut.value` (amount) and `TxOut.script_pubkey` (locking condition)
  - **How it connects inputs to outputs (spends/payments)**:
    - on validation, each `TxIn.previous_output` must exist in the UTXO set (“not already spent”)
    - on apply, we remove spent outpoints and insert new ones created by the transaction’s outputs

---

Business view (UTXO flow):

```
# Legend (how to read this diagram):
# - OutPoint(X, i) = "output i created by transaction X" (txid + output index)
# - TxOut(value, lock=...) = amount + spending condition (lock is `script_pubkey`)
# - New outputs become new UTXOs keyed by OutPoint(this_txid, output_index)
# - Inputs - Outputs = fee (simplified; real Bitcoin has additional rules)

Before (UTXO set):
  OutPoint(A, 0) ──► TxOut(value=5, lock=Alice)
  OutPoint(B, 2) ──► TxOut(value=7, lock=Alice)
  (Alice controls two spendable outputs totaling 12)

Transaction (spend + create):
  inputs  : [OutPoint(A,0), OutPoint(B,2)]   // we are consuming these UTXOs
  outputs : [
    (value=9, lock=Bob),     // payment
    (value=2, lock=Alice),   // change back to sender
  ]
  fee     : 12 - (9 + 2) = 1

After (UTXO set update):
  - remove: OutPoint(A, 0), OutPoint(B, 2)         // spent (no longer spendable)
  + add   : OutPoint(TxID, 0) ──► TxOut(9, lock=Bob)
          OutPoint(TxID, 1) ──► TxOut(2, lock=Alice)  // change
```

What we implement from this diagram:

- **Lookups**: for each input, we fetch the referenced output from the UTXO set (`OutPoint -> TxOut`). If it’s missing, the input is invalid at this tip (“already spent” or unknown).
- **Authorization**: we validate that the input’s unlocking data satisfies the referenced output’s lock (`script_sig` vs `script_pubkey` in this simplified model).
- **Value rules**: we sum input values (from referenced `TxOut.value`) and compare to sum of output values.
  - outputs can include a **payment** and a **change** output back to the spender
  - the difference is the **fee** (if positive)
- **State update** (only after validation passes): remove spent outpoints; add new outpoints for each created output of this transaction (new txid + output index).


### 0.2.1 Object relationships (how the types fit together)

We can understand the same system from two directions. We include both models because they reinforce each other and help us implement the whitepaper concepts more reliably in Rust.

- **Model A (spend-first)**: starts from *spendable state* (UTXO set) and explains how `TxIn` spends a specific prior `TxOut` via `OutPoint`.
- **Model B (chain-first)**: starts from *consensus commitments* (headers + PoW) and explains how blocks commit to transactions and therefore to UTXO state transitions.

#### Model A (**spend-first**): follow value through the UTXO set

At the ledger level, Bitcoin is simple: **outputs create spendable value**, and **inputs spend a specific prior output**. Everything else exists to make those two statements enforceable and verifiable by independent nodes.

- **`TxOut` (output)**: creates spendable value with a locking condition (`script_pubkey`).
- **`TxIn` (input)**: spends value by referencing an `OutPoint(txid, vout)` and providing unlocking data (e.g., `script_sig`).
- **UTXO set (the spendable database)**: conceptually `HashMap<OutPoint, TxOut>`.
  - “not already spent” means: we refuse to spend any `OutPoint` that is not present in our UTXO view at the chosen chain tip.

```
TxIn  ──spends──►  OutPoint(txid, vout)  ──selects──►  TxOut(value, script_pubkey)

UTXO set (current spendable outputs):
  OutPoint(txid, vout)  ->  TxOut(...)
```

#### Model B (**chain-first**): follow what consensus secures

This model starts from the “thing we do proof-of-work on” and works downward into what it commits to.

- **`BlockHeader`** is what we hash for PoW, and it links to history via `prev_hash`.
- **`merkle_root`** is the header’s commitment to the block’s transaction list.
- **`Block`** is “header + transactions”; accepting a block means we accept a state transition.

```
BlockHeader(prev_hash, merkle_root, nBits, nonce)
   │
   ├─ prev_hash ──► previous BlockHeader (the chain)
   │
   └─ merkle_root ──► commits to txids of Block.txs
                         │
                         └─ Transaction: inputs spend OutPoints → select TxOuts → update UTXO set
```

If we keep both models in mind, it becomes clear how “money movement” (UTXO transitions) is anchored to “history” (headers + PoW + longest chain).

### 0.2.2 Whitepaper citations (business objects) + why we encode them this way in Rust

The whitepaper doesn’t list “Rust structs”, but it does define the *business objects* and their relationships. Below, each core type includes:

- **Whitepaper citation**: what the paper says (with a short quote and section)
- **Rust representation rationale**: why the encoding choices are practical for an interoperable implementation

#### `Transaction`
- **Whitepaper citation (Bitcoin Whitepaper Section 2)**:
  - “**We define an electronic coin as a chain of digital signatures.**” ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
  - “Each owner transfers the coin to the next by **digitally signing a hash of the previous transaction and the public key of the next owner** …” ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:
  - A `Transaction` is the **atomic state transition** of Bitcoin’s ledger.
  - It **consumes** previously created value by spending one or more `TxOut`s (via `TxIn.previous_output: OutPoint`), and it **creates** new value by producing new `TxOut`s.
  - Its job is to make “ownership transfer” verifiable:
    - each `TxIn` must be authorized relative to the referenced output’s locking condition
    - the transaction must not create money (value conservation; fees are the difference between inputs and outputs)
  - Relationship-wise, `Transaction` is the unit we relay, validate, and then commit to a block:
    - transactions are identified by a hash over canonical bytes (`txid`)
    - blocks commit to the set of txids via the Merkle root
- **Rust representation rationale**:
```rust
#[derive(Clone, Debug)]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}
```
  - `inputs: Vec<TxIn>` and `outputs: Vec<TxOut>` directly encode “spend previous / create new”.
  - `version`, `lock_time` are integers because consensus encoding must be deterministic and byte-for-byte reproducible across nodes.

#### `TxIn` (transaction input)
- **Whitepaper citation (Bitcoin Whitepaper Section 2)**:
  - The “chain of signatures” description implies every transfer must reference the *previous* transaction and prove authorization (“signing a hash of the previous transaction …”). ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:  
  - A `TxIn` is a **claim on an existing UTXO**: it names exactly which prior output we want to spend and provides data that proves we are allowed to spend it.
  - Its two core roles are:
    - **Reference**: `previous_output: OutPoint` identifies the UTXO being spent (`txid`, `vout`)
    - **Unlock**: `script_sig` (or witness in modern Bitcoin) provides signatures/keys that satisfy the referenced output’s locking script
  - Relationship-wise:
    - `TxIn` points to an `OutPoint`
    - the `OutPoint` selects a `TxOut`
    - the selected `TxOut.script_pubkey` defines what must be proven for this `TxIn` to be valid
- **Rust representation rationale**:
```rust
#[derive(Clone, Debug)]
pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>, // legacy input script (P2PKH-style)
    pub sequence: u32,
}
```
  - `previous_output: OutPoint` makes the spend target explicit as `(txid, vout)` (the canonical way inputs identify what they spend in real Bitcoin).
  - `script_sig: Vec<u8>` uses raw bytes because script/witness are consensus-serialized byte programs; keeping them as bytes avoids accidental re-encoding differences.
  - `sequence: u32` matches consensus-style fixed-width integer fields and supports canonical little-endian serialization.

#### `TxOut` (transaction output)
- **Whitepaper citation (Bitcoin Whitepaper Section 2)**:
  - The recipient is described as “the **public key of the next owner**” in the transfer rule. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:  
  - A `TxOut` is **spendable value plus a rule for spending it**.
  - It becomes part of the UTXO set when created, and it remains spendable until a future `TxIn` references it via an `OutPoint`.
  - Relationship-wise:
    - `TxOut` is produced by a `Transaction`
    - it can later be selected by `OutPoint(txid, vout)`
    - its `script_pubkey` defines the spending condition that the future `TxIn` must satisfy
- **Rust representation rationale**:
```rust
#[derive(Clone, Debug)]
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Vec<u8>, // “public key of the next owner” is represented by a locking script
}
```
  - `value: u64` is the standard “amount in smallest units” encoding (Bitcoin uses satoshis) to avoid floating-point ambiguity.
  - `script_pubkey: Vec<u8>` generalizes “public key of the next owner” into a *locking condition* (real Bitcoin uses Script; P2PKH/P2WPKH/etc. are all byte encodings).

#### `OutPoint`
The Bitcoin whitepaper does not define an OutPoint type or the (txid, vout) concept by name.
What it does specify is the underlying requirement:
- In Section 2 it describes **spending** as “signing a hash of the previous transaction…”, which implies an **input must reference a previous transaction**.
- In Section 5 it requires blocks be accepted only if transactions are “not already spent”, which implies nodes must identify which **prior output is being spent**.
**OutPoint** (txid, vout) is the concrete representation used by the deployed Bitcoin protocol to satisfy those requirements: 
- txid identifies the previous transaction,
- vout selects a specific output within it.

<pre style="white-space: pre; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace;">
TxOut (creates spendable value)
  ▲
  │ referenced by <span style="color: red;">OutPoint(txid, index)</span>
  │
TxIn (authorizes spending)  ──► Transaction ──► included in Block ──► committed by merkle_root in header
</pre>

- **Whitepaper citation (Bitcoin Whitepaper Section 2 + Bitcoin Whitepaper Section 5)**:
  - Section 2 implies spends reference a prior transaction (“hash of the previous transaction”).
  - Section 5 states nodes only accept a block if transactions are “**not already spent**”, which requires identifying *which* output is being spent. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:  
  - An `OutPoint` is the **portable identifier** for a specific transaction output:
    - `txid` identifies the transaction
    - `vout` selects the output index within that transaction
  - It is the unit we track for “not already spent”:
    - UTXO set membership answers “is this spendable right now?”
    - spending removes the `OutPoint` from the UTXO set
  - Relationship-wise, it is the bridge between inputs and outputs:
    - `TxIn.previous_output` is an `OutPoint`
    - that `OutPoint` selects a `TxOut`
- **Rust representation rationale**:
    ```rust
    #[derive(Clone, Debug)]
    pub struct OutPoint {
        pub txid: [u8; 32],
        pub vout: u32,
    }
    ```
  - `txid: [u8; 32]` is fixed-size because Bitcoin hashes are 32 bytes; `[u8; 32]` prevents length bugs and keeps hashing/serialization simple and fast.
  - `vout: u32` is a stable output index. Treating it as an integer (not a pointer/reference) is critical: outpoints are *portable identifiers* used in hashing, signatures, and on-wire messages.

#### `UTXO Set` (Unspent Transaction Output set)

- **Whitepaper citation (Bitcoin Whitepaper Section 2 + Bitcoin Whitepaper Section 5)**:
  - Section 2 frames the core problem as double-spending and requires participants to be able to confirm that a spend hasn’t happened already (“The only way to confirm the absence of a transaction is to be aware of all transactions.”). ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
  - Section 5 defines the acceptance rule: “Nodes accept the block only if all transactions in it are valid and not already spent.” ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:
  - The UTXO set is the node’s **spendable-state index**: it answers “what can be spent right now at this chain tip?”
  - Conceptually it is a map:
    - `OutPoint(txid, vout) -> TxOut(value, script_pubkey)`
  - Relationship-wise, it is the bridge between *references* and *validation*:
    - `TxIn.previous_output` (an `OutPoint`) must exist in the UTXO set, otherwise the input is already spent or unknown.
    - the referenced `TxOut` provides the spend context used to validate authorization (locking script) and value rules.
  - When we connect a block, the UTXO set changes deterministically:
    - **spend**: remove every input `OutPoint`
    - **create**: insert every new output as a new `OutPoint` (new txid + output index)
- **Rust representation rationale**:
  - At minimum, we can model it as `HashMap<OutPoint, TxOut>`, because that matches the conceptual “lookup spend target by outpoint” operation.
  - In a real node we typically also need:
    - a persistent storage backing (disk) and/or a cached in-memory view
    - a “view” overlay for block connection and mempool conflict checks (so we can validate without mutating the canonical state until we accept)

#### `BlockHeader`
- **Whitepaper citation (Bitcoin Whitepaper Section 3 + Bitcoin Whitepaper Section 4)**:
  - “The timestamp server works by taking a **hash of a block of items to be timestamped** and widely publishing the hash…” (Bitcoin Whitepaper Section 3). ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
  - “The proof-of-work involves scanning for a value that when hashed, such as with **SHA‑256**, the hash begins with a number of zero bits…” (Bitcoin Whitepaper Section 4). ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:  
  - A `BlockHeader` is the **consensus-critical commitment** that gets proof-of-work applied to it.
  - It links and secures history:
    - `prev_hash` points to the previous block header hash (forming the chain)
    - `merkle_root` commits to the block’s transaction set
  - It carries the proof-of-work fields:
    - `difficulty_bits` defines the required target threshold
    - `nonce` is the search space we scan to find a header hash under the target
  - Relationship-wise:
    - the header commits to the block body (`txs`) via `merkle_root`
    - nodes validate and chain headers first, then validate and apply the transactions
- **Rust representation rationale**:
```rust
#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub version: i32,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,        // nTime
    pub difficulty_bits: u32,  // nBits (compact target encoding)
    pub nonce: u32,
}
```
  - `prev_hash: [u8; 32]` is the *link* that makes blocks a chain.
  - `merkle_root: [u8; 32]` is the commitment to the transaction set (enables SPV proofs and efficient inclusion proofs).
  - `timestamp: u32`, `difficulty_bits: u32`, `nonce: u32` are fixed-width integers because the header is hashed; stable serialization is mandatory for consensus.

#### `Block`
- **Whitepaper citation (Bitcoin Whitepaper Section 3 + Bitcoin Whitepaper Section 7)**:
  - Section 3’s “block of items to be timestamped” is precisely a batch of transactions that the header commits to. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
  - Section 7 uses a Merkle tree to support efficient verification without downloading every transaction (SPV), which requires the block to commit to txs via a Merkle root. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))
- **Description**:  
  - A `Block` is the **container of a ledger step**: a header (what we hash for PoW) plus the transaction list (what we are committing to).
  - When we accept a block, we accept a state transition:
    - validate the header (links to known history + meets PoW target)
    - validate each transaction (valid spends + “not already spent”)
    - apply UTXO updates atomically (spend inputs, create outputs)
  - Relationship-wise:
    - `Block.header.merkle_root` commits to `Block.txs`
    - the header hash becomes the `prev_hash` of the next block
- **Rust representation rationale**:
```rust
#[derive(Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}
```
  - `header: BlockHeader` and `txs: Vec<Transaction>` mirrors the conceptual split between “what gets hashed for PoW” and “the data being committed”.
  - Keeping `txs` as full objects (not just hashes) is practical for full validation, while SPV clients can work from headers + Merkle branches.

---

<div align="center">

**[← Bitcoin Whitepaper → Rust Implementation (Rust Encoding)](README.md)** | **Business Objects** | **[Introduction (Bitcoin Whitepaper Section 1) →](01-Introduction-Bitcoin-Whitepaper-Section-1.md)**

</div>
