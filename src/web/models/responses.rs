use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

/// Blockchain information response
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfoResponse {
    pub height: usize,
    pub difficulty: u32,
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub mempool_size: usize,
    pub last_block_hash: String,
    pub last_block_timestamp: DateTime<Utc>,
}

/// Block response model
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub height: usize,
    pub nonce: u64,
    pub difficulty: u32,
    pub transaction_count: usize,
    pub merkle_root: String,
    pub size_bytes: usize,
}

/// Transaction response model
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub txid: String,
    pub is_coinbase: bool,
    pub input_count: usize,
    pub output_count: usize,
    pub total_input_value: i32,
    pub total_output_value: i32,
    pub fee: i32,
    pub timestamp: DateTime<Utc>,
    pub size_bytes: usize,
}

/// Wallet response model
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    pub address: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
}

/// Balance response model
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: i32,
    pub unconfirmed_balance: i32,
    pub utxo_count: usize,
    pub last_updated: DateTime<Utc>,
}

/// Mining status response
#[derive(Debug, Serialize, Deserialize)]
pub struct MiningStatusResponse {
    pub is_mining: bool,
    pub mining_address: Option<String>,
    pub hash_rate: f64,
    pub blocks_mined: usize,
    pub last_block_time: Option<DateTime<Utc>>,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub blockchain_height: usize,
    pub connected_peers: usize,
    pub memory_usage_mb: f64,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, limit: u32, total: u32) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as u32;
        Self {
            has_next: page + 1 < total_pages,
            has_prev: page > 0,
            items,
            page,
            limit,
            total,
            total_pages,
        }
    }
}
