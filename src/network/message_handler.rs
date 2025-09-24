use crate::error::BtcError;
use crate::network::operations::{
    mine_empty_block, process_known_nodes, process_transaction, remove_from_memory_pool,
    send_block, send_get_blocks, send_get_data, send_inv, send_message, send_tx, send_version,
};
use crate::network::server::{
    AdminNodeQueryType, GLOBAL_BLOCKS_IN_TRANSIT, GLOBAL_MEMORY_POOL, GLOBAL_NODES, MessageType,
    OpType, Package,
};
use crate::{Block, BlockchainService, GLOBAL_CONFIG, Transaction, UTXOSet, validate_address};
use data_encoding::HEXLOWER;
use serde_json::Deserializer;
use std::error::Error;
use std::io::BufReader;
use std::net::{Shutdown, TcpStream};
use tracing::{debug, error, info, instrument, trace, warn};

#[instrument(skip(blockchain, stream))]
pub async fn process_stream(
    blockchain: BlockchainService,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    // peer_addr is the address of the peer that is sending the request.
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(&stream);
    let pkg_reader = Deserializer::from_reader(reader).into_iter::<Package>();

    // The `serve` function processes incoming network requests from a TCP stream.
    // It handles different types of packages, including blocks, transactions, and version information.
    // The function processes each package based on its type and performs the appropriate actions.
    // It also manages the block in transit set and the memory pool to ensure proper synchronization
    // and validation of the blockchain.
    // The function returns an error if the stream cannot be read or if the package cannot be deserialized.
    // It also shuts down the stream after processing the package.
    // Iterate over the deserialized packages from the stream.
    for pkg in pkg_reader {
        let pkg = pkg?;
        info!("Receive request from {}: {:?}", peer_addr, pkg);

        match pkg {
            // When a node receives a block, it adds it to the blockchain and sends a request for the next block.
            // It deserializes the block and adds it to the blockchain.
            // If there are blocks in transit, it sends a get_data request for the next block.
            // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
            Package::Block { addr_from, block } => {
                let block =
                    Block::deserialize(block.as_slice()).expect("Block deserialization error");
                // If the block is not the best block, do nothing
                // `add_block` will not add the block if its height is less than current tip height in the block chain.
                blockchain
                    .add_block(&block)
                    .await
                    .expect("Blockchain write error");
                let added_block_hash = block.get_hash_bytes();
                info!("Added block {:?}", added_block_hash.as_slice());

                // Remove transactions in block from memory pool functionally, since they have already been mined by other nodes
                for tx in block.get_transactions() {
                    remove_from_memory_pool(tx.clone(), &blockchain).await;
                }

                // The add_block() method already handles UTXO updates internally through the reorganization process.
                // Calling update_utxo_set() here would cause double UTXO updates, leading to multiple SUBSIDY rewards.
                // This was the root cause of the consensus mechanism allowing all nodes to keep their SUBSIDY.

                let removed_block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                    .remove(added_block_hash.as_ref())
                    .expect("Block removal error");
                if removed_block_hash.is_some() {
                    info!(
                        "Removed block {:?} FROM GLOBAL_BLOCKS_IN_TRANSIT",
                        removed_block_hash.expect("Block removal error").as_slice()
                    );
                }

                // If there are blocks in transit, it sends a get_data request for the next block.
                // It removes the block from the blocks in transit set when it is added to the blockchain when
                // it is receives Package::Inv message{OpType::Block, items: [block_hash]}
                // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
                if GLOBAL_BLOCKS_IN_TRANSIT
                    .is_not_empty()
                    .expect("Blocks in transit error")
                {
                    let block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                        .first()
                        .expect("Blocks in transit error")
                        .expect("Blocks in transit error");
                    send_get_data(&addr_from, OpType::Block, &block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
            }
            // Retrieves all block hashes from the blockchain and sends an
            // inv message with a list of hashes to the requesting peer.
            Package::GetBlocks { addr_from } => {
                let blocks = blockchain
                    .get_block_hashes()
                    .await
                    .expect("Blockchain read error");
                // Send an inv message with a list of hashes to the requesting peer.
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
            // Retrieves the requested block or transaction from the blockchain
            // or the global memory pool and sends it back to the requesting peer.
            Package::GetData {
                addr_from,
                op_type,
                id,
            } => match op_type {
                // When a node receives a block, it adds it to the blockchain and sends a request for the next block.
                OpType::Block => {
                    if let Some(block) = blockchain
                        .get_block(id.as_slice())
                        .await
                        .expect("Blockchain read error")
                    {
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    if let Some(tx) = GLOBAL_MEMORY_POOL
                        .get(txid_hex.as_str())
                        .expect("Memory pool get error")
                    {
                        send_tx(&addr_from, &tx).await;
                    } else {
                        info!("Received request to forward a Transaction that is not found in memory pool. 
                        Most likely it has been mined!!!: {:?}", txid_hex);
                    }
                }
            },
            // Adds the received blocks or transactions to the global blocks in transit
            // or the memory pool and requests missing blocks or transactions via get_data if necessary.
            Package::Inv {
                addr_from,
                op_type,
                items,
            } => match op_type {
                // When a node receives a block, it adds it to the blocks in transit set and sends a request for the first block.
                OpType::Block => {
                    GLOBAL_BLOCKS_IN_TRANSIT
                        .add_blocks(items.as_slice())
                        .expect("Blocks in transit add error");

                    let block_hash = items.first().expect("Blocks in transit add error");
                    send_get_data(&addr_from, OpType::Block, block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
                // When a node receives a transaction, it adds it to the memory pool and sends a request for the transaction.
                OpType::Tx => {
                    let txid = items.first().expect("Blocks in transit add error");
                    let txid_hex = HEXLOWER.encode(txid);

                    if !GLOBAL_MEMORY_POOL
                        .contains(txid_hex.as_str())
                        .expect("Memory pool contains error")
                    {
                        send_get_data(&addr_from, OpType::Tx, txid).await;
                    }
                }
            },
            // deserializes the transaction and adds it to the global memory pool.
            // If the node is a miner and the memory pool has reached a certain threshold,
            // it creates a new block containing transactions from the memory pool, mines it,
            // and broadcasts the new block to other nodes via inv.
            Package::Tx {
                addr_from,
                transaction,
            } => {
                let tx = Transaction::deserialize(transaction.as_slice())
                    .expect("Transaction deserialization error");
                // CPU intensive operation.
                // It will create a new transaction and add it to the memory pool.
                // It will also broadcast the transaction to all other nodes.
                // It will also mine a new block if the memory pool has reached a certain threshold.
                process_transaction(&addr_from, tx, &blockchain).await;
            }

            // CPU intensive operation.
            // It will create a new transaction and add it to the memory pool.
            // It will also broadcast the transaction to all other nodes.
            // It will also mine a new block if the memory pool has reached a certain threshold.
            Package::SendBitCoin {
                addr_from,
                wlt_frm_addr,
                wlt_to_addr,
                amount,
            } => {
                if !validate_address(wlt_frm_addr.as_str()).expect("Address validation error") {
                    send_message(
                        &addr_from,
                        MessageType::Error,
                        "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                    )
                    .await;
                } else if !validate_address(wlt_to_addr.as_str()).expect("Address validation error")
                {
                    send_message(
                        &addr_from,
                        MessageType::Error,
                        "Invalid addr_to: ${wlt_to_addr}".to_string(),
                    )
                    .await;
                } else {
                    let utxo_set = UTXOSet::new(blockchain.clone());

                    match Transaction::new_utxo_transaction(
                        wlt_frm_addr.as_str(),
                        wlt_to_addr.as_str(),
                        amount,
                        &utxo_set,
                    )
                    .await
                    {
                        Ok(transaction) => {
                            process_transaction(&addr_from, transaction, &blockchain).await;
                        }
                        Err(BtcError::NotEnoughFunds) => {
                            // Get current balance for detailed error message
                            let current_balance = utxo_set
                                .get_balance(wlt_frm_addr.as_str())
                                .await
                                .unwrap_or(0);

                            send_message(
                                &addr_from,
                                MessageType::Error,
                                format!(
                                    "Insufficient funds: cannot send {} bitcoin. Current balance: {} bitcoin",
                                    amount, current_balance
                                ),
                            )
                            .await;

                            // Log the error for debugging
                            error!(
                                "Transaction rejected: insufficient funds. From: {}, To: {}, Amount: {}, Balance: {}",
                                wlt_frm_addr, wlt_to_addr, amount, current_balance
                            );
                        }
                        Err(e) => {
                            send_message(
                                &addr_from,
                                MessageType::Error,
                                format!("Transaction creation failed: {}", e),
                            )
                            .await;

                            error!("Transaction creation failed: {}", e);
                        }
                    }
                }
            }
            Package::Version {
                addr_from,
                version,
                best_height,
            } => {
                debug!("version = {}, best_height = {}", version, best_height);
                let local_best_height = blockchain
                    .get_best_height()
                    .await
                    .expect("Blockchain read error");
                if local_best_height < best_height {
                    send_get_blocks(&addr_from).await;
                }
                if local_best_height > best_height {
                    send_version(
                        &addr_from,
                        blockchain
                            .get_best_height()
                            .await
                            .expect("Blockchain read error"),
                    )
                    .await;
                }

                // If height is the same then get the first and last block hashes for comparison

                if !GLOBAL_NODES
                    .node_is_known(&addr_from)
                    .expect("Node is known error")
                {
                    GLOBAL_NODES.add_node(addr_from).expect("Node add error");
                }
            }
            Package::KnownNodes { addr_from, nodes } => {
                process_known_nodes(blockchain.clone(), &addr_from, nodes).await;
            }
            Package::Message {
                addr_from,
                message_type,
                message,
            } => match message_type {
                MessageType::Error => {
                    error!("{} sent error: {}", addr_from, message);
                }
                MessageType::Warning => {
                    warn!("{} sent warning: {}", addr_from, message);
                }
                MessageType::Info => {
                    debug!("{} sent info: {}", addr_from, message);
                }
                MessageType::Success => {
                    debug!("{} sent success: {}", addr_from, message);
                }
                MessageType::Ack => {
                    debug!("{} sent ack: {}", addr_from, message);
                }
            },
            Package::AdminNodeQuery {
                addr_from,
                query_type,
            } => match query_type {
                AdminNodeQueryType::GetBalance { wlt_address } => {
                    let address_valid =
                        validate_address(wlt_address.as_str()).expect("Address validation error");
                    if !address_valid {
                        error!("Invalid address: {}", wlt_address);
                        return Err(Box::new(BtcError::InvalidAddress(wlt_address.clone())));
                    }

                    let utxo_set = UTXOSet::new(blockchain.clone());
                    let balance = utxo_set
                        .get_balance(wlt_address.as_str())
                        .await
                        .expect("UTXO set get balance error");
                    debug!("Balance of {}: {}", addr_from, balance);
                }
                AdminNodeQueryType::GetAllTransactions => {
                    let transactions_summary = blockchain
                        .find_all_transactions()
                        .await
                        .expect("Blockchain find all transactions error");

                    info!("═══════════════════════════════════════════════════════════════");
                    info!("                    BLOCKCHAIN TRANSACTIONS");
                    info!("═══════════════════════════════════════════════════════════════");

                    for (idx, (cur_txid_hex, tx_summary)) in transactions_summary.iter().enumerate()
                    {
                        let mut tx_summary_input = tx_summary.clone();
                        let mut tx_summary_output = tx_summary.clone();
                        let tx_summary_inputs = tx_summary_input.get_inputs();
                        let tx_summary_outputs = tx_summary_output.get_outputs();
                        info!("");
                        info!("┌─ Transaction #{}", idx + 1);
                        info!("│  ID: {}", cur_txid_hex);
                        info!(
                            "│  Type: {}",
                            if tx_summary_inputs.is_empty() {
                                "Coinbase"
                            } else {
                                "Regular"
                            }
                        );

                        if !tx_summary_inputs.is_empty() {
                            info!("│  ┌─ Inputs ({}):", tx_summary_inputs.len());
                            for (input_idx, input_summary) in tx_summary_inputs.iter().enumerate() {
                                info!(
                                    "│  │  {} └─ From: {} (txid: {}, vout: {})",
                                    if input_idx == tx_summary_inputs.len() - 1 {
                                        "└"
                                    } else {
                                        "├"
                                    },
                                    input_summary.get_wlt_addr(),
                                    input_summary.get_txid_hex(),
                                    input_summary.get_output_idx()
                                );
                            }
                        }

                        info!("│  ┌─ Outputs ({}):", tx_summary_outputs.len());
                        for (output_idx, output_summary) in tx_summary_outputs.iter().enumerate() {
                            info!(
                                "│  │  {} └─ To: {} (value: {} BTC)",
                                if output_idx == tx_summary_outputs.len() - 1 {
                                    "└"
                                } else {
                                    "├"
                                },
                                output_summary.get_wlt_addr(),
                                output_summary.get_value()
                            );
                        }
                        info!("└─────────────────────────────────────────────────────────────");
                    }

                    info!("");
                    info!("═══════════════════════════════════════════════════════════════");
                    info!("Total Transactions: {}", transactions_summary.len());
                    info!("═══════════════════════════════════════════════════════════════");
                }
                AdminNodeQueryType::GetBlockHeight => {
                    let height = blockchain
                        .get_best_height()
                        .await
                        .expect("Blockchain read error");
                    trace!("Block height: {}", height);
                }
                AdminNodeQueryType::MineEmptyBlock => {
                    if GLOBAL_CONFIG.is_miner() {
                        mine_empty_block(&blockchain).await;
                    } else {
                        trace!("Not a miner");
                    }
                    trace!("Mining empty block");
                }
                AdminNodeQueryType::ReindexUtxo => {
                    let utxo_set = UTXOSet::new(blockchain.clone());
                    utxo_set.reindex().await.expect("UTXO set reindex error");
                    let count = utxo_set
                        .count_transactions()
                        .await
                        .expect("UTXO set count error");
                    trace!(
                        "Reindexed UTXO set. There are {} transactions in the UTXO set.",
                        count
                    );
                }
            },
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}
