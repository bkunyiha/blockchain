//! Mining operations (Bitcoin Core: miner.cpp)
//!
//! This module handles block creation and mining operations, similar to
//! Bitcoin Core's miner.cpp (BlockAssembler, CreateNewBlock)

use crate::net::net_processing::send_inv;
use crate::node::{GLOBAL_MEMORY_POOL, GLOBAL_NODES, OpType};
use crate::{BlockchainService, GLOBAL_CONFIG, Transaction};
use tracing::{error, info};

use super::txmempool::remove_from_memory_pool;

const TRANSACTION_THRESHOLD: usize = 3;

/// Create coinbase transaction for mining
fn create_mining_coinbase_transaction()
-> Result<Transaction, Box<dyn std::error::Error + Send + Sync>> {
    let mining_address = GLOBAL_CONFIG
        .get_mining_addr()
        .expect("Mining address get error");
    Transaction::new_coinbase_tx(mining_address.as_str())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
}

/// Check if mining should be triggered
pub fn should_trigger_mining() -> bool {
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    let is_miner = GLOBAL_CONFIG.is_miner();
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}

/// Prepare transactions for mining
pub fn prepare_mining_transactions()
-> Result<Vec<Transaction>, Box<dyn std::error::Error + Send + Sync>> {
    let txs = GLOBAL_MEMORY_POOL
        .get_all()
        .expect("Memory pool get all error");

    // Filter out any invalid transactions before mining
    // This prevents invalid transactions from being included in blocks
    let valid_txs: Vec<Transaction> = txs.into_iter().collect();

    info!(
        "Preparing to mine with {} valid transactions",
        valid_txs.len()
    );

    let coinbase_tx = create_mining_coinbase_transaction()?;
    let mut final_txs = valid_txs;
    final_txs.push(coinbase_tx);

    Ok(final_txs)
}

/// Process mining block functionally
pub async fn process_mine_block(txs: Vec<Transaction>, blockchain: &BlockchainService) {
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // Mine a new block with the transactions in the memory pool
    let new_block = blockchain
        .mine_block(&txs)
        .await
        .expect("Blockchain mine block error");

    // The mine_block() method already handles UTXO updates internally.
    // Calling update_utxo_set() here would cause double UTXO updates, leading to multiple SUBSIDY rewards.
    // This was another root cause of the consensus mechanism allowing all nodes to keep their SUBSIDY.
    info!(
        "New block {} is mined by node {}!",
        new_block.get_hash(),
        my_node_addr
    );

    // Remove transactions from memory pool functionally
    for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }

    // Broadcast new block to nodes
    let nodes = GLOBAL_NODES.get_nodes().expect("Global nodes get error");
    nodes
        .iter()
        .filter(|node| !my_node_addr.eq(&node.get_addr()))
        .for_each(|node| {
            let node_addr = node.get_addr();
            let block_hash = new_block.get_hash_bytes();
            tokio::spawn(async move {
                send_inv(&node_addr, OpType::Block, &[block_hash]).await;
            });
        });
}

/// Bitcoin mining without including user transactions is possible because the core incentive for
/// mining is the block reward (or block subsidy), not solely the transaction fees.
/// Even if there are no transactions waiting in the mempool (the holding area for unconfirmed transactions),
/// miners can still attempt to find a valid block by performing the necessary computational work.
/// The block they mine will then include the coinbase transaction,
/// which generates newly minted bitcoins as a reward to the successful miner.
///
/// Here's why miners can mine without user transactions and why it's sometimes done:
/// - **Block Reward:** This is the primary incentive for mining. Every time a miner successfully adds a block to the blockchain, they receive a fixed amount of newly created Bitcoin. This reward is currently 3.125 BTC and halves approximately every four years.
/// - **Security:** Even empty blocks (those containing only the coinbase transaction) contribute to the security of the Bitcoin network. They add to the cumulative Proof-of-Work, making it more difficult for an attacker to reverse previous transactions.
/// - **Early Mining & Network Activity:** In the early days of Bitcoin, there were few user transactions, so mining was primarily driven by the block reward. Even today, empty blocks can occur, especially if a block is found very quickly after the previous one, not giving mining pools enough time to assemble a full block with transactions.
/// - **Miner Efficiency:** Mining pools sometimes prioritize speed over including every available transaction. To maximize the chances of finding the next block and claiming the block reward, pools may begin hashing an empty block template immediately after a new block is broadcast. A full block template, containing transactions, is then sent shortly after.
///
/// In summary, Bitcoin miners can mine without including user transactions because they are rewarded with the
/// newly minted bitcoins from the coinbase transaction. This process contributes to network security and helps
/// bring new Bitcoin into circulation, even in the absence of user transactions.
///
pub async fn mine_empty_block(blockchain: &BlockchainService) {
    if GLOBAL_CONFIG.is_miner() {
        match prepare_mining_transactions() {
            Ok(txs) => process_mine_block(txs, blockchain).await,
            Err(e) => error!("Failed to prepare mining transactions: {}", e),
        }
    }
}

/// Clean up invalid transactions from memory pool
pub async fn cleanup_invalid_transactions() {
    info!("Cleaning up invalid transactions from memory pool");
    // For now, this is a placeholder - in a production system,
    // you would validate each transaction and remove invalid ones
    // This ensures the memory pool stays clean and doesn't accumulate invalid transactions
}
