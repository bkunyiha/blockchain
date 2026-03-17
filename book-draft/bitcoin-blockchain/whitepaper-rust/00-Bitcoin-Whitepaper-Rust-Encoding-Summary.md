<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. **Chapter 4: Bitcoin Whitepaper In Rust** ← *You are here*
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
# Bitcoin Whitepaper Summary: Rust Implementation (Rust Encoding)

**Part I: Chapter 4** Bitcoin Whitepaper To Rust Implementation (Rust Encoding)
<div align="center">

**[← Bitcoin Whitepaper Summary](00-Bitcoin-Whitepaper-Summary.md)** | Bitcoin Whitepaper → Rust Implementation (Rust Encoding) | **[Business Objects →](00-business-objects.md)**

</div>

---


## **Chapter 4** Bitcoin Whitepaper → Rust Implementation (Rust Encoding)

Now that we’ve walked through the Bitcoin whitepaper, we switch from **theory to implementation**. In the sections that follow, we translate the whitepaper’s ideas into **concrete Rust modules and methods**, with complete code listings and explanations—calling out what we implement exactly, what we intentionally simplify, and where real-world Bitcoin Core adds additional rules.

> **What you will learn in this chapter:**
> - Map each Bitcoin whitepaper concept to a concrete Rust data structure
> - Explain the hashing and serialization rules that connect whitepaper theory to working code
> - Trace how each whitepaper section corresponds to modules in the repository
> - Understand the business objects that form the foundation of the implementation

> **Tip:** Read this chapter alongside Chapter 3 (Bitcoin Whitepaper Summary). Each section here maps directly to a whitepaper section, and understanding the theory first makes the Rust encoding decisions much clearer.

### Rust refresher (optional)

If we want to brush up on Rust before working through the implementations, we read the Rust guide:

- **Chapter 24: Rust Language Guide** — start with **Rust Installation & Setup**, then continue to **Introduction**.

### Read options

- **Split sections (recommended)**: use the links below

### Split sections (0–12 + appendices)

- Business objects
- Introduction Bitcoin Whitepaper Section 1
- Transactions chain of signatures Bitcoin Whitepaper Section 2
- Timestamp server block header chaining Bitcoin Whitepaper Section 3
- Proof of work Bitcoin Whitepaper Section 4
- Network operation Bitcoin Whitepaper Section 5
- Incentive mechanism Bitcoin Whitepaper Section 6
- Reclaiming disk space Bitcoin Whitepaper Section 7
- Merkle trees and SPV Bitcoin Whitepaper Sections 7 8
- Combining splitting value Bitcoin Whitepaper Section 9
- Privacy Bitcoin Whitepaper Section 10
- Confirmations and attacker probability Bitcoin Whitepaper Section 11
- Conclusion Bitcoin Whitepaper Section 12
- Appendix A Object connectivity end to end flow
- Appendix B Mapping to this repository

---

## 0. How to read this section (node lifecycle)

We structure this section around the core node pipeline, from parsing bytes to committing state. In Rust terms, the data path is:
**bytes → structs → canonical bytes → hashes → validation → state update**.

- **We accept a transaction**: parse bytes into `Transaction` → serialize canonically → compute `txid` → validate (authorization + value rules) → check inputs are unspent in our UTXO view → add to mempool → relay.
- **We accept a block**: parse into `Block` → validate `BlockHeader` (prev link + PoW target) → validate tx set (including “not already spent”) → apply UTXO transition atomically → maybe reorg → relay.

When we feel lost, we come back to this Rust checklist (whitepaper phrase → concrete implementation question):

- **Whitepaper says “hash”**: “What exact bytes are hashed, in what order, and with what endianness?” (consensus serialization)
- **Whitepaper says “sign”**: “What exact digest do we sign/verify per-input, and what prevout/script context is included?” (signature hashing / script context)
- **Whitepaper says “not already spent”**: “Does `TxIn.previous_output: OutPoint` exist in our UTXO view at this tip?” (state + double-spend prevention)

Implementation roadmap (with Rust encoding):

- **1) Bytes (canonical encoding)**: Implement CompactSize/varints and deterministic serialization for the core objects. Outcome: “same object → same bytes” (always).
- **2) Identity (hashes as IDs)**: Implement `sha256d`, `txid`, Merkle root, and `block_hash`. Outcome: “same bytes → same IDs” across nodes.
- **3) Authorization (spends are permitted)**: Implement script/signature verification using the referenced output as the spend context. Outcome: “only the key-holder can spend” (no forged spends).
- **4) State (UTXO transitions)**: Implement UTXO updates as a pure state transition: spend inputs, create outputs. Outcome: “not already spent” is enforceable at the state layer.
- **5) Consensus (block acceptance + best chain)**: Implement block validation and chain selection (and reorg handling if needed). Outcome: nodes converge on the same history under the longest/most-work rule.

---

## 0.1. What the whitepaper specifies (and what it doesn’t)

In the summary chapter we focused on *what* the paper says. Here we focus on *how that becomes code* without losing the paper’s intent and meaning.

The whitepaper specifies the *core relationships*:

- **Transactions** are transfers of ownership via digital signatures (Bitcoin Whitepaper Section 2).
- **Blocks** timestamp transactions by hashing them into a chain (Bitcoin Whitepaper Section 3).
- **Proof-of-work** makes rewriting history computationally impractical (Bitcoin Whitepaper Section 4).
- **Nodes** broadcast transactions and blocks and converge on the “longest” chain (Bitcoin Whitepaper Section 5).
- **Merkle trees** enable compact commitments and SPV proofs (Bitcoin Whitepaper Sections 7–22).

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

- It is **smaller** (20 bytes vs 32 bytes), which reduces output sizes and UTXO set footprint.
- It provides **defense in depth** by combining two hash functions (a practical break would need to affect both).
- It is a **historical standard** baked into early script/address conventions (changing it would be a broad compatibility and consensus break).

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

## Further Reading

- **[Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)** — Read alongside this chapter for the full context behind each Rust encoding.
- **[serde Documentation](https://serde.rs/)** — The serialization framework used throughout the implementation.
- **[Bitcoin Developer Reference](https://developer.bitcoin.org/reference/)** — Technical specifications for Bitcoin data structures.

---

## What We Covered

- We mapped every Bitcoin whitepaper concept to a concrete Rust data structure, showing how theory translates into working code.
- We defined the core business objects — Block, Transaction, BlockHeader, and their supporting types — that form the implementation's foundation.
- We traced hashing and serialization rules from whitepaper descriptions to Rust implementations using SHA-256 and serde.
- We established the mapping between whitepaper sections and repository modules, creating a bridge between concept and code.

In the next chapter, we examine how the Rust project is organized — the crate workspace, dependency graph, and build configuration that make all of this compile and run.

---

<div align="center">

**[← Bitcoin Whitepaper Summary](00-Bitcoin-Whitepaper-Summary.md)** | Bitcoin Whitepaper → Rust Implementation (Rust Encoding) | **[Business Objects →](00-business-objects.md)**

</div>
