//!
//!  wallet
//!
//! These functions together allow for the creation, extraction, validation, and conversion of addresses
//! associated with a cryptographic wallet, enabling secure transactions and identity management (IdM)
//! within a blockchain system.
//!
//! # Examples
//!
//! ```
//! use blockchain::Wallet;
//! let wallet = Wallet::new().expect("Failed to create wallet");
//! let address = wallet.get_address().expect("Failed to get address");
//! ```

use crate::domain::error::{BtcError, Result};
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde::{Deserialize, Serialize};

// P2TR version byte (0x01 for Taproot addresses)
// The version byte is used in address validation to ensure the address format is correct
// 0x01 is the official version byte for P2TR addresses in Bitcoin
const VERSION: u8 = 0x01;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    // The pkcs8 field stores the private key in PKCS#8 format:
    pkcs8: Vec<u8>,
    // The public_key field stores the public key as a byte slice:
    public_key: Vec<u8>,
}

impl Wallet {
    ///
    /// The `new` function creates a new wallet by generating a new key pair and returning a `Wallet` instance.
    /// It uses ECDSA (Elliptic Curve Digital Signature Algorithm) by parsing an unencrypted PKCS#8 key
    /// to generate a new key pair.
    ///
    /// # Returns
    ///
    /// A new `Wallet` instance.
    pub fn new() -> Result<Wallet> {
        // Generates new key pair.
        // The `new_key_pair` function generates a new ECDSA key pair and returns the private key as a byte vector.
        // It utilizes EcdsaKeyPair and SystemRandom from the ring crate to generate a private key in PKCS#8 format
        // and converts it to a byte vector.
        let pkcs8 = crate::new_key_pair()?;
        // Generate public key from private key.
        let rng = ring::rand::SystemRandom::new();
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng)
                .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
        let public_key = key_pair.public_key().as_ref().to_vec();
        Ok(Wallet { pkcs8, public_key })
    }

    ///
    /// The `get_address` function generates a P2TR (Pay-to-Taproot) address from the public key.
    /// It hashes the public key using SHA256, adds a P2TR version byte, computes a checksum,
    /// and encodes the result in Base58.
    ///
    /// # Returns
    ///
    /// A P2TR Bitcoin address as a `String`.
    pub fn get_address(&self) -> Result<String> {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload: Vec<u8> = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        // version + pub_key_hash + checksum
        crate::base58_encode(payload.as_slice())
    }

    ///
    /// The `get_public_key` function returns the public key as a byte slice.
    ///
    /// # Returns
    ///
    /// A reference to the public key.
    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }

    ///
    /// The `get_pkcs8` function returns the private key as a byte slice.
    ///
    /// # Returns
    ///
    /// A reference to the private key.
    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }
}

///
/// The `hash_pub_key` function hashes the public key using SHA-256 for P2TR compatibility.
///
/// For P2TR (Pay-to-Taproot), we use a single SHA256 hash instead of the traditional
/// SHA256 + RIPEMD160 combination. This provides better security and is compatible
/// with Bitcoin's Taproot upgrade.
///
/// # Arguments
///
/// * `pub_key` - A reference to the public key.
///
/// # Returns
///
/// A 32-byte hash suitable for P2TR addresses.
pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    // For P2TR (Pay-to-Taproot), we use a single SHA256 hash instead of SHA256 + RIPEMD160
    // This provides better security and is compatible with Bitcoin's Taproot upgrade
    crate::taproot_hash(pub_key)
}

///
/// The `checksum` function generates a checksum for a payload by applying a double SHA256 hash
/// and extracting the first bytes, resulting in a verification code:
///
/// # Arguments
///
/// * `payload` - A reference to the payload.
///
/// # Returns
///
/// A checksum as a `Vec<u8>`.
fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = crate::sha256_digest(payload);
    let second_sha = crate::sha256_digest(first_sha.as_slice());
    second_sha[0..ADDRESS_CHECK_SUM_LEN].to_vec()
}

///
/// The `validate_address` function validates a Bitcoin address by decoding it,
/// separating its components, and recomputing the checksum.
///
/// # Arguments
///
/// * `address` - A reference to the address.
///
/// # Returns
///
/// A boolean indicating whether the address is valid.
pub fn validate_address(address: &str) -> Result<bool> {
    let payload = crate::base58_decode(address)?;
    let actual_checksum = payload[payload.len() - ADDRESS_CHECK_SUM_LEN..].to_vec();
    let version = payload[0];
    let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN].to_vec();

    let mut target_vec = vec![];
    target_vec.push(version);
    target_vec.extend(pub_key_hash);
    let target_checksum = checksum(target_vec.as_slice());
    Ok(actual_checksum.eq(target_checksum.as_slice()))
}

///
/// The `convert_address` function converts a public key hash to a P2TR Bitcoin address.
/// It appends a P2TR version number, the public key hash, and a checksum,
/// then encodes it using Base58 encoding.
///
/// # Arguments
///
/// * `pub_hash_key` - A reference to the public key hash.
///
/// # Returns
///
/// A P2TR Bitcoin address as a `String`.
pub fn convert_address(pub_hash_key: &[u8]) -> Result<String> {
    let mut payload: Vec<u8> = vec![];
    payload.push(VERSION);
    payload.extend(pub_hash_key);
    let checksum = checksum(payload.as_slice());
    payload.extend(checksum.as_slice());
    crate::base58_encode(payload.as_slice())
}
