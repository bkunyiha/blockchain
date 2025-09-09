use crate::error::{BtcError, Result};

///
/// The `base58_encode` function encodes the given byte slice using the Base58 encoding scheme
/// and returns the encoded string representation.
/// It utilizes bs58 crate to perform the encoding and converts the byte data into a Base58-encoded string.
///
/// # Usage Examples
///
/// - **Address generation**: Used in `convert_address()` to encode P2TR Bitcoin addresses
/// - **Address validation**: Used in `validate_address()` for address checksum verification
/// - **Transaction display**: Used indirectly through `convert_address()` in transaction formatting
/// - **UTXO management**: Used in transaction summary generation for address display
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/wallet.rs`**: Used in `convert_address()` function for P2TR address encoding
///
/// ### Indirect Usage via `convert_address()`:
/// - **`src/domain/transaction.rs`**: Transaction output address display in debug logging
/// - **`src/store/file_system_db_chain.rs`**: Transaction summary generation for input/output addresses
/// - **`src/main.rs`**: Transaction input/output formatting for CLI display
///
/// # Arguments
///
/// * `data` - A reference to the input data to be Base58 encoded (typically address payload with version, hash, and checksum).
///
/// # Returns
///
/// A Base58-encoded string representation of the input data.
pub fn base58_encode(data: &[u8]) -> Result<String> {
    Ok(bs58::encode(data).into_string())
}

///
/// The `base58_decode` function decodes a Base58-encoded string back to its original byte representation.
/// It uses the bs58 crate to decode the input string and returns the decoded byte vector.
///
/// # Usage Examples
///
/// - **Address validation**: Used in `validate_address()` to decode and verify Bitcoin address structure
/// - **Public key extraction**: Used in `get_pub_key_hash()` to extract public key hash from addresses
/// - **Address parsing**: Used to decode Bitcoin addresses into their component parts (version, hash, checksum)
/// - **Transaction processing**: Used indirectly through address validation in transaction handling
///
/// # Usage Locations
///
/// ### Direct Usage:
/// - **`src/domain/wallet.rs`**: Used in `validate_address()` function for address structure validation
/// - **`src/domain/wallet.rs`**: Used in `get_pub_key_hash()` function to extract public key hash from addresses
///
/// ### Indirect Usage via `get_pub_key_hash()`:
/// - **`src/domain/transaction.rs`**: Used in `TXOutput::lock()` for address validation during output creation
/// - **`src/domain/utxo_set.rs`**: Used in `get_balance()` for address validation and public key hash extraction
/// - **`src/store/file_system_db_chain.rs`**: Used in transaction summary generation for address processing
///
/// # Arguments
///
/// * `data` - A reference to the Base58-encoded string to be decoded (typically a Bitcoin address).
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the decoded byte vector, or an error if decoding fails.
///
/// # Error Handling
///
/// Returns `BtcError::AddressDecodingError` if the input string is not valid Base58 or cannot be decoded.
pub fn base58_decode(data: &str) -> Result<Vec<u8>> {
    bs58::decode(data)
        .into_vec()
        .map_err(|e| BtcError::AddressDecodingError(e.to_string()))
}
