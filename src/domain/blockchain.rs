use super::block::Block;
use super::error::{BtcError, Result};
use super::transaction::TXOutput;
use super::transaction::Transaction;

use sled::Db;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

pub mod blockchain;
pub use blockchain::*;
pub mod file_system_db_chain;
pub use file_system_db_chain::*;

#[derive(Debug)]
pub struct BlockchainService(Arc<TokioRwLock<BlockchainFileSystem>>);

impl Clone for BlockchainService {
    fn clone(&self) -> Self {
        BlockchainService(self.0.clone())
    }
}
impl BlockchainService {
    pub async fn initialize(genesis_address: &str) -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::create_blockchain(genesis_address).await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
    pub async fn default() -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::open_blockchain().await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }
    pub async fn empty() -> Result<BlockchainService> {
        let blockchain = BlockchainFileSystem::open_blockchain_empty().await?;
        Ok(BlockchainService(Arc::new(TokioRwLock::new(blockchain))))
    }

    /// Apply a readfunction to a blockchain and return the result
    async fn read<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(BlockchainFileSystem) -> Fut + Send,
        Fut: Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let blockchain_guard = self.0.read().await;
        f(blockchain_guard.clone()).await
    }

    // /// Apply a write function to a blockchain and return the result
    // async fn write<F, T>(&self, mut f: F) -> Result<T>
    // where
    //     F: FnMut(&mut BlockchainFileSystem) -> Result<T> + Send,
    //     T: Send + 'static,
    // {
    //     let mut blockchain_guard = self.0.write().await;
    //     f(&mut blockchain_guard)
    // }

    pub async fn get_db(&self) -> Result<Db> {
        self.read(|blockchain: BlockchainFileSystem| async move { Ok(blockchain.get_db().clone()) })
            .await
    }

    /// Get the best height of the blockchain
    pub async fn get_best_height(&self) -> Result<usize> {
        self.read(
            |blockchain: BlockchainFileSystem| async move { blockchain.get_best_height().await },
        )
        .await
    }

    /// Get the block hashes of the blockchain
    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        self.read(
            |blockchain: BlockchainFileSystem| async move { blockchain.get_block_hashes().await },
        )
        .await
    }

    /// Get the block of the blockchain
    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.get_block(block_hash).await
        })
        .await
    }

    /// Add a block to the blockchain
    pub async fn add_block(&self, block: &Block) -> Result<()> {
        let mut blockchain_guard = self.0.write().await;
        blockchain_guard.add_block(block).await
    }

    /// Mine a block with the transactions in the memory pool
    pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        for trasaction in transactions {
            let is_valid = trasaction.verify(self).await?;
            if !is_valid {
                return Err(BtcError::InvalidTransaction);
            }
        }
        let blockchain_guard = self.0.write().await;
        blockchain_guard.mine_block(transactions).await
    }

    /// Find a transaction in the blockchain
    pub async fn find_transaction(&self, txid: &[u8]) -> Result<Option<Transaction>> {
        self.read(|blockchain: BlockchainFileSystem| async move {
            blockchain.find_transaction(txid).await
        })
        .await
    }
    pub async fn find_utxo(&self) -> Result<HashMap<String, Vec<TXOutput>>> {
        self.read(|blockchain: BlockchainFileSystem| async move { blockchain.find_utxo().await })
            .await
    }
    pub async fn iterator(&self) -> Result<BlockchainIterator> {
        self.read(|blockchain: BlockchainFileSystem| async move { blockchain.iterator().await })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transaction::Transaction;

    use std::fs;

    fn generate_test_genesis_address() -> String {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::domain::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    async fn create_test_blockchain() -> (BlockchainService, String) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_db_path = format!("test_blockchain_db_{}_{}", timestamp, uuid::Uuid::new_v4());

        // Clean up any existing test database
        let _ = fs::remove_dir_all(&test_db_path);

        // Set environment variable for unique database path
        unsafe {
            std::env::set_var("TREE_DIR", &test_db_path);
        }
        unsafe {
            std::env::set_var("BLOCKS_TREE", &test_db_path);
        }

        let genesis_address = generate_test_genesis_address();
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create test blockchain");
        (blockchain, test_db_path)
    }

    fn cleanup_test_blockchain(db_path: &str) {
        let _ = fs::remove_dir_all(db_path);
    }

    #[tokio::test]
    async fn test_blockchain_creation() {
        let (blockchain, db_path) = create_test_blockchain().await;

        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            1
        );
        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_genesis_block_creation() {
        let (blockchain, db_path) = create_test_blockchain().await;

        // Genesis block should be created automatically
        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            1
        );

        // Get the genesis block using the iterator
        let mut iterator = blockchain
            .iterator()
            .await
            .expect("Failed to create iterator");
        let genesis_block = iterator.next().expect("Genesis block should exist");
        assert_eq!(genesis_block.get_height(), 1);
        assert_eq!(genesis_block.get_pre_block_hash(), "None");

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_add_block() {
        let (blockchain, db_path) = create_test_blockchain().await;
        let genesis_address = generate_test_genesis_address();

        // Create a new block
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = blockchain
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");

        // Add the block
        blockchain
            .add_block(&new_block)
            .await
            .expect("Failed to add block");

        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            2
        );

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_get_block() {
        let (blockchain, db_path) = create_test_blockchain().await;
        let genesis_address = generate_test_genesis_address();

        // Create and add a block
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = blockchain
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");
        blockchain
            .add_block(&new_block)
            .await
            .expect("Failed to add block");

        // Get the block by hash
        let retrieved_block = blockchain
            .get_block(new_block.get_hash_bytes().as_slice())
            .await
            .expect("Failed to get block")
            .expect("Block should exist");

        assert_eq!(retrieved_block.get_hash(), new_block.get_hash());
        assert_eq!(retrieved_block.get_height(), 2);

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_get_block_hashes() {
        let (blockchain, db_path) = create_test_blockchain().await;
        let genesis_address = generate_test_genesis_address();

        // Add a few blocks
        for _i in 0..3 {
            let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address)
                .expect("Failed to create coinbase tx");
            let transactions = vec![coinbase_tx];
            let new_block = blockchain
                .mine_block(transactions.as_slice())
                .await
                .expect("Failed to mine block");
            blockchain
                .add_block(&new_block)
                .await
                .expect("Failed to add block");
        }

        let block_hashes = blockchain
            .get_block_hashes()
            .await
            .expect("Failed to get block hashes");

        // Should have genesis block + 3 new blocks = 4 total
        assert_eq!(block_hashes.len(), 4);

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_blockchain_iterator() {
        let (blockchain, db_path) = create_test_blockchain().await;

        // Add a block
        let genesis_address = generate_test_genesis_address();
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = blockchain
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");
        blockchain
            .add_block(&new_block)
            .await
            .expect("Failed to add block");

        let mut iterator = blockchain
            .iterator()
            .await
            .expect("Failed to create iterator");
        let mut block_count = 0;

        while iterator.next().is_some() {
            block_count += 1;
        }

        // Should have genesis block + 1 new block = 2 total
        assert_eq!(block_count, 2);

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_blockchain_persistence() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let db_path = format!("test_persistence_db_{}_{}", timestamp, uuid::Uuid::new_v4());
        let _ = fs::remove_dir_all(&db_path);

        // Set environment variable for unique database path
        unsafe {
            std::env::set_var("TREE_DIR", &db_path);
        }
        unsafe {
            std::env::set_var("BLOCKS_TREE", &db_path);
        }

        let genesis_address = generate_test_genesis_address();

        {
            let blockchain = BlockchainService::initialize(&genesis_address)
                .await
                .expect("Failed to create blockchain");

            // Add a block
            let coinbase_tx = Transaction::new_coinbase_tx(&genesis_address)
                .expect("Failed to create coinbase tx");
            let transactions = vec![coinbase_tx];
            let new_block = blockchain
                .mine_block(transactions.as_slice())
                .await
                .expect("Failed to mine block");
            blockchain
                .add_block(&new_block)
                .await
                .expect("Failed to add block");
        } // blockchain goes out of scope here

        // Create a new blockchain instance with the same database
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create new blockchain");

        // Should still have the block we added
        assert_eq!(
            blockchain
                .get_best_height()
                .await
                .expect("Failed to get height"),
            2
        );

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_mine_block() {
        let (blockchain, db_path) = create_test_blockchain().await;

        let genesis_address = generate_test_genesis_address();
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];

        let new_block = blockchain
            .mine_block(transactions.as_slice())
            .await
            .expect("Failed to mine block");

        // Check that the block was mined correctly
        assert_eq!(new_block.get_height(), 2); // Height 2 because genesis block is height 1
        assert!(!new_block.get_hash().is_empty());
        assert!(new_block.get_transactions().len() > 0);

        cleanup_test_blockchain(&db_path);
    }
}
