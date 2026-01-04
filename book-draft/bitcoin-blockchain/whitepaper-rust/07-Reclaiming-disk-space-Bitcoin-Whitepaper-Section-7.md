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
## 7. Reclaiming disk space (Bitcoin Whitepaper Section 7)

Section 7 of the paper describes how nodes can reclaim disk space by **pruning** data that is no longer needed for full validation, while keeping the commitments needed to prove inclusion (via the Merkle root in the header). ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

### 7.0 Overview (what we are trying to achieve)

In this section, we translate the whitepaper’s Section 7 idea into an implementation mental model:

- we want a node that can still **validate new blocks** correctly,
- while optionally storing less historical data on disk.

Implementation lens (what we must keep vs what we can drop):

- **Must keep (for consensus verification of new blocks)**:
  - the current **UTXO set** (so we can enforce “not already spent”)
  - enough **block headers / chain index** to follow the best chain and verify PoW
- **Can potentially drop (for a pruned node)**:
  - old fully-spent transaction data, once its outputs are spent and it’s deep enough in the chain that we are comfortable pruning it (policy choice)

### 7.1 Rust-facing storage interfaces (the APIs our validation code wants)

When we implement a node in Rust, we rarely want “open a file and scan blocks” scattered throughout validation logic. Instead, we define small interfaces that answer the questions validation needs (lookup, connect, best tip).
```rust
pub trait UtxoStore {
    fn get(&self, outpoint: &OutPoint) -> Result<TxOut, TxError>;
    fn spend(&mut self, outpoint: &OutPoint) -> Result<(), TxError>;
    fn insert(&mut self, outpoint: OutPoint, txout: TxOut) -> Result<(), TxError>;
}

pub trait HeaderChain {
    fn best_tip(&self) -> [u8; 32];
    fn get_header(&self, hash: &[u8; 32]) -> Option<BlockHeader>;
    fn connect_header(&mut self, header: BlockHeader) -> Result<(), ChainError>;
}
```

Key idea: “reclaiming disk space” is mostly an engineering/storage decision; consensus validation still depends on the **UTXO view** and the **header chain**.

### 7.2 What these business objects represent (and why they exist)

- **`UtxoStore`**: the interface to our node’s **UTXO database (chainstate)**.
  - **What it stores**: a mapping from **OutPoint** → **TxOut**.
    - key: `OutPoint(txid, vout)` (which specific output we mean)
    - value: `TxOut { value, script_pubkey }` (how much + how it is locked)
  - **What it is used for**:
    - during tx/block validation, it answers “is this input spending an unspent output?” and provides the prevout needed to verify scripts/signatures
    - during block connection, it applies the state transition: remove spent outpoints, insert newly created outpoints
  - **Where it is stored**:
    - not “inside the blockchain” as a literal object
    - it is a **derived index** computed by replaying blocks up to the current tip and persisted locally (often as a key-value database on disk)

