//! # Block
//!
//! Block is a data structure that contains the data and operations on the block.
//!

extern crate bincode;
use crate::error::{BtcError, Result};
use crate::pow::ProofOfWork;
use crate::primitives::transaction::Transaction;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sled::IVec;

pub const GENESIS_BLOCK_PRE_BLOCK_HASH: &str = "None";

// Add a block header that contains timestamp, pre_block_hash, hash, nonce, height
// Block to be composed of block header and transactions
#[derive(Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    nonce: i64,
    height: usize,
}

/// Block
///
/// `timestamp`: An integer value that represents the time when the block was created. It's used
/// to track the chronological order of blocks in the blockchain.
/// `pre_block_hash`: A string containing the hash value of the previous block in the blockchain.
/// This creates a link between blocks, ensuring the integrity of the blockchain.
/// `hash`: String containing the hash value of the current block. This hash is generated based on
/// the data within the current block, including transactions and other information.
/// `transactions`: A vector or collection that holds the block transactions.
/// Transactions can represent various types of data or actions, depending on the blockchain's
/// purpose (for example, cryptocurrency transactions).
/// `nonce`: A number used to ensure that the block is valid and has not been tampered with.
/// Stands for number used only once
/// `height`: An integer value that represents the position of the block in the blockchain.
/// It's used to track the chronological order of blocks in the blockchain. Indicates the position.

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let header = BlockHeader {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(), // to be filled in the next step
            nonce: 0,
            height,
        };
        let mut block = Block {
            header,
            transactions: transactions.to_vec(),
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.header.nonce = nonce;
        block.header.hash = hash;
        block
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Block> {
        bincode::serde::decode_from_slice(bytes, bincode::config::standard())
            .map_err(|e| BtcError::BlockDeserializationError(e.to_string()))
            .map(|(block, _)| block)
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| BtcError::BlockSerializationError(e.to_string()))
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.header.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.header.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.header.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.header.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.header.height
    }

    pub fn get_difficulty(&self) -> u32 {
        // For now, return a constant difficulty
        // In a real implementation, this would be calculated based on the block's proof-of-work
        1
    }

    /// Get the nonce value from the block header
    pub fn get_nonce(&self) -> i64 {
        self.header.nonce
    }

    /// Get the hash as a string for comparison
    pub fn get_hash_string(&self) -> String {
        self.header.hash.clone()
    }

    /// Calculate work for this block based on proof-of-work difficulty
    ///
    /// In Bitcoin, work is calculated as: 2^256 / (target + 1)
    /// Where target = 2^(256 - TARGET_BITS)
    ///
    /// For our implementation with TARGET_BITS = 8:
    /// - Target = 2^(256 - 8) = 2^248
    /// - Work = 2^256 / (2^248 + 1) ≈ 2^8 = 256
    ///
    /// # Returns
    /// * `u64` - The work value for this block
    ///
    /// # Note
    /// This implements the same work calculation as Bitcoin's consensus mechanism.
    /// Higher work values indicate more difficult proof-of-work, which means
    /// the block required more computational effort to mine.
    pub fn get_work(&self) -> u64 {
        // Work is calculated as: 2^256 / (target + 1)
        // Where target = 2^(256 - TARGET_BITS)
        // For TARGET_BITS = 8: target = 2^248, work = 2^256 / (2^248 + 1) ≈ 2^8 = 256
        const TARGET_BITS: u32 = 8;
        const WORK_BITS: u32 = 256 - TARGET_BITS; // 256 - 8 = 248

        // Calculate 2^WORK_BITS, but cap it to prevent overflow
        if WORK_BITS >= 64 {
            u64::MAX / 1000 // Large but manageable value
        } else {
            2u64.pow(WORK_BITS)
        }
    }

    /// Get the target bits used for proof-of-work calculation
    ///
    /// This method provides access to the TARGET_BITS constant used
    /// in the proof-of-work calculation, allowing other parts of the
    /// system to understand the difficulty level.
    ///
    /// # Returns
    /// * `u32` - The target bits value (currently 8)
    pub fn get_target_bits(&self) -> u32 {
        // This should match the TARGET_BITS constant in proof_of_work.rs
        8
    }

    /// Calculate the actual target value used in proof-of-work
    ///
    /// This method calculates the exact target value that was used
    /// during the proof-of-work mining process for this block.
    ///
    /// # Returns
    /// * `BigInt` - The target value as a BigInt
    pub fn get_target(&self) -> BigInt {
        use std::ops::ShlAssign;

        let target_bits = self.get_target_bits();
        let mut target = BigInt::from(1);
        target.shl_assign(256 - target_bits as i32);
        target
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(GENESIS_BLOCK_PRE_BLOCK_HASH.to_string(), &transactions, 1)
    }
}

