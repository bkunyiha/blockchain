use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request model for creating a new wallet
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateWalletRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Wallet name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
}

/// Request model for sending a transaction
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SendTransactionRequest {
    #[validate(length(min = 26, max = 35, message = "Invalid from address format"))]
    pub from_address: String,

    #[validate(length(min = 26, max = 35, message = "Invalid to address format"))]
    pub to_address: String,

    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i32,
}

/// Request model for mining operations
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MiningRequest {
    #[validate(length(min = 26, max = 35, message = "Invalid mining address format"))]
    pub mining_address: String,

    #[validate(range(min = 1, max = 10, message = "Thread count must be between 1 and 10"))]
    pub thread_count: Option<u8>,
}

/// Request model for querying blocks
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BlockQuery {
    #[validate(range(min = 0, message = "Page must be 0 or greater"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,

    pub hash: Option<String>,
}

/// Request model for querying transactions
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TransactionQuery {
    #[validate(range(min = 0, message = "Page must be 0 or greater"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,

    pub txid: Option<String>,
}

/// Request model for balance queries
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BalanceQuery {
    #[validate(length(min = 26, max = 35, message = "Invalid address format"))]
    pub address: String,
}
