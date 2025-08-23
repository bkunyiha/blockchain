// Declare and defines a module for the domain layer
pub mod operations;
pub mod process_messages;

// Re-export the  modules
pub use operations::{send_known_nodes, send_version};
pub use process_messages::process_stream;

use crate::{BlockInTransit, Blockchain, MemoryPool, Nodes};
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub const NODE_VERSION: usize = 1;

pub static CENTERAL_NODE: Lazy<SocketAddr> = Lazy::new(|| {
    env::var("CENTERAL_NODE")
        .unwrap_or_else(|_| "127.0.0.1:2001".to_string())
        .parse()
        .expect("CENTERAL_NODE environment variable is not a valid socket address")
});

pub const TRANSACTION_THRESHOLD: usize = 2;

pub static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();

    nodes.add_node(*CENTERAL_NODE).expect("Node add error");
    nodes
});

/// The `GLOBAL_MEMORY_POOL` is a lazy static variable that holds a `MemoryPool` instance.
/// It is used to store transactions that are in the memory pool.
///
/// # Returns
///
/// A `MemoryPool` instance.
///
pub static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> = Lazy::new(MemoryPool::new);

/// The `GLOBAL_BLOCKS_IN_TRANSIT` is a lazy static variable that holds a `BlockInTransit` instance.
/// It is used to store blocks that are in transit between nodes.
///
/// # Returns
///
/// A `BlockInTransit` instance.
///
pub static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> = Lazy::new(BlockInTransit::new);

pub const TCP_WRITE_TIMEOUT: u64 = 1000;

#[derive(PartialEq, Eq, Hash, Debug)]
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
    blockchain: Arc<RwLock<Blockchain>>,
}

impl Server {
    pub fn new(blockchain: Arc<RwLock<Blockchain>>) -> Server {
        Server { blockchain }
    }

    pub async fn run(&self, addrs: &SocketAddr, connect_nodes: HashSet<ConnectNode>) {
        let listener = TcpListener::bind(addrs).expect("TcpListener bind error");

        // If the node is not the central node, send the version message to the central node.
        if !addrs.eq(&CENTERAL_NODE) {
            let best_height = self
                .blockchain
                .read()
                .await
                .get_best_height()
                .expect("Blockchain read error");
            send_version(&CENTERAL_NODE, best_height).await;
        } else {
            info!("Register with node {:?}", connect_nodes);
            // Add the connect node to the global nodes set.

            let remote_nodes: HashSet<SocketAddr> = connect_nodes
                .iter()
                .filter(|node| node.is_remote())
                .map(|node| node.get_addr())
                .collect();

            GLOBAL_NODES
                .add_nodes(remote_nodes.clone())
                .expect("Global nodes add error");

            for remote_node in remote_nodes {
                send_known_nodes(
                    &remote_node,
                    GLOBAL_NODES
                        .get_nodes()
                        .expect("Global nodes get error")
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
                        process_stream(blockchain.clone(), stream)
                            .await
                            .expect("Serve error");
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            });
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
pub enum AdminNodeQueryType {
    GetBalance { wlt_address: String },
    GetAllTransactions,
    GetBlockHeight,
    MineEmptyBlock,
    ReindexUtxo,
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
    AdminNodeQuery {
        addr_from: SocketAddr,
        query_type: AdminNodeQueryType,
    },
}
