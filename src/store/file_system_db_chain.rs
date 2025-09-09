use crate::domain::block::Block;
use crate::domain::blockchain::Blockchain;
use crate::domain::error::{BtcError, Result};
use crate::domain::transaction::{
    TXOutput, Transaction, TxInputSummary, TxOutputSummary, TxSummary,
};
use crate::wallet::{convert_address, hash_pub_key};
use sled::transaction::{TransactionResult, UnabortableTransactionError};
use sled::{Db, IVec, Tree};
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use tracing::info;

const DEFAULT_TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE: &str = "empty";
const DEFAULT_BLOCKS_TREE: &str = "blocks1";
const DEFAULT_TREE_DIR: &str = "data1";

#[derive(Clone, Debug)]
pub struct BlockchainFileSystem {
    blockchain: Blockchain<Db>,
    file_system_tree_dir: String,
}

impl BlockchainFileSystem {
    pub async fn create_blockchain(genesis_address: &str) -> Result<Self> {
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        let file_system_tree_dir =
            env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string());
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let blocks_tree = db
            .open_tree(file_system_tree_dir.clone())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        let data = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;
        let tip_hash = if let Some(data) = data {
            String::from_utf8(data.to_vec())
                .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?
        } else {
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address)?;
            let block = Block::generate_genesis_block(&coinbase_tx);
            Self::update_blocks_tree(&blocks_tree, &block).await?;
            String::from(block.get_hash())
        };

        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: false,
            },
            file_system_tree_dir,
        })
    }

    async fn update_blocks_tree(blocks_tree: &Tree, block: &Block) -> Result<()> {
        let block_hash = block.get_hash();
        let block_ivec = IVec::try_from(block.clone())?;
        let transaction_result: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block_ivec.clone())?;
            let _ = tx_db.insert(DEFAULT_TIP_BLOCK_HASH_KEY, block_hash)?;
            Ok(())
        });
        transaction_result
            .map(|_| ())
            .map_err(|e| BtcError::BlockchainDBconnection(format!("{:?}", e)))
    }

    pub async fn open_blockchain() -> Result<BlockchainFileSystem> {
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        let file_system_tree_dir =
            env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string());
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let blocks_tree = db
            .open_tree(file_system_tree_dir.clone())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        let tip_bytes = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
            .ok_or(BtcError::BlockchainNotFoundError(
                "No existing blockchain found. Connect to a blcock chain cluster first."
                    .to_string(),
            ))?;
        let tip_hash = String::from_utf8(tip_bytes.to_vec())
            .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))?;
        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: false,
            },
            file_system_tree_dir,
        })
    }

    pub async fn open_blockchain_empty() -> Result<BlockchainFileSystem> {
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        let file_system_tree_dir =
            env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string());
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let tip_hash = DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE.to_string();

        Ok(BlockchainFileSystem {
            blockchain: Blockchain {
                tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
                db,
                is_empty: true,
            },
            file_system_tree_dir,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.blockchain.is_empty
    }

    pub fn get_db(&self) -> &Db {
        &self.blockchain.db
    }

    pub async fn get_tip_hash(&self) -> Result<String> {
        let tip_hash = self.blockchain.tip_hash.read().await;
        Ok(tip_hash.clone())
    }

    async fn set_tip_hash(&self, new_tip_hash: &str) -> Result<()> {
        let mut tip_hash = self.blockchain.tip_hash.write().await;
        *tip_hash = String::from(new_tip_hash);
        Ok(())
    }

    fn set_not_empty(&mut self) {
        self.blockchain.is_empty = false;
    }

    // The `mine_block` function mines a new block with the transactions in the memory pool.
    // It uses the `blockchain` instance to mine the block which also adds the new block to the blockchain.
    // It returns the new block.
    pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        let best_height = self.get_best_height().await?;

        let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self
            .blockchain
            .db
            .open_tree(self.get_blocks_tree_path())
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        // The `update_blocks_tree` function updates the blocks tree with the new block.
        // It uses the `blocks_tree` instance to update the blocks tree.
        // It uses the `block` instance to update the blocks tree.
        // It returns the new block.
        Self::update_blocks_tree(&blocks_tree, &block).await?;
        self.set_tip_hash(block_hash).await?;
        Ok(block)
    }

    pub async fn iterator(&self) -> Result<BlockchainIterator> {
        let hash = self.get_tip_hash().await?;
        Ok(BlockchainIterator::new(
            hash,
            self.blockchain.db.clone(),
            self.get_blocks_tree_path(),
        ))
    }

    /// The `find_utxo` function finds all unspent transaction outputs (UTXOs) in the blockchain.
    /// It iterates through the blockchain, finds all UTXOs, and returns them in a HashMap.
    ///
    /// # Returns
    ///
    /// A HashMap containing transaction IDs as keys and vectors of TXOutput as values.
    ///
    pub async fn find_utxo(&self) -> Result<HashMap<String, Vec<TXOutput>>> {
        let mut utxo: HashMap<String, Vec<TXOutput>> = HashMap::new();
        let mut spent_txos: HashMap<String, Vec<usize>> = HashMap::new();
        let mut iterator = self.iterator().await?;

        // First pass: collect all outputs from all transactions
        loop {
            match iterator.next() {
                None => break,
                Some(block) => {
                    for tx in block.get_transactions() {
                        let txid_hex = tx.get_tx_id_hex();

                        // Add all outputs to UTXO set
                        for tx_out in tx.get_vout() {
                            if utxo.contains_key(txid_hex.as_str()) {
                                utxo.get_mut(txid_hex.as_str())
                                    .ok_or(BtcError::UTXONotFoundError(format!(
                                        "UTXO not found for transaction {}",
                                        txid_hex
                                    )))?
                                    .push(tx_out.clone());
                            } else {
                                utxo.insert(txid_hex.clone(), vec![tx_out.clone()]);
                            }
                        }
                    }
                }
            }
        }

        // Second pass: mark outputs as spent when we encounter transactions that reference them
        let mut iterator = self.iterator().await?;
        loop {
            match iterator.next() {
                None => break,
                Some(block) => {
                    for tx in block.get_transactions() {
                        // Mark inputs as spent (only for non-coinbase transactions)
                        if tx.not_coinbase() {
                            for tx_in in tx.get_vin() {
                                let tx_in_id_hex = tx_in.get_input_tx_id_hex();
                                if spent_txos.contains_key(tx_in_id_hex.as_str()) {
                                    spent_txos
                                        .get_mut(tx_in_id_hex.as_str())
                                        .ok_or(BtcError::UTXONotFoundError(format!(
                                            "UTXO not found for transaction {}",
                                            tx_in_id_hex
                                        )))?
                                        .push(tx_in.get_vout());
                                } else {
                                    spent_txos.insert(tx_in_id_hex, vec![tx_in.get_vout()]);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Third pass: remove spent outputs from UTXO set
        for (txid_hex, spent_indices) in spent_txos {
            // Checks if this transaction still exists in the UTXO set
            // Gets a mutable reference to the outputs vector
            // If the transaction doesn't exist, we skip it (it was already fully spent)
            if let Some(outputs) = utxo.get_mut(&txid_hex) {
                // Remove spent outputs in reverse order to maintain indices
                // Why reverse order? Because when we remove elements from a vector,
                // the indices of subsequent elements shift down.
                // By removing from the end first, we don't affect the indices of elements we haven't processed yet.
                // We dont want to mess the indices of spent outputs since they are used to identify the outputs in the transaction.
                for &spent_idx in spent_indices.iter().rev() {
                    // The check here is just a safety check to ensure the index is valid
                    // Prevents panic if there's a mismatch between tracked spent outputs and actual outputs
                    if spent_idx < outputs.len() {
                        outputs.remove(spent_idx);
                    }
                }
                // Remove empty transaction entries
                if outputs.is_empty() {
                    utxo.remove(&txid_hex);
                }
            }
        }

        Ok(utxo)
    }

    pub async fn find_transaction(&self, txid: &[u8]) -> Result<Option<Transaction>> {
        let mut iterator = self.iterator().await?;
        loop {
            match iterator.next() {
                None => break,
                Some(block) => {
                    for transaction in block.get_transactions() {
                        if txid.eq(transaction.get_id()) {
                            return Ok(Some(transaction.clone()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    pub async fn find_all_transactions(&self) -> Result<HashMap<String, TxSummary>> {
        let mut transactions = HashMap::new();
        let mut iterator = self.iterator().await?;
        loop {
            match iterator.next() {
                None => break,
                Some(block) => {
                    for tx in block.get_transactions() {
                        let cur_txid_hex = tx.get_tx_id_hex();
                        let mut current_transactions_summary = TxSummary::new(cur_txid_hex.clone());

                        // Containbase transactions dont have inputs.
                        if tx.not_coinbase() {
                            for input in tx.get_vin() {
                                let input_txid_hex = input.get_input_tx_id_hex();
                                let pub_key_hash = hash_pub_key(input.get_pub_key());
                                let address = convert_address(pub_key_hash.as_slice())
                                    .expect("Convert address error");
                                current_transactions_summary.add_input(TxInputSummary::new(
                                    input_txid_hex,
                                    input.get_vout(),
                                    address,
                                ));
                            }
                        }

                        for output in tx.get_vout() {
                            let pub_key_hash = output.get_pub_key_hash();
                            let address =
                                convert_address(pub_key_hash).expect("Convert address error");
                            current_transactions_summary
                                .add_output(TxOutputSummary::new(address, output.get_value()));
                        }
                        transactions.insert(cur_txid_hex, current_transactions_summary);
                    }
                }
            }
        }
        Ok(transactions)
    }

    // The `add_block` function adds a block to the blockchain.
    // It uses the `block_tree` instance to add the block to the blockchain.
    // It uses the `block` instance to add the block to the blockchain.
    // It will not add the block if its height is less than current tip height in the block chain.
    pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
        let block_tree = self
            .blockchain
            .db
            .open_tree(self.get_blocks_tree_path())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        if self.is_empty() {
            info!("Blockchain is empty, adding block");

            self.set_not_empty();
            info!("Blockchain is now not empty");
            Self::update_blocks_tree(&block_tree, new_block).await?;
            self.set_tip_hash(new_block.get_hash()).await?;
            let best_height = self.get_best_height().await?;
            info!(
                "Blockchain is now not empty, best height is {}",
                best_height
            );
            return Ok(());
        } else {
            let block_bytes = block_tree
                .get(new_block.get_hash())
                .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;
            // If the block is already in the blockchain, return Ok(())
            if block_bytes.is_some() {
                return Ok(());
            }
            let block_bytes = new_block.serialize()?;
            let tip_hash = self.get_tip_hash().await?;
            let _: TransactionResult<(), ()> = block_tree.transaction(|transaction| {
                let _ = transaction.insert(new_block.get_hash(), block_bytes.clone())?;

                let tip_block_bytes = transaction.get(tip_hash.clone())?.ok_or(
                    UnabortableTransactionError::Storage(sled::Error::CollectionNotFound(
                        IVec::from(tip_hash.as_bytes()),
                    )),
                )?;

                let tip_block = Block::deserialize(tip_block_bytes.as_ref()).map_err(|e| {
                    UnabortableTransactionError::Storage(sled::Error::Unsupported(e.to_string()))
                })?;

                if self.is_empty() || new_block.get_height() > tip_block.get_height() {
                    let _ = transaction.insert(DEFAULT_TIP_BLOCK_HASH_KEY, new_block.get_hash())?;
                } else {
                    info!(
                        "Block {:?} not added because its height is less than mine",
                        new_block.get_hash()
                    );
                }

                Ok(())
            });

            // Update tip hash if block was added
            if !self.is_empty() && new_block.get_height() > self.get_best_height().await? {
                self.set_tip_hash(new_block.get_hash()).await?;
            }
        }

        Ok(())
    }

    pub async fn get_best_height(&self) -> Result<usize> {
        if self.is_empty() {
            info!("Blockchain is empty, returning height 0");
            Ok(0)
        } else {
            let block_tree = self
                .blockchain
                .db
                .open_tree(self.get_blocks_tree_path())
                .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;
            let tip_block_bytes = block_tree
                .get(self.get_tip_hash().await?)
                .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
                .ok_or(BtcError::GetBlockchainError("tip is invalid".to_string()))?;
            let tip_block = Block::deserialize(tip_block_bytes.as_ref())?;
            Ok(tip_block.get_height())
        }
    }

    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        let block_tree = self
            .blockchain
            .db
            .open_tree(self.get_blocks_tree_path())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;
        let block_bytes = block_tree
            .get(block_hash)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;

        if let Some(block_bytes) = block_bytes {
            let block = Block::deserialize(block_bytes.as_ref())?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        let mut iterator = self.iterator().await?;
        let mut blocks = vec![];
        loop {
            match iterator.next() {
                None => break,
                Some(block) => {
                    blocks.push(block.get_hash_bytes());
                }
            }
        }
        Ok(blocks)
    }

    pub fn get_blocks_tree_path(&self) -> String {
        self.file_system_tree_dir.clone()
    }

    pub fn apply_fn<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&BlockchainFileSystem) -> Result<T>,
    {
        f(self)
    }
}

pub struct BlockchainIterator {
    db: Db,
    file_system_blocks_tree: String,
    current_hash: String,
}

impl BlockchainIterator {
    fn new(tip_hash: String, db: Db, file_system_blocks_tree: String) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: tip_hash,
            file_system_blocks_tree,
            db,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self
            .db
            .open_tree(self.file_system_blocks_tree.clone())
            .unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap()?;

        let block = Block::deserialize(data.to_vec().as_slice()).unwrap();
        self.current_hash = block.get_pre_block_hash().clone();
        Some(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transaction::Transaction;

    use std::fs;

    fn generate_test_genesis_address() -> String {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    async fn create_test_blockchain() -> (BlockchainFileSystem, String) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Use process ID and random number for better isolation
        let process_id = std::process::id();
        let random_num = rand::random::<u32>();
        let test_db_path = format!(
            "test_blockchain_db_{}_{}_{}_{}",
            timestamp,
            process_id,
            random_num,
            uuid::Uuid::new_v4()
        );

        // Clean up any existing test database with retry logic
        let _ = cleanup_test_blockchain_with_retry(&test_db_path);

        // Set environment variable for unique database path
        unsafe {
            std::env::set_var("TREE_DIR", &test_db_path);
        }
        unsafe {
            std::env::set_var("BLOCKS_TREE", &test_db_path);
        }

        let genesis_address = generate_test_genesis_address();
        let blockchain = BlockchainFileSystem::create_blockchain(&genesis_address)
            .await
            .expect("Failed to create test blockchain");
        (blockchain, test_db_path)
    }

    /// Clean up test database with retry logic to handle lock issues
    fn cleanup_test_blockchain_with_retry(db_path: &str) -> std::io::Result<()> {
        for attempt in 1..=3 {
            match fs::remove_dir_all(db_path) {
                Ok(_) => return Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if attempt < 3 {
                        std::thread::sleep(std::time::Duration::from_millis(100 * attempt));
                        continue;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(()); // Directory doesn't exist, that's fine
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn cleanup_test_blockchain(db_path: &str) {
        let _ = cleanup_test_blockchain_with_retry(db_path);
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

        // Get the genesis block using the tip hash
        let tip_hash = blockchain
            .get_tip_hash()
            .await
            .expect("Failed to get tip hash");
        let genesis_block = blockchain
            .get_block(tip_hash.as_bytes())
            .await
            .expect("Failed to get genesis block")
            .expect("Genesis block should exist");
        assert_eq!(genesis_block.get_height(), 1);
        assert_eq!(genesis_block.get_pre_block_hash(), "None");

        cleanup_test_blockchain(&db_path);
    }

    #[tokio::test]
    async fn test_add_block() {
        let (mut blockchain, db_path) = create_test_blockchain().await;
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
        let (mut blockchain, db_path) = create_test_blockchain().await;
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
        let (mut blockchain, db_path) = create_test_blockchain().await;
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
        let (mut blockchain, db_path) = create_test_blockchain().await;

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

    struct TestPersistenceBlockchain {
        db_path: String,
    }

    impl TestPersistenceBlockchain {
        async fn new() -> Self {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let test_db_path =
                format!("test_persistence_db_{}_{}", timestamp, uuid::Uuid::new_v4());

            // Clean up any existing test database
            let _ = fs::remove_dir_all(&test_db_path);

            // Create a unique subdirectory for this test
            let unique_db_path = format!("{}/db", test_db_path);
            let _ = fs::create_dir_all(&unique_db_path);

            // Set environment variable for unique database path
            unsafe {
                std::env::set_var("TREE_DIR", &unique_db_path);
            }
            unsafe {
                std::env::set_var("BLOCKS_TREE", &unique_db_path);
            }

            TestPersistenceBlockchain {
                db_path: test_db_path,
            }
        }
    }

    impl Drop for TestPersistenceBlockchain {
        fn drop(&mut self) {
            // Ensure cleanup happens even if test panics
            let _ = fs::remove_dir_all(&self.db_path);
        }
    }

    #[tokio::test]
    async fn test_blockchain_persistence() {
        let _ = TestPersistenceBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();

        {
            let mut blockchain = BlockchainFileSystem::create_blockchain(&genesis_address)
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
        let blockchain = BlockchainFileSystem::create_blockchain(&genesis_address)
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
    }
}
