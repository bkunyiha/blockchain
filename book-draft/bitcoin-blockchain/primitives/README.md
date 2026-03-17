<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. **Chapter 6: Primitives** ← *You are here*
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
# Primitives: Core Data Structures

**Part I: Foundations & Core Implementation** | **Chapter 6: Primitives**

<div align="center">

**[← Chapter 5: Rust Project](../Rust-Project-Index.md)** | **Chapter 6: Primitives** | **[Chapter 7: Utilities →](../util/README.md)** 
</div>

---

## Overview

In this chapter, we define the core data structures that every other module depends on. The primitives module (`bitcoin/src/primitives`) contains pure data types — `Block`, `Transaction`, `TransactionInput`, `TransactionOutput` — with no business logic attached. Following Bitcoin Core's separation of concerns, we keep the "what data looks like" question here and push "what to do with data" into the chain, node, and validation modules.

These are the atomic building blocks from which we construct all blockchain operations. By the end of this chapter, you will understand every field in a block and a transaction, why we chose `Vec<u8>` for hashes, and how Serde derives make these types serialization-ready from day one.

> **What you will learn in this chapter:**
> - Define the Block, Transaction, and Blockchain data structures that form the system's foundation
> - Explain why transaction IDs use `Vec<u8>` rather than hex strings
> - Describe the role of each field in the block header and transaction structure
> - Understand how these pure data types are used by every other module in the codebase

**Figure 6-1: Block Structure**

```text
┌─────────────────────────────────────────┐
│                   Block                      │
├─────────────────────────────────────────┤
│  ┌─────────────────────────────────────┐    │
│  │           Block Header              │    │
│  ├─────────────────────────────────────┤    │
│  │  hash:           [u8; 32]           │    │
│  │  previous_hash:  [u8; 32]  ──────────┐  │
│  │  timestamp:      i64                │ │  │
│  │  nonce:          u64                │ │  │
│  │  difficulty:     u32                │ │  │
│  │  merkle_root:    [u8; 32]           │ │  │
│  └─────────────────────────────────────┘ │  │
│                                           │  │
│  ┌─────────────────────────────────────┐ │  │
│  │        Transactions: Vec<Tx>        │ │  │
│  ├─────────────────────────────────────┤ │  │
│  │  tx[0]: Coinbase (miner reward)     │ │  │
│  │  tx[1]: Alice → Bob (3.5 BTC)      │ │  │
│  │  tx[2]: Bob → Carol (1.2 BTC)      │ │  │
│  │  ...                                │ │  │
│  └─────────────────────────────────────┘ │  │
└──────────────────────────────────────────┘  │
                                              │
     ┌────────────────────────────────────────┘
     │  Points to previous block's hash
     ▼
┌────────────────┐
│ Previous Block │
└────────────────┘
```

## Key Components

### Block

The `Block` structure represents a single block in the blockchain:

**Block Structure:**
- **BlockHeader**: Contains metadata (timestamp, previous hash, nonce, height)
- **Transactions**: Vector of transactions included in the block
- **Hash**: Cryptographic hash of the block
- **Merkle Root**: Root hash of transaction Merkle tree

**Key Operations:**
- Block creation and validation
- Hash calculation
- Serialization/deserialization
- Transaction inclusion

### Transaction

The `Transaction` structure represents a single transaction:

**Transaction Components:**
- **TXInput**: References to previous transaction outputs (UTXOs)
- **TXOutput**: New outputs created by the transaction
- **Transaction ID**: Hash of the transaction
- **Signatures**: Cryptographic signatures authorizing spends

**Transaction Types:**
- **Coinbase Transactions**: Mining rewards (no inputs)
- **Regular Transactions**: Transfers between addresses
- **Wallet Transactions**: User-facing transaction representation

### Blockchain

The `Blockchain` structure represents the complete chain:

**Blockchain Components:**
- **Blocks**: Collection of blocks in sequence
- **Tip Hash**: Hash of the latest block
- **Height**: Current chain height
- **Genesis Block**: First block in the chain

**Key Operations:**
- Block addition
- Chain validation
- Block retrieval
- Chain traversal

## Relationship to Bitcoin Core

This module aligns with Bitcoin Core's primitives directory:

- **Bitcoin Core's `primitives/block.h`**: Block data structure
- **Bitcoin Core's `primitives/transaction.h`**: Transaction data structure
- **Bitcoin Core's separation**: Pure data structures, no business logic

## Design Decisions

