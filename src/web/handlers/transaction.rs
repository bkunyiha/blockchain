use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::node::NodeContext;
use crate::web::models::{
    ApiResponse, PaginatedResponse, SendBitCoinResponse, SendTransactionRequest, TransactionQuery,
    TransactionResponse,
};

/// Send a transaction
///
/// Creates and broadcasts a new transaction to the blockchain network.
#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transaction",
    request_body = SendTransactionRequest,
    responses(
        (status = 202, description = "Transaction has been accepted and is being processed", body = ApiResponse<SendBitCoinResponse>),
        (status = 400, description = "Bad request - invalid addresses or amount"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    let txid = node
        .btc_transaction(&request.from_address, &request.to_address, request.amount)
        .await
        .map_err(|e| {
            error!("Failed to create transaction: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    info!("Transaction {} submitted successfully", txid);

    // Create response using the actual TransactionResponse structure
    let response = SendBitCoinResponse {
        txid,
        timestamp: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Get transaction by ID
///
/// Retrieves a specific transaction by its transaction ID.
#[utoipa::path(
    get,
    path = "/api/v1/transactions/{txid}",
    tag = "Transaction",
    params(
        ("txid" = String, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction retrieved successfully", body = ApiResponse<TransactionResponse>),
        (status = 404, description = "Transaction not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transaction(
    State(node): State<Arc<NodeContext>>,
    Path(txid): Path<String>,
) -> Result<Json<ApiResponse<TransactionResponse>>, StatusCode> {
    // Get transaction from mempool
    let tx = node
        .get_transaction(&txid)
        .map_err(|e| {
            error!("Failed to get transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Convert to response
    let response = TransactionResponse {
        txid: tx.get_tx_id_hex(),
        is_coinbase: tx.is_coinbase(),
        input_count: tx.get_vin().len(),
        output_count: tx.get_vout().len(),
        total_input_value: 0, // TODO: Calculate from inputs
        total_output_value: tx.get_vout().iter().map(|o| o.get_value()).sum(),
        fee: 0, // TODO: Calculate fee
        timestamp: chrono::Utc::now(),
        size_bytes: tx.serialize().unwrap_or_default().len(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Get transactions with pagination
///
/// Retrieves a paginated list of transactions from the blockchain.
#[utoipa::path(
    get,
    path = "/api/v1/transactions",
    tag = "Transaction",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 10)")
    ),
    responses(
        (status = 200, description = "Transactions retrieved successfully", body = ApiResponse<PaginatedResponse<TransactionResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transactions(
    State(node): State<Arc<NodeContext>>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<TransactionResponse>>>, StatusCode> {
    // Get all mempool transactions for now
    let transactions = node.get_mempool_transactions().map_err(|e| {
        error!("Failed to get transactions: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    // Convert to response format
    let responses: Vec<TransactionResponse> = transactions
        .iter()
        .map(|tx| TransactionResponse {
            txid: tx.get_tx_id_hex(),
            is_coinbase: tx.is_coinbase(),
            input_count: tx.get_vin().len(),
            output_count: tx.get_vout().len(),
            total_input_value: 0, // TODO: Calculate from inputs
            total_output_value: tx.get_vout().iter().map(|o| o.get_value()).sum(),
            fee: 0, // TODO: Calculate fee
            timestamp: chrono::Utc::now(),
            size_bytes: tx.serialize().unwrap_or_default().len(),
        })
        .collect();

    let total = responses.len() as u32;
    let paginated = PaginatedResponse::new(responses, page, limit, total);
    Ok(Json(ApiResponse::success(paginated)))
}

/// Get mempool transactions
///
/// Retrieves all transactions currently in the memory pool.
#[utoipa::path(
    get,
    path = "/api/v1/transactions/mempool",
    tag = "Transaction",
    responses(
        (status = 200, description = "Mempool transactions retrieved successfully", body = ApiResponse<Vec<TransactionResponse>>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_mempool(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<Vec<TransactionResponse>>>, StatusCode> {
    // Get transactions from memory pool through node context
    let transactions = node.get_mempool_transactions().map_err(|e| {
        error!("Failed to get mempool transactions: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert to response format
    let responses: Vec<TransactionResponse> = transactions
        .iter()
        .map(|tx| TransactionResponse {
            txid: tx.get_tx_id_hex(),
            is_coinbase: tx.is_coinbase(),
            input_count: tx.get_vin().len(),
            output_count: tx.get_vout().len(),
            total_input_value: 0, // TODO: Calculate from inputs
            total_output_value: tx.get_vout().iter().map(|o| o.get_value()).sum(),
            fee: 0, // TODO: Calculate fee
            timestamp: chrono::Utc::now(),
            size_bytes: tx.serialize().unwrap_or_default().len(),
        })
        .collect();

    Ok(Json(ApiResponse::success(responses)))
}

/// Get transaction history for an address
///
/// Retrieves all transactions associated with a specific address.
#[utoipa::path(
    get,
    path = "/api/v1/transactions/address/{address}",
    tag = "Transaction",
    params(
        ("address" = String, Path, description = "Wallet address"),
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 10)")
    ),
    responses(
        (status = 200, description = "Address transactions retrieved successfully", body = ApiResponse<PaginatedResponse<TransactionResponse>>),
        (status = 400, description = "Invalid address format"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_address_transactions(
    State(node): State<Arc<NodeContext>>,
    Path(_address): Path<String>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<TransactionResponse>>>, StatusCode> {
    // Get all mempool transactions and filter by address
    let transactions = node.get_mempool_transactions().map_err(|e| {
        error!("Failed to get transactions: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    // Filter and convert to response format
    // TODO: Implement proper address filtering
    let responses: Vec<TransactionResponse> = transactions
        .iter()
        .map(|tx| TransactionResponse {
            txid: tx.get_tx_id_hex(),
            is_coinbase: tx.is_coinbase(),
            input_count: tx.get_vin().len(),
            output_count: tx.get_vout().len(),
            total_input_value: 0, // TODO: Calculate from inputs
            total_output_value: tx.get_vout().iter().map(|o| o.get_value()).sum(),
            fee: 0, // TODO: Calculate fee
            timestamp: chrono::Utc::now(),
            size_bytes: tx.serialize().unwrap_or_default().len(),
        })
        .collect();

    let total = responses.len() as u32;
    let paginated = PaginatedResponse::new(responses, page, limit, total);
    Ok(Json(ApiResponse::success(paginated)))
}
