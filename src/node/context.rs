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

use crate::chain::{BlockchainService, UTXOSet};
use crate::error::Result;
use crate::{Block, Transaction};
use std::net::SocketAddr;

/// Node context - holds references to all node subsystems
///
/// This is the Bitcoin Core way: one central context that orchestrates
/// between mempool, network, blockchain, and validation.
#[derive(Clone)]
pub struct NodeContext {
    /// Blockchain service
    blockchain: BlockchainService,
}

impl NodeContext {
    /// Create new node context
    pub fn new(blockchain: BlockchainService) -> Self {
        Self { blockchain }
    }

    // ========== Transaction Operations ==========

    /// Submit transaction to mempool and broadcast to network
    ///
    /// This is what your web layer should call!
    /// It handles validation, mempool addition, and network broadcast.
    pub async fn submit_transaction(&self, tx: Transaction) -> Result<String> {
        // Import validation operations
        use crate::node::validation;

        // Get transaction ID before moving
        let txid = tx.get_tx_id_hex();

        // Get node address
        let addr = crate::GLOBAL_CONFIG.get_node_addr();

        // Process transaction (validates, adds to mempool, broadcasts)
        validation::process_transaction(&addr, tx, &self.blockchain).await;

        // Return transaction ID
        Ok(txid)
    }

    /// Get transaction from mempool
    pub fn get_transaction(&self, txid: &str) -> Result<Option<Transaction>> {
        use crate::node::GLOBAL_MEMORY_POOL;
        GLOBAL_MEMORY_POOL.get(txid)
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

    // ========== Blockchain Operations ==========

    /// Get blockchain height
    pub async fn get_blockchain_height(&self) -> Result<usize> {
        self.blockchain.get_best_height().await
    }

    /// Get block by hash
    pub async fn get_block(&self, hash: &str) -> Result<Option<Block>> {
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
    pub async fn mine_empty_block(&self) -> Result<()> {
        use crate::node::miner;
        miner::mine_empty_block(&self.blockchain).await;
        Ok(())
    }

    // ========== Wallet Operations ==========

    /// Get balance for address
    pub async fn get_balance(&self, address: &str) -> Result<i32> {
        let utxo_set = UTXOSet::new(self.blockchain.clone());
        utxo_set.get_balance(address).await
    }

    /// Create new wallet
    pub fn create_wallet(&self) -> Result<crate::Wallet> {
        crate::Wallet::new()
    }

    /// Get wallet by address
    pub fn get_wallet(&self, address: &str) -> Result<Option<crate::Wallet>> {
        use crate::wallet::WalletService;
        let wallets = WalletService::new()?;
        Ok(wallets.get_wallet(address).cloned())
    }

    /// List all wallet addresses
    pub fn list_wallet_addresses(&self) -> Result<Vec<String>> {
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

    fn generate_test_address() -> String {
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
        let balance = node.get_balance(&genesis_address).await;
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
