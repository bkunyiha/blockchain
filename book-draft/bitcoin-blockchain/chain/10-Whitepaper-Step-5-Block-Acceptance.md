<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. **Chapter 10: Block Acceptance** ← *You are here*
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. <a href="../node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---

# Whitepaper Step 5: Block Acceptance — “Valid and Not Already Spent”

**Part I: Foundations & Core Implementation** | **Section 10: Block Acceptance (Whitepaper §5, Step 5)**

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

> **Why this section exists**: in **Section 9.7 (Consensus and Validation)** we explain the Step‑5 contract, but the current implementation does **not** consistently enforce “**valid AND not already spent**” as a hard **Validate → Connect** gate for inbound blocks. This chapter (**Section 10**) isolates that missing piece and shows the exact boundary where it must be implemented.

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

```text
validate(block, view) -> Ok(()) or Err(...)
connect(block, state) -> Ok(()) or Err(...)
```

And it gives you a diagram you can keep in your head while reading code:

```text
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
                // Intra-block double spend
                return Err(
                    BtcError::InvalidTransactionInput
                );
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

> **Checkpoint:** At this point, the chain validation logic is complete. You can verify it works by running `cargo test -p bitcoin --lib` — the chain and consensus tests should pass, confirming that block acceptance, UTXO updates, and fork-choice all behave correctly. If any test fails, re-read the validation rules in this chapter and trace the failing assertion back to the relevant method.

---

<div align="center">

**[← Chapter 9 (Blockchain Core)](README.md)** | **Chapter 10: Block Acceptance (Whitepaper §5, Step 5)** | **[Next: Chapter 11 (Storage Layer) →](../store/README.md)** **[Up: Book Index](../../README.md#table-of-contents)**

</div>

---

