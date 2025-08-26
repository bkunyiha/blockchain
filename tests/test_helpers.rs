use blockchain::{Blockchain, Transaction, Wallets};
use tempfile::TempDir;

/// Generate a unique genesis address for testing
pub fn generate_test_genesis_address() -> String {
    // Create a wallet to get a valid Bitcoin address
    let wallet = blockchain::Wallet::new().expect("Failed to create test wallet");
    wallet.get_address().expect("Failed to get wallet address")
}

/// Helper function to create a temporary blockchain for testing
pub fn create_temp_blockchain() -> (Blockchain, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_blockchain");

    // Set environment variable for unique database path
    unsafe {
        std::env::set_var("TREE_DIR", db_path.to_str().unwrap());
        std::env::set_var("BLOCKS_TREE", db_path.to_str().unwrap());
    }

    let genesis_address = generate_test_genesis_address();
    let blockchain =
        Blockchain::create_blockchain(&genesis_address).expect("Failed to create test blockchain");

    (blockchain, temp_dir)
}

/// Helper function to create a blockchain with some initial blocks
pub fn create_blockchain_with_blocks(num_blocks: usize) -> (Blockchain, TempDir) {
    let (mut blockchain, temp_dir) = create_temp_blockchain();
    let genesis_address = generate_test_genesis_address();

    for _i in 0..num_blocks {
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

    (blockchain, temp_dir)
}

/// Helper function to create test wallets
pub fn create_test_wallets() -> Wallets {
    Wallets::new().expect("Failed to create test wallets")
}

/// Helper function to verify blockchain integrity
pub fn verify_blockchain_integrity(blockchain: &Blockchain) -> bool {
    let mut iterator = match blockchain.iterator() {
        Ok(iter) => iter,
        Err(_) => return false,
    };

    // Collect all blocks first (iterator goes from newest to oldest)
    let mut blocks = Vec::new();
    while let Some(block) = iterator.next() {
        blocks.push(block);
    }

    // Sort by height to get them in ascending order
    blocks.sort_by_key(|block| block.get_height());

    // Now verify integrity
    for (i, block) in blocks.iter().enumerate() {
        let expected_height = i + 1;
        if block.get_height() != expected_height {
            return false;
        }

        // Check hash chain (except for genesis block)
        if expected_height > 1 {
            let prev_block = &blocks[i - 1];
            if block.get_pre_block_hash() != prev_block.get_hash() {
                return false;
            }
        }
    }

    true
}

/// Helper function to create multiple test addresses
pub fn create_test_addresses(count: usize) -> Vec<String> {
    let mut wallets = create_test_wallets();
    let mut addresses = Vec::new();

    for _ in 0..count {
        let address = wallets.create_wallet().expect("Failed to create wallet");
        addresses.push(address);
    }

    addresses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_temp_blockchain() {
        let (blockchain, temp_dir) = create_temp_blockchain();
        assert_eq!(
            blockchain.get_best_height().expect("Failed to get height"),
            1
        );
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_create_blockchain_with_blocks() {
        let (blockchain, _temp_dir) = create_blockchain_with_blocks(3);
        assert_eq!(
            blockchain.get_best_height().expect("Failed to get height"),
            4
        );
    }

    #[test]
    fn test_create_test_wallets() {
        let mut wallets = create_test_wallets();
        let address = wallets.create_wallet().expect("Failed to create wallet");
        assert!(!address.is_empty());
    }

    #[test]
    fn test_verify_blockchain_integrity() {
        let (blockchain, _temp_dir) = create_blockchain_with_blocks(2);
        assert!(verify_blockchain_integrity(&blockchain));
    }

    #[test]
    fn test_create_test_addresses() {
        let addresses = create_test_addresses(3);
        assert_eq!(addresses.len(), 3);
        for address in addresses {
            assert!(!address.is_empty());
        }
    }
}
