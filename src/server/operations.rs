use crate::node::Node;
use crate::server::{
    CENTERAL_NODE, GLOBAL_MEMORY_POOL, GLOBAL_NODES, MessageType, NODE_VERSION, OpType, Package,
    TCP_WRITE_TIMEOUT, TRANSACTION_THRESHOLD,
};
use crate::{Block, Blockchain, GLOBAL_CONFIG, Transaction, UTXOSet};

use data_encoding::HEXLOWER;
use log::{error, info};
use std::collections::HashSet;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// The `send_get_data` function sends a get_data request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `op_type` - A reference to the operation type.
/// * `id` - A reference to the id.
pub async fn send_get_data(addr_to: &SocketAddr, op_type: OpType, id: &[u8]) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    )
    .await;
}

/// The `send_inv` function abstracts the process of sending inventory information (Inv) to a specified address
/// using a standardized package format, including source address, operation type, and a collection of
/// byte vector items, which in this case represent blocks. This function will help broadcast inventory
/// notifications for specific data items to the indicated network address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `op_type` - A reference to the operation type.
/// * `blocks` - A reference to the blocks.
pub async fn send_inv(addr_to: &SocketAddr, op_type: OpType, blocks: &[Vec<u8>]) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Inv {
            addr_from: node_addr,
            op_type,
            items: blocks.to_vec(),
        },
    )
    .await;
}

/// The `send_block` function sends a block to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `block` - A reference to the block.
pub async fn send_block(addr_to: &SocketAddr, block: &Block) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize().expect("Block serialization error"),
        },
    )
    .await;
}

/// The `send_tx` function sends a transaction to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `tx` - A reference to the transaction.
pub async fn send_tx(addr_to: &SocketAddr, tx: &Transaction) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Tx {
            addr_from: node_addr,
            transaction: tx.serialize().expect("Transaction serialization error"),
        },
    )
    .await;
}

/// The `send_block` function sends a block to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `block` - A reference to the block.
pub async fn send_known_nodes(addr_to: &SocketAddr, nodes: Vec<SocketAddr>) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::KnownNodes {
            addr_from: node_addr,
            nodes,
        },
    )
    .await;
}

/// The `send_version` function sends a version request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `height` - A reference to the height.
pub async fn send_version(addr_to: &SocketAddr, height: usize) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Version {
            addr_from: node_addr,
            version: NODE_VERSION,
            best_height: height,
        },
    )
    .await;
}

/// The `send_get_blocks` function sends a get_blocks request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
pub async fn send_get_blocks(addr_to: &SocketAddr) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetBlocks {
            addr_from: node_addr,
        },
    )
    .await;
}

/// The `send_message` function sends a message to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
pub async fn send_message(addr_to: &SocketAddr, message_type: MessageType, message: String) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Message {
            addr_from: node_addr,
            message_type,
            message,
        },
    )
    .await;
}

///
/// The `send_data` function abstracts the process of sending data to a specified address
/// using a standardized package format. It includes source address, operation type, and a collection of
/// byte vector items, which in this case represent blocks. This function will help broadcast inventory
/// notifications for specific data items to the indicated network address.
///
async fn send_data(addr_to: &SocketAddr, pkg: Package) {
    info!("send package: {:?}", &pkg);
    let stream = TcpStream::connect(addr_to);
    if stream.is_err() {
        error!("The {} is not valid", addr_to);

        GLOBAL_NODES
            .evict_node(addr_to)
            .expect("Node eviction error");
        return;
    }

    let mut stream = stream.expect("Stream connect error");
    let _ = stream.set_write_timeout(Option::from(Duration::from_millis(TCP_WRITE_TIMEOUT)));
    let _ = serde_json::to_writer(&stream, &pkg);
    let _ = stream.flush();
}

async fn add_to_memory_pool(tx: Transaction, blockchain: &Arc<RwLock<Blockchain>>) {
    GLOBAL_MEMORY_POOL
        .add(tx.clone())
        .expect("Memory pool add error");
    let utxo_set = UTXOSet::new(blockchain.read().await.clone());
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), true)
        .expect("UTXO set pool flag error");
}

async fn remove_from_memory_pool(tx: Transaction, blockchain: &Arc<RwLock<Blockchain>>) {
    let txid_hex = HEXLOWER.encode(tx.get_id());
    GLOBAL_MEMORY_POOL
        .remove(txid_hex.as_str())
        .expect("Memory pool remove error");
    let utxo_set = UTXOSet::new(blockchain.read().await.clone());
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), false)
        .expect("UTXO set pool flag error");
}

