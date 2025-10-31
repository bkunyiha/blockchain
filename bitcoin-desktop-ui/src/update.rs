use crate::app::AdminApp;
use crate::api::*;
use crate::runtime::spawn_on_tokio;
use crate::types::{DataSection, Message};
use bitcoin_api::{ApiConfig, CreateWalletRequest};
use iced::Task;
use serde_json::Value;

pub fn update(app: &mut AdminApp, message: Message) -> Task<Message> {
    match message {
        Message::MenuChanged(m) => {
            app.menu = m;
            Task::none()
        }
        Message::BaseUrlChanged(v) => {
            app.base_url = v;
            Task::none()
        }
        Message::ApiKeyChanged(v) => {
            app.api_key = v;
            Task::none()
        }
        Message::BlockHashChanged(v) => {
            app.block_hash_input = v;
            Task::none()
        }
        Message::MiningAddressChanged(v) => {
            app.mining_address_input = v;
            Task::none()
        }
        Message::MiningNBlocksChanged(v) => {
            app.mining_nblocks_input = v;
            Task::none()
        }
        Message::MiningMaxTriesChanged(v) => {
            app.mining_maxtries_input = v;
            Task::none()
        }
        Message::TxidChanged(v) => {
            app.txid_input = v;
            Task::none()
        }
        Message::AddrTxChanged(v) => {
            app.addr_tx_input = v;
            Task::none()
        }
        Message::WalletLabelChanged(v) => {
            app.wallet_label_input = v;
            Task::none()
        }
        Message::WalletAddressChanged(v) => {
            app.wallet_address_input = v;
            Task::none()
        }
        // Wallet admin operations
        Message::CreateWalletAdmin => {
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            let req = CreateWalletRequest {
                label: if app.wallet_label_input.trim().is_empty() {
                    None
                } else {
                    Some(app.wallet_label_input.trim().to_string())
                },
            };
            Task::perform(spawn_on_tokio(create_wallet_admin(cfg, req)), Message::CreateWalletAdminDone)
        }
        Message::CreateWalletAdminDone(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.created_wallet_address = api.data.as_ref().map(|d| d.address.clone());
                        app.status = format!("Wallet created: {}", 
                            api.data.as_ref().map(|d| d.address.as_str()).unwrap_or("unknown"));
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error creating wallet".into());
                    }
                }
                Err(e) => {
                    app.status = e;
                }
            }
            Task::none()
        }
        Message::FetchAddressesAdmin => {
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_addresses_admin(cfg)), Message::AddressesAdminLoaded)
        }
        Message::AddressesAdminLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        // Try to parse addresses from JSON response
                        if let Some(data) = api.data {
                            app.addresses = match serde_json::from_value::<Vec<String>>(data.clone()) {
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
                            app.status = format!("Loaded {} addresses", app.addresses.len());
                        } else {
                            app.status = "No addresses found".into();
                        }
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading addresses".into());
                    }
                }
                Err(e) => {
                    app.status = e;
                }
            }
            Task::none()
        }
        Message::FetchWalletInfoAdmin(address) => {
            app.clear_related_data(DataSection::WalletInfo);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_wallet_info_admin(cfg, address)), Message::WalletInfoAdminLoaded)
        }
        Message::WalletInfoAdminLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.wallet_info = api.data;
                        app.status = "Wallet info loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading wallet info".into());
                        app.wallet_info = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.wallet_info = None;
                }
            }
            Task::none()
        }
        Message::FetchBalanceAdmin(address) => {
            app.clear_related_data(DataSection::WalletBalance);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_balance_admin(cfg, address)), Message::BalanceAdminLoaded)
        }
        Message::BalanceAdminLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.wallet_balance = api.data;
                        app.status = "Balance loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading balance".into());
                        app.wallet_balance = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.wallet_balance = None;
                }
            }
            Task::none()
        }
        Message::FetchInfo => {
            app.clear_related_data(DataSection::BlockchainInfo);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_info(cfg)), Message::InfoLoaded)
        }
        Message::FetchBlocks => {
            app.clear_related_data(DataSection::Blocks);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_blocks(cfg)), Message::BlocksLoaded)
        }
        Message::BlocksLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.blocks = api.data.unwrap_or_default();
                        app.status = format!("Loaded {} blocks", app.blocks.len());
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error".into());
                    }
                }
                Err(e) => {
                    app.status = e;
                }
            }
            Task::none()
        }
        Message::InfoLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.info = api.data;
                        app.status = "Loaded blockchain info".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error".into());
                    }
                }
                Err(e) => {
                    app.status = e;
                }
            }
            Task::none()
        }
        // Extra blockchain
        Message::FetchBlocksAll => {
            app.clear_related_data(DataSection::BlocksAll);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_blocks_all(cfg)), Message::BlocksAllLoaded)
        }
        Message::FetchBlockByHash(hash) => {
            app.clear_related_data(DataSection::BlockByHash);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_block_by_hash(cfg, hash)), Message::BlockByHashLoaded)
        }
        Message::BlocksAllLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.blocks_all_data = api.data;
                        app.status = "All blocks loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading blocks".into());
                        app.blocks_all_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.blocks_all_data = None;
                }
            }
            Task::none()
        }
        Message::BlockByHashLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.block_by_hash_data = api.data;
                        app.status = "Block loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading block".into());
                        app.block_by_hash_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.block_by_hash_data = None;
                }
            }
            Task::none()
        }
        // Mining
        Message::FetchMiningInfo => {
            app.clear_related_data(DataSection::MiningInfo);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_mining_info(cfg)), Message::MiningInfoLoaded)
        }
        Message::MiningInfoLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.mining_info_data = api.data;
                        app.status = "Mining info loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading mining info".into());
                        app.mining_info_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.mining_info_data = None;
                }
            }
            Task::none()
        }
        Message::GenerateToAddressDone(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.generate_result = api.data;
                        app.status = "Blocks generated".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error generating blocks".into());
                        app.generate_result = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.generate_result = None;
                }
            }
            Task::none()
        }
        Message::GenerateToAddress {
            address,
            nblocks,
            maxtries,
        } => {
            app.clear_related_data(DataSection::Generate);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(generate_to_address(cfg, address, nblocks, maxtries)), Message::GenerateToAddressDone)
        }
        // Health
        Message::FetchHealth => {
            app.clear_related_data(DataSection::Health);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_health(cfg)), Message::HealthLoaded)
        }
        // Liveness
        Message::FetchLiveness => {
            app.clear_related_data(DataSection::Liveness);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_liveness(cfg)), Message::LivenessLoaded)
        }
        Message::FetchReadiness => {
            app.clear_related_data(DataSection::Readiness);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_readiness(cfg)), Message::ReadinessLoaded)
        }
        Message::HealthLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.health_data = api.data;
                        app.status = "Health check loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading health".into());
                        app.health_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.health_data = None;
                }
            }
            Task::none()
        }
        Message::LivenessLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.liveness_data = api.data;
                        app.status = "Liveness check loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading liveness".into());
                        app.liveness_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.liveness_data = None;
                }
            }
            Task::none()
        }
        Message::ReadinessLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.readiness_data = api.data;
                        app.status = "Readiness check loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading readiness".into());
                        app.readiness_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.readiness_data = None;
                }
            }
            Task::none()
        }
        // Transactions
        Message::FetchMempool => {
            app.clear_related_data(DataSection::Mempool);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_mempool(cfg)), Message::MempoolLoaded)
        }
        Message::FetchMempoolTx(txid) => {
            app.clear_related_data(DataSection::MempoolTx);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_mempool_tx(cfg, txid)), Message::MempoolTxLoaded)
        }
        Message::FetchTransactions => {
            app.clear_related_data(DataSection::Transactions);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_transactions(cfg)), Message::TransactionsLoaded)
        }
        Message::FetchAddressTransactions(address) => {
            app.clear_related_data(DataSection::AddressTransactions);
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_address_transactions(cfg, address)), Message::AddressTransactionsLoaded)
        }
        Message::MempoolLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.mempool_data = api.data;
                        app.status = "Mempool loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading mempool".into());
                        app.mempool_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.mempool_data = None;
                }
            }
            Task::none()
        }
        Message::MempoolTxLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.mempool_tx_data = api.data;
                        app.status = "Mempool transaction loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading mempool tx".into());
                        app.mempool_tx_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.mempool_tx_data = None;
                }
            }
            Task::none()
        }
        Message::TransactionsLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.transactions_data = api.data;
                        app.status = "Transactions loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading transactions".into());
                        app.transactions_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.transactions_data = None;
                }
            }
            Task::none()
        }
        Message::AddressTransactionsLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.address_transactions_data = api.data;
                        app.status = "Address transactions loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading address transactions".into());
                        app.address_transactions_data = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.address_transactions_data = None;
                }
            }
            Task::none()
        }
    }
}

