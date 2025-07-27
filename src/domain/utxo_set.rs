use super::block::Block;
use super::blockchain::Blockchain;
use super::transaction::TXOutput;
use super::transaction::Transaction;
use crate::domain::error::{BtcError, Result};
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
    ) -> Result<(i32, HashMap<String, Vec<usize>>)> {
        let mut unspent_outputs: HashMap<String, Vec<usize>> = HashMap::new();
        let mut accmulated = 0;
        let db = self.blockchain.get_db();
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        for item in utxo_tree.iter() {
            let (k, v) = item.unwrap();
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let tx_out: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?;
            for (idx, out) in tx_out
                .iter()
                .filter(|out| out.not_in_global_mem_pool())
                .enumerate()
            {
                if out.is_locked_with_key(pub_key_hash) && accmulated < amount {
                    accmulated += out.get_value();
                    if unspent_outputs.contains_key(txid_hex.as_str()) {
                        unspent_outputs
                            .get_mut(txid_hex.as_str())
                            .ok_or(BtcError::UTXONotFoundError(format!(
                                "(find_spendable_outputs) UTXO {} not found",
                                txid_hex
                            )))?
                            .push(idx);
                    } else {
                        unspent_outputs.insert(txid_hex.clone(), vec![idx]);
                    }
                }
            }
        }
        Ok((accmulated, unspent_outputs))
    }

    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Result<Vec<TXOutput>> {
        let db = self.blockchain.get_db();
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        let mut utxos = vec![];
        for item in utxo_tree.iter() {
            let (_, v) = item.unwrap();
            let outs: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice())
                .map_err(|e| BtcError::TransactionDeserializationError(e.to_string()))?;
            for out in outs.iter() {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.push(out.clone())
                }
            }
        }
        Ok(utxos)
    }

    pub fn count_transactions(&self) -> Result<i32> {
        let db = self.blockchain.get_db();
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        let mut counter = 0;
        for _ in utxo_tree.iter() {
            counter += 1;
        }
        Ok(counter)
    }

    /// The `reindex` function reindexes the UTXO set by clearing the existing UTXO tree and rebuilding it from the blockchain.
    /// It iterates through the blockchain, finds all UTXOs, and inserts them into the UTXO tree.
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    ///
    pub fn reindex(&self) -> Result<()> {
        let db = self.blockchain.get_db();
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        utxo_tree
            .clear()
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

        let utxo_map = self.blockchain.find_utxo()?;
        for (txid_hex, outs) in &utxo_map {
            let txid = HEXLOWER
                .decode(txid_hex.as_bytes())
                .map_err(|e| BtcError::TransactionIdHexDecodingError(e.to_string()))?;
            let value = bincode::serialize(outs)
                .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
            let _ = utxo_tree
                .insert(txid.as_slice(), value)
                .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
        }
        Ok(())
    }

    pub fn update(&self, block: &Block) -> Result<()> {
        let db = self.blockchain.get_db();
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
        for block_tx in block.get_transactions() {
            if !block_tx.is_coinbase() {
                for curr_blc_tx_inpt in block_tx.get_vin() {
                    let mut updated_outs = vec![];
                    let curr_blc_tx_inpt_utxo_ivec = utxo_tree
                        .get(curr_blc_tx_inpt.get_txid())
                        .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                        .ok_or(BtcError::UTXONotFoundError(format!(
                            "(update) UTXO {} not found",
                            HEXLOWER.encode(curr_blc_tx_inpt.get_txid())
                        )))?;
                    let curr_blc_tx_inpt_utxo_list: Vec<TXOutput> =
                        bincode::deserialize(curr_blc_tx_inpt_utxo_ivec.as_ref()).map_err(|e| {
                            BtcError::TransactionDeserializationError(e.to_string())
                        })?;
                    for (utxo_curr_utxo_idx, db_curr_utxo) in
                        curr_blc_tx_inpt_utxo_list.iter().enumerate()
                    {
                        if utxo_curr_utxo_idx != curr_blc_tx_inpt.get_vout() {
                            updated_outs.push(db_curr_utxo.clone())
                        }
                    }
                    if updated_outs.is_empty() {
                        utxo_tree
                            .remove(curr_blc_tx_inpt.get_txid())
                            .map_err(|e| BtcError::RemovingUTXOError(e.to_string()))?;
                    } else {
                        let outs_bytes = bincode::serialize(&updated_outs)
                            .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
                        utxo_tree
                            .insert(curr_blc_tx_inpt.get_txid(), outs_bytes)
                            .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                    }
                }
            }
            let mut new_outputs = vec![];
            for curr_tx_out in block_tx.get_vout() {
                new_outputs.push(curr_tx_out.clone())
            }
            let outs_bytes = bincode::serialize(&new_outputs)
                .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
            let _ = utxo_tree
                .insert(block_tx.get_id(), outs_bytes)
                .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
        }
        Ok(())
    }

    pub fn update_global_mem_pool(&self, tx: &Transaction) -> Result<()> {
        let db = self.blockchain.get_db();
        let utxo_tree = db
            .open_tree(UTXO_TREE)
            .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

        if !tx.is_coinbase() {
            for curr_tx_inpt in tx.get_vin() {
                if let Some(curr_tx_inpt_utxo_ivec) = utxo_tree
                    .get(curr_tx_inpt.get_txid())
                    .map_err(|e| BtcError::GettingUTXOError(e.to_string()))?
                {
                    let mut curr_tx_inpt_utxo_list: Vec<TXOutput> =
                        bincode::deserialize(curr_tx_inpt_utxo_ivec.as_ref()).map_err(|e| {
                            BtcError::TransactionDeserializationError(e.to_string())
                        })?;
                    for (utxo_curr_utxo_idx, db_curr_utxo) in
                        curr_tx_inpt_utxo_list.iter_mut().enumerate()
                    {
                        if utxo_curr_utxo_idx == curr_tx_inpt.get_vout() {
                            // Flag the TXOutput as in global mem pool
                            db_curr_utxo.set_in_global_mem_pool(true);
                            log::info!(
                                "\n\n ------------------------------------------------------"
                            );
                            log::info!("Set TXOUT to Intransit");
                            log::info!("utxo_curr_utxo_idx: {:?}", utxo_curr_utxo_idx);
                            log::info!("db_curr_utxo.get_value(): {:?}", db_curr_utxo.get_value());
                            for tx_out in tx.get_vout() {
                                log::info!("tx_out.get_value(): {:?}", tx_out.get_value());
                            }
                            log::info!("------------------------------------------------------");
                        }
                    }
                    log::info!("Update UTXO in DB");
                    let outs_bytes = bincode::serialize(&curr_tx_inpt_utxo_list)
                        .map_err(|e| BtcError::TransactionSerializationError(e.to_string()))?;
                    utxo_tree
                        .insert(curr_tx_inpt.get_txid(), outs_bytes)
                        .map_err(|e| BtcError::SavingUTXOError(e.to_string()))?;
                } else {
                    log::info!("TXOUT not found in DB");
                }
            }
        }
        Ok(())
    }
}
