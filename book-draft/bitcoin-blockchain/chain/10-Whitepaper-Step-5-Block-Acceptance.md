<div align="left">

<details>
<summary><b>📑 Section Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Section 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Section 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Section 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. Section 1.4: Bitcoin Whitepaper In Rust - Bitcoin Whitepaper In Rust
5. Section 2.0: Rust Blockchain Project - Blockchain Project
6. Section 2.1: Primitives - Core data structures
7. Section 2.2: Utilities - Utility functions and helpers
8. Section 2.3: Cryptography - Cryptographic primitives and libraries
9. Section 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. Section 2.5: Storage Layer - Persistent storage implementation
11. **Section 2.6: Block Acceptance (Whitepaper §5, Step 5)** ← *You are here*
12. Section 2.7: Network Layer - Peer-to-peer networking and protocol
13. Section 2.8: Node Orchestration - Node context and coordination
14. Section 2.9: Wallet System - Wallet implementation and key management
15. Section 3: Web API Architecture - REST API implementation
16. Section 4: Desktop Admin Interface - Iced framework architecture
17. Section 5: Wallet User Interface - Wallet UI implementation
18. Section 6: Embedded Database & Persistence - SQLCipher integration
19. Section 7: Web Admin Interface - React/TypeScript web UI

### Part II: Deployment & Operations

20. Section 8: Docker Compose Deployment - Docker Compose guide
21. Section 9: Kubernetes Deployment - Kubernetes production guide
22. Section 10: Rust Language Guide - Rust programming language reference

</details>

</div>

---

# Whitepaper Step 5: Block Acceptance — “Valid and Not Already Spent”

**Part I: Core Blockchain Implementation** | **Section 2.6: Block Acceptance (Whitepaper §5, Step 5)**

Bitcoin’s whitepaper compresses the most important safety rule in the entire system into one sentence:

