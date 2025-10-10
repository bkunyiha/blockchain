use crate::node::NodeContext;
use crate::web::openapi::create_swagger_ui;
use axum::{Router, response::Html, routing::get};
use std::sync::Arc;

/// Create web UI routes (for future web interface)
pub fn create_web_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        .route("/", get(dashboard))
        .route("/dashboard", get(dashboard))
        .route("/blocks", get(blocks_page))
        .route("/transactions", get(transactions_page))
        .route("/wallet", get(wallet_page))
        .route("/mining", get(mining_page))
        .route("/network", get(network_page))
        .merge(create_swagger_ui())
}

/// Dashboard page
async fn dashboard() -> Html<&'static str> {
    Html(include_str!("../static/dashboard.html"))
}

/// Blocks page
async fn blocks_page() -> Html<&'static str> {
    Html(include_str!("../static/blocks.html"))
}

/// Transactions page
async fn transactions_page() -> Html<&'static str> {
    Html("<html><body><h1>Transactions</h1><p>Coming soon...</p></body></html>")
}

/// Wallet page
async fn wallet_page() -> Html<&'static str> {
    Html("<html><body><h1>Wallet</h1><p>Coming soon...</p></body></html>")
}

/// Mining page
async fn mining_page() -> Html<&'static str> {
    Html("<html><body><h1>Mining</h1><p>Coming soon...</p></body></html>")
}

/// Network page
async fn network_page() -> Html<&'static str> {
    Html("<html><body><h1>Network</h1><p>Coming soon...</p></body></html>")
}
