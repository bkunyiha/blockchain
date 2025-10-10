//! Transaction validation and processing (Bitcoin Core: validation.cpp)
//!
//! This module handles transaction acceptance to mempool and validation,
//! similar to Bitcoin Core's validation.cpp (AcceptToMemoryPool)

use crate::net::net_processing::{send_inv, send_message};
use crate::node::{CENTERAL_NODE, GLOBAL_NODES, MessageType, Node, OpType};
use crate::{BlockchainService, GLOBAL_CONFIG, Transaction};
use tracing::{error, info, warn};

use super::miner::{
    cleanup_invalid_transactions, prepare_mining_transactions, process_mine_block,
    should_trigger_mining,
};
use super::txmempool::{add_to_memory_pool, transaction_exists_in_pool};

/// Get nodes excluding the sender
fn get_nodes_excluding_sender(addr_from: &std::net::SocketAddr) -> Vec<Node> {
    GLOBAL_NODES
        .get_nodes()
        .expect("Global nodes get error")
        .into_iter()
        .filter(|node| {
            let node_addr = node.get_addr();
            let my_addr = GLOBAL_CONFIG.get_node_addr();
            node_addr != *addr_from && node_addr != my_addr
        })
        .collect()
}

/// Broadcast transaction to nodes functionally
async fn broadcast_transaction_to_nodes(nodes: &[Node], txid: Vec<u8>) {
    let txid_clone = txid.clone();
    nodes.iter().for_each(|node| {
        let node_addr = node.get_addr();
        let txid = txid_clone.clone();
        tokio::spawn(async move {
            send_inv(&node_addr, OpType::Tx, &[txid]).await;
        });
    });
}

/// Process transaction - validates, adds to mempool, and triggers mining if needed
///
/// This is the main entry point for processing new transactions, similar to
/// Bitcoin Core's AcceptToMemoryPool and ProcessNewTransaction
pub async fn process_transaction(
    addr_from: &std::net::SocketAddr,
    tx: Transaction,
    blockchain: &BlockchainService,
) {
    // Check if transaction exists
    if transaction_exists_in_pool(&tx) {
        info!("Transaction: {:?} already exists", tx.get_id());
        send_message(
            addr_from,
            MessageType::Error,
            format!("Transaction: {} already exists", tx.get_tx_id_hex()),
        )
        .await;
        return;
    }

    let txid = tx.get_id_bytes();

    // Add to Memory Pool
    add_to_memory_pool(tx, blockchain).await;

    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // If the node is the central node, broadcast the transaction to all other nodes
    if my_node_addr.eq(&CENTERAL_NODE) {
        let nodes = get_nodes_excluding_sender(addr_from);
        broadcast_transaction_to_nodes(&nodes, txid).await;
    }

    // Check if mining should be triggered
    if should_trigger_mining() {
        match prepare_mining_transactions() {
            Ok(txs) => {
                if !txs.is_empty() {
                    process_mine_block(txs, blockchain).await;
                } else {
                    warn!("Mining triggered but no valid transactions to mine");
                }
            }
            Err(e) => {
                error!("Failed to prepare mining transactions: {}", e);
                // Clean up any invalid transactions from memory pool
                cleanup_invalid_transactions().await;
            }
        }
    }
}