pub async fn process_transaction(
    addr_from: &SocketAddr,
    tx: Transaction,
    blockchain: &Arc<RwLock<Blockchain>>,
) {
    // If transaction exists, do nothing
    // This is to prevent duplicate transactions and retransmission of existing transactions to other nodes
    if GLOBAL_MEMORY_POOL
        .contains_transaction(&tx)
        .expect("Memory pool contains transaction error")
    {
        info!("Transaction: {:?} already exists", tx.get_id());
        send_message(
            addr_from,
            MessageType::Error,
            "Transaction: ${tx.get_id()} already exists".to_string(),
        )
        .await;
        return;
    }

    let txid = tx.get_id_bytes();

    // Add to Memory Pool
    add_to_memory_pool(tx, blockchain).await;

    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // If the node is the central node, it broadcasts the transaction to all other nodes.
    if my_node_addr.eq(&CENTERAL_NODE) {
        let nodes = GLOBAL_NODES
            .get_nodes()
            .expect("Global nodes get error")
            .into_iter()
            .filter(|node| node.get_addr().ne(addr_from))
            .collect::<Vec<Node>>();

        for node in &nodes {
            if my_node_addr.eq(&node.get_addr()) {
                continue;
            }
            if addr_from.eq(&node.get_addr()) {
                continue;
            }
            send_inv(&node.get_addr(), OpType::Tx, &[txid.clone()]).await;
        }
    }

    if GLOBAL_MEMORY_POOL.len().expect("Memory pool length error") >= TRANSACTION_THRESHOLD
        && GLOBAL_CONFIG.is_miner()
    {
        let mining_address = GLOBAL_CONFIG
            .get_mining_addr()
            .expect("Mining address get error");
        let coinbase_tx = Transaction::new_coinbase_tx(mining_address.as_str())
            .expect("Coinbase transaction error");
        let mut txs = GLOBAL_MEMORY_POOL
            .get_all()
            .expect("Memory pool get all error");
        txs.push(coinbase_tx);

        process_mine_block(txs, blockchain).await;
    }
}

async fn process_mine_block(txs: Vec<Transaction>, blockchain: &Arc<RwLock<Blockchain>>) {
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // Mine a new block with the transactions in the memory pool.
    // The `mine_block` function mines a new block with the transactions in the memory pool.
    // It uses the `blockchain` instance to mine the block which also adds the new block to the blockchain.
    // It returns the new block.
    let new_block = blockchain
        .write()
        .await
        .mine_block(txs.as_slice())
        .expect("Blockchain mine block error");

    // The `reindex` function reindexes the UTXO set of the blockchain.
    // It uses the `blockchain` instance to reindex the UTXO set.
    // It returns the new UTXO set.
    let utxo_set = UTXOSet::new(blockchain.read().await.clone());
    utxo_set.reindex().expect("UTXO set reindex error");
    info!("New block {} is mined!", new_block.get_hash());

    for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }

    let nodes = GLOBAL_NODES.get_nodes().expect("Global nodes get error");
    for node in &nodes {
        if my_node_addr.eq(&node.get_addr()) {
            continue;
        }
        send_inv(
            &node.get_addr(),
            OpType::Block,
            &[new_block.get_hash_bytes()],
        )
        .await;
    }
}

/// The `process_known_nodes` function processes known nodes.
/// 1) It will add new nodes to the global nodes set and send version to all new nodes plus sender.
/// 2) If I know nodes not known by sender, then i will
///     - Send all known nodes to the sender
///     - Send all known nodes to all new nodes that i received from the sender
/// 3) It will also send version to all new noded that i received from the sender.
///
/// # Arguments
///
/// * `blockchain` - A reference to the blockchain.
/// * `addr_from` - A reference to the address of the sender.
/// * `nodes` - A reference to the nodes.
pub async fn process_known_nodes(
    blockchain: Blockchain,
    addr_from: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    let new_nodes: HashSet<SocketAddr> = nodes
        .iter()
        .filter(|current_new_node_candidate| {
            !GLOBAL_NODES
                .node_is_known(current_new_node_candidate)
                .expect("Node is known error")
        })
        .cloned()
        .collect();
    info!("new_nodes: {:?}", new_nodes);

    // Add host and new nodes to the global nodes set.
    GLOBAL_NODES
        .add_nodes(new_nodes.clone())
        .expect("Global nodes add error");

    let all_known_nodes_addresses: Vec<SocketAddr> = GLOBAL_NODES
        .get_nodes()
        .expect("Global nodes get error")
        .into_iter()
        .map(|node| node.get_addr())
        .collect();

    let mut nodes_to_add: HashSet<SocketAddr> = HashSet::new();
    // Add new nodes to the nodes to add.
    nodes_to_add.extend(new_nodes.clone());
    // Add sender to the nodes to add.
    nodes_to_add.insert(*addr_from);

    // Empty nodes sent or have sender doest know all nodes that i know
    if all_known_nodes_addresses.len() > nodes.len() {
        // Send All know nodes to to sender
        // Send All know nodes to to sender new nodes

        for node in nodes_to_add.clone().into_iter() {
            send_known_nodes(&node, all_known_nodes_addresses.clone()).await;
        }
    }

    // Send Version to all new nodes plus sender
    let best_height = blockchain
        .get_best_height()
        .expect("Blockchain get best height error");

    send_version(addr_from, best_height).await;
    for node in nodes_to_add.into_iter().filter(|node| node.ne(addr_from)) {
        send_version(&node, best_height).await;
    }
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
pub async fn mine_empty_block(blockchain: &Arc<RwLock<Blockchain>>) {
    if GLOBAL_CONFIG.is_miner() {
        let mining_address = GLOBAL_CONFIG
            .get_mining_addr()
            .expect("Mining address get error");
        let coinbase_tx = Transaction::new_coinbase_tx(mining_address.as_str())
            .expect("Coinbase transaction error");
        let mut txs = GLOBAL_MEMORY_POOL
            .get_all()
            .expect("Memory pool get all error");
        txs.push(coinbase_tx);

        process_mine_block(txs, blockchain).await;
    }
}
