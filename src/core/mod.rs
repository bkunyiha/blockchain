// Declare and defines a module for the core layer
pub mod block;
pub mod blockchain;
pub mod memory_pool;

pub mod proof_of_work;
pub mod transaction;
pub mod utxo_set;

// Re-export the  modules
pub use block::{Block, GENESIS_BLOCK_PRE_BLOCK_HASH};
pub use blockchain::Blockchain;
pub use memory_pool::BlockInTransit;
pub use memory_pool::MemoryPool;
pub use proof_of_work::ProofOfWork;
pub use transaction::{TXInput, TXOutput, Transaction};
pub use utxo_set::UTXOSet;