> **“Nodes accept the block only if all transactions in it are valid and not already spent.”**  
> — [Bitcoin Whitepaper, Section 5](https://bitcoin.org/bitcoin.pdf)

This is the moment where your node decides whether incoming data becomes **durable state**. Get this wrong and the chain can still “look like a blockchain” while quietly violating the two rules that make Bitcoin Bitcoin:

- **Ownership**: only the holder of the right key can spend an output.
- **Scarcity**: each output can be spent **at most once** (no double-spends).

In the previous `chain/` chapters we learned the pieces:

- **Validity** lives in code like `Transaction::verify(...)` (signatures, structure).
- **Spendability** lives in the UTXO set (what is unspent right now).
- **Fork choice** decides which branch we build on.

> **Why this section exists**: in **Section 2.4.7 (Consensus and Validation)** we explain the Step‑5 contract, but the current implementation does **not** consistently enforce “**valid AND not already spent**” as a hard **Validate → Connect** gate for inbound blocks. This chapter (**Section 2.6**) isolates that missing piece and shows the exact boundary where it must be implemented.

This capstone is about the *boundary* where those pieces must be composed correctly:

> **Validate first. Connect only if valid.**

---

## Scope (where Step 5 appears in our codebase)

In this repository, the “block became canonical” boundary is in:

- `bitcoin/src/store/file_system_db_chain.rs` → `BlockchainFileSystem::add_block(...)`

That’s the method that must enforce the Step‑5 gate **before** mutating the canonical tip and the UTXO set.

This chapter is written like a book chapter for Rust implementers:

- We’ll start with the runtime story (“a block arrives”).
- Then we’ll translate Step 5 into a two‑phase design (Validate → Connect).
- Finally we’ll map that design to the exact Rust entry points you already saw in `chain/`.

---

## The runtime story: “a block just arrived”

Imagine your node receives a `Package::Block` from a peer. At that moment, you have a choice:

1. Treat it as *data you can store* (fine: you can persist blocks as candidates).
2. Treat it as *state you will build on* (dangerous: this is where Step 5 applies).

The whitepaper’s Step 5 is about the second choice:

> “Accept the block” means “let this block change my node’s authoritative view of money.”

That includes:

- Moving the canonical tip pointer.
- Updating derived state (UTXO set) so “what is spendable” matches the new tip.
- Potentially reorganizing (disconnect old tip blocks, connect new branch blocks).

This is why Step 5 is not a “nice-to-have validation”; it is the contract that prevents your node from turning invalid history into real balances.

---

## A Rust mental model: the acceptance boundary is an API boundary

As a Rust developer, a clean way to think about Step 5 is:

- **Validation** is a *pure function*: it can read chain state and UTXO state, but must not mutate durable state.
- **Connection** is a *state transition*: it mutates tip + UTXO (and maybe indexes), and must be atomic or rollback-safe.

That gives you a two-phase acceptance interface:

```
validate(block, view) -> Ok(()) or Err(...)
connect(block, state) -> Ok(()) or Err(...)
```

And it gives you a diagram you can keep in your head while reading code:

```
Incoming block bytes
   |
   v
Validate (read-only)
  - all txs valid (signatures, structure)
  - inputs reference unspent outpoints (UTXO view)
  - no intra-block double-spends
   |
   v   only if VALID
Connect (mutating)
  - write canonical tip
  - update UTXO set (spend inputs, add outputs)
  - (reorg path: rollback/apply in order)
```

If your code discovers invalidity “mid-update”, you have already violated the contract: you let invalid data partially become state.

---

## Step 5, expanded: what “valid” and “not already spent” mean

The whitepaper is minimal, so we’ll be explicit. In a learning implementation like this repo, the smallest useful Step‑5 set is:

### 1) Block-level sanity (structural)

At minimum:

- **Coinbase rules**: a block should contain **exactly one coinbase** transaction (simplified rule used throughout the book).
- **Parent linkage**: if we are connecting the block, its parent must be known (or we keep it as an orphan/candidate).
- **Proof-of-work**: if your chain is PoW-secured, the header hash must satisfy the target (some paths in this repo intentionally simplify/todo this).

### 2) Transaction-level validity (authorization)

For each non-coinbase transaction, “valid” includes:

- **Signature verification** against the reconstructed digest.

In this repo, that primitive already exists:

- `bitcoin/src/primitives/transaction.rs` → `Transaction::verify(&BlockchainService)`

### 3) State-level validity (“not already spent”)

The key phrase is **“not already spent”**, which is *not* a signature property. It is a statement about the current UTXO view.

This is outpoint-level logic:

- **Outpoint**: \((txid, vout)\)
- **Rule**: every input must reference an outpoint that is currently unspent in the UTXO view.

Two pitfalls that Step 5 must guard against:

- **Inter-block double spend**: spending an outpoint that was already spent in the current canonical chain.
- **Intra-block double spend**: the same outpoint appears twice as an input across the block’s transactions.

The intra-block case is easy to miss and easy to fix:

- Build a `HashSet<(txid, vout)>` while scanning the block.
- If you see the same outpoint twice, reject the block immediately.

---

## Mapping Step 5 to our repo: where the gate must live

The core acceptance entry point is already labeled in the code with a FIXME.

### The acceptance boundary in code

`bitcoin/src/store/file_system_db_chain.rs` → `BlockchainFileSystem::add_block(...)`

Inside this method, you’ll find a comment like:

```rust
// FIXME: From bitcoin whitepaper, only add block if:
// A) “All transactions in it are valid”
// B) “Not already spent”
```

That is the right instinct, and it points to the right place: **Step 5 belongs in `add_block`**.

The important subtlety is what “add” means here:

- It’s fine to *store the block* as “data we’ve heard about.”
- It is not fine to *connect the block* (tip + UTXO mutation) without Step‑5 validation.

---

## A concrete design: implement “Validate → Connect”

This section is intentionally practical. It’s not “Bitcoin Core”; it’s a clear shape you can implement in Rust in this codebase.

### Phase 1: Validate (read-only)

Inputs:

- The block being considered.
- A read-only chain API (`BlockchainService`) to locate referenced previous transactions (needed for signature verification).
- A UTXO view to answer “is this outpoint unspent?”

Outputs:

- `Ok(())` if Step‑5 holds.
- A domain error that explains which rule was violated.

Rust shape (documentation-level pseudocode):

```rust
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: Vec<u8>,
    pub vout: usize,
}

pub async fn validate_step5(
    block: &Block,
    chain: &BlockchainService,
    utxo: &UTXOSet,
) -> Result<()> {
    // 1) Exactly one coinbase (simplified)
    let txs = block.get_transactions().await?;
    let coinbase_count = txs.iter().filter(|tx| tx.is_coinbase()).count();
    if coinbase_count != 1 {
        return Err(BtcError::InvalidTransaction);
    }

    // 2) No outpoint spent twice in the same block
    let mut seen: HashSet<OutPoint> = HashSet::new();

    for tx in &txs {
        if tx.is_coinbase() {
            continue;
        }

        // 3) Authorization (signatures)
        if !tx.verify(chain).await? {
            return Err(BtcError::InvalidSignature);
        }

        // 4) Spendability (UTXO view)
        for vin in tx.get_vin() {
            let op = OutPoint {
                txid: vin.get_txid().to_vec(),
                vout: vin.get_vout(),
            };

            if !seen.insert(op.clone()) {
                return Err(BtcError::InvalidTransactionInput); // intra-block double spend
            }

            // Your UTXO API wants to answer: is (txid, vout) currently unspent?
            // (In this repo, it may require improving UTXO lookup by outpoint.)
            utxo.assert_unspent(&op).await?;
        }
    }

    Ok(())
}
```

> If you take one thing away: Step 5 needs an outpoint-level query (`assert_unspent`), not just “find UTXOs by owner”.

### Phase 2: Connect (mutating)

Once validation passes, connecting the block is “just” the state transition:

- Spend the referenced outpoints.
- Insert the new outputs.
- Advance tip (or apply reorg logic).

But there is one critical systems requirement:

> **Connecting must be atomic or rollback-safe.**

If your UTXO update is multiple DB writes and you can fail mid-way, your node can end up in a state that doesn’t correspond to any valid chain.

In this repo, the connector is implemented as UTXO update logic (e.g. `UTXOSet::update(...)`), and the persistence layer is sled. The design goal is:

- compute the delta
- apply the delta in one sled transaction (or a clearly defined “write then commit tip” transaction)

---

## Reorgs: Step 5 is not optional on the “apply blocks” path

A common misunderstanding is: “we validated blocks when we first saw them, so reorg apply is safe.”

In a real node, and in this repo’s learning node, reorg application still needs the same gate:

- When you **disconnect** blocks: rollback derived state safely.
- When you **connect** blocks on the winning branch: each connected block must satisfy Step 5 against the evolving UTXO view.

So any method that effectively does “connect a sequence of blocks” must call the same validator you use for normal extension.

In the current codebase, look for reorg helpers like:

- `reorganize_chain(...)`
- `apply_chain_from_ancestor(...)`

and treat them as “batch connect”: Step‑5 validation should be enforced per block.

---

## A practical note about UTXO representation (why Step 5 gets hard otherwise)

Step 5 is an outpoint-level rule. Your UTXO layer must preserve stable outpoint identity:

- In Bitcoin, `(txid, vout)` is stable forever.
- If your UTXO storage is “`txid -> Vec<TXOutput>`” and you *remove elements* from the `Vec`, you can accidentally shift indices and break the meaning of `vout`.

That’s why many implementations model UTXO state as something like:

- `outpoint -> TXOutput`

or keep a stable vector plus a spent marker rather than physically removing entries.

You don’t have to implement a perfect data model for this book, but you do need one property:

> `vout` must keep its meaning, so “is (txid, vout) unspent?” is deterministic.

---

## Short conclusion

The whitepaper’s Step 5 is not a comment, and not “an error we hope happens during update”.

It is a hard gate:

- **Validate all transactions** (authorization).
- **Reject double spends** (state-level spendability).
- **Only then** mutate the canonical tip and the UTXO set.

That’s the moment where consensus becomes state.

---

## Navigation

- **← Previous: Consensus & Validation** - chain selection rules
- **Section 2.4 Index** - Section map and reading order
- **Next: Network Layer →** - peer-to-peer protocol and syncing

---

<div align="center">

**📚 [← Previous: Section 2.4.7 (Consensus & Validation)](07-Consensus-and-Validation.md)** | **Block Acceptance (Whitepaper §5, Step 5)** | **[Next: Section 2.7 (Network Layer) →](../net/README.md)** 📚

</div>

---

