use crate::node::Node;
use crate::{
    Block, BlockInTransit, Blockchain, GLOBAL_CONFIG, MemoryPool, Nodes, Transaction, UTXOSet,
    validate_address,
};
use data_encoding::HEXLOWER;
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::{BufReader, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::time::Duration;

const NODE_VERSION: usize = 1;

pub static CENTERAL_NODE: Lazy<SocketAddr> = Lazy::new(|| {
    env::var("CENTERAL_NODE")
        .unwrap_or_else(|_| "127.0.0.1:2001".to_string())
        .parse()
        .unwrap()
});

pub const TRANSACTION_THRESHOLD: usize = 2;

static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();

    nodes.add_node(*CENTERAL_NODE);
    nodes
});

/// The `GLOBAL_MEMORY_POOL` is a lazy static variable that holds a `MemoryPool` instance.
/// It is used to store transactions that are in the memory pool.
///
/// # Returns
///
/// A `MemoryPool` instance.
///
static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> = Lazy::new(MemoryPool::new);

/// The `GLOBAL_BLOCKS_IN_TRANSIT` is a lazy static variable that holds a `BlockInTransit` instance.
/// It is used to store blocks that are in transit between nodes.
///
/// # Returns
///
/// A `BlockInTransit` instance.
///
static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> = Lazy::new(BlockInTransit::new);

const TCP_WRITE_TIMEOUT: u64 = 1000;

#[derive(Debug)]
pub enum ConnectNode {
    Local,
    Remote(SocketAddr),
}

impl ConnectNode {
    pub fn is_remote(&self) -> bool {
        matches!(self, ConnectNode::Remote(_))
    }

    pub fn get_addr(&self) -> SocketAddr {
        match self {
            ConnectNode::Remote(addr) => *addr,
            ConnectNode::Local => *CENTERAL_NODE,
        }
    }
}

impl FromStr for ConnectNode {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "local" {
            Ok(ConnectNode::Local)
        } else {
            let ip_addr = s.parse()?;
            Ok(ConnectNode::Remote(ip_addr))
        }
    }
}

pub struct Server {
    blockchain: Blockchain,
}

impl Server {
    pub fn new(blockchain: Blockchain) -> Server {
        Server { blockchain }
    }

    pub async fn run(&self, addrs: &SocketAddr, connect_nodes: Vec<ConnectNode>) {
        let listener = TcpListener::bind(addrs).unwrap();

        // If the node is not the central node, send the version message to the central node.
        if !addrs.eq(&CENTERAL_NODE) {
            let best_height = self.blockchain.get_best_height();
            send_version(&CENTERAL_NODE, best_height).await;
        } else {
            info!("Register with node {:?}", connect_nodes);
            // Add the connect node to the global nodes set.

            let remote_nodes: Vec<SocketAddr> = connect_nodes
                .iter()
                .filter(|node| node.is_remote())
                .map(|node| node.get_addr())
                .collect();

            GLOBAL_NODES.add_nodes(remote_nodes.clone());

            for remote_node in remote_nodes {
                send_known_nodes(
                    &remote_node,
                    GLOBAL_NODES
                        .get_nodes()
                        .iter()
                        .map(|node| node.get_addr())
                        .collect(),
                )
                .await;
            }
        }

        // Serve the incoming connections.
        for stream in listener.incoming() {
            let blockchain = self.blockchain.clone();

            tokio::spawn(async move {
                match stream {
                    Ok(stream) => {
                        serve(blockchain, stream).await.unwrap();
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            });

            // thread::spawn(|| match stream {
            //     Ok(stream) => {
            //         serve(blockchain, stream).unwrap();
            //     }
            //     Err(e) => {
            //         error!("Error: {}", e);
            //     }
            // });
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OpType {
    Tx,
    Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Error,
    Success,
    Info,
    Warning,
    Ack,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Package {
    Block {
        addr_from: SocketAddr,
        block: Vec<u8>,
    },
    GetBlocks {
        addr_from: SocketAddr,
    },
    GetData {
        addr_from: SocketAddr,
        op_type: OpType,
        id: Vec<u8>,
    },
    Inv {
        addr_from: SocketAddr,
        op_type: OpType,
        items: Vec<Vec<u8>>,
    },
    Tx {
        addr_from: SocketAddr,
        transaction: Vec<u8>,
    },
    SendBitCoin {
        addr_from: SocketAddr,
        wlt_frm_addr: String,
        wlt_to_addr: String,
        amount: i32,
    },
    KnownNodes {
        addr_from: SocketAddr,
        nodes: Vec<SocketAddr>,
    },
    Version {
        addr_from: SocketAddr,
        version: usize,
        best_height: usize,
    },
    Message {
        addr_from: SocketAddr,
        message_type: MessageType,
        message: String,
    },
}

/// The `send_get_data` function sends a get_data request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `op_type` - A reference to the operation type.
/// * `id` - A reference to the id.
async fn send_get_data(addr_to: &SocketAddr, op_type: OpType, id: &[u8]) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    );
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
async fn send_inv(addr_to: &SocketAddr, op_type: OpType, blocks: &[Vec<u8>]) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Inv {
            addr_from: node_addr,
            op_type,
            items: blocks.to_vec(),
        },
    );
}

/// The `send_block` function sends a block to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `block` - A reference to the block.
async fn send_block(addr_to: &SocketAddr, block: &Block) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize(),
        },
    );
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
            transaction: tx.serialize(),
        },
    );
}

