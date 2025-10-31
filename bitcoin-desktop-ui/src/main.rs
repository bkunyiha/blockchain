use bitcoin_api::{AdminClient, ApiConfig, ApiResponse, BlockSummary, BlockchainInfo, CreateWalletRequest, CreateWalletResponse};
use iced::{
    Element, Task, Theme, application,
    widget::{button, column, pick_list, row, text, text_input, scrollable, container},
};
use serde_json::Value;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Menu {
    Blockchain,
    Wallet,
    Transactions,
    Mining,
    Health,
}

impl Menu {
    const ALL: [Menu; 5] = [
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
enum DataSection {
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

#[derive(Debug)]
struct AdminApp {
    menu: Menu,
    base_url: String,
    api_key: String,
    status: String,
    info: Option<BlockchainInfo>,
    blocks: Vec<BlockSummary>,
    // Inputs for actions
    block_hash_input: String,
    mining_address_input: String,
    mining_nblocks_input: String,
    mining_maxtries_input: String,
    txid_input: String,
    addr_tx_input: String,
    // Wallet admin state
    wallet_label_input: String,
    wallet_address_input: String,
    addresses: Vec<String>,
    wallet_info: Option<Value>,
    wallet_balance: Option<Value>,
    created_wallet_address: Option<String>,
    // Response data storage
    blocks_all_data: Option<Value>,
    block_by_hash_data: Option<Value>,
    mining_info_data: Option<Value>,
    generate_result: Option<Value>,
    health_data: Option<Value>,
    liveness_data: Option<Value>,
    readiness_data: Option<Value>,
    mempool_data: Option<Value>,
    mempool_tx_data: Option<Value>,
    transactions_data: Option<Value>,
    address_transactions_data: Option<Value>,
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
    /// Clear previously loaded data for a specific section
    fn clear_related_data(&mut self, section: DataSection) {
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

    fn new() -> (Self, Task<Message>) {
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
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MenuChanged(m) => {
                self.menu = m;
                Task::none()
            }
            Message::BaseUrlChanged(v) => {
                self.base_url = v;
                Task::none()
            }
            Message::ApiKeyChanged(v) => {
                self.api_key = v;
                Task::none()
            }
            Message::BlockHashChanged(v) => {
                self.block_hash_input = v;
                Task::none()
            }
            Message::MiningAddressChanged(v) => {
                self.mining_address_input = v;
                Task::none()
            }
            Message::MiningNBlocksChanged(v) => {
                self.mining_nblocks_input = v;
                Task::none()
            }
            Message::MiningMaxTriesChanged(v) => {
                self.mining_maxtries_input = v;
                Task::none()
            }
            Message::TxidChanged(v) => {
                self.txid_input = v;
                Task::none()
            }
            Message::AddrTxChanged(v) => {
                self.addr_tx_input = v;
                Task::none()
            }
            Message::WalletLabelChanged(v) => {
                self.wallet_label_input = v;
                Task::none()
            }
            Message::WalletAddressChanged(v) => {
                self.wallet_address_input = v;
                Task::none()
            }
            // Wallet admin operations
            Message::CreateWalletAdmin => {
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                let req = CreateWalletRequest {
                    label: if self.wallet_label_input.trim().is_empty() {
                        None
                    } else {
                        Some(self.wallet_label_input.trim().to_string())
                    },
                };
                Task::perform(spawn_on_tokio(create_wallet_admin(cfg, req)), Message::CreateWalletAdminDone)
            }
            Message::CreateWalletAdminDone(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.created_wallet_address = api.data.as_ref().map(|d| d.address.clone());
                            self.status = format!("Wallet created: {}", 
                                api.data.as_ref().map(|d| d.address.as_str()).unwrap_or("unknown"));
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error creating wallet".into());
                        }
                    }
                    Err(e) => {
                        self.status = e;
                    }
                }
                Task::none()
            }
            Message::FetchAddressesAdmin => {
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_addresses_admin(cfg)), Message::AddressesAdminLoaded)
            }
            Message::AddressesAdminLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            // Try to parse addresses from JSON response
                            if let Some(data) = api.data {
                                self.addresses = match serde_json::from_value::<Vec<String>>(data.clone()) {
                                    Ok(addrs) => addrs,
                                    Err(_) => {
                                        // Try parsing as array of objects with address field
                                        match serde_json::from_value::<Vec<serde_json::Map<String, Value>>>(data) {
                                            Ok(maps) => maps.iter()
                                                .filter_map(|m| m.get("address")
                                                    .and_then(|v| v.as_str())
                                                    .map(|s| s.to_string()))
                                                .collect(),
                                            Err(_) => Vec::new(),
                                        }
                                    }
                                };
                                self.status = format!("Loaded {} addresses", self.addresses.len());
                            } else {
                                self.status = "No addresses found".into();
                            }
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading addresses".into());
                        }
                    }
                    Err(e) => {
                        self.status = e;
                    }
                }
                Task::none()
            }
            Message::FetchWalletInfoAdmin(address) => {
                self.clear_related_data(DataSection::WalletInfo);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_wallet_info_admin(cfg, address)), Message::WalletInfoAdminLoaded)
            }
            Message::WalletInfoAdminLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.wallet_info = api.data;
                            self.status = "Wallet info loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading wallet info".into());
                            self.wallet_info = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.wallet_info = None;
                    }
                }
                Task::none()
            }
            Message::FetchBalanceAdmin(address) => {
                self.clear_related_data(DataSection::WalletBalance);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_balance_admin(cfg, address)), Message::BalanceAdminLoaded)
            }
            Message::BalanceAdminLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.wallet_balance = api.data;
                            self.status = "Balance loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading balance".into());
                            self.wallet_balance = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.wallet_balance = None;
                    }
                }
                Task::none()
            }
            Message::FetchInfo => {
                self.clear_related_data(DataSection::BlockchainInfo);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_info(cfg)), Message::InfoLoaded)
            }
            Message::FetchBlocks => {
                self.clear_related_data(DataSection::Blocks);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_blocks(cfg)), Message::BlocksLoaded)
            }
            Message::BlocksLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.blocks = api.data.unwrap_or_default();
                            self.status = format!("Loaded {} blocks", self.blocks.len());
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error".into());
                        }
                    }
                    Err(e) => {
                        self.status = e;
                    }
                }
                Task::none()
            }
            Message::InfoLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.info = api.data;
                            self.status = "Loaded blockchain info".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error".into());
                        }
                    }
                    Err(e) => {
                        self.status = e;
                    }
                }
                Task::none()
            }
            // Extra blockchain
            Message::FetchBlocksAll => {
                self.clear_related_data(DataSection::BlocksAll);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_blocks_all(cfg)), Message::BlocksAllLoaded)
            }
            Message::FetchBlockByHash(hash) => {
                self.clear_related_data(DataSection::BlockByHash);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_block_by_hash(cfg, hash)), Message::BlockByHashLoaded)
            }
            Message::BlocksAllLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.blocks_all_data = api.data;
                            self.status = "All blocks loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading blocks".into());
                            self.blocks_all_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.blocks_all_data = None;
                    }
                }
                Task::none()
            }
            Message::BlockByHashLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.block_by_hash_data = api.data;
                            self.status = "Block loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading block".into());
                            self.block_by_hash_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.block_by_hash_data = None;
                    }
                }
                Task::none()
            }
            // Mining
            Message::FetchMiningInfo => {
                self.clear_related_data(DataSection::MiningInfo);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_mining_info(cfg)), Message::MiningInfoLoaded)
            }
            Message::MiningInfoLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.mining_info_data = api.data;
                            self.status = "Mining info loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading mining info".into());
                            self.mining_info_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.mining_info_data = None;
                    }
                }
                Task::none()
            }
            Message::GenerateToAddressDone(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.generate_result = api.data;
                            self.status = "Blocks generated".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error generating blocks".into());
                            self.generate_result = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.generate_result = None;
                    }
                }
                Task::none()
            }
            Message::GenerateToAddress {
                address,
                nblocks,
                maxtries,
            } => {
                self.clear_related_data(DataSection::Generate);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(generate_to_address(cfg, address, nblocks, maxtries)), Message::GenerateToAddressDone)
            }
            // Health
            Message::FetchHealth => {
                self.clear_related_data(DataSection::Health);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_health(cfg)), Message::HealthLoaded)
            }
            // Liveness
            Message::FetchLiveness => {
                self.clear_related_data(DataSection::Liveness);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_liveness(cfg)), Message::LivenessLoaded)
            }
            Message::FetchReadiness => {
                self.clear_related_data(DataSection::Readiness);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_readiness(cfg)), Message::ReadinessLoaded)
            }
            Message::HealthLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.health_data = api.data;
                            self.status = "Health check loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading health".into());
                            self.health_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.health_data = None;
                    }
                }
                Task::none()
            }
            Message::LivenessLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.liveness_data = api.data;
                            self.status = "Liveness check loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading liveness".into());
                            self.liveness_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.liveness_data = None;
                    }
                }
                Task::none()
            }
            Message::ReadinessLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.readiness_data = api.data;
                            self.status = "Readiness check loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading readiness".into());
                            self.readiness_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.readiness_data = None;
                    }
                }
                Task::none()
            }
            // Transactions
            Message::FetchMempool => {
                self.clear_related_data(DataSection::Mempool);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_mempool(cfg)), Message::MempoolLoaded)
            }
            Message::FetchMempoolTx(txid) => {
                self.clear_related_data(DataSection::MempoolTx);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_mempool_tx(cfg, txid)), Message::MempoolTxLoaded)
            }
            Message::FetchTransactions => {
                self.clear_related_data(DataSection::Transactions);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_transactions(cfg)), Message::TransactionsLoaded)
            }
            Message::FetchAddressTransactions(address) => {
                self.clear_related_data(DataSection::AddressTransactions);
                let cfg = ApiConfig {
                    base_url: self.base_url.clone(),
                    api_key: Some(self.api_key.clone()),
                };
                Task::perform(spawn_on_tokio(fetch_address_transactions(cfg, address)), Message::AddressTransactionsLoaded)
            }
            Message::MempoolLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.mempool_data = api.data;
                            self.status = "Mempool loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading mempool".into());
                            self.mempool_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.mempool_data = None;
                    }
                }
                Task::none()
            }
            Message::MempoolTxLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.mempool_tx_data = api.data;
                            self.status = "Mempool transaction loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading mempool tx".into());
                            self.mempool_tx_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.mempool_tx_data = None;
                    }
                }
                Task::none()
            }
            Message::TransactionsLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.transactions_data = api.data;
                            self.status = "Transactions loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading transactions".into());
                            self.transactions_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.transactions_data = None;
                    }
                }
                Task::none()
            }
            Message::AddressTransactionsLoaded(res) => {
                match res {
                    Ok(api) => {
                        if api.success {
                            self.address_transactions_data = api.data;
                            self.status = "Address transactions loaded".into();
                        } else {
                            self.status = api.error.unwrap_or_else(|| "Error loading address transactions".into());
                            self.address_transactions_data = None;
                        }
                    }
                    Err(e) => {
                        self.status = e;
                        self.address_transactions_data = None;
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let toolbar = row![
            pick_list(&Menu::ALL[..], Some(self.menu), Message::MenuChanged),
            text_input("Base URL", &self.base_url)
                .on_input(Message::BaseUrlChanged)
                .width(250),
            text_input("Admin API Key", &self.api_key)
                .on_input(Message::ApiKeyChanged)
                .width(250),
        ]
        .spacing(10);

        let section: Element<Message> = match self.menu {
            Menu::Blockchain => {
                // Inputs
                let hash_input = text_input("Block hash", &self.block_hash_input)
                    .on_input(Message::BlockHashChanged)
                    .width(300);
                let info: Element<Message> = match &self.info {
                    Some(i) => text(format!(
                        "height: {} blocks:{} difficulty:{} last_block:{}",
                        i.height, i.total_blocks, i.difficulty, i.last_block_hash
                    ))
                    .into(),
                    None => text("no info").into(),
                };
                // Display latest blocks in a scrollable list
                let blocks_display: Element<Message> = if self.blocks.is_empty() {
                    text("No blocks loaded").into()
                } else {
                    container(
                        scrollable(
                            column(
                                self.blocks
                                    .iter()
                                    .map(|block| {
                                        text(format!(
                                            "Height: {} Hash: {} Txns: {}",
                                            block.height, block.hash, block.transaction_count
                                        ))
                                        .size(11)
                                        .width(iced::Length::Fill)
                                        .into()
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .spacing(5)
                            .width(iced::Length::Fill)
                        )
                        .height(iced::Length::Fixed(150.0))
                        .width(iced::Length::Fill)
                    )
                    .width(iced::Length::Fill)
                    .padding(8)
                    .into()
                };

                // Display all blocks data
                let all_blocks_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.blocks_all_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Display block by hash data
                let block_hash_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.block_by_hash_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                column![
                    row![
                        button("Get Info").on_press(Message::FetchInfo),
                        button("Latest Blocks").on_press(Message::FetchBlocks),
                        button("All Blocks").on_press(Message::FetchBlocksAll),
                    ]
                    .spacing(10),
                    info,
                    text("Latest Blocks").size(14),
                    blocks_display,
                    text("All Blocks").size(14),
                    all_blocks_display,
                    row![
                        hash_input,
                        button("Find by Hash")
                            .on_press(Message::FetchBlockByHash(self.block_hash_input.clone())),
                    ]
                    .spacing(10),
                    text("Block by Hash").size(14),
                    block_hash_display,
                ]
                .width(iced::Length::Fill)
                .spacing(8)
                .into()
            }
            Menu::Wallet => {
                // Create wallet section
                let create_section = column![
                    text("Create Wallet").size(16),
                    row![
                        text_input("Label (optional)", &self.wallet_label_input)
                            .on_input(Message::WalletLabelChanged)
                            .width(250),
                        button("Create Wallet").on_press(Message::CreateWalletAdmin),
                    ]
                    .spacing(10),
                    if let Some(addr) = &self.created_wallet_address {
                        text(format!("Created: {}", addr))
                    } else {
                        text("")
                    },
                ]
                .spacing(8);

                // List addresses section
                let addresses_section = column![
                    row![
                        text("Addresses").size(16),
                        button("Refresh").on_press(Message::FetchAddressesAdmin),
                    ]
                    .spacing(10),
                    {
                        let addresses_list: Element<Message> = if self.addresses.is_empty() {
                            text("No addresses loaded").into()
                        } else {
                            scrollable(
                                column(
                                    self.addresses
                                        .iter()
                                        .map(|addr| -> Element<Message> {
                                            row![
                                                text(addr).size(12),
                                                button("Info").on_press(Message::FetchWalletInfoAdmin(addr.clone())),
                                                button("Balance").on_press(Message::FetchBalanceAdmin(addr.clone())),
                                            ]
                                            .spacing(5)
                                            .into()
                                        })
                                        .collect::<Vec<_>>(),
                                )
                                .spacing(5)
                            )
                            .into()
                        };
                        addresses_list
                    },
                ]
                .spacing(8);

                // Query wallet section
                let query_section = column![
                    text("Query Wallet").size(16),
                    row![
                        text_input("Wallet Address", &self.wallet_address_input)
                            .on_input(Message::WalletAddressChanged)
                            .width(350),
                        button("Get Info").on_press(Message::FetchWalletInfoAdmin(
                            self.wallet_address_input.clone()
                        )),
                        button("Get Balance").on_press(Message::FetchBalanceAdmin(
                            self.wallet_address_input.clone()
                        )),
                    ]
                    .spacing(10),
                    {
                        let info_display: Element<Message> = if let Some(ref info) = self.wallet_info {
                            container(
                                text(format!("{}", serde_json::to_string_pretty(info).unwrap_or_else(|_| "Error formatting".into())))
                                    .size(11)
                            )
                            .padding(8)
                            .into()
                        } else {
                            text("").into()
                        };
                        info_display
                    },
                    {
                        let balance_display: Element<Message> = if let Some(ref balance) = self.wallet_balance {
                            container(
                                text(format!("{}", serde_json::to_string_pretty(balance).unwrap_or_else(|_| "Error formatting".into())))
                                    .size(11)
                            )
                            .padding(8)
                            .into()
                        } else {
                            text("").into()
                        };
                        balance_display
                    },
                ]
                .spacing(8);

                column![
                    create_section,
                    text("").size(1), // Spacer
                    addresses_section,
                    text("").size(1), // Spacer
                    query_section,
                ]
                .width(iced::Length::Fill)
                .spacing(15)
                .into()
            }
            Menu::Transactions => {
                let txid_input = text_input("Txid", &self.txid_input)
                    .on_input(Message::TxidChanged)
                    .width(300);
                let addr_input = text_input("Address", &self.addr_tx_input)
                    .on_input(Message::AddrTxChanged)
                    .width(300);
                
                // Mempool display
                let mempool_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.mempool_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Mempool transaction display
                let mempool_tx_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.mempool_tx_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Transactions display
                let transactions_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.transactions_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Address transactions display
                let addr_tx_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.address_transactions_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                column![
                    row![
                        button("Mempool").on_press(Message::FetchMempool),
                        txid_input,
                        button("Mempool Tx")
                            .on_press(Message::FetchMempoolTx(self.txid_input.clone())),
                        button("All Transactions").on_press(Message::FetchTransactions),
                    ]
                    .spacing(10),
                    text("Mempool").size(14),
                    mempool_display,
                    text("Mempool Transaction").size(14),
                    mempool_tx_display,
                    text("All Transactions").size(14),
                    transactions_display,
                    row![
                        addr_input,
                        button("Address Transactions").on_press(Message::FetchAddressTransactions(
                            self.addr_tx_input.clone()
                        )),
                    ]
                    .spacing(10),
                    text("Address Transactions").size(14),
                    addr_tx_display,
                ]
                .width(iced::Length::Fill)
                .spacing(8)
                .into()
            }
            Menu::Mining => {
                let addr = text_input("Mining reward address", &self.mining_address_input)
                    .on_input(Message::MiningAddressChanged)
                    .width(320);
                let nblocks = text_input("Blocks", &self.mining_nblocks_input)
                    .on_input(Message::MiningNBlocksChanged)
                    .width(100);
                let maxtries = text_input("Max tries (opt)", &self.mining_maxtries_input)
                    .on_input(Message::MiningMaxTriesChanged)
                    .width(140);
                let parsed_nblocks = self.mining_nblocks_input.trim().parse::<u32>().unwrap_or(1);
                let parsed_maxtries = match self.mining_maxtries_input.trim() {
                    "" => None,
                    s => s.parse::<u32>().ok(),
                };

                // Mining info display
                let mining_info_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.mining_info_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Generate result display
                let generate_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.generate_result {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(150.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                column![
                    row![button("Mining Info").on_press(Message::FetchMiningInfo),].spacing(10),
                    text("Mining Info").size(14),
                    mining_info_display,
                    row![
                        addr,
                        nblocks,
                        maxtries,
                        button("Generate").on_press(Message::GenerateToAddress {
                            address: self.mining_address_input.clone(),
                            nblocks: parsed_nblocks,
                            maxtries: parsed_maxtries
                        }),
                    ]
                    .spacing(10),
                    text("Generate Result").size(14),
                    generate_display,
                ]
                .width(iced::Length::Fill)
                .spacing(8)
                .into()
            }
            Menu::Health => {
                // Health display
                let health_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.health_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(200.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Liveness display
                let liveness_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.liveness_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(150.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                // Readiness display
                let readiness_display: Element<Message> = {
                    let content: Element<Message> = if let Some(ref data) = self.readiness_data {
                        container(
                            scrollable(
                                text(serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into()))
                                    .size(10)
                                    .width(iced::Length::Fill)
                            )
                            .height(iced::Length::Fixed(150.0))
                            .width(iced::Length::Fill)
                        )
                        .padding(8)
                        .width(iced::Length::Fill)
                        .into()
                    } else {
                        text("").into()
                    };
                    content
                };

                column![
                    row![
                        button("Health").on_press(Message::FetchHealth),
                        button("Liveness").on_press(Message::FetchLiveness),
                        button("Readiness").on_press(Message::FetchReadiness),
                    ]
                    .spacing(10),
                    text("Health Check").size(14),
                    health_display,
                    text("Liveness Check").size(14),
                    liveness_display,
                    text("Readiness Check").size(14),
                    readiness_display,
                ]
                .width(iced::Length::Fill)
                .spacing(8)
                .into()
            }
        };

        column![toolbar, text(&self.status), section]
            .spacing(12)
            .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
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
}

async fn fetch_info(cfg: ApiConfig) -> Result<ApiResponse<BlockchainInfo>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_blockchain_info()
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_blocks(cfg: ApiConfig) -> Result<ApiResponse<Vec<BlockSummary>>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_latest_blocks().await.map_err(|e| e.to_string())
}

// Additional admin endpoints
async fn fetch_blocks_all(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_blocks().await.map_err(|e| e.to_string())
}

async fn fetch_block_by_hash(cfg: ApiConfig, hash: String) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_block_by_hash(&hash)
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_mining_info(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_mining_info().await.map_err(|e| e.to_string())
}

async fn generate_to_address(
    cfg: ApiConfig,
    address: String,
    nblocks: u32,
    maxtries: Option<u32>,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .generate_to_address(&address, nblocks, maxtries)
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_health(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.health().await.map_err(|e| e.to_string())
}

async fn fetch_liveness(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.liveness().await.map_err(|e| e.to_string())
}

async fn fetch_readiness(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.readiness().await.map_err(|e| e.to_string())
}

async fn fetch_mempool(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_mempool().await.map_err(|e| e.to_string())
}

async fn fetch_mempool_tx(cfg: ApiConfig, txid: String) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_mempool_transaction(&txid)
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_transactions(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_transactions().await.map_err(|e| e.to_string())
}

async fn fetch_address_transactions(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_address_transactions(&address)
        .await
        .map_err(|e| e.to_string())
}

// Wallet admin functions
async fn create_wallet_admin(
    cfg: ApiConfig,
    req: CreateWalletRequest,
) -> Result<ApiResponse<CreateWalletResponse>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .create_wallet_admin(&req)
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_addresses_admin(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_addresses_admin()
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_wallet_info_admin(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_wallet_info_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_balance_admin(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_balance_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

// Store the Tokio runtime handle globally so tasks can access it
static TOKIO_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

// Helper function to wrap a future to ensure it runs on Tokio runtime
fn spawn_on_tokio<F>(fut: F) -> impl std::future::Future<Output = F::Output> + Send
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    let handle = TOKIO_HANDLE.get().expect("Tokio runtime not initialized").clone();
    async move {
        handle.spawn(fut).await.unwrap()
    }
}

fn main() -> iced::Result {
    // Create a Tokio runtime for async operations
    // This must outlive the application to keep the reactor running
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime");
    
    // Store the handle globally so it can be accessed from any thread
    TOKIO_HANDLE.set(rt.handle().clone())
        .expect("Failed to set Tokio handle");
    
    // Keep the runtime alive in a background thread
    std::thread::spawn(move || {
        rt.block_on(async {
            // Keep the runtime alive indefinitely
            std::future::pending::<()>().await;
        });
    });
    
    // Run the application
    application("Bitcoin Admin UI", AdminApp::update, AdminApp::view)
        .theme(|_| Theme::Dark)
        .run_with(AdminApp::new)
}
