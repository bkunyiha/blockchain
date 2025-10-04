use axum::{extract::State, http::StatusCode, response::Json};
use std::sync::Arc;
use std::time::Instant;

use crate::service::blockchain_service::BlockchainService;
use crate::web::models::{ApiResponse, HealthResponse};

/// Health check endpoint
pub async fn health_check(
    State(blockchain): State<Arc<BlockchainService>>,
) -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let start_time = Instant::now();

    // Get blockchain information
    let height = blockchain
        .get_best_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Calculate uptime (simplified - in real implementation, track start time)
    let uptime_seconds = start_time.elapsed().as_secs();

    // Get memory usage (simplified)
    let memory_usage_mb = get_memory_usage();

    let health_response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds,
        blockchain_height: height,
        connected_peers: 0, // TODO: Get from network layer
        memory_usage_mb,
    };

    Ok(Json(ApiResponse::success(health_response)))
}

/// Liveness probe endpoint
pub async fn liveness() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("alive".to_string())))
}

/// Readiness probe endpoint
pub async fn readiness(
    State(blockchain): State<Arc<BlockchainService>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Check if blockchain is accessible
    match blockchain.get_best_height().await {
        Ok(_) => Ok(Json(ApiResponse::success("ready".to_string()))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Get memory usage in MB (simplified implementation)
fn get_memory_usage() -> f64 {
    // This is a simplified implementation
    // In a real application, you'd use a proper memory monitoring library
    0.0
}
