use crypto::digest::Digest;
use ring::digest::{Context, SHA256};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};
use std::iter::repeat_n;
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
/// The `ripemd160_digest` function calculates the RIPEMD-160 hash of the wallet public key.
///
/// RIPEMD-160 is a cryptographic hash function that operates on a 160-bit (20-byte) message digest.
/// It is designed to be faster than MD5 while providing a similar level of security.
/// RIPEMD-160 is widely used in various cryptographic applications, including digital signatures,
/// message authentication codes, and hash-based message authentication codes.
///
/// # Arguments
///
/// The `ripemd160_digest` function calculates the RIPEMD-160 hash of the input,
/// returning the resulting hash as a vector of bytes.
/// It creates a RIPEMD-160 hasher, inputs the data, collects the resulting hash into a byte vector,
/// and returns it.
///
/// # Arguments
///
/// * `data` - A reference to the input data.
pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = crypto::ripemd160::Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat_n(0, ripemd160.output_bytes()).collect(); // repeat_n faster than repeat then take
    ripemd160.result(&mut buf);
    buf
}

///
/// The `base58_encode` function encodes the given byte slice using the Base58 encoding scheme
/// and returns the encoded string representation.
/// It utilizes bs58 crate to perform the encoding and converts the byte data into a Base58-encoded string.
///
/// # Arguments
///
/// * `data` - A reference to the input data.
pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

///
/// The `base58_decode` function decodes a Base58-encoded string back to its original byte representation.
/// It uses the bs58 crate to decode the input string and returns the decoded byte vector.
///
/// # Arguments
///
/// * `data` - A reference to the input data.
pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
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
pub fn new_key_pair() -> Vec<u8> {
    let rng = SystemRandom::new();
    // Generates new key pair serialized as a PKCS#8 document.
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    pkcs8.as_ref().to_vec()
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
pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Vec<u8> {
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8).unwrap();
    let rng = ring::rand::SystemRandom::new();
    key_pair.sign(&rng, message).unwrap().as_ref().to_vec()
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
