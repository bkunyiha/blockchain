use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::node::NodeContext;
use crate::web::handlers::{blockchain, health, mining, transaction, wallet};

/// Create the main API router
pub fn create_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        // Blockchain endpoints
        .route("/blockchain", get(blockchain::get_blockchain_info))
        .route("/blockchain/blocks", get(blockchain::get_blocks))
        .route(
            "/blockchain/blocks/latest",
            get(blockchain::get_latest_blocks),
        )
        .route(
            "/blockchain/blocks/{hash}",
            get(blockchain::get_block_by_hash),
        )
        // Wallet endpoints
        .route("/wallet", post(wallet::create_wallet))
        .route("/wallet/addresses", get(wallet::get_addresses))
        .route("/wallet/{address}", get(wallet::get_wallet_info))
        .route("/wallet/{address}/balance", get(wallet::get_balance))
        // Transaction endpoints(mempool)
        .route("/transactions", post(transaction::send_transaction))
        .route(
            "/transactions/mempool/{txid}",
            get(transaction::get_mempool_transaction),
        )
        .route("/transactions/mempool", get(transaction::get_mempool))
        // Transaction endpoints(blockchain)
        .route("/transactions", get(transaction::get_transactions))
        .route(
            "/transactions/address/{address}",
            get(transaction::get_address_transactions),
        )
        // Mining endpoints
        .route("/mining/info", get(mining::get_mining_info))
        .route("/mining/generatetoaddress", post(mining::generate_to_address))
}

pub fn create_monitor_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        // Health endpoints
        .route("/health", get(health::health_check))
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
}

/// Create admin API routes
pub fn create_admin_api_routes() -> Router<Arc<NodeContext>> {
    Router::new().nest("/api/admin", create_api_routes())
}

/// Create API v1 router with version prefix
pub fn create_api_v1_routes() -> Router<Arc<NodeContext>> {
    Router::new().nest("/api/v1", create_api_routes())
}

/// Create all API routes (v1 and future versions)
pub fn create_all_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .merge(create_api_v1_routes())
        .merge(create_monitor_api_routes())
        .merge(create_admin_api_routes())
}
