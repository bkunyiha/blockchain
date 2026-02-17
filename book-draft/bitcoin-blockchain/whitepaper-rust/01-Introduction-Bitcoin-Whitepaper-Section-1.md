<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. Chapter 2.5: Storage Layer - Persistent storage implementation
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
12. Chapter 2.7: Network Layer - Peer-to-peer networking and protocol
13. Chapter 2.8: Node Orchestration - Node context and coordination
14. Chapter 2.9: Wallet System - Wallet implementation and key management
15. Chapter 3: Web API Architecture - REST API implementation
16. Chapter 4: Desktop Admin Interface - Iced framework architecture
17. Chapter 5: Wallet User Interface - Wallet UI implementation
18. Chapter 6: Embedded Database & Persistence - SQLCipher integration
19. Chapter 7: Web Admin Interface - React/TypeScript web UI

### Part II: Deployment & Operations

20. Chapter 8: Docker Compose Deployment - Docker Compose guide
21. Chapter 9: Kubernetes Deployment - Kubernetes production guide
22. Chapter 10: Rust Language Guide - Rust programming language reference

</details>

</div>

---
## 1. Introduction (Bitcoin Whitepaper Section 1)

Section 1 (“Introduction”) frames the goal: replace the trust-based model (reversible payments, dispute mediation, fraud costs) with an electronic payment system based on **cryptographic proof instead of trust**. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Implementation lens (for Rust developers):

- We want “payment” to be enforceable by verification, not by an intermediary.
- That means we need deterministic **byte encodings** (consensus serialization), deterministic **hashes** (identifiers/commitments), and deterministic **validation + state updates** so independent nodes converge on the same history.

### Implementation implications (what Section 1 requirements)

Section 1 does not define wire formats or code, but it commits us to a system where any node can independently verify the history. In practice, that implies a small set of implementation building blocks:

- **Canonical data model + consensus serialization**: each transaction and block must have one agreed byte form, because identifiers and commitments are hashes of bytes.
- **Cryptographic authorization**: each spend must be authorized (signature/script verification against the referenced previous outputs).
- **State tracking (“not already spent”)**: we maintain a chainstate/UTXO view so we can reject double-spends deterministically at a chosen tip.
- **Proof-of-work + chain selection**: we validate PoW on headers and track the best chain (most work / “longest” in the whitepaper’s terminology).
- **A message-driven network loop**: we receive transactions/blocks, validate them, persist them, and relay them.

To make this maintainable in Rust, we keep clear boundaries between **types**, **bytes**, and **state**:

- **Types (Rust structs/enums)**: the in-memory model we program against (`Transaction`, `TxIn`, `TxOut`, `BlockHeader`, `Block`, …).
  - **Usefulness**: gives us compile-time structure and makes validation logic readable and testable.
- **Bytes (consensus/wire encoding)**: the exact serialized form we send on the network, hash for IDs/commitments, and sign/verify.
  - **Usefulness**: consensus is “agree on bytes” — if two nodes produce different bytes, they produce different hashes and disagree on validity.
- **State (databases / persistent views)**: the node’s durable view of “what is currently true”, primarily:
  - **mempool** (unconfirmed txs we might mine/relay),
  - **chainstate/UTXO set** (what outputs are spendable at the current best tip),
  - **block/header store** (history and indexing to support sync and reorgs).
  - **Usefulness**: turns validation rules like “not already spent” into a concrete lookup (`OutPoint -> TxOut`) at a chosen tip.

Transition / flow (from network bytes to “a transaction in a block”):

```text
NET (bytes)  ->  TYPES (Rust)  ->  BYTES (canonical)  ->  HASHES  ->  VALIDATE  ->  STATE  ->  COMMIT

1. Receive bytes from a peer (P2P message payload)
2. Decode bytes into Rust types (Transaction / Block / BlockHeader)
3. Serialize canonically back to bytes (the bytes we hash and sign/verify)
4. Hash bytes to get identifiers/commitments (txid / merkle_root / block_hash)
5. Validate:
   - scripts/sigs + value rules
   - “not already spent” via chainstate/UTXO lookups
6. Update state:
   - tx valid   => add to mempool (and relay)
   - block valid=> persist block + atomically update UTXO set (and relay)
7. Commit to history:
   - miners select mempool txs into a block
   - the header commits to them via merkle_root
```

```text
net (messages) -> decode -> validate -> mempool/chainstate -> persist -> relay
```

As we go through the next sections, we will keep asking the same engineering questions:

- **What exact bytes are hashed?**
- **What exact bytes are signed and verified (and with what context)?**
- **What state is read/written when we accept a transaction or a block?**

---

<div align="center">

**[← Business Objects](00-business-objects.md)** | Introduction (Bitcoin Whitepaper Section 1) | **[Transactions = chain of signatures (Bitcoin Whitepaper Section 2) →](02-Transactions-chain-of-signatures-Bitcoin-Whitepaper-Section-2.md)**

</div>
