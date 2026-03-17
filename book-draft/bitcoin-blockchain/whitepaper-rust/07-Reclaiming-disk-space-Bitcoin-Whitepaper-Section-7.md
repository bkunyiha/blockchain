<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
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
    fn spend(&mut self, outpoint: &OutPoint)
        -> Result<(), TxError>;
    fn insert(
        &mut self,
        outpoint: OutPoint,
        txout: TxOut,
    ) -> Result<(), TxError>;
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
- **`get_ancestor(hash, height)`**: reorg support, fork finding
- **`headers_from(locator, stop)`**: headers-first sync support
- **batched updates / flush**: efficient, durable header updates

Quick mental model (three different “stores” a full node maintains):

```text
1) HeaderChain: block_hash -> header + metadata
2) Block store: block_hash -> body (may be pruned)
3) Chainstate: OutPoint(txid, vout) -> TxOut (kept)
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
**Merkle trees and SPV (Bitcoin Whitepaper Sections 7–22)**.

---

<div align="center">

**[← Incentive mechanism (Bitcoin Whitepaper Section 6)](06-Incentive-mechanism-Bitcoin-Whitepaper-Section-6.md)** | Reclaiming disk space (Bitcoin Whitepaper Section 7) | **[Merkle trees and SPV (Bitcoin Whitepaper Sections 7–22) →](08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)**

</div>
