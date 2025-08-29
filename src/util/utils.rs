use crate::domain::error::{BtcError, Result};
use rand::SeedableRng;
use rand::rngs::StdRng;
use ring::digest::{Context, SHA256};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
use secp256k1::{Keypair, Message, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey, schnorr};
use sha2::{Digest as Sha2Digest, Sha256 as Sha2Hash};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

///
/// Hash functions are used to create a unique identifier for a block or transaction.
///
/// The `sha256_digest` function performs a SHA-256 hash operation on the provided data input,
/// returning the resulting hash as a vector of bytes.
/// It initializes a hashing context with SHA-256, updates the context with the input data,
/// generates the hash digest, and converts it to a vector of bytes for output.
///
/// # Arguments
///
/// * `data` - A reference to the input data.
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
/// # Arguments
///
/// * `data` - A reference to the input data.
///
/// # Returns
///
/// A 32-byte hash as a vector of bytes.
pub fn taproot_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha2Hash::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

///
/// The `base58_encode` function encodes the given byte slice using the Base58 encoding scheme
/// and returns the encoded string representation.
/// It utilizes bs58 crate to perform the encoding and converts the byte data into a Base58-encoded string.
///
/// # Arguments
///
/// * `data` - A reference to the input data.
pub fn base58_encode(data: &[u8]) -> Result<String> {
    Ok(bs58::encode(data).into_string())
}

///
/// The `base58_decode` function decodes a Base58-encoded string back to its original byte representation.
/// It uses the bs58 crate to decode the input string and returns the decoded byte vector.
///
/// # Arguments
///
/// * `data` - A reference to the input data.
pub fn base58_decode(data: &str) -> Result<Vec<u8>> {
    bs58::decode(data)
        .into_vec()
        .map_err(|e| crate::domain::error::BtcError::AddressDecodingError(e.to_string()))
}

///
/// The `new_key_pair` function generates a new ECDSA key pair and returns the private key as a byte vector.
/// It utilizes EcdsaKeyPair and SystemRandom from the ring crate to generate a private key in PKCS#8 format
/// and converts it to a byte vector.
///
/// # Returns
///
/// A new key pair.
///
pub fn new_key_pair() -> Result<Vec<u8>> {
    let rng = SystemRandom::new();
    // Generates new key pair serialized as a PKCS#8 document.
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    Ok(pkcs8.as_ref().to_vec())
}

///
/// The `ecdsa_p256_sha256_sign_digest` function signs the provided message parameter using the ECDSA P-256
/// SHA-256 algorithm. Given a private key in PKCS#8 format (pkcs8), it creates an ECDSA
/// key pair, signs the message, and returns the resulting signature as a byte vector.
///
/// # Arguments
///
/// * `pkcs8` - A reference to the PKCS#8 document.
/// * `message` - A reference to the message.
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
/// returning a Boolean indicating the signature’s validity.
///
/// # Arguments
///
/// * `public_key` - A reference to the public key.
/// * `signature` - A reference to the signature.
/// * `message` - A reference to the message.
pub fn ecdsa_p256_sha256_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key =
        ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}

///
/// The `new_schnorr_key_pair` function generates a new Schnorr key pair using secp256k1.
/// This is used for P2TR (Pay-to-Taproot) addresses with true Schnorr signatures.
///
/// # Returns
///
/// A new Schnorr key pair private key as bytes.
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
/// # Arguments
///
/// * `private_key` - A reference to the private key bytes.
///
/// # Returns
///
/// The corresponding public key as bytes.
///
pub fn get_schnorr_public_key(private_key: &[u8]) -> Result<Vec<u8>> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)
        .map_err(|e| BtcError::WalletKeyPairError(e.to_string()))?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    Ok(public_key.serialize().to_vec())
}

///
/// The `schnorr_sign_digest` function signs the provided message using Schnorr signatures
/// with secp256k1. This is the signature scheme used by P2TR (Pay-to-Taproot) addresses.
///
/// # Arguments
///
/// * `private_key` - A reference to the private key bytes.
/// * `message` - A reference to the message to sign.
///
/// # Returns
///
/// The Schnorr signature as bytes.
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
/// # Arguments
///
/// * `public_key` - A reference to the public key bytes.
/// * `signature` - A reference to the signature bytes.
/// * `message` - A reference to the message that was signed.
///
/// # Returns
///
/// A boolean indicating whether the signature is valid.
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

#[cfg(test)]
mod tests {
    use super::*;

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
