use blockchain::{Blockchain, ConnectNode, GLOBAL_CONFIG, Transaction, UTXOSet, Wallets};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
mod test_helpers;

fn generate_test_genesis_address() -> String {
    // Create a wallet to get a valid Bitcoin address
    let wallet = blockchain::Wallet::new().expect("Failed to create test wallet");
    wallet.get_address().expect("Failed to get wallet address")
}

fn create_test_blockchain() -> (Blockchain, String) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let test_db_path = format!("test_integration_db_{}_{}", timestamp, uuid::Uuid::new_v4());
    let _ = std::fs::remove_dir_all(&test_db_path);

    unsafe {
        std::env::set_var("TREE_DIR", &test_db_path);
        std::env::set_var("BLOCKS_TREE", &test_db_path);
    }

    let genesis_address = generate_test_genesis_address();
    let blockchain =
        Blockchain::create_blockchain(&genesis_address).expect("Failed to create test blockchain");
    (blockchain, test_db_path)
}

fn cleanup_test_blockchain(db_path: &str) {
    let _ = std::fs::remove_dir_all(db_path);
}

struct TestDatabaseGuard {
    db_path: String,
}

impl TestDatabaseGuard {
    fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let db_path = format!("test_integration_db_{}_{}", timestamp, uuid::Uuid::new_v4());
        let _ = std::fs::remove_dir_all(&db_path);

        unsafe {
            std::env::set_var("TREE_DIR", &db_path);
            std::env::set_var("BLOCKS_TREE", &db_path);
        }

        TestDatabaseGuard { db_path }
    }
}

impl Drop for TestDatabaseGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.db_path);
    }
}

