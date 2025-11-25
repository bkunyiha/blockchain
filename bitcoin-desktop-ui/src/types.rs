use bitcoin_api::{ApiResponse, BlockSummary, BlockchainInfo, CreateWalletResponse};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Blockchain,
    Wallet,
    Transactions,
    Mining,
    Health,
}

impl Menu {
    pub const ALL: [Menu; 5] = [
        Menu::Blockchain,
        Menu::Wallet,
        Menu::Transactions,
        Menu::Mining,
        Menu::Health,
    ];
}

impl core::fmt::Display for Menu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Menu::Blockchain => "Blockchain",
            Menu::Wallet => "Wallet",
            Menu::Transactions => "Transactions",
            Menu::Mining => "Mining",
            Menu::Health => "Health",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataSection {
    BlockchainInfo,
    Blocks,
    BlocksAll,
    BlockByHash,
    MiningInfo,
    Generate,
    Health,
    Liveness,
    Readiness,
    Mempool,
    MempoolTx,
    Transactions,
    AddressTransactions,
    WalletInfo,
    WalletBalance,
}

#[derive(Debug, Clone)]
pub enum Message {
    MenuChanged(Menu),
    BaseUrlChanged(String),
    ApiKeyChanged(String),
    // Inputs
    BlockHashChanged(String),
    MiningAddressChanged(String),
    MiningNBlocksChanged(String),
    MiningMaxTriesChanged(String),
    TxidChanged(String),
    AddrTxChanged(String),
    FetchInfo,
    FetchBlocks,
    InfoLoaded(Result<ApiResponse<BlockchainInfo>, String>),
    BlocksLoaded(Result<ApiResponse<Vec<BlockSummary>>, String>),
    // Extra blockchain
    FetchBlocksAll,
    BlocksAllLoaded(Result<ApiResponse<Value>, String>),
    FetchBlockByHash(String),
    BlockByHashLoaded(Result<ApiResponse<Value>, String>),
    // Mining
    FetchMiningInfo,
    MiningInfoLoaded(Result<ApiResponse<Value>, String>),
    GenerateToAddress {
        address: String,
        nblocks: u32,
        maxtries: Option<u32>,
    },
    GenerateToAddressDone(Result<ApiResponse<Value>, String>),
    // Health
    FetchHealth,
    HealthLoaded(Result<ApiResponse<Value>, String>),
    FetchLiveness,
    LivenessLoaded(Result<ApiResponse<Value>, String>),
    FetchReadiness,
    ReadinessLoaded(Result<ApiResponse<Value>, String>),
    // Transactions
    FetchMempool,
    MempoolLoaded(Result<ApiResponse<Value>, String>),
    FetchMempoolTx(String),
    MempoolTxLoaded(Result<ApiResponse<Value>, String>),
    FetchTransactions,
    TransactionsLoaded(Result<ApiResponse<Value>, String>),
    FetchAddressTransactions(String),
    AddressTransactionsLoaded(Result<ApiResponse<Value>, String>),
    // Wallet admin
    WalletLabelChanged(String),
    WalletAddressChanged(String),
    CreateWalletAdmin,
    CreateWalletAdminDone(Result<ApiResponse<CreateWalletResponse>, String>),
    FetchAddressesAdmin,
    AddressesAdminLoaded(Result<ApiResponse<Value>, String>),
    FetchWalletInfoAdmin(String),
    WalletInfoAdminLoaded(Result<ApiResponse<Value>, String>),
    FetchBalanceAdmin(String),
    BalanceAdminLoaded(Result<ApiResponse<Value>, String>),
    // Clipboard
    CopyToClipboard(String),
    ClipboardCopied(bool), // true = success, false = failed
}

