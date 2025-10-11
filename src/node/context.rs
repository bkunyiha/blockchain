//! Node context - central coordination point for all node operations
//!
//! This replaces the scattered service layer and provides a clean interface
//! for the web/RPC layer to interact with the blockchain node.
//!
//! Following Bitcoin Core's architecture, this module orchestrates between:
//! - Blockchain state
//! - Transaction mempool
//! - Network operations
//! - Validation

use crate::GLOBAL_CONFIG;
use crate::chain::{BlockchainService, UTXOSet};
use crate::error::{BtcError, Result};
use crate::net::net_processing::send_inv;
use crate::node::miner;
use crate::node::miner::{
    cleanup_invalid_transactions, prepare_mining_transactions, process_mine_block,
    should_trigger_mining,
};
use crate::node::txmempool::{
    add_to_memory_pool, remove_from_memory_pool, transaction_exists_in_pool,
};
use crate::node::{CENTERAL_NODE, GLOBAL_NODES, Node, OpType};
use crate::transaction::TxSummary;
use crate::{Block, Transaction, WalletAddress};
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::{error, info, warn};

/// Node context - holds references to all node subsystems
///
/// This is the Bitcoin Core way: one central context that orchestrates
/// between mempool, network, blockchain, and validation.
#[derive(Clone, Debug)]
pub struct NodeContext {
    /// Blockchain service
    blockchain: BlockchainService,
}

impl NodeContext {
    /// Create new node context
    pub fn new(blockchain: BlockchainService) -> Self {
        Self { blockchain }
    }

