pub mod functional_operations;
pub mod utils;

pub use utils::{
    base58_decode, base58_encode, current_timestamp, get_schnorr_public_key, new_key_pair,
    new_schnorr_key_pair, schnorr_sign_digest, schnorr_sign_verify, sha256_digest, taproot_hash,
};

// Re-export functional utilities
pub use functional_operations::transaction as functional_transaction;
