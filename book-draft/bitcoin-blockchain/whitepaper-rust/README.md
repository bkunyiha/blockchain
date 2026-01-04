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
# Bitcoin Whitepaper Summary: Rust Implementation (Rust Encoding)

**Part III: Chapter 1.4** Bitcoin Whitepaper → Rust Implementation (Rust Encoding)
<div align="center">

**[← Bitcoin Whitepaper Summary](../00-Bitcoin-Whitepaper-Summary.md)** | Bitcoin Whitepaper → Rust Implementation (Rust Encoding) | **[Business Objects →](00-business-objects.md)**

</div>

---


## **Chapter 1.4** Bitcoin Whitepaper → Rust Implementation (Rust Encoding)

This page is the entry point. It links to the split section files so we don't have to read one huge page.

### Rust refresher (optional)

If we want to brush up on Rust before working through the implementations, we can read the Rust guide:

- **[Chapter 10: Rust Language Guide](../../rust/README.md)** — start with **[Rust Installation & Setup](../../rust/00-Rust-Installation-Setup.md)**, then continue to **[Introduction](../../rust/01-Introduction.md)**.

### Read options

- **Split sections (recommended)**: use the links below

### Split sections (0–12 + appendices)

- [Business objects](00-business-objects.md)
- [Introduction Bitcoin Whitepaper Section 1](01-Introduction-Bitcoin-Whitepaper-Section-1.md)
- [Transactions chain of signatures Bitcoin Whitepaper Section 2](02-Transactions-chain-of-signatures-Bitcoin-Whitepaper-Section-2.md)
- [Timestamp server block header chaining Bitcoin Whitepaper Section 3](03-Timestamp-server-block-header-chaining-Bitcoin-Whitepaper-Section-3.md)
- [Proof of work Bitcoin Whitepaper Section 4](04-Proof-of-work-Bitcoin-Whitepaper-Section-4.md)
- [Network operation Bitcoin Whitepaper Section 5](05-Network-operation-Bitcoin-Whitepaper-Section-5.md)
- [Incentive mechanism Bitcoin Whitepaper Section 6](06-Incentive-mechanism-Bitcoin-Whitepaper-Section-6.md)
- [Reclaiming disk space Bitcoin Whitepaper Section 7](07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)
- [Merkle trees and SPV Bitcoin Whitepaper Sections 7 8](08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)
- [Combining splitting value Bitcoin Whitepaper Section 9](09-Combining-splitting-value-Bitcoin-Whitepaper-Section-9.md)
- [Privacy Bitcoin Whitepaper Section 10](10-Privacy-Bitcoin-Whitepaper-Section-10.md)
- [Confirmations and attacker probability Bitcoin Whitepaper Section 11](11-Confirmations-and-attacker-probability-Bitcoin-Whitepaper-Section-11.md)
- [Conclusion Bitcoin Whitepaper Section 12](12-Conclusion-Bitcoin-Whitepaper-Section-12.md)
- [Appendix A Object connectivity end to end flow](Appendix-A-Object-connectivity-end-to-end-flow.md)
- [Appendix B Mapping to this repository](Appendix-B-Mapping-to-this-repository.md)

---

## 0. How to read this section (node lifecycle)

We structure this section around the core node pipeline, from parsing bytes to committing state. In Rust terms, the data path is:
**bytes → structs → canonical bytes → hashes → validation → state update**.

- **We accept a transaction**:
  - parse bytes into `Transaction` → serialize canonically → compute `txid` → validate (authorization + value rules) → check inputs are unspent in our UTXO view → add to mempool → relay
- **We accept a block**:
  - parse into `Block` → validate `BlockHeader` (prev link + PoW target) → validate tx set (including “not already spent”) → apply UTXO transition atomically → maybe reorg → relay

When we feel “lost”, we can come back to this Rust checklist (whitepaper phrase → concrete implementation question):

- **Whitepaper says “hash”** → “What exact bytes are hashed, in what order, and with what endianness?” (consensus serialization)
- **Whitepaper says “sign”** → “What exact digest do we sign/verify per-input, and what prevout/script context is included?” (signature hashing / script context)
- **Whitepaper says “not already spent”** → “Does `TxIn.previous_output: OutPoint` exist in our UTXO view at this tip?” (state + double-spend prevention)

Implementation roadmap (with Rust encoding):

- **1) Bytes (canonical encoding)**: implement CompactSize/varints and deterministic serialization for the core objects.
  - Outcome: “same object → same bytes” (always).
- **2) Identity (hashes as IDs)**: implement `sha256d`, `txid`, Merkle root, and `block_hash`.
  - Outcome: “same bytes → same IDs” across nodes.
