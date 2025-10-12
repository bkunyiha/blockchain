use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

use crate::node::NodeContext;
use crate::primitives::Block;
use crate::web::models::{
    ApiResponse, BlockQuery, BlockResponse, BlockchainInfoResponse, PaginatedResponse,
};

/// Get blockchain information
///
/// Returns comprehensive blockchain statistics including height, difficulty,
/// total blocks, transactions, and mempool status.
#[utoipa::path(
    get,
    path = "/api/v1/blockchain",
    tag = "Blockchain",
    responses(
        (status = 200, description = "Blockchain information retrieved successfully", body = ApiResponse<BlockchainInfoResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    let height = node
        .get_blockchain_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get the last block by height
    let last_block = node
        .blockchain()
        .get_last_block()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let last_block_hash = last_block
        .map(|block| block.get_hash().to_string())
        .unwrap_or_else(|| "genesis".to_string());

    // Get mempool size
    let mempool_size = node
        .get_mempool_size()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let info = BlockchainInfoResponse {
        height,
        difficulty: 1, // TODO: Get actual difficulty
        total_blocks: height + 1,
        total_transactions: 0, // TODO: Calculate total transactions
        mempool_size,
        last_block_hash,
        last_block_timestamp: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(info)))
}

/// Get block by hash
///
/// Retrieves a specific block from the blockchain using its hash.
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks/{hash}",
    tag = "Blockchain",
    params(
        ("hash" = String, Path, description = "Block hash")
    ),
    responses(
        (status = 200, description = "Block retrieved successfully", body = ApiResponse<BlockResponse>),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_block_by_hash(
    State(node): State<Arc<NodeContext>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    let block = node
        .get_block_by_hash(&hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match block {
        Some(block) => {
            let height = block.get_height();
            let response = block_to_response(block, height).await;
            Ok(Json(ApiResponse::success(response)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get blocks with pagination
///
/// Retrieves a paginated list of blocks from the blockchain.
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks",
    tag = "Blockchain",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 10)")
    ),
    responses(
        (status = 200, description = "Blocks retrieved successfully", body = ApiResponse<PaginatedResponse<BlockResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_blocks(
    State(node): State<Arc<NodeContext>>,
    Query(query): Query<BlockQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<BlockResponse>>>, StatusCode> {
    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let height = node
        .get_blockchain_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let start_height = (page * limit) as usize;
    let end_height = std::cmp::min(start_height + limit as usize, height + 1);

    let blocks_result = node
        .blockchain()
        .get_blocks_by_height(start_height, end_height)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut blocks = Vec::new();
    for block in blocks_result {
        let response = block_to_response(block.clone(), block.get_height()).await;
        blocks.push(response);
    }

    let total = height + 1;
    let paginated = PaginatedResponse::new(blocks, page, limit, total as u32);

    Ok(Json(ApiResponse::success(paginated)))
}

/// Get latest blocks
///
/// Retrieves the most recent blocks from the blockchain.
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks/latest",
    tag = "Blockchain",
    params(
        ("limit" = Option<u32>, Query, description = "Number of blocks to retrieve (default: 10)")
    ),
    responses(
        (status = 200, description = "Latest blocks retrieved successfully", body = ApiResponse<Vec<BlockResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_latest_blocks(
    State(node): State<Arc<NodeContext>>,
    Query(query): Query<BlockQuery>,
) -> Result<Json<ApiResponse<Vec<BlockResponse>>>, StatusCode> {
    let limit = query.limit.unwrap_or(10) as usize;

    let blocks_result = node
        .get_latest_blocks(limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut blocks = Vec::new();
    for block in blocks_result {
        let response = block_to_response(block.clone(), block.get_height()).await;
        blocks.push(response);
    }

    Ok(Json(ApiResponse::success(blocks)))
}

/// Convert Block to BlockResponse
async fn block_to_response(block: Block, height: usize) -> BlockResponse {
    BlockResponse {
        hash: block.get_hash().to_string(),
        previous_hash: block.get_pre_block_hash().to_string(),
        timestamp: chrono::DateTime::from_timestamp(block.get_timestamp(), 0)
            .unwrap_or_else(chrono::Utc::now),
        height,
        nonce: 0,      // TODO: Get actual nonce
        difficulty: 1, // TODO: Get actual difficulty
        transaction_count: block.get_transactions().await.unwrap_or(&[]).len(),
        merkle_root: "".to_string(), // TODO: Calculate merkle root
        size_bytes: 0,               // TODO: Calculate block size
    }
}
