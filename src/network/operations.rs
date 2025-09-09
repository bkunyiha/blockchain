use crate::node::Node;
use crate::server::{
    CENTERAL_NODE, GLOBAL_MEMORY_POOL, GLOBAL_NODES, MessageType, NODE_VERSION, OpType, Package,
    TCP_WRITE_TIMEOUT, TRANSACTION_THRESHOLD,
};
use crate::{Block, BlockchainService, GLOBAL_CONFIG, Transaction, UTXOSet};

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

/// Add transaction to memory pool functionally
async fn add_to_memory_pool(tx: Transaction, blockchain_service: &BlockchainService) {
    info!("\n");
    info!(
        "******************************************************************************************************"
    );
    info!(
        "Adding transaction to memory pool: {:?}",
        tx.get_tx_id_hex()
    );
    info!(
        "******************************************************************************************************\n"
    );
    GLOBAL_MEMORY_POOL
        .add(tx.clone())
        .expect("Memory pool add error");

    let utxo_set = UTXOSet::new(blockchain_service.clone());
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), true)
        .await
        .expect("Failed to get blockchain");
}

/// Remove transaction from memory pool functionally
pub async fn remove_from_memory_pool(tx: Transaction, blockchain: &BlockchainService) {
    GLOBAL_MEMORY_POOL
        .remove(tx.clone())
        .expect("Memory pool remove error");

    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set
        .set_global_mem_pool_flag(&tx.clone(), false)
        .await
        .expect("Failed to get blockchain");
}

/// Check if transaction exists in memory pool
fn transaction_exists_in_pool(tx: &Transaction) -> bool {
    GLOBAL_MEMORY_POOL.contains_transaction(tx).unwrap_or(false)
}

/// Get nodes excluding the sender
fn get_nodes_excluding_sender(addr_from: &SocketAddr) -> Vec<Node> {
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
fn should_trigger_mining() -> bool {
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    let is_miner = GLOBAL_CONFIG.is_miner();
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}

/// Prepare transactions for mining
fn prepare_mining_transactions()
-> Result<Vec<Transaction>, Box<dyn std::error::Error + Send + Sync>> {
    let mut txs = GLOBAL_MEMORY_POOL
        .get_all()
        .expect("Memory pool get all error");

    let coinbase_tx = create_mining_coinbase_transaction()?;
    txs.push(coinbase_tx);

    Ok(txs)
}

pub async fn process_transaction(
    addr_from: &SocketAddr,
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
            Ok(txs) => process_mine_block(txs, blockchain).await,
            Err(e) => error!("Failed to prepare mining transactions: {}", e),
        }
    }
}

/// Process mining block functionally
async fn process_mine_block(txs: Vec<Transaction>, blockchain: &BlockchainService) {
    let my_node_addr = GLOBAL_CONFIG.get_node_addr();

    // Mine a new block with the transactions in the memory pool
    let new_block = blockchain
        .mine_block(&txs)
        .await
        .expect("Blockchain mine block error");

    // Reindex UTXO set to ensure it's in sync
    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set
        .reindex()
        .await
        .expect("Failed to reindex UTXO set");
    info!("New block {} is mined!", new_block.get_hash());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transaction::Transaction;
    use crate::service::blockchain_service::BlockchainService;
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
    async fn test_process_transaction() {
        let test_blockchain = TestBlockchain::new().await;
        let genesis_address = generate_test_genesis_address();
        let tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create transaction");
        let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Failed to parse address");

        // This should not panic
        process_transaction(&addr, tx, test_blockchain.blockchain()).await;
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

    #[tokio::test]
    async fn test_mine_empty_block() {
        let test_blockchain = TestBlockchain::new().await;

        // This should not panic
        mine_empty_block(test_blockchain.blockchain()).await;
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
