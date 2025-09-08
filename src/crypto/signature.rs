//! # Signature Functions Module
//!
//! This module provides cryptographic signature functions for the blockchain.
//!
//! ## Four Signature Functions Overview
//!
//! This module contains four main signature functions that serve different purposes in the blockchain:
//!
//! ### 1. ECDSA Functions (Legacy/Alternative)
//! - **`ecdsa_p256_sha256_sign_digest`**: Signs messages with ECDSA P-256 SHA-256 signatures
//! - **`ecdsa_p256_sha256_sign_verify`**: Verifies ECDSA P-256 SHA-256 signatures
//!
//! ### 2. Schnorr Functions (Primary/Modern)
//! - **`schnorr_sign_digest`**: Signs messages with Schnorr signatures (P2TR/Taproot)
//! - **`schnorr_sign_verify`**: Verifies Schnorr signatures (P2TR/Taproot)
//!
//! ## Why Four Different Functions?
//!
//! ### 1. **ECDSA vs Schnorr - Different Signature Schemes**
//! - **ECDSA**: Traditional signature scheme used in older Bitcoin implementations
//! - **Schnorr**: Modern signature scheme introduced with Bitcoin's Taproot upgrade
//! - **Purpose**: Provides both legacy compatibility and modern security
//!
//! ### 2. **Different Cryptographic Libraries**
//! - **ECDSA functions**: Use `ring` crate (BoringSSL-based, comprehensive crypto library)
//! - **Schnorr functions**: Use `secp256k1` crate (Bitcoin-specific, optimized for secp256k1 curve)
//! - **Purpose**: Each library is optimized for its specific use case
//!
//! ### 3. **Different Key Formats**
//! - **ECDSA**: Uses PKCS#8 format private keys (more complex, standardized)
//! - **Schnorr**: Uses raw 32-byte private keys (simpler, Bitcoin-native)
//! - **Purpose**: Matches the requirements of each signature scheme
//!
//! ## Current Usage in the Codebase
//!
//! ### **Primary Usage: Schnorr Functions (Modern Bitcoin)**
//! - **`schnorr_sign_digest`**: Used in `Transaction::sign()` for signing transaction inputs
//! - **`schnorr_sign_verify`**: Used in `Transaction::verify()` for verifying transaction signatures
//! - **Why**: Bitcoin's Taproot upgrade uses Schnorr signatures for better security and efficiency
//!
//! ### **Legacy/Alternative Usage: ECDSA Functions**
//! - **`ecdsa_p256_sha256_sign_digest`**: Available for legacy transaction signing
//! - **`ecdsa_p256_sha256_sign_verify`**: Available for legacy transaction verification
//! - **Why**: Provides backward compatibility and alternative signature schemes
//!
//! ## Usage Locations
//!
//! ### **Schnorr Functions (Primary)**:
//! - **`src/domain/transaction.rs`**: Used in `Transaction::sign()` and `Transaction::verify()`
//! - **`src/service/blockchain_service.rs`**: Used indirectly through transaction verification in `mine_block()`
//! - **`src/server/operations.rs`**: Used indirectly through transaction validation
//! - **`src/main.rs`**: Used indirectly through transaction operations in CLI commands
//!
//! ### **ECDSA Functions (Legacy/Alternative)**:
//! - **Available for use**: Can be used for alternative signature schemes
//! - **Not currently used**: The codebase primarily uses Schnorr signatures
//! - **Purpose**: Provides flexibility for different signature requirements
//!
//! ## Technical Differences
//!
//! ### **Signature Sizes**:
//! - **ECDSA**: Variable size signatures (typically 70-72 bytes)
//! - **Schnorr**: Fixed 64-byte signatures (more efficient)
//!
//! ### **Security Properties**:
//! - **ECDSA**: Well-established, widely used
//! - **Schnorr**: Better security properties, linearity, batch verification support
//!
//! ### **Bitcoin Compatibility**:
//! - **ECDSA**: Traditional Bitcoin signatures
//! - **Schnorr**: Modern Bitcoin Taproot signatures (P2TR addresses)

use crate::domain::error::{BtcError, Result};
use rand::SeedableRng;
use rand::rngs::StdRng;
use ring::rand::SecureRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
use secp256k1::{Keypair, Message, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey, schnorr};

