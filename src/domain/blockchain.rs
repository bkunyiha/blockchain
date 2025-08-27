use super::block::Block;
use super::error::{BtcError, Result};
use super::transaction::TXOutput;
use super::transaction::Transaction;
use data_encoding::HEXLOWER;
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
pub struct Blockchain {
    tip_hash: Arc<TokioRwLock<String>>, // hash of last block
    db: Db,
    is_empty: bool,
}

impl Blockchain {
    pub async fn create_blockchain(genesis_address: &str) -> Result<Blockchain> {
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

        Ok(Blockchain {
            tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
            db,
            is_empty: false,
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

    pub async fn open_blockchain() -> Result<Blockchain> {
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
        Ok(Blockchain {
            tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
            db,
            is_empty: false,
        })
    }

    pub async fn open_blockchain_empty() -> Result<Blockchain> {
        let file_system_blocks_tree = env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string());
        let path = current_dir()
            .map(|p| p.join(file_system_blocks_tree.clone()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let tip_hash = DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE.to_string();

        Ok(Blockchain {
            tip_hash: Arc::new(TokioRwLock::new(tip_hash)),
            db,
            is_empty: true,
        })
    }

    fn is_empty(&self) -> bool {
        self.is_empty
    }

    fn get_db(&self) -> &Db {
        &self.db
    }

    async fn get_tip_hash(&self) -> Result<String> {
        let tip_hash = self.tip_hash.read().await;
        Ok(tip_hash.clone())
    }

    pub async fn set_tip_hash(&self, new_tip_hash: &str) -> Result<()> {
        let mut tip_hash = self.tip_hash.write().await;
        *tip_hash = String::from(new_tip_hash);
        Ok(())
    }

    fn set_not_empty(&mut self) {
        self.is_empty = false;
    }

    // The `mine_block` function mines a new block with the transactions in the memory pool.
    // It uses the `blockchain` instance to mine the block which also adds the new block to the blockchain.
    // It returns the new block.
    pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        let best_height = self.get_best_height().await?;

        let block = Block::new_block(self.get_tip_hash().await?, transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self
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
            self.db.clone(),
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
        // Iterate through the blockchain, find all UTXOs, and return them in a HashMap.
        loop {
            match iterator.next() {
                None => break, // if no more blocks, break the loop
                Some(block) => {
                    'outer: for tx in block.get_transactions() {
                        let txid_hex = HEXLOWER.encode(tx.get_id());
                        for (idx, tx_out) in tx.get_vout().iter().enumerate() {
                            if let Some(outs) = spent_txos.get(txid_hex.as_str()) {
                                for spend_out_idx in outs {
                                    // If the output is spent(ie output of a previous transaction in-input), continue to the next output
                                    if idx.eq(spend_out_idx) {
                                        continue 'outer;
                                    }
                                }
                            }
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

                        // Coinbase transactions dont have inputs
                        if tx.is_coinbase() {
                            continue;
                        }

                        // Add the spend transaction(ie inputs) to the spent_txos map
                        for tx_in in tx.get_vin() {
                            let tx_in_id_hex = HEXLOWER.encode(tx_in.get_txid());
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

    // The `add_block` function adds a block to the blockchain.
    // It uses the `block_tree` instance to add the block to the blockchain.
    // It uses the `block` instance to add the block to the blockchain.
    // It will not add the block if its height is less than current tip height in the block chain.
    pub async fn add_block(&mut self, new_block: &Block) -> Result<()> {
        let block_tree = self
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

    pub fn get_db_path(&self) -> String {
        env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string())
    }

    pub fn get_blocks_tree_path(&self) -> String {
        env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string())
    }

    pub fn apply_fn<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Blockchain) -> Result<T>,
    {
        f(self)
    }
}

#[derive(Debug)]
pub struct BlockchainService(Arc<TokioRwLock<Blockchain>>);

impl Clone for BlockchainService {
    fn clone(&self) -> Self {
        BlockchainService(self.0.clone())
    }
}
impl BlockchainService {
    pub fn new(blockchain: Blockchain) -> BlockchainService {
        BlockchainService(Arc::new(TokioRwLock::new(blockchain)))
    }

    // /// Apply a readfunction to a blockchain and return the result
    // async fn read<F, T>(&self, f: F) -> Result<T>
    // where
    //     F: AsyncFnOnce(&Blockchain) -> Result<T>,
    // {
    //     let blockchain_guard = self.0.read().await;
    //     f(&blockchain_guard).await
    // }

    // /// Apply a write function to a blockchain and return the result
    // async fn write<F, T>(&self, f: F) -> Result<T>
    // where
    //     F: AsyncFnOnce(&mut Blockchain) -> Result<T>,
    // {
    //     let mut blockchain_guard = self.0.write().await;
    //     f(&mut blockchain_guard).await
    // }

    pub async fn get_db(&self) -> Result<Db> {
        let blockchain_guard = self.0.read().await;
        Ok(blockchain_guard.get_db().clone())
    }

    /// Get the best height of the blockchain
    pub async fn get_best_height(&self) -> Result<usize> {
        let blockchain_guard = self.0.read().await;
        blockchain_guard.get_best_height().await
    }

    /// Get the block hashes of the blockchain
    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        let blockchain_guard = self.0.read().await;
        blockchain_guard.get_block_hashes().await
    }

    /// Get the block of the blockchain
    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        let blockchain_guard = self.0.read().await;
        blockchain_guard.get_block(block_hash).await
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
        let blockchain_guard = self.0.read().await;
        blockchain_guard.find_transaction(txid).await
    }
    pub async fn find_utxo(&self) -> Result<HashMap<String, Vec<TXOutput>>> {
        let blockchain_guard = self.0.read().await;
        blockchain_guard.find_utxo().await
    }
    pub async fn iterator(&self) -> Result<BlockchainIterator> {
        let blockchain_guard = self.0.read().await;
        blockchain_guard.iterator().await
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
        let wallet = crate::domain::wallet::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    async fn create_test_blockchain() -> (Blockchain, String) {
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
        let blockchain = Blockchain::create_blockchain(&genesis_address)
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
            let mut blockchain = Blockchain::create_blockchain(&genesis_address)
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
        let blockchain = Blockchain::create_blockchain(&genesis_address)
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
