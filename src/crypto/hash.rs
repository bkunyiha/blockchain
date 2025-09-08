//! # Hash Functions Module
//!
//! This module provides cryptographic hash functions for the blockchain.
//!
//! ## Key Differences Between `sha256_digest` and `taproot_hash`
//!
//! ### 1. Underlying Libraries
//! - **`sha256_digest`**: Uses the `ring` crate (`ring::digest::Context` and `ring::digest::SHA256`)
//! - **`taproot_hash`**: Uses the `sha2` crate (`sha2::Sha256`)
//!
//! ### 2. Purpose and Context
//!
//! **`sha256_digest`**:
//! - General-purpose SHA-256 hashing
//! - Used throughout the blockchain for various hashing needs
//! - Part of the core cryptographic infrastructure
//! - Used in transaction signing
//!
//! **`taproot_hash`**:
//! - Specifically designed for Taproot (P2TR) compatibility
//! - Part of Bitcoin's Taproot upgrade implementation
//! - Used for P2TR (Pay-to-Taproot) address generation
//! - Replaces RIPEMD160 for better security
//!
//! ## Usage Locations
//!
//! ### `sha256_digest` Usage:
//! - **`src/domain/transaction.rs`**: Transaction ID generation (`hash()` method)
//! - **`src/domain/block.rs`**: Merkle tree root calculation
//! - **`src/domain/proof_of_work.rs`**: Block hash calculation for mining
//! - **`src/util/utils.rs`**: Message hashing for Schnorr signatures
//! - **`src/crypto/hash.rs`**: Core hashing infrastructure
//!
//! ### `taproot_hash` Usage:
//! - **`src/domain/wallet.rs`**: Public key hashing for P2TR addresses (`hash_pub_key()`)
//! - **`src/service/wallet_service.rs`**: Wallet service public key hashing
//! - **`src/crypto/hash.rs`**: Core Taproot hashing infrastructure
//!
//! ### Indirect Usage via `hash_pub_key`:
//! - **`src/domain/transaction.rs`**: Transaction input validation (`uses_key()` method)
//! - **`src/domain/transaction.rs`**: UTXO transaction creation
//! - **`src/store/file_system_db_chain.rs`**: Transaction summary generation
//! - **`src/main.rs`**: Address validation and transaction processing
//!
//! ### 3. Why Two Different Libraries?
//!
//! This appears to be a legacy issue and inconsistency in the codebase:
//! - **Historical Reasons**: The codebase likely started with `ring` for general hashing
//! - **Taproot Implementation**: When Taproot support was added, `sha2` was chosen for the new functionality
//! - **Different Requirements**: Taproot has specific requirements that might have led to choosing `sha2`
//!
//! ### 4. Technical Differences
//!
//! Both functions produce identical SHA-256 output, but:
//! - **`ring`**: More comprehensive cryptographic library, used by other parts of the codebase
//! - **`sha2`**: More focused on hashing algorithms, potentially lighter weight
//!
//! ## Recommendation
//!
//! The codebase should be refactored to use a single SHA-256 implementation for consistency,
//! reduced dependencies, and improved maintainability.

use ring::digest::{Context, SHA256};
use sha2::{Digest as Sha2Digest, Sha256 as Sha2Hash};

///
/// Hash functions are used to create a unique identifier for a block or transaction.
///
/// The `sha256_digest` function performs a SHA-256 hash operation on the provided data input,
/// returning the resulting hash as a vector of bytes.
/// It initializes a hashing context with SHA-256, updates the context with the input data,
/// generates the hash digest, and converts it to a vector of bytes for output.
///
/// # Usage Examples
///
/// - **Transaction ID generation**: Used in `Transaction::hash()` to create unique transaction identifiers
/// - **Block hashing**: Used in `Block::get_hash()` for Merkle tree root calculation
/// - **Mining**: Used in `ProofOfWork::run()` for block hash calculation during mining
/// - **Signature verification**: Used in Schnorr signature verification for message hashing
///
/// # Arguments
///
/// * `data` - A reference to the input data to be hashed.
///
/// # Returns
///
/// A 32-byte SHA-256 hash as a vector of bytes.
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

///
/// The `taproot_hash` function calculates the Taproot-compatible hash of the input data.
///
/// For P2TR (Pay-to-Taproot), we use SHA256 as the primary hash function instead of RIPEMD160.
/// This provides better security and is compatible with Bitcoin's Taproot upgrade.
/// The function takes input data and returns a 32-byte hash suitable for P2TR addresses.
///
/// # Usage Examples
///
/// - **Address generation**: Used in `hash_pub_key()` for P2TR address creation
/// - **Wallet operations**: Used in wallet service for public key hashing
/// - **Transaction validation**: Used indirectly through `hash_pub_key()` in transaction input validation
/// - **UTXO management**: Used in UTXO set operations for address-based lookups
///
/// # Arguments
///
/// * `data` - A reference to the input data to be hashed (typically a public key).
///
/// # Returns
///
/// A 32-byte hash as a vector of bytes, suitable for P2TR address generation.
pub fn taproot_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha2Hash::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