// Import the hash function from utils
use crate::sha256_digest;

///
/// The `new_schnorr_key_pair` function generates a new Schnorr key pair using secp256k1.
/// This is used for P2TR (Pay-to-Taproot) addresses with true Schnorr signatures.
///
/// # Usage Examples
///
/// - **P2TR wallet creation**: Used in wallet generation for creating new Taproot addresses
/// - **Schnorr operations**: Used as the foundation for Schnorr signing and verification
/// - **Modern Bitcoin**: Used for Bitcoin's Taproot upgrade implementation
/// - **Enhanced security**: Used for improved signature schemes with better security properties
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the private key as a 32-byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::WalletKeyPairError` if key generation fails due to insufficient randomness or other cryptographic errors.
///
pub fn new_schnorr_key_pair() -> Result<Vec<u8>> {
    let mut secret_key_bytes = [0u8; 32];
    ring::rand::SystemRandom::new()
        .fill(&mut secret_key_bytes)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    let _secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&secret_key_bytes)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    Ok(secret_key.secret_bytes().to_vec())
}

///
/// The `get_schnorr_public_key` function derives the public key from a private key
/// using secp256k1 for Schnorr signatures.
///
/// # Usage Examples
///
/// - **Public key derivation**: Used to derive public keys from private keys for Schnorr operations
/// - **Address generation**: Used in P2TR address generation from private keys
/// - **Signature verification**: Used to get public keys for signature verification
/// - **Wallet operations**: Used in wallet operations that require public key derivation
///
/// # Arguments
///
/// * `private_key` - A reference to the private key bytes (32 bytes for secp256k1).
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the corresponding public key as a 33-byte compressed public key.
///
/// # Error Handling
///
/// Returns `BtcError::WalletKeyPairError` if the private key is invalid or public key derivation fails.
///
pub fn get_schnorr_public_key(private_key: &[u8]) -> Result<Vec<u8>> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    Ok(public_key.serialize().to_vec())
}

///
/// The `ecdsa_p256_sha256_sign_digest` function signs the provided message parameter using the ECDSA P-256
/// SHA-256 algorithm. Given a private key in PKCS#8 format (pkcs8), it creates an ECDSA
/// key pair, signs the message, and returns the resulting signature as a byte vector.
///
/// # Usage Examples
///
/// - **Legacy transaction signing**: Available for signing transactions with ECDSA signatures
/// - **Message authentication**: Available for authenticating messages and data
/// - **Alternative signature schemes**: Provides ECDSA as an alternative to Schnorr signatures
/// - **Backward compatibility**: Available for systems that require ECDSA signatures
///
/// # Usage Locations
///
/// ### Current Status:
/// - **Not currently used**: The codebase primarily uses Schnorr signatures for transactions
/// - **Available for use**: Can be used for alternative signature schemes or legacy compatibility
/// - **Purpose**: Provides flexibility for different signature requirements
///
/// # Arguments
///
/// * `pkcs8` - A reference to the PKCS#8 document containing the private key.
/// * `message` - A reference to the message to be signed.
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the ECDSA signature as a byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::TransactionSignatureError` if signing fails due to invalid key format or other cryptographic errors.
pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let rng = ring::rand::SystemRandom::new();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8, &rng)
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))?;
    key_pair
        .sign(&rng, message)
        .map(|signature| signature.as_ref().to_vec())
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))
}

///
/// The `ecdsa_p256_sha256_sign_verify` function verifies an ECDSA P-256
/// SHA-256 signature against a provided message parameter using the corresponding
/// public_key value. It constructs an unparsed public key from the public_key byte slice
/// and uses it to verify the provided signature against the message parameter,
/// returning a Boolean indicating the signature's validity.
///
/// # Usage Examples
///
/// - **Legacy transaction verification**: Available for verifying ECDSA signatures on transactions
/// - **Message authentication**: Available for verifying the authenticity of signed messages
/// - **Alternative signature schemes**: Provides ECDSA verification as an alternative to Schnorr
/// - **Backward compatibility**: Available for systems that require ECDSA signature verification
///
/// # Usage Locations
///
/// ### Current Status:
/// - **Not currently used**: The codebase primarily uses Schnorr signature verification for transactions
/// - **Available for use**: Can be used for alternative signature schemes or legacy compatibility
/// - **Purpose**: Provides flexibility for different signature verification requirements
///
/// # Arguments
///
/// * `public_key` - A reference to the public key used for verification.
/// * `signature` - A reference to the signature to be verified.
/// * `message` - A reference to the original message that was signed.
///
/// # Returns
///
/// A boolean indicating whether the signature is valid (`true`) or invalid (`false`).
pub fn ecdsa_p256_sha256_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key =
        ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}

