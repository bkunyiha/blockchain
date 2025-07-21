//! # Block
//!
//! Block is a data structure that contains the data and operations on the block.
//!

extern crate bincode;
use super::proof_of_work::ProofOfWork;
use super::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sled::IVec;

// TODO: Add a block header that contains timestamp, pre_block_hash, hash, nonce, height
// TODO: Block to be composed of block header and transactions
//pub struct BlockHeader {
//    timestamp: i64,
//    pre_block_hash: String,
//    hash: String,
//    nonce: i64,
//    height: usize,
//}

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
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        block
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
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
        Block::new_block(String::from("None"), &transactions, 0)
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}
