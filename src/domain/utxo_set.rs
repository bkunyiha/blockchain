use super::block::Block;
use super::blockchain::Blockchain;
use super::transaction::TXOutput;
use data_encoding::HEXLOWER;
use std::collections::HashMap;

const UTXO_TREE: &str = "chainstate";

pub struct UTXOSet {
    blockchain: Blockchain,
}

impl UTXOSet {
    pub fn new(blockchain: Blockchain) -> UTXOSet {
        UTXOSet { blockchain }
    }

    pub fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    ///
    /// The `find_spendable_outputs` function finds the spendable outputs for a given public key hash and amount.
    /// It iterates through UTXOs, checks ownership, accumulates values,
    /// and forms a HashMap of transaction IDs to output indices for spendable outputs.
    ///
    /// # Arguments
    ///
    /// * `pub_key_hash` - A reference to the public key hash.
    /// * `amount` - The required amount.
    ///
    /// # Returns
    ///
    /// A tuple containing the accumulated amount and a HashMap of transaction IDs to output indices for spendable outputs.
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<usize>>) {
        let mut unspent_outputs: HashMap<String, Vec<usize>> = HashMap::new();
        let mut accmulated = 0;
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        for item in utxo_tree.iter() {
            let (k, v) = item.unwrap();
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let tx_out: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .expect("unable to deserialize TXOutput");
            for (idx, out) in tx_out.iter().enumerate() {
                if out.is_locked_with_key(pub_key_hash) && accmulated < amount {
                    accmulated += out.get_value();
                    if unspent_outputs.contains_key(txid_hex.as_str()) {
                        unspent_outputs
                            .get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(idx);
                    } else {
                        unspent_outputs.insert(txid_hex.clone(), vec![idx]);
                    }
                }
            }
        }
        (accmulated, unspent_outputs)
    }

    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Vec<TXOutput> {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        let mut utxos = vec![];
        for item in utxo_tree.iter() {
            let (_, v) = item.unwrap();
            let outs: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .expect("unable to deserialize TXOutput");
            for out in outs.iter() {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.push(out.clone())
                }
            }
        }
        utxos
    }

    pub fn count_transactions(&self) -> i32 {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        let mut counter = 0;
        for _ in utxo_tree.iter() {
            counter += 1;
        }
        counter
    }

    /// The `reindex` function reindexes the UTXO set by clearing the existing UTXO tree and rebuilding it from the blockchain.
    /// It iterates through the blockchain, finds all UTXOs, and inserts them into the UTXO tree.
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    ///
    pub fn reindex(&self) {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        utxo_tree.clear().unwrap();

        let utxo_map = self.blockchain.find_utxo();
        for (txid_hex, outs) in &utxo_map {
            let txid = HEXLOWER.decode(txid_hex.as_bytes()).unwrap();
            let value = bincode::serialize(outs).unwrap();
            let _ = utxo_tree.insert(txid.as_slice(), value).unwrap();
        }
    }

    pub fn update(&self, block: &Block) {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        for block_tx in block.get_transactions() {
            if !block_tx.is_coinbase() {
                for curr_tx_inpt in block_tx.get_vin() {
                    let mut updated_outs = vec![];
                    let outs_bytes = utxo_tree.get(curr_tx_inpt.get_txid()).unwrap().unwrap();
                    let utxo_curr_tx_outs: Vec<TXOutput> =
                        bincode::deserialize(outs_bytes.as_ref())
                            .expect("unable to deserialize TXOutput");
                    for (utxo_curr_utxo_idx, db_curr_utxo) in utxo_curr_tx_outs.iter().enumerate() {
                        if utxo_curr_utxo_idx != curr_tx_inpt.get_vout() {
                            updated_outs.push(db_curr_utxo.clone())
                        }
                    }
                    if updated_outs.is_empty() {
                        utxo_tree.remove(curr_tx_inpt.get_txid()).unwrap();
                    } else {
                        let outs_bytes = bincode::serialize(&updated_outs)
                            .expect("unable to serialize TXOutput");
                        utxo_tree
                            .insert(curr_tx_inpt.get_txid(), outs_bytes)
                            .unwrap();
                    }
                }
            }
            let mut new_outputs = vec![];
            for curr_tx_out in block_tx.get_vout() {
                new_outputs.push(curr_tx_out.clone())
            }
            let outs_bytes =
                bincode::serialize(&new_outputs).expect("unable to serialize TXOutput");
            let _ = utxo_tree.insert(block_tx.get_id(), outs_bytes).unwrap();
        }
    }
}
