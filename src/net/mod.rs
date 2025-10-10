// Network layer (Bitcoin Core: src/net/)
// P2P networking and protocol operations
pub mod net_processing;
pub mod network;

// Re-export the modules
pub use net_processing::*;
pub use network::*;
