use crate::types::{DataSection, Menu, Message};
use bitcoin_api::{BlockSummary, BlockchainInfo};
use serde_json::Value;

#[derive(Debug)]
pub struct AdminApp {
    pub menu: Menu,
    pub base_url: String,
    pub api_key: String,
    pub status: String,
    pub info: Option<BlockchainInfo>,
    pub blocks: Vec<BlockSummary>,
    // Inputs for actions
    pub block_hash_input: String,
    pub mining_address_input: String,
    pub mining_nblocks_input: String,
    pub mining_maxtries_input: String,
    pub txid_input: String,
    pub addr_tx_input: String,
    // Wallet admin state
    pub wallet_label_input: String,
    pub wallet_address_input: String,
    pub addresses: Vec<String>,
    pub wallet_info: Option<Value>,
    pub wallet_balance: Option<Value>,
    pub created_wallet_address: Option<String>,
    // Response data storage
    pub blocks_all_data: Option<Value>,
    pub block_by_hash_data: Option<Value>,
    pub mining_info_data: Option<Value>,
    pub generate_result: Option<Value>,
    pub health_data: Option<Value>,
    pub liveness_data: Option<Value>,
    pub readiness_data: Option<Value>,
    pub mempool_data: Option<Value>,
    pub mempool_tx_data: Option<Value>,
    pub transactions_data: Option<Value>,
    pub address_transactions_data: Option<Value>,
}

impl Default for AdminApp {
    fn default() -> Self {
        Self {
            menu: Menu::Blockchain,
            base_url: String::new(),
            api_key: String::new(),
            status: String::new(),
            info: None,
            blocks: Vec::new(),
            block_hash_input: String::new(),
            mining_address_input: String::new(),
            mining_nblocks_input: String::new(),
            mining_maxtries_input: String::new(),
            txid_input: String::new(),
            addr_tx_input: String::new(),
            wallet_label_input: String::new(),
            wallet_address_input: String::new(),
            addresses: Vec::new(),
            wallet_info: None,
            wallet_balance: None,
            created_wallet_address: None,
            blocks_all_data: None,
            block_by_hash_data: None,
            mining_info_data: None,
            generate_result: None,
            health_data: None,
            liveness_data: None,
            readiness_data: None,
            mempool_data: None,
            mempool_tx_data: None,
            transactions_data: None,
            address_transactions_data: None,
        }
    }
}

impl AdminApp {
    pub fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                menu: Menu::Blockchain,
                base_url: "http://127.0.0.1:8080".into(),
                api_key: std::env::var("BITCOIN_API_ADMIN_KEY")
                    .unwrap_or_else(|_| "admin-secret".into()),
                status: String::new(),
                info: None,
                blocks: Vec::new(),
                block_hash_input: String::new(),
                mining_address_input: String::new(),
                mining_nblocks_input: String::new(),
                mining_maxtries_input: String::new(),
                txid_input: String::new(),
                addr_tx_input: String::new(),
                wallet_label_input: String::new(),
                wallet_address_input: String::new(),
                addresses: Vec::new(),
                wallet_info: None,
                wallet_balance: None,
                created_wallet_address: None,
                blocks_all_data: None,
                block_by_hash_data: None,
                mining_info_data: None,
                generate_result: None,
                health_data: None,
                liveness_data: None,
                readiness_data: None,
                mempool_data: None,
                mempool_tx_data: None,
                transactions_data: None,
                address_transactions_data: None,
            },
            iced::Task::none(),
        )
    }

    /// Clear previously loaded data for a specific section
    pub fn clear_related_data(&mut self, section: DataSection) {
        match section {
            DataSection::BlockchainInfo => {
                self.blocks_all_data = None;
                self.block_by_hash_data = None;
            }
            DataSection::Blocks => {
                self.blocks_all_data = None;
                self.block_by_hash_data = None;
            }
            DataSection::BlocksAll => {
                self.block_by_hash_data = None;
            }
            DataSection::BlockByHash => {
                self.blocks_all_data = None;
            }
            DataSection::MiningInfo => {
                self.generate_result = None;
            }
            DataSection::Generate => {
                self.mining_info_data = None;
            }
            DataSection::Health => {
                self.liveness_data = None;
                self.readiness_data = None;
            }
            DataSection::Liveness => {
                self.health_data = None;
                self.readiness_data = None;
            }
            DataSection::Readiness => {
                self.health_data = None;
                self.liveness_data = None;
            }
            DataSection::Mempool => {
                self.mempool_tx_data = None;
                self.transactions_data = None;
                self.address_transactions_data = None;
            }
            DataSection::MempoolTx => {
                self.mempool_data = None;
                self.transactions_data = None;
                self.address_transactions_data = None;
            }
            DataSection::Transactions => {
                self.mempool_data = None;
                self.mempool_tx_data = None;
                self.address_transactions_data = None;
            }
            DataSection::AddressTransactions => {
                self.mempool_data = None;
                self.mempool_tx_data = None;
                self.transactions_data = None;
            }
            DataSection::WalletInfo => {
                self.wallet_balance = None;
            }
            DataSection::WalletBalance => {
                self.wallet_info = None;
            }
        }
    }
}