    pub fn get_blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }
    // ========== Transaction Operations ==========

    pub async fn btc_transaction(
        &self,
        wlt_frm_addr: &WalletAddress,
        wlt_to_addr: &WalletAddress,
        amount: i32,
    ) -> Result<()> {
        // Create UTXO set
        let utxo_set = UTXOSet::new(self.blockchain.clone());

        // Create and submit transaction
        let transaction =
            Transaction::new_utxo_transaction(wlt_frm_addr, wlt_to_addr, amount, &utxo_set).await?;

        // Get node address
        let node_addr = crate::GLOBAL_CONFIG.get_node_addr();
        self.process_transaction(&node_addr, transaction).await
    }

    /// Submit transaction to mempool and broadcast to network
    ///
    /// This is what your web layer should call!
    /// It handles validation, mempool addition, and network broadcast.
    pub async fn submit_transaction(&self, tx: Transaction) -> Result<()> {
        // Get node address
        let addr = crate::GLOBAL_CONFIG.get_node_addr();

        // Process transaction (validates, adds to mempool, broadcasts)
        self.process_transaction(&addr, tx).await
    }

    /// Process transaction - validates, adds to mempool, and triggers mining if needed
    ///
    /// This is the main entry point for processing new transactions, similar to
    /// Bitcoin Core's AcceptToMemoryPool and ProcessNewTransaction
    pub async fn process_transaction(
        &self,
        addr_from: &std::net::SocketAddr,
        tx: Transaction,
    ) -> Result<()> {
        // Check if transaction exists
        if transaction_exists_in_pool(&tx) {
            info!("Transaction: {:?} already exists", tx.get_id());
            return Err(BtcError::TransactionAlreadyExistsInMemoryPool(
                tx.get_tx_id_hex(),
            ));
        }

        let txid = tx.get_id_bytes();

        // Add to Memory Pool
        add_to_memory_pool(tx, &self.blockchain).await;

        let my_node_addr = GLOBAL_CONFIG.get_node_addr();

        // If the node is the central node, broadcast the transaction to all other nodes
        if my_node_addr.eq(&CENTERAL_NODE) {
            let nodes = self.get_nodes_excluding_sender(addr_from).await?;
            self.broadcast_transaction_to_nodes(&nodes, txid).await;
        }

        // Check if mining should be triggered
        if should_trigger_mining() {
            match prepare_mining_transactions() {
                Ok(txs) => {
                    if !txs.is_empty() {
                        process_mine_block(txs, &self.blockchain).await.map(|_| ())
                    } else {
                        warn!("Mining triggered but no valid transactions to mine");
                        Ok(())
                    }
                }
                Err(e) => {
                    error!("Failed to prepare mining transactions: {}", e);
                    // Clean up any invalid transactions from memory pool
                    cleanup_invalid_transactions().await
                }
            }
        } else {
            Ok(())
        }
    }

    /// Get nodes excluding the sender
    async fn get_nodes_excluding_sender(
        &self,
        addr_from: &std::net::SocketAddr,
    ) -> Result<Vec<Node>> {
        let nodes = GLOBAL_NODES
            .get_nodes()
            .expect("Global nodes get error")
            .into_iter()
            .filter(|node| {
                let node_addr = node.get_addr();
                let my_addr = GLOBAL_CONFIG.get_node_addr();
                node_addr != *addr_from && node_addr != my_addr
            })
            .collect();
        Ok(nodes)
    }

    /// Broadcast transaction to nodes functionally
    async fn broadcast_transaction_to_nodes(&self, nodes: &[Node], txid: Vec<u8>) {
        let txid_clone = txid.clone();
        nodes.iter().for_each(|node| {
            let node_addr = node.get_addr();
            let txid = txid_clone.clone();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Tx, &[txid]).await;
            });
        });
    }

    /// Get transaction from mempool
    pub fn get_transaction(&self, txid: &str) -> Result<Option<Transaction>> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.get(txid)
    }

    pub async fn find_all_transactions(&self) -> Result<HashMap<String, TxSummary>> {
        self.blockchain.find_all_transactions().await
    }

    /// Get all mempool transactions
    pub fn get_mempool_transactions(&self) -> Result<Vec<Transaction>> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.get_all()
    }

    /// Get mempool size
    pub fn get_mempool_size(&self) -> Result<usize> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.len()
    }

    pub async fn remove_from_memory_pool(&self, tx: Transaction) {
        remove_from_memory_pool(tx, &self.blockchain).await;
    }

    // ========== Blockchain Operations ==========

    pub async fn add_block(&self, block: &Block) -> Result<()> {
        self.blockchain.add_block(block).await
    }

    /// Get blockchain height
    pub async fn get_blockchain_height(&self) -> Result<usize> {
        self.blockchain.get_best_height().await
    }

    pub async fn get_block_hashes(&self) -> Result<Vec<Vec<u8>>> {
        self.blockchain.get_block_hashes().await
    }

    /// Get block by
    pub async fn get_block(&self, block_hash: &[u8]) -> Result<Option<Block>> {
        self.blockchain.get_block(block_hash).await
    }
    /// Get block by hash
    pub async fn get_block_by_hash(&self, hash: &str) -> Result<Option<Block>> {
        self.blockchain.get_block_by_hash(hash.as_bytes()).await
    }

    /// Get latest blocks
    pub async fn get_latest_blocks(&self, count: usize) -> Result<Vec<Block>> {
        let height = self.blockchain.get_best_height().await?;
        let start_height = height.saturating_sub(count);

        // Use get_blocks_by_height which returns a Vec<Block>
        self.blockchain
            .get_blocks_by_height(start_height, height)
            .await
    }

    /// Mine a block
    pub async fn mine_block(&self, transactions: &[Transaction]) -> Result<Block> {
        self.blockchain.mine_block(transactions).await
    }

    /// Mine empty block
    pub async fn mine_empty_block(&self) -> Result<Block> {
        miner::mine_empty_block(&self.blockchain).await
    }

    // ========== Wallet Operations ==========

    /// Get balance for address
    pub async fn get_balance(&self, address: &WalletAddress) -> Result<i32> {
        let utxo_set = UTXOSet::new(self.blockchain.clone());
        utxo_set.get_balance(address).await
    }

    /// Create new wallet
    pub fn create_wallet(&self) -> Result<crate::Wallet> {
        crate::Wallet::new()
    }

    /// Get wallet by address
    pub fn get_wallet(&self, address: &WalletAddress) -> Result<Option<crate::Wallet>> {
        use crate::wallet::WalletService;
        let wallets = WalletService::new()?;
        Ok(wallets.get_wallet(address).cloned())
    }

    /// List all wallet addresses
    pub fn list_wallet_addresses(&self) -> Result<Vec<WalletAddress>> {
        use crate::wallet::WalletService;
        let wallets = WalletService::new()?;
        Ok(wallets.get_addresses())
    }

    // ========== Network Operations ==========

    /// Get connected peers
    pub fn get_peers(&self) -> Result<Vec<SocketAddr>> {
        use crate::node::GLOBAL_NODES;
        let nodes = GLOBAL_NODES.get_nodes()?;
        Ok(nodes.into_iter().map(|n| n.get_addr()).collect())
    }

    /// Get peer count
    pub fn get_peer_count(&self) -> Result<usize> {
        use crate::node::GLOBAL_NODES;
        let nodes = GLOBAL_NODES.get_nodes()?;
        Ok(nodes.len())
    }

    // ========== Validation Operations ==========

    /// Validate transaction
    pub async fn validate_transaction(&self, tx: &Transaction) -> Result<bool> {
        let _utxo_set = UTXOSet::new(self.blockchain.clone());

        // Check if transaction is valid
        if tx.is_coinbase() {
            return Ok(true);
        }

        // Verify transaction has valid inputs/outputs
        if tx.get_vin().is_empty() || tx.get_vout().is_empty() {
            return Ok(false);
        }

        // Verify outputs are not in mempool
        for output in tx.get_vout() {
            if output.is_in_global_mem_pool() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Validate block
    pub fn validate_block(&self, block: &Block) -> Result<bool> {
        // Check block has transactions
        if block.get_transactions().is_empty() {
            return Ok(false);
        }

        // Check first transaction is coinbase
        if !block.get_transactions()[0].is_coinbase() {
            return Ok(false);
        }

        // Check only first transaction is coinbase
        for tx in &block.get_transactions()[1..] {
            if tx.is_coinbase() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    // ========== Internal Helpers ==========

    /// Get reference to blockchain service
    pub fn blockchain(&self) -> &BlockchainService {
        &self.blockchain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_address() -> crate::WalletAddress {
        let wallet = crate::Wallet::new().expect("Failed to create wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    #[tokio::test]
    async fn test_node_context_creation() {
        let genesis_address = generate_test_address();
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create blockchain");

        let node = NodeContext::new(blockchain);

        // Should be able to get height
        let height = node.get_blockchain_height().await;
        assert!(height.is_ok());
    }

    #[tokio::test]
    async fn test_get_balance() {
        let genesis_address = generate_test_address();
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create blockchain");

        let node = NodeContext::new(blockchain);

        // Should be able to get balance
        let balance = node.get_balance(&genesis_address.clone()).await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_create_wallet() {
        let genesis_address = generate_test_address();
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create blockchain");

        let node = NodeContext::new(blockchain);

        // Should be able to create wallet
        let wallet = node.create_wallet();
        assert!(wallet.is_ok());
    }

    #[tokio::test]
    async fn test_list_wallet_addresses() {
        let genesis_address = generate_test_address();
        let blockchain = BlockchainService::initialize(&genesis_address)
            .await
            .expect("Failed to create blockchain");

        let node = NodeContext::new(blockchain);

        // Should be able to list addresses
        let addresses = node.list_wallet_addresses();
        assert!(addresses.is_ok());
    }
}
