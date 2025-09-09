// Declare and defines a module for the domain layer
pub mod blockchain_service;
pub mod wallet_service;

// Re-export the  modules
pub use blockchain_service::BlockchainService;
pub use wallet_service::WalletService;