- **`HeaderChain`**: the interface to our node’s **header index / chain index**.
  - **Why we keep a `HeaderChain` index instead of “just reading the blockchain”**:
    - The blockchain is an **append-only history**. “Reading the blockchain” means scanning and parsing a lot of bytes repeatedly.
    - A node needs to answer header questions *constantly* and *randomly*, for example:
      - “Does this header connect to something we already have?” (follow `prev_hash`)
      - “Which chain has more total work?” (chain selection / reorg decisions)
      - “What is our current best tip?” (best chain tracking)
    - If we solved those by re-reading block files from disk each time, the node would be slow:
      - disk I/O heavy (scan many blocks repeatedly)
      - CPU heavy (parse headers repeatedly)
      - awkward for reorgs (finding ancestors/fork points requires random access)
    - `HeaderChain` is therefore a **derived local index**: it stores just enough header/linkage metadata to make those queries fast.
    - This is especially important for pruning:
      - pruning may delete old **block bodies**, so “reading the blockchain” (full blocks) is not always possible
      - but we still keep **headers + header index**, because headers are what we use for PoW verification and chain selection
  - **How the `HeaderChain` data is stored (common layouts)**:
    - **Key-value index (most common)**:
      - key: `block_hash`
      - value: `BlockHeader` + small index metadata (height, parent hash, accumulated work, status flags)
      - why nodes like it: direct lookup (“do we have this header?”), fast parent walking for reorgs, and cheap best-tip queries
    - **Append-only header file + index (also common)**:
      - store headers sequentially in a file (compact and cache-friendly)
      - keep a separate index `block_hash -> file_offset` so we can jump directly to any header without scanning
      - this keeps disk usage low while still providing random access
  - **What it stores**: block headers plus “how they connect” metadata, keyed by block hash.
    - header fields: `prev_hash`, `merkle_root`, `timestamp`, `nBits`, `nonce`, …
    - index metadata (implementation choice): height, accumulated work, parent pointer, and whether the header is part of the current best chain
  - **What it is used for**:
    - validate PoW and prev-hash linkage
    - track the best chain tip and support reorg decisions
    - allow fast sync via `headers` messages (we can update our best-known tip without downloading all block bodies immediately)
  - **Where it is stored**:
    - typically as a persistent index on disk (or in a database), separate from the UTXO store
    - block bodies may be stored separately again (a “block store”), and a pruned node may delete many old bodies while keeping headers + UTXO state
  - **Relationship to “the blockchain” (history vs index)**:
    - The **blockchain (history)** is the sequence of blocks: each block has a header and a body (transactions).
    - `HeaderChain` is not “another blockchain”; it is the node’s **local index over the header portion** of that history.
    - We keep this index so we can answer questions quickly:
      - “What is our best tip?”
      - “Is this header valid PoW and does it connect to something we know?”
      - “If a competing chain arrives, which chain has more work and should we reorg?”
  - **Relationship to pruning (what gets deleted vs what remains)**:
    - Pruning removes **old block bodies** (transactions) from local disk.
    - Pruning does **not** remove headers or the header index:
      - we still need headers to validate PoW, follow `prev_hash` links, and choose the best chain
      - we still need the UTXO set to validate new spends (“not already spent”)
    - That is why a pruned node can stay consensus-correct for new blocks: it validates the next block by checking its inputs against the current UTXO set, not by rereading ancient transactions from disk.
  - **Should `HeaderChain` have a method to “get the database” or “all connections”?**
    - **In general, no**: we usually do *not* expose raw database handles or internal connectivity from a domain trait like `HeaderChain`.
      - **Why**: it leaks storage details into consensus logic, makes testing harder, and encourages bypassing invariants (e.g., updating an index without updating related metadata).
    - Instead, the trait exposes the **operations the node needs** (query + connect), and the concrete implementation owns the DB handle privately.

What additional methods a production `HeaderChain` often adds (optional, depending on the node design):

- **`contains_header(hash)` / `height_of(hash)`**: quick existence and height queries
- **`get_best_header()`**: return the full best-tip header (not just its hash)
- **`get_ancestor(hash, height)`**: reorg support and fork finding
- **`headers_from(locator, stop)`**: support for headers-first sync patterns
- **batched updates / flush**: apply many header connects efficiently and durably

Quick mental model (three different “stores” a full node maintains):

```text
1) HeaderChain (header index):   block_hash -> header + linkage/work metadata
2) Block store (history):        block_hash -> full block body (may be pruned for old blocks)
3) Chainstate / UTXO database:   OutPoint(txid, vout) -> TxOut (must be kept)
```


### How Merkle trees enable pruning (and why the header is enough)

Pruning only works because blocks separate:

- **the commitment** (in the 80-byte header), and
- **the committed data** (the transaction list / block body).

The Merkle tree is the mechanism that makes this separation compact:

- The block header stores **`merkle_root`**, which is a 32-byte commitment to **all transaction IDs in the block**.
- If a pruned node keeps headers, it still keeps the `merkle_root` commitments for every historical block header it has.
- That means the node can still participate in chain selection (PoW/headers) even if it has deleted some old block bodies.

What pruning does **not** mean:

- A pruned full node is not an SPV client. It still fully validates new blocks using its **UTXO set**.
- Pruning is about disk usage for **old history**, not about weakening consensus checks for new blocks.

We explain Merkle trees (how the root is computed, and how Merkle branches prove inclusion) in the next section:
**[Merkle trees and SPV (Bitcoin Whitepaper Sections 7–8)](08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)**.

---

<div align="center">

**[← Incentive mechanism (Bitcoin Whitepaper Section 6)](06-Incentive-mechanism-Bitcoin-Whitepaper-Section-6.md)** | Reclaiming disk space (Bitcoin Whitepaper Section 7) | **[Merkle trees and SPV (Bitcoin Whitepaper Sections 7–8) →](08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)**

</div>
