// Declare and defines a module for the domain layer
pub mod block;
pub mod blockchain;
pub mod error;
pub mod memory_pool;
pub mod node;

pub mod proof_of_work;
pub mod transaction;
pub mod utxo_set;
pub mod wallet;

// Re-export the  modules
pub use block::Block;
pub use blockchain::Blockchain;
pub use error::*;
pub use memory_pool::BlockInTransit;
pub use memory_pool::MemoryPool;
pub use node::Nodes;
pub use proof_of_work::ProofOfWork;
pub use transaction::{TXInput, TXOutput, Transaction};
pub use utxo_set::UTXOSet;
pub use wallet::{ADDRESS_CHECK_SUM_LEN, Wallet, convert_address, hash_pub_key, validate_address};
