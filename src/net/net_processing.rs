//! Network P2P operations (Bitcoin Core: net_processing)
//!
//! This module handles peer-to-peer communication operations,
//! similar to Bitcoin Core's net_processing.cpp

use crate::node::{GLOBAL_NODES, MessageType, NODE_VERSION, OpType, Package, TCP_WRITE_TIMEOUT};
use crate::{Block, BlockchainService, GLOBAL_CONFIG, Transaction};

use std::collections::HashSet;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tracing::{error, info};

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

/// The `send_known_nodes` function sends known nodes to a specified address.
///
/// # Arguments
///
/// * `addr` - A reference to the address.
/// * `nodes` - A vector of socket addresses.
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
    blockchain: BlockchainService,
    addr_from: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    // Find new nodes functionally
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

    // Add host and new nodes to the global nodes set
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
    // Add new nodes to the nodes to add
    nodes_to_add.extend(new_nodes.clone());
    // Add sender to the nodes to add
    nodes_to_add.insert(*addr_from);

    // Empty nodes sent or have sender doesn't know all nodes that i know
    if all_known_nodes_addresses.len() > nodes.len() {
        // Send All known nodes to sender and new nodes
        nodes_to_add.iter().for_each(|node| {
            let node_addr = *node;
            let all_nodes = all_known_nodes_addresses.clone();
            tokio::spawn(async move {
                send_known_nodes(&node_addr, all_nodes).await;
            });
        });
    }

    // Send Version to all new nodes plus sender
    let best_height = blockchain
        .get_best_height()
        .await
        .expect("Blockchain get best height error");

    send_version(addr_from, best_height).await;

    nodes_to_add
        .into_iter()
        .filter(|node| node.ne(addr_from))
        .for_each(|node| {
            let node_addr = node;
            let height = best_height;
            tokio::spawn(async move {
                send_version(&node_addr, height).await;
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::BlockchainService;
    use crate::primitives::transaction::Transaction;
    use std::fs;
    use std::net::SocketAddr;
    use std::str::FromStr;

    fn generate_test_genesis_address() -> String {
        // Create a wallet to get a valid Bitcoin address
        let wallet = crate::Wallet::new().expect("Failed to create test wallet");
        wallet.get_address().expect("Failed to get wallet address")
    }

    struct TestBlockchain {
        blockchain: BlockchainService,
        db_path: String,
    }

    impl TestBlockchain {
        async fn new() -> Self {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            // Use process ID and random number for better isolation
            let process_id = std::process::id();
            let random_num = rand::random::<u32>();
            let test_db_path = format!(
                "test_blockchain_db_{}_{}_{}_{}",
                timestamp,
                process_id,
                random_num,
                uuid::Uuid::new_v4()
            );

            // Clean up any existing test database with retry logic
            let _ = Self::cleanup_with_retry(&test_db_path);

            // Create a unique subdirectory for this test
            let unique_db_path = format!("{}/db", test_db_path);
            let _ = fs::create_dir_all(&unique_db_path);

            // Set environment variable for unique database path
            unsafe {
                std::env::set_var("TREE_DIR", &unique_db_path);
            }
            unsafe {
                std::env::set_var("BLOCKS_TREE", &unique_db_path);
            }

            let genesis_address = generate_test_genesis_address();
            let blockchain = BlockchainService::initialize(&genesis_address)
                .await
                .expect("Failed to create test blockchain");

            TestBlockchain {
                blockchain,
                db_path: test_db_path,
            }
        }

        /// Clean up test database with retry logic to handle lock issues
        fn cleanup_with_retry(db_path: &str) -> std::io::Result<()> {
            for attempt in 1..=3 {
                match fs::remove_dir_all(db_path) {
                    Ok(_) => return Ok(()),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        if attempt < 3 {
                            std::thread::sleep(std::time::Duration::from_millis(100 * attempt));
                            continue;
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        return Ok(()); // Directory doesn't exist, that's fine
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }

        fn blockchain(&self) -> &BlockchainService {
            &self.blockchain
        }
    }

    impl Drop for TestBlockchain {
        fn drop(&mut self) {
            // Ensure cleanup happens even if test panics
            let _ = Self::cleanup_with_retry(&self.db_path);
        }
    }

    #[tokio::test]
    async fn test_send_tx() {
        let genesis_address = generate_test_genesis_address();
        let tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create transaction");
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic even if the connection fails
        send_tx(&addr, &tx).await;
    }

    #[tokio::test]
    async fn test_send_block() {
        let test_blockchain = TestBlockchain::new().await;
        let block = test_blockchain
            .blockchain()
            .mine_block(&[])
            .await
            .expect("Failed to mine block");
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic even if the connection fails
        send_block(&addr, &block).await;
    }

    #[tokio::test]
    async fn test_send_get_data() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let id = vec![1, 2, 3, 4];

        // This should not panic even if the connection fails
        send_get_data(&addr, OpType::Block, &id).await;
        send_get_data(&addr, OpType::Tx, &id).await;
    }

    #[tokio::test]
    async fn test_send_inv() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let items = vec![vec![1, 2, 3], vec![4, 5, 6]];

        // This should not panic even if the connection fails
        send_inv(&addr, OpType::Block, &items).await;
        send_inv(&addr, OpType::Tx, &items).await;
    }

    #[tokio::test]
    async fn test_send_message() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let message = "Test message".to_string();

        // This should not panic even if the connection fails
        send_message(&addr, MessageType::Info, message.clone()).await;
        send_message(&addr, MessageType::Error, message.clone()).await;
        send_message(&addr, MessageType::Success, message.clone()).await;
        send_message(&addr, MessageType::Warning, message.clone()).await;
        send_message(&addr, MessageType::Ack, message).await;
    }

    #[tokio::test]
    async fn test_send_version() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let height = 42;

        // This should not panic even if the connection fails
        send_version(&addr, height).await;
    }

    #[tokio::test]
    async fn test_send_get_blocks() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic even if the connection fails
        send_get_blocks(&addr).await;
    }

    #[tokio::test]
    async fn test_send_known_nodes() {
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let nodes = vec![
            SocketAddr::from_str("127.0.0.1:8081").expect("Failed to parse address"),
            SocketAddr::from_str("127.0.0.1:8082").expect("Failed to parse address"),
        ];

        // This should not panic even if the connection fails
        send_known_nodes(&addr, nodes).await;
    }

    #[tokio::test]
    async fn test_process_known_nodes() {
        let test_blockchain = TestBlockchain::new().await;
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");
        let nodes = vec![
            SocketAddr::from_str("127.0.0.1:8081").expect("Failed to parse address"),
            SocketAddr::from_str("127.0.0.1:8082").expect("Failed to parse address"),
        ];

        // This should not panic
        process_known_nodes(test_blockchain.blockchain().clone(), &addr, nodes).await;
    }

    #[test]
    fn test_op_type_serialization() {
        let op_type_block = OpType::Block;
        let op_type_tx = OpType::Tx;

        // Test that they can be serialized (this would fail at compile time if not)
        let _block_json = serde_json::to_string(&op_type_block).expect("Failed to serialize Block");
        let _tx_json = serde_json::to_string(&op_type_tx).expect("Failed to serialize Tx");
    }

    #[test]
    fn test_message_type_serialization() {
        let message_types = vec![
            MessageType::Error,
            MessageType::Success,
            MessageType::Info,
            MessageType::Warning,
            MessageType::Ack,
        ];

        for msg_type in message_types {
            let _json = serde_json::to_string(&msg_type).expect("Failed to serialize MessageType");
        }
    }
}
