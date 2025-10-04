use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

use crate::core::Block;
use crate::service::blockchain_service::BlockchainService;
use crate::web::models::{
    ApiResponse, BlockQuery, BlockResponse, BlockchainInfoResponse, PaginatedResponse,
};

/// Get blockchain information
pub async fn get_blockchain_info(
    State(blockchain): State<Arc<BlockchainService>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    let height = blockchain
        .get_best_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get the last block by height
    let last_block = blockchain
        .get_last_block()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let last_block_hash = last_block
        .map(|block| block.get_hash().to_string())
        .unwrap_or_else(|| "genesis".to_string());

    let info = BlockchainInfoResponse {
        height,
        difficulty: 1, // TODO: Get actual difficulty
        total_blocks: height + 1,
        total_transactions: 0, // TODO: Calculate total transactions
        mempool_size: 0,       // TODO: Get from memory pool
        last_block_hash,
        last_block_timestamp: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(info)))
}

/// Get block by hash
pub async fn get_block_by_hash(
    State(blockchain): State<Arc<BlockchainService>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    let block = blockchain
        .get_block_by_hash(hash.as_bytes())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let block_response = block
        .map(|block| block_to_response(block.clone(), block.get_height()))
        .ok_or(StatusCode::NOT_FOUND);

    Ok(Json(ApiResponse::success(block_response?)))
}

/// Get blocks with pagination
pub async fn get_blocks(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<BlockQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<BlockResponse>>>, StatusCode> {
    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let height = blockchain
        .get_best_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let start_height = (page * limit) as usize;
    let end_height = std::cmp::min(start_height + limit as usize, height + 1);

    let blocks = blockchain
        .get_blocks_by_height(start_height, end_height)
        .await
        .map(|blocks| {
            blocks
                .iter()
                .map(|block| block_to_response(block.clone(), block.get_height()))
                .collect()
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = height + 1;
    let paginated = PaginatedResponse::new(blocks, page, limit, total as u32);

    Ok(Json(ApiResponse::success(paginated)))
}

/// Get latest blocks
pub async fn get_latest_blocks(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<BlockQuery>,
) -> Result<Json<ApiResponse<Vec<BlockResponse>>>, StatusCode> {
    let limit = query.limit.unwrap_or(10) as usize;

    let height = blockchain
        .get_best_height()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let start_height = if height >= limit {
        height - limit + 1
    } else {
        0
    };

    let blocks = blockchain
        .get_blocks_by_height(start_height, height)
        .await
        .map(|blocks| {
            blocks
                .iter()
                .map(|block| block_to_response(block.clone(), block.get_height()))
                .collect()
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::success(blocks)))
}

/// Convert Block to BlockResponse
fn block_to_response(block: Block, height: usize) -> BlockResponse {
    BlockResponse {
        hash: block.get_hash().to_string(),
        previous_hash: block.get_pre_block_hash().to_string(),
        timestamp: chrono::DateTime::from_timestamp(block.get_timestamp(), 0)
            .unwrap_or_else(chrono::Utc::now),
        height,
        nonce: 0,      // TODO: Get actual nonce
        difficulty: 1, // TODO: Get actual difficulty
        transaction_count: block.get_transactions().len(),
        merkle_root: "".to_string(), // TODO: Calculate merkle root
        size_bytes: 0,               // TODO: Calculate block size
    }
}