- **3) Authorization (spends are permitted)**: implement script/signature verification using the referenced output as the spend context.
  - Outcome: “only the key-holder can spend” (no forged spends).
- **4) State (UTXO transitions)**: implement UTXO updates as a pure state transition: spend inputs, create outputs.
  - Outcome: “not already spent” is enforceable at the state layer.
- **5) Consensus (block acceptance + best chain)**: implement block validation and chain selection (and reorg handling if needed).
  - Outcome: nodes converge on the same history under the longest/most-work rule.

---

## 0.1. What the whitepaper specifies (and what it doesn’t)

In the summary chapter we focused on *what* the paper says. Here we focus on *how that becomes code* without losing the paper’s intent.

The whitepaper specifies the *core relationships*:

- **Transactions** are transfers of ownership via digital signatures (Bitcoin Whitepaper Section 2).
- **Blocks** timestamp transactions by hashing them into a chain (Bitcoin Whitepaper Section 3).
- **Proof-of-work** makes rewriting history computationally impractical (Bitcoin Whitepaper Section 4).
- **Nodes** broadcast transactions and blocks and converge on the “longest” chain (Bitcoin Whitepaper Section 5).
- **Merkle trees** enable compact commitments and SPV proofs (Bitcoin Whitepaper Sections 7–8).

The deployed Bitcoin protocol adds important concrete details beyond the paper (still consistent with it), such as:

- **double-SHA256** (“hash twice”) for txids and block hashes
- **CompactSize/varints** and strict little-endian **consensus serialization**
- **Script** (e.g. P2PKH/P2WPKH. See description in next paragraph) and signature hashing (“sighash”) rules
- a real P2P message protocol (inv/getdata, headers, etc.)

#### Script: P2PKH vs P2WPKH

In Bitcoin, most outputs are “locked” by a **scriptPubKey** (a small stack program). Spending an output means providing data that makes that program succeed. Two common standard output types are:

- **P2PKH (Pay-to-PubKey-Hash)**: the output commits to a 20-byte `HASH160(pubkey)` and can be spent by providing a signature and the corresponding public key.
  - **Locking form (scriptPubKey)**:

```text
OP_DUP OP_HASH160 <20-byte pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
```

  - **Spending data (conceptually)**: `<sig> <pubkey>` (the node hashes `pubkey` to check it matches `pubKeyHash`, then verifies `sig` under the signature-hash rules).
  - **Common address encoding**: legacy Base58 addresses that start with `1...` on mainnet.

Why `HASH160(pubkey)` and not just `SHA256(pubkey)`? In standard **Script/address constructions** (like P2PKH/P2WPKH), Bitcoin uses `HASH160 = RIPEMD160(SHA256(x))` for pubkey hashes largely because:

- it is **smaller** (20 bytes vs 32 bytes), which reduces output sizes and UTXO set footprint
- it provides **defense in depth** by combining two hash functions (a practical break would need to help “through” the composition)
- it is a **historical standard** baked into early script/address conventions (changing it would be a broad compatibility and consensus break)

Note: this is separate from block/tx identifiers—**txids** (and **block hashes**) are computed with **double-SHA256** (i.e. `SHA256(SHA256(bytes))`) over consensus-serialized data (tx serialization for txids; block header serialization for block hashes).

- **P2WPKH (Pay-to-Witness-PubKey-Hash, SegWit v0)**: the same ownership model as P2PKH (pubkey hash), but signatures and pubkeys are carried in **witness** data (SegWit), not the legacy scriptSig path.
  - **Locking form (witness program)**:

```text
0 <20-byte pubKeyHash>
```

  - **Spending data**: witness stack provides `<sig> <pubkey>` (native P2WPKH has an empty `scriptSig`).
  - **Why it matters**: witness data is excluded from the legacy txid, which eliminates third‑party malleability for signature data and makes blocks more space-efficient under weight rules.
  - **Common address encoding**: Bech32 addresses that start with `bc1q...` on mainnet. (For compatibility there is also P2SH‑wrapped P2WPKH, often starting with `3...`.)

The whitepaper itself does **not** fully specify:

- exact wire encodings (byte-for-byte consensus serialization)
- Script details, signature hashing rules, and full validation edge cases
- production P2P protocol message formats

That’s fine for learning: the whitepaper is the conceptual contract, and implementations supply the missing exactness. Our goal is to make the missing pieces explicit (especially byte encoding), because consensus is ultimately “agree on bytes”.

---

<div align="center">

**[← Bitcoin Whitepaper Summary](../00-Bitcoin-Whitepaper-Summary.md)** | Bitcoin Whitepaper → Rust Implementation (Rust Encoding) | **[Business Objects →](00-business-objects.md)**

</div>
