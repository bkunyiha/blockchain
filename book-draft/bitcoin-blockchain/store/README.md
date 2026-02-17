<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. Chapter 1.4: Bitcoin Whitepaper In Rust - Bitcoin Whitepaper In Rust
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. **Chapter 2.5: Storage Layer** ← *You are here*
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
# Storage Layer

**Part I: Core Blockchain Implementation** | **Chapter 2.7: Storage Layer**

<div align="center">

**[📚 ← Chapter 2.4: Chain(POW & BLockchain)](../chain/README.md)** | **[Chapter 2.5: Storage Layer](README.md)** | **[Chapter 2.6: Chainstate / UTXO →](../chain/02-Blockchain-State-Management.md)** 📚

</div>

---

## Overview

The storage layer (`bitcoin/src/store`) provides persistent storage for blockchain data using a file system-based database. This module implements the `BlockchainFileSystem` structure, which uses the Sled embedded database to store blocks, chain state, and UTXO data on disk.

Following Bitcoin Core's architecture, this module handles the low-level storage operations, providing a clean interface for the chain module to persist and retrieve blockchain data efficiently.

## Key Components

### BlockchainFileSystem

The `BlockchainFileSystem` is the primary storage interface:

**Storage Structure:**
- **Blocks Tree**: Stores blocks indexed by hash
- **Tip Hash Key**: Tracks the current chain tip
- **Database Path**: Configurable storage location
- **Sled Database**: Embedded key-value database

**Key Operations:**
- Blockchain creation and initialization
- Block persistence and retrieval
- Tip hash management
- Database transaction support
- Chain state persistence

### Storage Architecture

**Database Organization:**
- Blocks stored in Sled tree structure
- Hash-based block indexing
- Efficient block retrieval
- Transaction-safe operations

**File System Integration:**
- Configurable data directory
- Environment variable configuration
- Directory creation and management
- Data persistence guarantees

## Relationship to Bitcoin Core

This module aligns with Bitcoin Core's storage architecture:

- **Bitcoin Core's `chainstate/`**: UTXO set storage
- **Bitcoin Core's `blocks/`**: Block file storage
- **Bitcoin Core's LevelDB**: Similar to our Sled database usage
- **Bitcoin Core's `CBlockIndex`**: Block indexing functionality

## Topics to Cover

### Core Concepts

1. **Storage Architecture**
   - File system-based storage design
   - Database tree organization
   - Block indexing strategies
   - Storage path management

2. **Sled Database Integration**
   - Sled database usage patterns
   - Tree creation and management
   - Key-value storage patterns
   - Transaction support

3. **Block Persistence**
   - Block serialization for storage
   - Hash-based block retrieval
   - Block file organization
   - Storage efficiency

### Implementation Details

4. **Blockchain Initialization**
   - Genesis block creation
   - Database setup
   - Initial state persistence
   - Empty blockchain handling

5. **Block Operations**
   - Adding blocks to storage
   - Retrieving blocks by hash
   - Tip hash updates
   - Block chain traversal

6. **State Persistence**
   - Chain state storage
   - Tip hash management
   - State recovery
   - Checkpoint mechanisms

### Advanced Topics

7. **Database Transactions**
   - Atomic operations
   - Transaction batching
   - Rollback handling
   - Consistency guarantees

8. **Performance Optimization**
   - Batch operations
   - Caching strategies
   - Index optimization
   - Storage compaction

9. **Data Migration and Upgrades**
   - Schema evolution
   - Data migration patterns
   - Version compatibility
   - Backup and recovery

## Related Chapters

- **Blockchain State Management**: Using storage layer
- **Primitives**: Data structures being stored
- **Node Orchestration**: Storage access patterns

## Code Examples

**Creating Blockchain Storage:**

```rust
use blockchain::store::BlockchainFileSystem;
use blockchain::WalletAddress;

// Create new blockchain with genesis block
let blockchain = BlockchainFileSystem::create_blockchain(&genesis_address).await?;

// Open existing blockchain
let blockchain = BlockchainFileSystem::open_blockchain().await?;
```

**Block Storage Operations:**

```rust
// Add block to storage
blockchain.add_block(block).await?;

// Get block by hash
let block = blockchain.get_block(&hash).await?;

// Get tip hash
let tip_hash = blockchain.get_tip_hash().await?;
```

**Database Configuration:**

```rust
// Set storage directory via environment variable
std::env::set_var("TREE_DIR", "./blockchain_data");
std::env::set_var("BLOCKS_TREE", "blocks");

// Open blockchain (uses environment variables)
let blockchain = BlockchainFileSystem::open_blockchain().await?;
```

---

<div align="center">

**[📚 ← Chapter 2.4: Chain(POW & BLockchain)](../chain/README.md)** | **[Chapter 2.5: Storage Layer](README.md)** | **[Chapter 2.6: Chainstate / UTXO →](../chain/02-Blockchain-State-Management.md)** 📚

</div>

---

*This chapter has explored the storage layer that provides persistent storage for blockchain data using a file system-based database. We've examined how the `BlockchainFileSystem` structure uses the Sled embedded database to store blocks, chain state, and UTXO data on disk, following Bitcoin Core's storage architecture patterns. The storage layer handles low-level storage operations, providing a clean interface for the chain module to persist and retrieve blockchain data efficiently. In the next chapter, we'll examine the Utilities module to understand the helper functions and utility operations used throughout the blockchain system.*