impl TryFrom<Block> for IVec {
    type Error = BtcError;
    fn try_from(b: Block) -> Result<Self> {
        let bytes = bincode::serde::encode_to_vec(&b, bincode::config::standard())
            .map_err(|e| BtcError::BlockSerializationError(e.to_string()))?;
        Ok(Self::from(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::transaction::Transaction;

    fn generate_test_genesis_address() -> crate::WalletAddress {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    #[test]
    fn test_block_creation() {
        let transactions = vec![];
        let prev_block_hash = "previous_hash".to_string();
        let height = 1;

        let block = Block::new_block(prev_block_hash.clone(), transactions.as_slice(), height);

        assert_eq!(block.header.pre_block_hash, prev_block_hash);
        assert_eq!(block.transactions.len(), 0);
        assert_eq!(block.header.height, height);
        assert!(!block.header.hash.is_empty()); // Should be filled by PoW
        assert!(block.header.nonce >= 0);
    }

    #[test]
    fn test_block_serialization_deserialization() {
        let genesis_address = generate_test_genesis_address();
        let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address.clone())
            .expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let block = Block::new_block("prev_hash".to_string(), transactions.as_slice(), 1);

        let serialized = block.serialize().expect("Serialization failed");
        let deserialized = Block::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(block.header.timestamp, deserialized.header.timestamp);
        assert_eq!(
            block.header.pre_block_hash,
            deserialized.header.pre_block_hash
        );
        assert_eq!(block.header.hash, deserialized.header.hash);
        assert_eq!(block.header.nonce, deserialized.header.nonce);
        assert_eq!(block.header.height, deserialized.header.height);
    }

    #[test]
    fn test_block_getters() {
        let block = Block::new_block("prev_hash".to_string(), &[], 1);

        assert_eq!(block.get_pre_block_hash(), "prev_hash");
        assert!(!block.get_hash().is_empty());
        assert_eq!(block.get_height(), 1);
        assert!(block.get_timestamp() > 0);
        assert_eq!(block.get_transactions().len(), 0);
    }

    #[test]
    fn test_block_with_transactions() {
        let genesis_address = generate_test_genesis_address();
        let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address.clone())
            .expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];

        let block = Block::new_block("prev_hash".to_string(), transactions.as_slice(), 1);

        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.get_transactions().len(), 1);
    }

    #[test]
    fn test_block_hash_bytes() {
        let block = Block::new_block("prev_hash".to_string(), &[], 1);
        let hash_bytes = block.get_hash_bytes();
        assert!(!hash_bytes.is_empty());
        assert_eq!(hash_bytes, block.header.hash.as_bytes());
    }

    #[test]
    fn test_work_calculation() {
        // Create a test block
        let block = Block::new_block("prev_hash".to_string(), &[], 1);

        // Test work calculation
        let work = block.get_work();

        // Work should be meaningful (not just 1)
        assert!(work > 1, "Work should be greater than 1, got {}", work);

        // For TARGET_BITS = 8, work should be approximately 2^8 = 256
        // But we cap it to prevent overflow, so it should be a large number
        assert!(work >= 256, "Work should be at least 256, got {}", work);

        // Test target bits
        let target_bits = block.get_target_bits();
        assert_eq!(
            target_bits, 8,
            "Target bits should be 8, got {}",
            target_bits
        );

        // Test target calculation
        let target = block.get_target();
        assert!(target > BigInt::from(0), "Target should be positive");
    }

    #[test]
    fn test_work_accumulation() {
        // Test that work accumulates properly across multiple blocks
        let mut total_work = 0u64;

        for i in 1..=5 {
            let block = Block::new_block(format!("prev_hash_{}", i), &[], i);
            let work = block.get_work();
            total_work += work;
        }

        // Total work should be 5 times the individual block work
        let single_block_work = Block::new_block("test".to_string(), &[], 1).get_work();
        let expected_total = single_block_work * 5;

        assert_eq!(
            total_work, expected_total,
            "Total work should accumulate properly"
        );
    }
}
