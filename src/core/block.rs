//! # Block
//!
//! Block is a data structure that contains the data and operations on the block.
//!

extern crate bincode;
use crate::core::proof_of_work::ProofOfWork;
use crate::core::transaction::Transaction;
use crate::error::{BtcError, Result};
use serde::{Deserialize, Serialize};
use sled::IVec;

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

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(String::from("None"), &transactions, 1)
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
    use crate::core::transaction::Transaction;

    fn generate_test_genesis_address() -> String {
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
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
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
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
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
}
