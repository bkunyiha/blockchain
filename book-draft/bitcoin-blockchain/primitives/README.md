<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. **Chapter 2.1: Primitives** ← *You are here*
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
# Primitives: Core Data Structures

**Part I: Core Blockchain Implementation** | **Chapter 2.1: Primitives**

<div align="center">

**📚 [← Chapter 2.0: Rust Project](../Rust-Project-Index.md)** | **Chapter 2.1: Primitives** | **[Chapter 2.2: Utilities →](../util/README.md)** 📚

</div>

---

## Overview

The primitives module (`bitcoin/src/primitives`) contains the core, pure data structures that form the foundation of the blockchain system. Following Bitcoin Core's architecture pattern, this module contains ONLY data structures—no business logic, which belongs in other modules (chain, node, validation, etc.).

These primitives are the atomic building blocks from which all blockchain operations are constructed. They define the structure of blocks, transactions, and the blockchain itself, providing the fundamental types used throughout the entire system.

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

## Topics to Cover

### Core Concepts

1. **Block Structure**
   - Block header components
   - Transaction inclusion
   - Hash calculation algorithms
   - Merkle tree construction
   - Block serialization formats

2. **Transaction Structure**
   - Input/output model (UTXO)
   - Transaction ID calculation
   - Signature structure
   - Coinbase vs regular transactions
   - Transaction serialization

3. **Blockchain Structure**
   - Chain linking mechanism
   - Genesis block creation
   - Chain validation rules
   - Block indexing

### Implementation Details

4. **Hash Functions**
   - SHA-256 usage in blocks
   - Transaction ID hashing
   - Merkle root calculation
   - Hash-based linking

5. **Serialization**
   - Serde serialization patterns
   - Binary encoding (bincode)
   - JSON serialization for network
   - Serialization performance

6. **Validation Primitives**
   - Structural validation
   - Hash verification
   - Signature format validation
   - Data integrity checks

### Advanced Topics

7. **Merkle Trees**
   - Merkle tree construction
   - Merkle proof generation
   - Efficient verification
   - Sparse Merkle trees

8. **Transaction Scripts**
   - Script structure (if implemented)
   - Script validation
   - Script execution
   - Script optimization

9. **Type Safety**
   - Rust type system usage
   - Zero-cost abstractions
   - Memory safety guarantees
   - Type-driven design

## Related Chapters

- **[Blockchain State Management](../chain/01-Technical-Foundations.md)**: Using primitives in state management
- **[Transaction ID Format](02-Transaction-ID-Format.md)**: Transaction ID representation and storage
- **[Cryptography](../crypto/README.md)**: Hash functions and digital signatures
- **[Storage Layer](../store/README.md)**: Persisting primitive structures

## Code Examples

**Creating a Block:**

```rust
use blockchain::primitives::block::Block;
use blockchain::primitives::transaction::Transaction;

// Create block with transactions
let block = Block::new_block(
    transactions,
    previous_hash,
    height
)?;
```

**Creating a Transaction:**

```rust
use blockchain::primitives::transaction::{Transaction, TXInput, TXOutput};

// Create coinbase transaction
let coinbase = Transaction::new_coinbase_tx(&mining_address)?;

// Create regular transaction
let tx = Transaction::new_utxo_transaction(
    from_address,
    to_address,
    amount,
    utxos,
    wallet_service
)?;
```

**Blockchain Operations:**

```rust
use blockchain::primitives::blockchain::Blockchain;

// Create blockchain
let mut blockchain = Blockchain::new();

// Add block
blockchain.add_block(block)?;

// Get block by hash
let block = blockchain.get_block(&hash)?;
```

---

<div align="center">

**📚 [← Chapter 2.0: Rust Project](../Rust-Project-Index.md)** | **Chapter 2.1: Primitives** | **[Transaction ID Format →](02-Transaction-ID-Format.md)** 📚

</div>

---

*This chapter has examined the primitives module, which contains the core, pure data structures that form the foundation of the blockchain system. We've explored the `Block`, `Transaction`, and `Blockchain` structures—the atomic building blocks from which all blockchain operations are constructed. These primitives follow Bitcoin Core's architecture pattern, containing only data structures without business logic, ensuring clear separation of concerns. In the next chapter, we'll explore the [Storage Layer](../store/README.md) to understand how these primitive structures are persistently stored on disk using the Sled embedded database.*
