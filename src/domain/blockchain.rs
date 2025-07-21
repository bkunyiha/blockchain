use super::block::Block;
use super::transaction::TXOutput;
use super::transaction::Transaction;
use data_encoding::HEXLOWER;
use log::info;
use sled::transaction::TransactionResult;
use sled::{Db, Tree};
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::sync::{Arc, RwLock};

const DEFAULT_TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const DEFAULT_BLOCKS_TREE: &str = "blocks1";
const DEFAULT_TREE_DIR: &str = "data1";

#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,
}

impl Blockchain {
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join(Blockchain::get_db_path())).unwrap();
        let blocks_tree = db.open_tree(Blockchain::get_blocks_tree_path()).unwrap();

        let data = blocks_tree.get(DEFAULT_TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash = data.map_or_else(
            || {
                let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
                let block = Block::generate_genesis_block(&coinbase_tx);
                Self::update_blocks_tree(&blocks_tree, &block);
                String::from(block.get_hash())
            },
            |data| String::from_utf8(data.to_vec()).unwrap(),
        );

        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert(DEFAULT_TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }

    pub fn new_blockchain() -> Blockchain {
        let db = sled::open(current_dir().unwrap().join(Blockchain::get_db_path())).unwrap();
        let blocks_tree = db.open_tree(Blockchain::get_blocks_tree_path()).unwrap();
        let tip_bytes = blocks_tree
            .get(DEFAULT_TIP_BLOCK_HASH_KEY)
            .unwrap()
            .expect("No existing blockchain found. Create one first.");
        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash)
    }

    // The `mine_block` function mines a new block with the transactions in the memory pool.
    // It uses the `blockchain` instance to mine the block which also adds the new block to the blockchain.
    // It returns the new block.
    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        for trasaction in transactions {
            if !trasaction.verify(self) {
                panic!("ERROR: Invalid transaction")
            }
        }
        let best_height = self.get_best_height();

        let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .unwrap();
        // The `update_blocks_tree` function updates the blocks tree with the new block.
        // It uses the `blocks_tree` instance to update the blocks tree.
        // It uses the `block` instance to update the blocks tree.
        // It returns the new block.
        Self::update_blocks_tree(&blocks_tree, &block);
        self.set_tip_hash(block_hash);
        block
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
    }

    /// The `find_utxo` function finds all unspent transaction outputs (UTXOs) in the blockchain.
    /// It iterates through the blockchain, finds all UTXOs, and returns them in a HashMap.
    ///
    /// # Returns
    ///
    /// A HashMap containing transaction IDs as keys and vectors of TXOutput as values.
    ///
    pub fn find_utxo(&self) -> HashMap<String, Vec<TXOutput>> {
        let mut utxo: HashMap<String, Vec<TXOutput>> = HashMap::new();
        let mut spent_txos: HashMap<String, Vec<usize>> = HashMap::new();
        let mut iterator = self.iterator();
        // Iterate through the blockchain, find all UTXOs, and return them in a HashMap.
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            'outer: for tx in block.get_transactions() {
                let txid_hex = HEXLOWER.encode(tx.get_id());
                for (idx, tx_out) in tx.get_vout().iter().enumerate() {
                    if let Some(outs) = spent_txos.get(txid_hex.as_str()) {
                        for spend_out_idx in outs {
                            if idx.eq(spend_out_idx) {
                                continue 'outer;
                            }
                        }
                    }
                    if utxo.contains_key(txid_hex.as_str()) {
                        utxo.get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(tx_out.clone());
                    } else {
                        utxo.insert(txid_hex.clone(), vec![tx_out.clone()]);
                    }
                }

                if tx.is_coinbase() {
                    continue;
                }

                for tx_in in tx.get_vin() {
                    let tx_in_id_hex = HEXLOWER.encode(tx_in.get_txid());
                    if spent_txos.contains_key(tx_in_id_hex.as_str()) {
                        spent_txos
                            .get_mut(tx_in_id_hex.as_str())
                            .unwrap()
                            .push(tx_in.get_vout());
                    } else {
                        spent_txos.insert(tx_in_id_hex, vec![tx_in.get_vout()]);
                    }
                }
            }
        }
        utxo
    }

    pub fn find_transaction(&self, txid: &[u8]) -> Option<Transaction> {
        let mut iterator = self.iterator();
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            for transaction in block.get_transactions() {
                if txid.eq(transaction.get_id()) {
                    return Some(transaction.clone());
                }
            }
        }
        None
    }

    // The `add_block` function adds a block to the blockchain.
    // It uses the `block_tree` instance to add the block to the blockchain.
    // It uses the `block` instance to add the block to the blockchain.
    // It will not add the block if its height is less than current tip height in the block chain.
    pub fn add_block(&self, block: &Block) {
        let block_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .unwrap();
        if block_tree.get(block.get_hash()).unwrap().is_some() {
            return;
        }
        let _: TransactionResult<(), ()> = block_tree.transaction(|transaction| {
            let _ = transaction
                .insert(block.get_hash(), block.serialize())
                .unwrap();

            let tip_block_bytes = transaction
                .get(self.get_tip_hash())
                .unwrap()
                .expect("The tip hash is not valid");
            let tip_block = Block::deserialize(tip_block_bytes.as_ref());
            if block.get_height() > tip_block.get_height() {
                let _ = transaction
                    .insert(DEFAULT_TIP_BLOCK_HASH_KEY, block.get_hash())
                    .unwrap();
                self.set_tip_hash(block.get_hash());
            } else {
                info!(
                    "Block {:?} not added because its height is less than mine",
                    block.get_hash()
                );
            }
            Ok(())
        });
    }

    pub fn get_best_height(&self) -> usize {
        let block_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .unwrap();
        let tip_block_bytes = block_tree
            .get(self.get_tip_hash())
            .unwrap()
            .expect("The tip hash is valid");
        let tip_block = Block::deserialize(tip_block_bytes.as_ref());
        tip_block.get_height()
    }

    pub fn get_block(&self, block_hash: &[u8]) -> Option<Block> {
        let block_tree = self
            .db
            .open_tree(Blockchain::get_blocks_tree_path())
            .unwrap();
        if let Some(block_bytes) = block_tree.get(block_hash).unwrap() {
            let block = Block::deserialize(block_bytes.as_ref());
            Some(block)
        } else {
            None
        }
    }

    pub fn get_block_hashes(&self) -> Vec<Vec<u8>> {
        let mut iterator = self.iterator();
        let mut blocks = vec![];
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            blocks.push(block.get_hash_bytes());
        }
        blocks
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

        let block = Block::deserialize(data.to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().clone();
        Some(block)
    }
}