**Why pure data structures?** The primitives module deliberately contains no business logic — no validation, no persistence, no networking. This mirrors Bitcoin Core's `primitives/` directory and keeps the types reusable across every other module. A `Block` struct can be constructed, serialized, and passed around without pulling in database dependencies or consensus rules.

**Why `Vec<u8>` for hashes and IDs?** Transaction IDs and block hashes are stored as raw byte vectors rather than hex strings. This avoids repeated hex-encode/decode round-trips and keeps hashing deterministic (we hash bytes, not string representations). Chapter 6.1 (Transaction ID Format) explores this decision in depth, including comparisons with Bitcoin Core's approach.

> **Important:** Transaction IDs are stored as `Vec<u8>` (raw bytes) rather than hex strings throughout the codebase. This is a deliberate design choice: byte comparisons are faster, storage is more compact, and we avoid repeated hex encoding/decoding. You will see this pattern in every module that handles transaction IDs.

**Serde for serialization.** All primitives derive `Serialize` and `Deserialize`, enabling JSON serialization for the REST API (Chapter 15) and binary encoding via bincode for storage (Chapter 11). The `#[derive(Serialize, Deserialize)]` pattern appears on every struct in this module.

> **Warning:** Changing any field in a block — even a single byte — invalidates the block hash and breaks the chain of hashes linking it to subsequent blocks. This is what makes blockchain tamper-evident.

## How These Primitives Connect to Later Chapters

The types defined here flow through the rest of the system. Chapter 8 (Cryptography) hashes and signs them. Chapter 9 (Blockchain Core) validates them against consensus rules and manages UTXO state transitions. Chapter 11 (Storage Layer) persists them to disk via Sled. Chapter 12 (Network Layer) serializes them for peer-to-peer transmission. Understanding the shape of `Block`, `Transaction`, `TXInput`, and `TXOutput` is a prerequisite for every chapter that follows.

## Code Examples

**Creating a Block:**

```rust
use blockchain::primitives::block::Block;
use blockchain::primitives::transaction::Transaction;

// Create block with transactions
let block = Block::new_block(
    previous_hash,
    &transactions,
    height
);
```

**Creating a Transaction:**

```rust
use blockchain::primitives::transaction::{Transaction, TXInput, TXOutput};

// Create coinbase transaction
let coinbase = Transaction::new_coinbase_tx(&mining_address)?;

// Create regular transaction
let tx = Transaction::new_utxo_transaction(
    &from_address,
    &to_address,
    amount,
    &utxo_set
)
.await?;
```

**Blockchain Operations:**

```rust
use blockchain::chain::BlockchainService;

// Initialize blockchain with genesis address
let blockchain_service = BlockchainService::initialize(&genesis_address).await?;

// Add block to the chain
blockchain_service.add_block(&block).await?;

// Get block by hash
let block = blockchain_service.get_block_by_hash(&hash).await?;
```

---

## Exercises

1. **Observe the Avalanche Effect in Block Hashing** — Create a `Block` with a specific timestamp, then change the timestamp by one second. Hash both blocks and compare the output. Verify that even a tiny change produces a completely different hash, demonstrating why blockchain is tamper-evident.

2. **Trace Transaction ID Encoding** — Take a sample transaction and manually trace how its ID is computed: serialize the transaction fields, apply SHA-256, and store the result as `Vec<u8>`. Compare this with calling the actual `hash_transaction` function. Explain why the ID changes if any field is modified.

---

## Further Reading

- **[Bitcoin Block Structure (Bitcoin Wiki)](https://en.bitcoin.it/wiki/Block)** — Detailed specification of the Bitcoin block format.
- **[serde_derive Documentation](https://docs.rs/serde_derive/)** — How derive macros generate serialization code for our data structures.
- **[Bitcoin Transaction (Bitcoin Wiki)](https://en.bitcoin.it/wiki/Transaction)** — Reference for the transaction data model.

---

## What We Covered

- We defined the Block, Transaction, and Blockchain structs that serve as the atomic building blocks for every operation in the system.
- We explained the Transaction ID format and the deliberate choice to store IDs as `Vec<u8>` rather than hex strings, optimizing for programmatic use.
- We examined every field in the block header — hash, previous hash, timestamp, nonce, difficulty — and its role in blockchain integrity.
- We saw how these pure data types remain dependency-free, allowing every other module to build on them without circular references.

In the next chapter, we build the utility functions that operate on these structures — timestamps, functional helpers, and cross-cutting concerns that every module in the system needs.

---

<div align="center">

**[← Chapter 5: Rust Project](../Rust-Project-Index.md)** | **Chapter 6: Primitives** | **[Transaction ID Format →](02-Transaction-ID-Format.md)**
</div>