///
/// The `schnorr_sign_digest` function signs the provided message using Schnorr signatures
/// with secp256k1. This is the signature scheme used by P2TR (Pay-to-Taproot) addresses.
///
/// # Usage Examples
///
/// - **Transaction signing**: Used in `Transaction::sign()` for signing transactions with Schnorr signatures
/// - **P2TR operations**: Used for all P2TR (Pay-to-Taproot) signature operations
/// - **Modern Bitcoin**: Used for Bitcoin's Taproot upgrade signature scheme
/// - **Enhanced security**: Used for improved signature schemes with better security properties
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/transaction.rs`**: Used in `Transaction::sign()` method for signing transaction inputs
///
/// ### Indirect Usage via Transaction Signing:
/// - **`src/domain/transaction.rs`**: Used in `Transaction::new_utxo_transaction()` for signing new transactions
/// - **`src/server/operations.rs`**: Used indirectly through transaction creation and signing
/// - **`src/main.rs`**: Used indirectly through transaction operations in CLI commands
///
/// # Arguments
///
/// * `private_key` - A reference to the private key bytes (32 bytes for secp256k1).
/// * `message` - A reference to the message to sign (typically a transaction hash).
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the Schnorr signature as a 64-byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::TransactionSignatureError` if signing fails due to invalid key format or other cryptographic errors.
///
pub fn schnorr_sign_digest(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))?;

    let message_hash = sha256_digest(message);
    let message_obj = Message::from_digest_slice(&message_hash)
        .map_err(|e| BtcError::TransactionSignatureError(e.to_string()))?;

    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let mut rng = StdRng::from_entropy();
    let signature = secp.sign_schnorr_with_rng(&message_obj, &keypair, &mut rng);
    Ok(signature.as_ref().to_vec())
}

///
/// The `schnorr_sign_verify` function verifies a Schnorr signature against a provided message
/// using the corresponding public key. This is used for P2TR (Pay-to-Taproot) signature verification.
///
/// # Usage Examples
///
/// - **Transaction verification**: Used in `Transaction::verify()` for verifying Schnorr signatures on transactions
/// - **P2TR operations**: Used for all P2TR (Pay-to-Taproot) signature verification operations
/// - **Modern Bitcoin**: Used for Bitcoin's Taproot upgrade signature verification
/// - **Enhanced security**: Used for improved signature verification with better security properties
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/transaction.rs`**: Used in `Transaction::verify()` method for verifying transaction input signatures
///
/// ### Indirect Usage via Transaction Verification:
/// - **`src/service/blockchain_service.rs`**: Used indirectly through transaction verification in `mine_block()`
/// - **`src/server/operations.rs`**: Used indirectly through transaction validation
/// - **`src/main.rs`**: Used indirectly through transaction operations in CLI commands
///
/// # Arguments
///
/// * `public_key` - A reference to the public key bytes (33 bytes for compressed secp256k1 public key).
/// * `signature` - A reference to the signature bytes (64 bytes for Schnorr signature).
/// * `message` - A reference to the original message that was signed (typically a transaction hash).
///
/// # Returns
///
/// A boolean indicating whether the signature is valid (`true`) or invalid (`false`).
///
pub fn schnorr_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let secp = Secp256k1::new();

    // Parse the public key
    let public_key_obj = match PublicKey::from_slice(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Convert to XOnlyPublicKey for Schnorr verification
    let xonly_public_key = match XOnlyPublicKey::from_slice(&public_key_obj.serialize()[1..33]) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Hash the message
    let message_hash = sha256_digest(message);
    let message_obj = match Message::from_digest_slice(&message_hash) {
        Ok(msg) => msg,
        Err(_) => return false,
    };

    // Parse the signature
    let signature_obj = match schnorr::Signature::from_slice(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    // Verify the Schnorr signature
    secp.verify_schnorr(&signature_obj, &message_obj, &xonly_public_key)
        .is_ok()
}
