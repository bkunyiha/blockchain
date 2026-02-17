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
## Appendix A. Object connectivity: end-to-end flow
This is the “developers’s trace” of the paper’s network steps: a single narrative that ties the objects, hashes, and state transitions together.

1. Construct a `Transaction` with `inputs` + `outputs`.
2. For each input, look up the referenced output via `OutPoint(txid, vout)`:
   - this provides the **spend context** (amount + locking script).
3. Add per-input unlocking data (`TxIn.script_sig` in legacy, or witness data in SegWit) to satisfy the referenced output’s `script_pubkey`.
4. Serialize the transaction canonically and compute its `txid` (double-SHA256 of the consensus bytes).
5. Broadcast the transaction.
6. Each receiving node validates it:
   - signatures/scripts verify against the referenced prevouts
   - inputs exist in the node’s current UTXO view (“not already spent”)
   - values conserve (no money creation)
   - if valid, it is admitted to the mempool
7. A miner builds a `Block`:
   - choose txs (usually including high-fee ones)
   - compute txids → compute `merkle_root`
   - set `prev_hash` to the current best tip
   - set `difficulty_bits` for the network’s required target
8. Run PoW by incrementing `nonce` until `block_hash <= target(difficulty_bits)`.
9. Broadcast the block.
10. Peers validate and (if accepted) update chainstate:
   - verify header PoW + linkage
   - verify Merkle root + transaction validity
   - apply the block as an atomic UTXO transition (spend inputs, create outputs)
   - select the best chain (and potentially reorg if a stronger chain appears)

If we keep the byte-level rules consistent, the whole system reduces to these objects plus deterministic hashing and deterministic state updates.

---

<div align="center">

**[← Conclusion (Bitcoin Whitepaper Section 12)](12-Conclusion-Bitcoin-Whitepaper-Section-12.md)** | Appendix A: Object connectivity | **[Appendix B: Mapping to this repository →](Appendix-B-Mapping-to-this-repository.md)**

</div>
