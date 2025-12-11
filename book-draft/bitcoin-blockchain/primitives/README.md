# Primitives: Core Data Structures

**Part I: Core Blockchain Implementation** | **Chapter 2.6: Primitives**

<div align="center">

**📚 [← Chapter 2.5: Node Orchestration](../node/README.md)** | **Chapter 2.6: Primitives** | **[Chapter 2.7: Storage Layer →](../store/README.md)** 📚

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

- **[Blockchain State Management](../chain/README.md)**: Using primitives in state management
- **[Transaction System](../02-Transaction-System.md)**: Transaction structure and validation
- **[Cryptography](crypto/README.md)**: Hash functions and digital signatures
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

**📚 [← Previous: Chapter 2.5: Node Orchestration](../node/README.md)** | **Chapter 2.6: Primitives** | **[Next: Storage Layer →](../store/README.md)** 📚

</div>

---

*This chapter has examined the primitives module, which contains the core, pure data structures that form the foundation of the blockchain system. We've explored the `Block`, `Transaction`, and `Blockchain` structures—the atomic building blocks from which all blockchain operations are constructed. These primitives follow Bitcoin Core's architecture pattern, containing only data structures without business logic, ensuring clear separation of concerns. In the next chapter, we'll explore the [Storage Layer](../store/README.md) to understand how these primitive structures are persistently stored on disk using the Sled embedded database.*
