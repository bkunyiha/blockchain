<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
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
# Business Objects: Core Types & Relationships

**Note**: For the complete Rust implementation of these types, including code walkthroughs and detailed struct field-by-field analysis, see **[Chapter 9: Domain Model](../chain/01-Domain-Model.md)**. This chapter presents the whitepaper business perspective; the domain model chapter covers the actual Rust code.

---

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
    // “public key of the next owner” is represented by a locking script
    pub script_pubkey: Vec<u8>,
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

```text
# Legend (how to read this diagram):
# - OutPoint(X, i) = "output i from transaction X"
# - TxOut(value, lock) = amount + spending condition
# - New outputs become new UTXOs keyed by (txid, index)
# - fee = Inputs - Outputs (simplified; real Bitcoin has more rules)

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
  - remove: OutPoint(A, 0), OutPoint(B, 2)  // spent
  + add   : OutPoint(TxID, 0) ──► TxOut(9, lock=Bob)
          OutPoint(TxID, 1) ──► TxOut(2, lock=Alice)
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

```text
TxIn ──spends──► OutPoint(txid, vout)
                 ──selects──► TxOut(value, script)

UTXO set (current spendable outputs):
  OutPoint(txid, vout)  ->  TxOut(...)
```

#### Model B (**chain-first**): follow what consensus secures

This model starts from the “thing we do proof-of-work on” and works downward into what it commits to.

- **`BlockHeader`** is what we hash for PoW, and it links to history via `prev_hash`.
- **`merkle_root`** is the header’s commitment to the block’s transaction list.
- **`Block`** is “header + transactions”; accepting a block means we accept a state transition.

```text
BlockHeader(prev_hash, merkle_root, nBits, nonce)
   │
   ├─ prev_hash ──► previous BlockHeader (the chain)
   │
   └─ merkle_root ──► commits to txids of Block.txs
                         │
      └─ Tx: inputs spend OutPoints, select TxOuts,
           update UTXO set
```

If we keep both models in mind, it becomes clear how “money movement” (UTXO transitions) is anchored to “history” (headers + PoW + longest chain).

### 0.2.2 Whitepaper citations (business objects) — mapping to Rust types

The whitepaper doesn’t list “Rust structs”, but it does define the *business objects* and their relationships. The mapping from whitepaper concepts to Rust types is summarized below. For complete Rust code implementations, field-by-field analysis, and detailed code walkthroughs, see **[Chapter 9: Domain Model](../chain/01-Domain-Model.md)**.

**Quick reference:**

- **`Transaction`** — atomic ledger transition (spend prior outputs + create new ones; Section 2)
- **`TxIn`** — claim on an existing UTXO (names the spend target + provides authorization; Section 2)
- **`TxOut`** — spendable value plus locking condition (Section 2)
- **`OutPoint`** — portable identifier for a specific output (txid + index; Section 2/5)
- **`UTXO Set`** — mapping of spendable outputs (Section 5, “not already spent”)
- **`BlockHeader`** — consensus-critical commitment (PoW input; Section 3/4)
- **`Block`** — container of ledger step (header + transaction list; Section 3/7)

---

<div align="center">

**[← Bitcoin Whitepaper → Rust Implementation (Rust Encoding)](00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md)** | **[Business Objects](00-business-objects.md)** | **[Introduction (Bitcoin Whitepaper Section 1) →](01-Introduction-Bitcoin-Whitepaper-Section-1.md)**

</div>