#[tokio::test]
async fn test_blockchain_integration() {
    let (mut blockchain, db_path) = create_test_blockchain();
    let genesis_address = generate_test_genesis_address();

    // Test creating a new blockchain
    assert_eq!(
        blockchain.get_best_height().expect("Failed to get height"),
        1
    );

    // Test mining a block with the same blockchain instance
    let coinbase_tx =
        Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
    let transactions = vec![coinbase_tx];
    let new_block = blockchain
        .mine_block(transactions.as_slice())
        .expect("Failed to mine block");

    // Test adding the block
    blockchain
        .add_block(&new_block)
        .expect("Failed to add block");
    assert_eq!(
        blockchain.get_best_height().expect("Failed to get height"),
        2
    );

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_wallet_integration() {
    // Test wallet creation with unique path
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp directory");
    let wallet_path = temp_dir.path().join("test_wallets.dat");

    // Set environment variable for unique wallet path
    unsafe {
        std::env::set_var("WALLET_FILE", wallet_path.to_str().unwrap());
    }

    let mut wallets = Wallets::new().expect("Failed to create wallets");
    let address = wallets.create_wallet().expect("Failed to create wallet");
    assert!(!address.is_empty());

    // Test getting wallet
    let wallet = wallets.get_wallet(&address).expect("Failed to get wallet");
    assert_eq!(
        wallet.get_address().expect("Failed to get address"),
        address
    );
}

#[tokio::test]
async fn test_utxo_set_integration() {
    let (mut blockchain, db_path) = create_test_blockchain();
    let genesis_address = generate_test_genesis_address();

    // Create blockchain and add a block
    let coinbase_tx =
        Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
    let transactions = vec![coinbase_tx];
    let new_block = blockchain
        .mine_block(transactions.as_slice())
        .expect("Failed to mine block");
    blockchain
        .add_block(&new_block)
        .expect("Failed to add block");

    // Test UTXO set - need to reindex first
    let utxo_set = UTXOSet::new(blockchain);
    utxo_set.reindex().expect("Failed to reindex UTXO set");
    let count = utxo_set
        .count_transactions()
        .expect("Failed to count transactions");
    assert!(count > 0);

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_server_creation() {
    let (blockchain, db_path) = create_test_blockchain();
    let blockchain_arc = Arc::new(RwLock::new(blockchain));

    // Test that we can create a server (the blockchain field is private, so we can't test it directly)
    let _server = blockchain::server::Server::new(blockchain_arc);
    // If we get here without panicking, the server was created successfully

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_connect_node_parsing() {
    // Test local node
    let local_node = ConnectNode::from_str("local").expect("Failed to parse local");
    assert!(!local_node.is_remote());

    // Test remote node
    let remote_node = ConnectNode::from_str("127.0.0.1:8080").expect("Failed to parse remote");
    assert!(remote_node.is_remote());

    // Test invalid address
    let invalid_result = ConnectNode::from_str("invalid_address");
    assert!(invalid_result.is_err());
}

#[tokio::test]
async fn test_global_config() {
    // Test that global config can be accessed
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    assert!(node_addr.port() > 0);

    let is_miner = GLOBAL_CONFIG.is_miner();
    // This should be a boolean value
    assert!(is_miner == true || is_miner == false);
}

#[tokio::test]
async fn test_transaction_creation_and_validation() {
    // Test coinbase transaction
    let genesis_address = generate_test_genesis_address();
    let coinbase_tx =
        Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
    assert!(coinbase_tx.is_coinbase());
    assert_eq!(coinbase_tx.get_vout().len(), 1);
    assert_eq!(coinbase_tx.get_vin().len(), 1);

    // Test transaction serialization
    let serialized = coinbase_tx.serialize().expect("Failed to serialize");
    let deserialized = Transaction::deserialize(&serialized).expect("Failed to deserialize");
    assert_eq!(coinbase_tx.get_id(), deserialized.get_id());
}

#[tokio::test]
async fn test_blockchain_persistence() {
    let _guard = TestDatabaseGuard::new();
    let genesis_address = generate_test_genesis_address();

    // Create blockchain and add a block
    {
        let mut blockchain =
            Blockchain::create_blockchain(&genesis_address).expect("Failed to create blockchain");
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = blockchain
            .mine_block(transactions.as_slice())
            .expect("Failed to mine block");
        blockchain
            .add_block(&new_block)
            .expect("Failed to add block");
    }

    // Create new blockchain instance and verify persistence
    let blockchain =
        Blockchain::create_blockchain(&genesis_address).expect("Failed to create new blockchain");
    assert_eq!(
        blockchain.get_best_height().expect("Failed to get height"),
        2
    );
    // Guard will automatically clean up when it goes out of scope
}

#[tokio::test]
async fn test_blockchain_iterator() {
    let (mut blockchain, db_path) = create_test_blockchain();
    let genesis_address = generate_test_genesis_address();

    // Add multiple blocks
    for _i in 0..3 {
        let coinbase_tx =
            Transaction::new_coinbase_tx(&genesis_address).expect("Failed to create coinbase tx");
        let transactions = vec![coinbase_tx];
        let new_block = blockchain
            .mine_block(transactions.as_slice())
            .expect("Failed to mine block");
        blockchain
            .add_block(&new_block)
            .expect("Failed to add block");
    }

    // Test iterator
    let mut iterator = blockchain.iterator().expect("Failed to create iterator");
    let mut block_count = 0;

    while let Some(block) = iterator.next() {
        block_count += 1;
        assert!(block.get_height() > 0); // Fixed: height should be > 0, not >= 0
        assert!(!block.get_hash().is_empty());
    }

    // Should have genesis block + 3 new blocks = 4 total
    assert_eq!(block_count, 4);

    // Cleanup
    cleanup_test_blockchain(&db_path);
}

#[tokio::test]
async fn test_wallet_transaction_creation() {
    let (mut blockchain, db_path) = create_test_blockchain();

    // Create wallets with unique path
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp directory");
    let wallet_path = temp_dir.path().join("test_wallets.dat");

    // Set environment variable for unique wallet path
    unsafe {
        std::env::set_var("WALLET_FILE", wallet_path.to_str().unwrap());
    }

    let mut wallets = Wallets::new().expect("Failed to create wallets");
    let address1 = wallets.create_wallet().expect("Failed to create wallet 1");
    let address2 = wallets.create_wallet().expect("Failed to create wallet 2");

    // Create blockchain with some initial balance to address1
    let coinbase_tx =
        Transaction::new_coinbase_tx(&address1).expect("Failed to create coinbase tx");
    let transactions = vec![coinbase_tx];
    let new_block = blockchain
        .mine_block(transactions.as_slice())
        .expect("Failed to mine block");
    blockchain
        .add_block(&new_block)
        .expect("Failed to add block");

    // Create UTXO set and reindex
    let utxo_set = UTXOSet::new(blockchain);
    utxo_set.reindex().expect("Failed to reindex UTXO set");

    // Test creating a transaction between wallets
    let transaction = Transaction::new_utxo_transaction(&address1, &address2, 5, &utxo_set);
    assert!(transaction.is_ok());

    let tx = transaction.expect("Failed to create transaction");
    assert!(!tx.is_coinbase());
    assert_eq!(tx.get_vout().len(), 2); // One output to address2, one change back to address1

    // Cleanup
    cleanup_test_blockchain(&db_path);
}
