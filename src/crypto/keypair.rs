use crate::error::{BtcError, Result};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
use secp256k1::{PublicKey, Secp256k1, SecretKey};

///
/// The `new_key_pair` function generates a new ECDSA key pair and returns the private key as a byte vector.
/// It utilizes EcdsaKeyPair and SystemRandom from the ring crate to generate a private key in PKCS#8 format
/// and converts it to a byte vector.
///
/// # Usage Examples
///
/// - **Wallet creation**: Used in wallet generation for creating new Bitcoin addresses
/// - **Key management**: Used for generating secure cryptographic key pairs
/// - **ECDSA operations**: Used as the foundation for ECDSA signing and verification
/// - **Legacy support**: Used for traditional ECDSA-based wallet operations
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/wallet.rs`**: Used in `Wallet::new()` for generating new wallet key pairs
/// - **`src/service/wallet_service.rs`**: Used in wallet service for key pair generation
///
/// ### Indirect Usage via Wallet Creation:
/// - **`src/main.rs`**: Used indirectly through wallet creation in CLI commands
/// - **`src/server.rs`**: Used indirectly through wallet operations in server functionality
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the private key in PKCS#8 format as a byte vector.
///
/// # Error Handling
///
/// Returns `BtcError::WalletKeyPairError` if key generation fails due to insufficient randomness or other cryptographic errors.
///
pub fn new_key_pair() -> Result<Vec<u8>> {
    let rng = SystemRandom::new();
    // Generates new key pair serialized as a PKCS#8 document.
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    Ok(pkcs8.as_ref().to_vec())
}

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
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/wallet.rs`**: Used in `Wallet::new()` for generating new Schnorr-based wallet key pairs
/// - **`src/service/wallet_service.rs`**: Used in wallet service for Schnorr key pair generation
///
/// ### Indirect Usage via Wallet Creation:
/// - **`src/main.rs`**: Used indirectly through wallet creation in CLI commands
/// - **`src/server.rs`**: Used indirectly through wallet operations in server functionality
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
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/wallet.rs`**: Used in `Wallet::new()` for deriving public keys from private keys
/// - **`src/service/wallet_service.rs`**: Used in wallet service for public key derivation
///
/// ### Indirect Usage via Wallet Operations:
/// - **`src/main.rs`**: Used indirectly through wallet operations in CLI commands
/// - **`src/server.rs`**: Used indirectly through wallet operations in server functionality
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::signature::{schnorr_sign_digest, schnorr_sign_verify};

    #[test]
    fn test_schnorr_signature_roundtrip() {
        // Generate a Schnorr key pair
        let private_key = new_schnorr_key_pair().expect("Failed to generate Schnorr key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        // Create a test message
        let message = b"Hello, P2TR Schnorr signatures!";

        // Sign the message
        let signature = schnorr_sign_digest(&private_key, message).expect("Failed to sign message");

        // Verify the signature
        let is_valid = schnorr_sign_verify(&public_key, &signature, message);

        assert!(is_valid, "Schnorr signature verification failed");

        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_invalid = schnorr_sign_verify(&public_key, &signature, wrong_message);

        assert!(
            !is_invalid,
            "Schnorr signature should be invalid for wrong message"
        );
    }

    #[test]
    fn test_schnorr_key_generation() {
        // Generate multiple key pairs to ensure randomness
        let key1 = new_schnorr_key_pair().expect("Failed to generate first key pair");
        let key2 = new_schnorr_key_pair().expect("Failed to generate second key pair");

        // Keys should be different (random)
        assert_ne!(key1, key2, "Generated keys should be different");

        // Keys should be 32 bytes
        assert_eq!(key1.len(), 32, "Private key should be 32 bytes");
        assert_eq!(key2.len(), 32, "Private key should be 32 bytes");
    }

    #[test]
    fn test_schnorr_public_key_derivation() {
        let private_key = new_schnorr_key_pair().expect("Failed to generate key pair");
        let public_key = get_schnorr_public_key(&private_key).expect("Failed to get public key");

        // Public key should be 33 bytes (compressed format)
        assert_eq!(public_key.len(), 33, "Public key should be 33 bytes");

        // Public key should start with 0x02 or 0x03 (compressed format)
        assert!(
            public_key[0] == 0x02 || public_key[0] == 0x03,
            "Public key should be in compressed format"
        );
    }
}
