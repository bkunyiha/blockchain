<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../ci/kubernetes/README.md) - Kubernetes production guide
22. **Chapter 10: Rust Language Guide** ← *You are here*

</details>

</div>

---
# Testing: Writing Reliable Rust Code

Testing is fundamental to building reliable blockchain software. Rust's type system catches many errors at compile time, but comprehensive testing ensures our code behaves correctly at runtime. In this chapter, we'll explore Rust's testing capabilities, different testing approaches, and how we use them in our blockchain implementation.

Testing in Rust is built into the language and tooling. The `cargo test` command runs all tests, and Rust's test framework provides everything we need for unit tests, integration tests, and more. We'll see how Rust's ownership system and type safety make testing easier and more reliable, and examine the specific testing patterns used in our blockchain codebase.

## Table of Contents

1. [Rust's Built-in Testing](#rusts-built-in-testing)
2. [Different Ways to Write Tests](#different-ways-to-write-tests)
3. [How Our Project Handles Tests](#how-our-project-handles-tests)
4. [Unit Tests](#unit-tests)
5. [Integration Tests](#integration-tests)
6. [Async Testing](#async-testing)
7. [Test Organization](#test-organization)
8. [Test Helpers and Fixtures](#test-helpers-and-fixtures)
9. [Test Coverage and Metrics](#test-coverage-and-metrics)

---

## Rust's Built-in Testing

Rust includes a comprehensive testing framework that requires no external dependencies. The `#[test]` attribute marks functions as tests, and `cargo test` discovers and runs them automatically.

### Basic Test Structure

```rust
// Simple unit test
#[test]
fn test_transaction_creation() {
    let tx = Transaction::new();
    assert!(tx.is_valid());
}

// Test with assertions
#[test]
fn test_transaction_id_generation() {
    let tx = create_test_transaction();
    let id = tx.get_id();
    
    assert_eq!(id.len(), 32);  // Transaction ID is 32 bytes
    assert!(!id.is_empty());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output (show println! output)
cargo test -- --nocapture

# Run a specific test
cargo test test_transaction_creation

# Run tests matching a pattern
cargo test transaction

# Run tests in parallel (default) or sequentially
cargo test -- --test-threads=1  # Sequential (needed for file-based DB)
cargo test -- --test-threads=4  # Parallel (default)

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run ignored tests
cargo test -- --ignored
```

---

## Different Ways to Write Tests

Rust provides multiple approaches to testing, each suited for different scenarios. Understanding these different approaches helps us choose the right testing strategy.

### 1. Standard Unit Tests with `#[test]`

The most common approach for synchronous code:

```rust
#[test]
fn test_hash_function() {
    let data = b"test data";
    let hash = sha256_digest(data);
    
    assert_eq!(hash.len(), 32);
    assert_eq!(hash, sha256_digest(data));  // Deterministic
}
```

**When to use:** Synchronous functions, pure functions, data transformations.

### 2. Async Tests with `#[tokio::test]`

For testing async code, we use Tokio's test attribute:

```rust
#[tokio::test]
async fn test_async_blockchain_operation() {
    let blockchain = create_test_blockchain().await;
    let result = blockchain.process_transaction(tx).await;
    assert!(result.is_ok());
}
```

**When to use:** Async functions, database operations, network operations, concurrent code.

**From our codebase:**

```rust
// src/chain/chainstate.rs
#[tokio::test]
async fn test_blockchain_creation() {
    crate::setup_test_environment();
    let test_blockchain = TestBlockchain::new().await;
    
    assert_eq!(
        test_blockchain.blockchain().get_best_height().await.unwrap(),
        1
    );
    
    crate::teardown_test_environment();
}
```

### 3. Test Modules with `#[cfg(test)]`

Tests are organized in modules that are only compiled in test mode:

```rust
// src/primitives/transaction.rs

pub struct Transaction {
    // ... implementation
}

// Test module - only compiled when running tests
#[cfg(test)]
mod tests {
    use super::*;  // Import parent module items
    
    #[test]
    fn test_transaction_creation() {
        // Test code
    }
    
    // Nested test modules for organization
    mod serialization {
        use super::*;
        
        #[test]
        fn test_serialize() { }
        
        #[test]
        fn test_deserialize() { }
    }
}
```

**When to use:** Unit tests in the same file as the code being tested.

### 4. Integration Tests in `tests/` Directory

Integration tests live in separate files in the `tests/` directory:

```rust
// tests/integration_tests.rs

use blockchain::{BlockchainService, Transaction};

#[tokio::test]
async fn test_end_to_end_workflow() {
    // Test multiple components working together
}
```

**When to use:** Testing component interactions, end-to-end workflows, external interfaces.

### 5. Doc Tests

Tests embedded in documentation that serve as examples:

```rust
/// Creates a new transaction.
///
/// # Example
///
/// ```
/// use blockchain::Transaction;
/// let tx = Transaction::new();
/// assert!(tx.is_valid());
/// ```
pub fn new() -> Transaction {
    // Implementation
}
```

**When to use:** Documentation examples that should also be tested.

### 6. Test Fixtures with RAII

Using Rust's ownership system for automatic cleanup:

```rust
struct TestDatabaseGuard {
    db_path: String,
}

impl Drop for TestDatabaseGuard {
    fn drop(&mut self) {
        // Automatic cleanup
        let _ = std::fs::remove_dir_all(&self.db_path);
    }
}

#[test]
fn test_with_fixture() {
    let _guard = TestDatabaseGuard::new();
    // Test code - guard automatically cleans up when test ends
}
```

**When to use:** Tests that need setup/teardown, resource management.

### 7. Parameterized Tests

Testing multiple inputs with a single test function:

```rust
#[test]
fn test_hash_known_values() {
    let test_cases = vec![
        (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    ];
    
    for (input, expected_hex) in test_cases {
        let hash = sha256_digest(input);
        let expected = hex::decode(expected_hex).unwrap();
        assert_eq!(hash, expected);
    }
}
```

**When to use:** Testing the same logic with different inputs.

### 8. Property-Based Testing

Using `proptest` for generating random test cases:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_transaction_serialization_roundtrip(tx in any::<Transaction>()) {
        let bytes = bincode::serialize(&tx).unwrap();
        let deserialized: Transaction = bincode::deserialize(&bytes).unwrap();
        assert_eq!(tx.get_id(), deserialized.get_id());
    }
}
```

**When to use:** Testing invariants that should hold for all inputs.

---

## How Our Project Handles Tests

Our blockchain project uses a comprehensive testing strategy that combines multiple approaches. Let's examine how tests are organized and executed.

### Test Structure

```
bitcoin/
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── primitives/
│   │   ├── transaction.rs        # Contains #[cfg(test)] mod tests
│   │   └── block.rs             # Contains #[cfg(test)] mod tests
│   ├── crypto/
│   │   ├── hash.rs              # Unit tests for hash functions
│   │   └── signature.rs         # Unit tests for signatures
│   └── chain/
│       └── chainstate.rs         # Async unit tests with TestBlockchain fixture
└── tests/
    ├── integration_tests.rs     # Integration tests
    └── test_helpers.rs          # Shared test utilities
```

### Test Categories in Our Project

**1. Unit Tests (158 tests)**

Located in `#[cfg(test)]` modules within source files:

```rust
// src/crypto/hash.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256_digest_basic() {
        let data = b"Block Chain Project";
        let hash = sha256_digest(data);
        
        assert_eq!(hash.len(), 32);
        let hash2 = sha256_digest(data);
        assert_eq!(hash, hash2);  // Deterministic
    }
    
    #[test]
    fn test_sha256_digest_known_values() {
        let test_cases = vec![
            (
                b"abc".as_slice(),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            // ... more test cases
        ];
        
        for (input, expected_hex) in test_cases {
            let hash = sha256_digest(input);
            let expected = hex::decode(expected_hex).unwrap();
            assert_eq!(hash, expected);
        }
    }
    
    #[test]
    fn test_hash_avalanche_effect() {
        // Test that small changes produce completely different hashes
        let hash1 = sha256_digest(b"Hello, World?");
        let hash2 = sha256_digest(b"Hello, World.");
        
        // Count different bits
        let diff_bits = hash1.iter()
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>();
        
        assert!(diff_bits > 100, "Hash should show avalanche effect");
    }
}
```

**2. Async Unit Tests (8 tests)**

Using `#[tokio::test]` for async blockchain operations:

```rust
// src/chain/chainstate.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test fixture with automatic cleanup
    struct TestBlockchain {
        blockchain: BlockchainService,
        db_path: String,
    }
    
    impl TestBlockchain {
        async fn new() -> Self {
            // Create unique database path
            let db_path = format!("test_blockchain_db_{}", uuid::Uuid::new_v4());
            // ... setup blockchain
            TestBlockchain { blockchain, db_path }
        }
    }
    
    impl Drop for TestBlockchain {
        fn drop(&mut self) {
            // Automatic cleanup
            let _ = std::fs::remove_dir_all(&self.db_path);
        }
    }
    
    #[tokio::test]
    async fn test_blockchain_creation() {
        crate::setup_test_environment();
        let test_blockchain = TestBlockchain::new().await;
        
        assert_eq!(
            test_blockchain.blockchain().get_best_height().await.unwrap(),
            1
        );
        
        crate::teardown_test_environment();
    }
    
    #[tokio::test]
    async fn test_add_block() {
        let test_blockchain = TestBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();
        
        let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address).unwrap();
        let new_block = test_blockchain
            .blockchain()
            .mine_block(&[coinbase_tx])
            .await
            .unwrap();
        
        test_blockchain.blockchain().add_block(&new_block).await.unwrap();
        
        assert_eq!(
            test_blockchain.blockchain().get_best_height().await.unwrap(),
            2
        );
    }
}
```

**3. Integration Tests (6 tests)**

Located in `tests/integration_tests.rs`:

```rust
// tests/integration_tests.rs

use blockchain::{BlockchainService, Transaction, UTXOSet, WalletService};

mod test_helpers;

#[tokio::test]
async fn test_blockchain_integration() {
    let (blockchain, db_path) = create_test_blockchain().await;
    let genesis_address = generate_test_genesis_address();
    
    // Test blockchain creation
    assert!(validate_blockchain_height(&blockchain, 1).await);
    
    // Test mining and adding blocks
    let _new_block = create_and_add_block(&blockchain, &genesis_address).await;
    assert!(validate_blockchain_height(&blockchain, 2).await);
    
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_wallet_integration() {
    let (mut wallets, _temp_dir) = create_wallet_with_temp_path();
    let address = wallets.create_wallet().expect("Failed to create wallet");
    
    let wallet = wallets.get_wallet(&address).expect("Failed to get wallet");
    assert_eq!(wallet.get_address().unwrap(), address);
}

#[tokio::test]
async fn test_utxo_set_integration() {
    let (blockchain, db_path) = create_test_blockchain().await;
    let genesis_address = generate_test_genesis_address();
    
    // Create blockchain and add blocks
    create_and_add_block(&blockchain, &genesis_address).await;
    
    // Create and reindex UTXO set
    let utxo_set = create_and_reindex_utxo_set(blockchain).await;
    
    // Verify UTXO set operations
    // ...
    
    cleanup_test_blockchain(&db_path);
}
```

### Test Environment Setup

Our project uses a global test setup to handle database locking and cleanup:

```rust
// src/lib.rs

#[cfg(test)]
mod test_utils {
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    pub fn setup_test_environment() {
        INIT.call_once(|| {
            // Force single-threaded tests to avoid database locks
            unsafe {
                std::env::set_var("RUST_TEST_THREADS", "1");
            }
            cleanup_existing_test_directories();
        });
    }
    
    pub fn teardown_test_environment() {
        cleanup_existing_test_directories();
    }
    
    fn cleanup_existing_test_directories() {
        // Clean up test database directories
        // Handles file system locks with retry logic
    }
}
```

### Test Helpers

Shared utilities in `tests/test_helpers.rs`:

```rust
// tests/test_helpers.rs

use blockchain::{BlockchainService, Transaction};
use tempfile::TempDir;

pub fn generate_test_genesis_address() -> WalletAddress {
    blockchain::Wallet::new()
        .and_then(|wallet| wallet.get_address())
        .expect("Failed to create test wallet address")
}

pub async fn create_test_blockchain() -> (BlockchainService, String) {
    let db_path = create_unique_db_path();
    let genesis_address = generate_test_genesis_address();
    let blockchain = create_blockchain_with_config_clean(&genesis_address, &db_path).await;
    (blockchain, db_path)
}

pub fn create_coinbase_transaction(address: &WalletAddress) -> Transaction {
    Transaction::new_coinbase_tx(address)
        .expect("Failed to create coinbase transaction")
}

pub async fn mine_block(
    blockchain: &BlockchainService,
    transactions: &[Transaction],
) -> blockchain::Block {
    blockchain.mine_block(transactions)
        .await
        .expect("Failed to mine block")
}

pub async fn create_and_add_block(
    blockchain: &BlockchainService,
    address: &WalletAddress,
) -> blockchain::Block {
    let coinbase_tx = create_coinbase_transaction(address);
    let transactions = vec![coinbase_tx];
    let block = mine_block(blockchain, &transactions).await;
    add_block(blockchain, &block).await;
    block
}
```

### Running Tests in Our Project

Due to file-based database usage, we run unit tests sequentially:

```bash
# Run all tests (unit tests run sequentially to avoid DB locks)
cargo test --lib -- --test-threads=1

# Run integration tests
cargo test --test integration_tests

# Run specific test category
cargo test test_sha256  # Hash function tests
cargo test test_blockchain  # Blockchain operation tests

# Run with output for debugging
cargo test -- --nocapture
```

### Test Coverage Breakdown

Our project has **164 tests** organized as follows:

- **Consensus Mechanisms** (35 tests): Chain reorganization, tie-breaking, work calculation
- **Cryptography** (55 tests): Schnorr signatures, ECDSA, P2TR addresses, SHA-256 hashing
- **Blockchain Operations** (30 tests): Block creation, transaction processing, UTXO management
- **Network Operations** (12 tests): Message serialization, protocol operations
- **Integration Tests** (6 tests): End-to-end workflows, multi-component interaction

---

## Unit Tests

Unit tests verify individual components in isolation. In our project, unit tests are located in `#[cfg(test)]` modules within source files.

### Example: Hash Function Tests

From `src/crypto/hash.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256_digest_basic() {
        let data = b"Block Chain Project";
        let hash = sha256_digest(data);
        
        // SHA-256 should always produce 32 bytes
        assert_eq!(hash.len(), 32);
        
        // Hash should be deterministic
        let hash2 = sha256_digest(data);
        assert_eq!(hash, hash2);
    }
    
    #[test]
    fn test_sha256_digest_empty() {
        let data = b"";
        let hash = sha256_digest(data);
        
        assert_eq!(hash.len(), 32);
        
        // Known empty string SHA-256 hash
        let expected = hex::decode(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        ).unwrap();
        assert_eq!(hash, expected);
    }
    
    #[test]
    fn test_sha256_digest_known_values() {
        let test_cases = vec![
            (
                b"abc".as_slice(),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_slice(),
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ];
        
        for (input, expected_hex) in test_cases {
            let hash = sha256_digest(input);
            let expected = hex::decode(expected_hex).unwrap();
            assert_eq!(hash, expected, "Hash mismatch for input: {:?}", input);
        }
    }
}
```

### Testing Error Cases

```rust
#[test]
fn test_invalid_transaction_rejected() {
    let invalid_tx = create_invalid_transaction();
    
    match Transaction::validate(&invalid_tx) {
        Ok(_) => panic!("Should have rejected invalid transaction"),
        Err(e) => {
            assert!(matches!(e, TransactionError::InvalidInput));
        }
    }
}
```

---

## Integration Tests

Integration tests verify that multiple components work together correctly. In our project, they're in `tests/integration_tests.rs`.

### Example: End-to-End Blockchain Test

```rust
#[tokio::test]
async fn test_blockchain_integration() {
    let (blockchain, db_path) = create_test_blockchain().await;
    let genesis_address = generate_test_genesis_address();
    
    // Test blockchain creation
    assert!(validate_blockchain_height(&blockchain, 1).await);
    
    // Test mining a block
    let _new_block = create_and_add_block(&blockchain, &genesis_address).await;
    assert!(validate_blockchain_height(&blockchain, 2).await);
    
    // Cleanup
    cleanup_test_blockchain(&db_path);
}
```

---

## Async Testing

Many blockchain operations are asynchronous. We use `#[tokio::test]` for async tests.

### Example: Async Blockchain Operations

```rust
#[tokio::test]
async fn test_add_block() {
    let test_blockchain = TestBlockchain::new().await;
    let genesis_address = generate_test_genesis_address();
    
    let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address).unwrap();
    let new_block = test_blockchain
        .blockchain()
        .mine_block(&[coinbase_tx])
        .await
        .unwrap();
    
    test_blockchain.blockchain().add_block(&new_block).await.unwrap();
    
    assert_eq!(
        test_blockchain.blockchain().get_best_height().await.unwrap(),
        2
    );
}
```

---

## Test Organization

### Test Modules by Feature

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    mod creation {
        use super::*;
        #[test] fn test_new_transaction() { }
    }
    
    mod validation {
        use super::*;
        #[test] fn test_valid_transaction() { }
    }
    
    mod serialization {
        use super::*;
        #[test] fn test_serialize() { }
    }
}
```

---

## Test Helpers and Fixtures

### RAII-Based Cleanup

```rust
struct TestBlockchain {
    blockchain: BlockchainService,
    db_path: String,
}

impl Drop for TestBlockchain {
    fn drop(&mut self) {
        // Automatic cleanup when test ends
        let _ = std::fs::remove_dir_all(&self.db_path);
    }
}
```

---

## Test Coverage and Metrics

### Running Tests with Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

### Our Test Metrics

- **Total Tests**: 164 (100% passing)
- **Unit Tests**: 158
- **Integration Tests**: 6
- **Test Categories**: 5 (Consensus, Cryptography, Blockchain, Network, Integration)

---

## Summary

Our project uses multiple testing approaches:

1. **Unit Tests**: `#[test]` in `#[cfg(test)]` modules for synchronous code
2. **Async Tests**: `#[tokio::test]` for async operations
3. **Integration Tests**: Separate files in `tests/` directory
4. **Test Fixtures**: RAII-based cleanup with `Drop` trait
5. **Test Helpers**: Shared utilities in `tests/test_helpers.rs`
6. **Sequential Execution**: Required for file-based database tests

**Key Patterns:**

- Tests are co-located with code (`#[cfg(test)]` modules)
- Async tests use `#[tokio::test]`
- Automatic cleanup via `Drop` trait
- Unique database paths prevent conflicts
- Sequential execution avoids database locks

---

## Navigation

- **[← Previous: Type Conversions](15-Type-Conversions.md)** - Converting between types
- **[Next: Best Practices →](17-Best-Practices.md)** - Rust idioms, patterns, and testing best practices
- **[Rust Guide Index](README.md)** - Complete guide overview and table of contents
- **[Error Handling](05-Error-Handling.md)** - Result and Option types
- **[Async/Await](11-Async-Await.md)** - Asynchronous programming

**Related Guides:**
- **[Transaction ID Format](../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - See testing in practice
- **[Web API Architecture](../bitcoin-blockchain/web/README.md)** - API testing patterns

---

<div align="center">

**📚 [← Rust Guide Index](README.md)** | **Testing** | **[← Previous: Type Conversions](15-Type-Conversions.md)** | **[Next: Best Practices →](17-Best-Practices.md)** 📚

</div>

---

*This chapter covers testing in Rust. Continue to [Best Practices](17-Best-Practices.md) for production patterns and testing best practices.*
