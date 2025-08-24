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
use std::sync::{Arc, RwLock};
use tracing::info;

const DEFAULT_TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE: &str = "empty";
const DEFAULT_BLOCKS_TREE: &str = "blocks1";
const DEFAULT_TREE_DIR: &str = "data1";

#[derive(Clone, Debug)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,
    is_empty: bool,
}

impl Blockchain {
    pub fn create_blockchain(genesis_address: &str) -> Result<Blockchain> {
        let path = current_dir()
            .map(|p| p.join(Blockchain::get_db_path()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let blocks_tree = db
            .open_tree(Blockchain::get_blocks_tree_path())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        let data = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)
            .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?;
        let tip_hash = data.map_or_else(
            || {
                let coinbase_tx = Transaction::new_coinbase_tx(genesis_address)?;
                let block = Block::generate_genesis_block(&coinbase_tx);
                Self::update_blocks_tree(&blocks_tree, &block)?;
                Ok(String::from(block.get_hash()))
            },
            |data| {
                String::from_utf8(data.to_vec())
                    .map_err(|e| BtcError::BlockChainTipHashError(e.to_string()))
            },
        )?;

        Ok(Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
            is_empty: false,
        })
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) -> Result<()> {
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

    pub fn open_blockchain() -> Result<Blockchain> {
        let path = current_dir()
            .map(|p| p.join(Blockchain::get_db_path()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let blocks_tree = db
            .open_tree(Blockchain::get_blocks_tree_path())
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
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
            is_empty: false,
        })
    }

    pub fn open_blockchain_empty() -> Result<Blockchain> {
        let path = current_dir()
            .map(|p| p.join(Blockchain::get_db_path()))
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let db = sled::open(path).map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        let tip_hash = DEFAULT_EMPTY_TIP_BLOCK_HASH_VALUE.to_string();

        Ok(Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
            is_empty: true,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> Result<String> {
        self.tip_hash
            .read()
            .map_err(|e| BtcError::BlockchainTipHashPoisonedLockError(e.to_string()))
            .map(|v| v.clone())
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) -> Result<()> {
        let mut tip_hash = self
            .tip_hash
            .write()
            .map_err(|e| BtcError::BlockchainTipHashPoisonedLockError(e.to_string()))?;
        *tip_hash = String::from(new_tip_hash);
        Ok(())
    }

    fn set_not_empty(&mut self) {
        self.is_empty = false;
    }

    // The `mine_block` function mines a new block with the transactions in the memory pool.
    // It uses the `blockchain` instance to mine the block which also adds the new block to the blockchain.
    // It returns the new block.
    pub fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        for trasaction in transactions {
            let is_valid = trasaction.verify(self)?;
            if !is_valid {
                return Err(BtcError::InvalidTransaction);
            }
        }
        let best_height = self.get_best_height()?;

        let block = Block::new_block(self.get_tip_hash()?, transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .map_err(|e| BtcError::BlockchainDBconnection(e.to_string()))?;
        // The `update_blocks_tree` function updates the blocks tree with the new block.
        // It uses the `blocks_tree` instance to update the blocks tree.
        // It uses the `block` instance to update the blocks tree.
        // It returns the new block.
        Self::update_blocks_tree(&blocks_tree, &block)?;
        self.set_tip_hash(block_hash)?;
        Ok(block)
    }

    pub fn iterator(&self) -> Result<BlockchainIterator> {
        self.get_tip_hash()
            .map(|hash| BlockchainIterator::new(hash, self.db.clone()))
    }

    /// The `find_utxo` function finds all unspent transaction outputs (UTXOs) in the blockchain.
    /// It iterates through the blockchain, finds all UTXOs, and returns them in a HashMap.
    ///
    /// # Returns
    ///
    /// A HashMap containing transaction IDs as keys and vectors of TXOutput as values.
    ///
    pub fn find_utxo(&self) -> Result<HashMap<String, Vec<TXOutput>>> {
        let mut utxo: HashMap<String, Vec<TXOutput>> = HashMap::new();
        let mut spent_txos: HashMap<String, Vec<usize>> = HashMap::new();
        let mut iterator = self.iterator()?;
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

    pub fn find_transaction(&self, txid: &[u8]) -> Result<Option<Transaction>> {
        let mut iterator = self.iterator()?;
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
    pub fn add_block(&mut self, new_block: &Block) -> Result<()> {
        let block_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

        if self.is_empty() {
            info!("Blockchain is empty, adding block");

            self.set_not_empty();
            info!("Blockchain is now not empty");
            Self::update_blocks_tree(&block_tree, new_block)?;
            self.set_tip_hash(new_block.get_hash())?;
            let best_height = self.get_best_height()?;
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
            let tip_hash = self.get_tip_hash()?;
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

                    self.set_tip_hash(new_block.get_hash()).unwrap();
                } else {
                    info!(
                        "Block {:?} not added because its height is less than mine",
                        new_block.get_hash()
                    );
                }

                Ok(())
            });
        }

        Ok(())
    }

    pub fn get_best_height(&self) -> Result<usize> {
        if self.is_empty() {
            info!("Blockchain is empty, returning height 0");
            Ok(0)
        } else {
            let block_tree = self
                .db
                .open_tree(Blockchain::get_blocks_tree_path())
                .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;
            let tip_block_bytes = block_tree
                .get(self.get_tip_hash()?)
                .map_err(|e| BtcError::GetBlockchainError(e.to_string()))?
                .ok_or(BtcError::GetBlockchainError("tip is invalid".to_string()))?;
            let tip_block = Block::deserialize(tip_block_bytes.as_ref())?;
            Ok(tip_block.get_height())
        }
    }

    pub fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        let block_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
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

    pub fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        let mut iterator = self.iterator()?;
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

    pub fn get_db_path() -> String {
        env::var("TREE_DIR").unwrap_or(DEFAULT_TREE_DIR.to_string())
    }

    pub fn get_blocks_tree_path() -> String {
        env::var("BLOCKS_TREE").unwrap_or(DEFAULT_BLOCKS_TREE.to_string())
    }
}

pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}

impl BlockchainIterator {
    fn new(tip_hash: String, db: Db) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: tip_hash,
            db,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap()?;

        let block = Block::deserialize(data.to_vec().as_slice()).unwrap();
        self.current_hash = block.get_pre_block_hash().clone();
        Some(block)
    }
}