/// The `send_block` function sends a block to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `block` - A reference to the block.
async fn send_known_nodes(addr_to: &SocketAddr, nodes: Vec<SocketAddr>) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::KnownNodes {
            addr_from: node_addr,
            nodes,
        },
    );
}

/// The `send_version` function sends a version request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `height` - A reference to the height.
async fn send_version(addr_to: &SocketAddr, height: usize) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Version {
            addr_from: node_addr,
            version: NODE_VERSION,
            best_height: height,
        },
    );
}

/// The `send_get_blocks` function sends a get_blocks request to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
async fn send_get_blocks(addr_to: &SocketAddr) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetBlocks {
            addr_from: node_addr,
        },
    );
}

/// The `send_message` function sends a message to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
async fn send_message(addr_to: &SocketAddr, message_type: MessageType, message: String) {
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Message {
            addr_from: node_addr,
            message_type,
            message,
        },
    );
}

async fn serve(blockchain: Blockchain, stream: TcpStream) -> Result<(), Box<dyn Error>> {
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
                let block = Block::deserialize(block.as_slice());
                // If the block is not the best block, do nothing
                // `add_block` will not add the block if its height is less than current tip height in the block chain.
                blockchain.add_block(&block);
                let added_block_hash = block.get_hash_bytes();
                info!("Added block {:?}", added_block_hash.as_slice());

                let removed_block_hash = GLOBAL_BLOCKS_IN_TRANSIT.remove(added_block_hash.as_ref());
                if removed_block_hash.is_some() {
                    info!(
                        "Removed block {:?} FROM GLOBAL_BLOCKS_IN_TRANSIT",
                        removed_block_hash.unwrap().as_slice()
                    );
                }

                // If there are blocks in transit, it sends a get_data request for the next block.
                // It removes the block from the blocks in transit set when it is added to the blockchain when
                // it is receives Package::Inv message{OpType::Block, items: [block_hash]}
                // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
                if GLOBAL_BLOCKS_IN_TRANSIT.is_not_empty() {
                    let block_hash = GLOBAL_BLOCKS_IN_TRANSIT.first().unwrap();
                    send_get_data(&addr_from, OpType::Block, &block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                } else {
                    let utxo_set = UTXOSet::new(blockchain.clone());
                    // The `reindex` function reindexes the UTXO set by clearing the existing UTXO tree and rebuilding it from the blockchain.
                    // It iterates through the blockchain, finds all UTXOs, and inserts them into the UTXO tree.
                    //
                    // # Arguments
                    //
                    // * `blockchain` - A reference to the blockchain.
                    utxo_set.reindex();
                }
            }
            // Retrieves all block hashes from the blockchain and sends an
            // inv message with a list of hashes to the requesting peer.
            Package::GetBlocks { addr_from } => {
                let blocks = blockchain.get_block_hashes();
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
                    if let Some(block) = blockchain.get_block(id.as_slice()) {
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    if let Some(tx) = GLOBAL_MEMORY_POOL.get(txid_hex.as_str()) {
                        send_tx(&addr_from, &tx).await;
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
                    GLOBAL_BLOCKS_IN_TRANSIT.add_blocks(items.as_slice());

                    let block_hash = items.first().unwrap();
                    send_get_data(&addr_from, OpType::Block, block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
                // When a node receives a transaction, it adds it to the memory pool and sends a request for the transaction.
                OpType::Tx => {
                    let txid = items.first().unwrap();
                    let txid_hex = HEXLOWER.encode(txid);

                    if !GLOBAL_MEMORY_POOL.contains(txid_hex.as_str()) {
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
                let tx = Transaction::deserialize(transaction.as_slice());
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
                if !validate_address(wlt_frm_addr.as_str()) {
                    send_message(
                        &addr_from,
                        MessageType::Error,
                        "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                    )
                    .await;
                } else if !validate_address(wlt_to_addr.as_str()) {
                    send_message(
                        &addr_from,
                        MessageType::Error,
                        "Invalid addr_to: ${wlt_to_addr}".to_string(),
                    )
                    .await;
                } else {
                    let utxo_set = UTXOSet::new(blockchain.clone());

                    let transaction = Transaction::new_utxo_transaction(
                        wlt_frm_addr.as_str(),
                        wlt_to_addr.as_str(),
                        amount,
                        &utxo_set,
                    );
                    process_transaction(&addr_from, transaction, &blockchain).await;
                }
            }
            Package::Version {
                addr_from,
                version,
                best_height,
            } => {
                info!("version = {}, best_height = {}", version, best_height);
                let local_best_height = blockchain.get_best_height();
                if local_best_height < best_height {
                    send_get_blocks(&addr_from).await;
                }
                if local_best_height > best_height {
                    send_version(&addr_from, blockchain.get_best_height()).await;
                }

                if !GLOBAL_NODES.node_is_known(&addr_from) {
                    GLOBAL_NODES.add_node(addr_from);
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
                    info!("{} sent warning: {}", addr_from, message);
                }
                MessageType::Info => {
                    info!("{} sent info: {}", addr_from, message);
                }
                MessageType::Success => {
                    info!("{} sent success: {}", addr_from, message);
                }
                MessageType::Ack => {
                    info!("{} sent ack: {}", addr_from, message);
                }
            },
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}

///
/// The `send_data` function abstracts the process of sending data to a specified address
/// using a standardized package format. It includes source address, operation type, and a collection of
/// byte vector items, which in this case represent blocks. This function will help broadcast inventory
/// notifications for specific data items to the indicated network address.
///
fn send_data(addr_to: &SocketAddr, pkg: Package) {
    info!("send package: {:?}", &pkg);
    let stream = TcpStream::connect(addr_to);
    if stream.is_err() {
        error!("The {} is not valid", addr_to);

        GLOBAL_NODES.evict_node(addr_to);
        return;
    }

    let mut stream = stream.unwrap();
    let _ = stream.set_write_timeout(Option::from(Duration::from_millis(TCP_WRITE_TIMEOUT)));
    let _ = serde_json::to_writer(&stream, &pkg);
    let _ = stream.flush();
}

async fn process_transaction(addr_from: &SocketAddr, tx: Transaction, blockchain: &Blockchain) {
    // If transaction exists, do nothing
    // This is to prevent duplicate transactions and retransmission of existing transactions to other nodes
    if GLOBAL_MEMORY_POOL.contains_transaction(&tx) {
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
    GLOBAL_MEMORY_POOL.add(tx);

    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // If the node is the central node, it broadcasts the transaction to all other nodes.
    if my_node_addr.eq(&CENTERAL_NODE) {
        let nodes = GLOBAL_NODES
            .get_nodes()
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

    if GLOBAL_MEMORY_POOL.len() >= TRANSACTION_THRESHOLD && GLOBAL_CONFIG.is_miner() {
        let mining_address = GLOBAL_CONFIG.get_mining_addr().unwrap();
        let coinbase_tx = Transaction::new_coinbase_tx(mining_address.as_str());
        let mut txs = GLOBAL_MEMORY_POOL.get_all();
        txs.push(coinbase_tx);
        // Mine a new block with the transactions in the memory pool.
        // The `mine_block` function mines a new block with the transactions in the memory pool.
        // It uses the `blockchain` instance to mine the block which also adds the new block to the blockchain.
        // It returns the new block.
        let new_block = blockchain.mine_block(&txs);

        // The `reindex` function reindexes the UTXO set of the blockchain.
        // It uses the `blockchain` instance to reindex the UTXO set.
        // It returns the new UTXO set.
        let utxo_set = UTXOSet::new(blockchain.clone());
        utxo_set.reindex();
        info!("New block {} is mined!", new_block.get_hash());

        for tx in &txs {
            let txid_hex = HEXLOWER.encode(tx.get_id());
            GLOBAL_MEMORY_POOL.remove(txid_hex.as_str());
        }

        let nodes = GLOBAL_NODES.get_nodes();
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
async fn process_known_nodes(
    blockchain: Blockchain,
    addr_from: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    let new_nodes: Vec<SocketAddr> = nodes
        .iter()
        .filter(|current_new_node_candidate| {
            !GLOBAL_NODES.node_is_known(current_new_node_candidate)
        })
        .cloned()
        .collect();
    info!("new_nodes: {:?}", new_nodes);

    // Add host and new nodes to the global nodes set.
    GLOBAL_NODES.add_nodes(new_nodes.clone());

    let all_known_nodes_addresses: Vec<SocketAddr> = GLOBAL_NODES
        .get_nodes()
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
    let best_height = blockchain.get_best_height();

    send_version(addr_from, best_height).await;
    for node in nodes_to_add.into_iter().filter(|node| node.ne(addr_from)) {
        send_version(&node, best_height).await;
    }
}
