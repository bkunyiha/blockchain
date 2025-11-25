use crate::app::AdminApp;
use crate::api::*;
use crate::runtime::spawn_on_tokio;
use crate::types::{DataSection, Message, Menu};
use bitcoin_api::{ApiConfig, CreateWalletRequest, SendTransactionRequest};
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
        Message::WalletSectionChanged(section) => {
            app.wallet_section = section;
            app.menu = Menu::Wallet; // Switch to Wallet menu to show the content
            app.wallet_menu_hovered = false; // Close popup when section is selected
            Task::none()
        }
        Message::TransactionSectionChanged(section) => {
            app.transaction_section = section;
            app.menu = Menu::Transactions; // Switch to Transactions menu to show the content
            app.transaction_menu_hovered = false; // Close popup when section is selected
            Task::none()
        }
        Message::BlockchainSectionChanged(section) => {
            app.blockchain_section = section;
            app.menu = Menu::Blockchain; // Switch to Blockchain menu to show the content
            app.blockchain_menu_hovered = false; // Close popup when section is selected
            Task::none()
        }
        Message::BlockchainMenuHovered(hovered) => {
            app.blockchain_menu_hovered = hovered;
            Task::none()
        }
        Message::WalletMenuHovered(hovered) => {
            app.wallet_menu_hovered = hovered;
            Task::none()
        }
        Message::TransactionMenuHovered(hovered) => {
            app.transaction_menu_hovered = hovered;
            Task::none()
        }
        Message::MiningSectionChanged(section) => {
            app.mining_section = section;
            app.menu = Menu::Mining; // Switch to Mining menu to show the content
            app.mining_menu_hovered = false; // Close popup when section is selected
            Task::none()
        }
        Message::MiningMenuHovered(hovered) => {
            app.mining_menu_hovered = hovered;
            Task::none()
        }
        Message::HealthSectionChanged(section) => {
            app.health_section = section;
            app.menu = Menu::Health; // Switch to Health menu to show the content
            app.health_menu_hovered = false; // Close popup when section is selected
            Task::none()
        }
        Message::HealthMenuHovered(hovered) => {
            app.health_menu_hovered = hovered;
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
        // Send transaction handlers
        Message::SendFromChanged(v) => {
            app.send_from_address = v;
            Task::none()
        }
        Message::SendToChanged(v) => {
            app.send_to_address = v;
            Task::none()
        }
        Message::SendAmountChanged(v) => {
            app.send_amount = v;
            Task::none()
        }
        Message::SendTx => {
            let amount_sat = app.send_amount.trim().parse::<u64>().unwrap_or(0);
            if amount_sat == 0 {
                app.status = "Amount must be greater than 0".into();
                return Task::none();
            }
            if app.send_from_address.trim().is_empty() || app.send_to_address.trim().is_empty() {
                app.status = "From and To addresses are required".into();
                return Task::none();
            }
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            let req = SendTransactionRequest {
                from_address: app.send_from_address.clone(),
                to_address: app.send_to_address.clone(),
                amount_satoshis: amount_sat,
            };
            Task::perform(spawn_on_tokio(send_transaction(cfg, req)), Message::TxSent)
        }
        Message::TxSent(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.last_txid = api.data.as_ref().map(|d| d.txid.clone());
                        app.status = format!("Transaction sent: {}", 
                            api.data.as_ref().map(|d| d.txid.as_str()).unwrap_or("unknown"));
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error sending transaction".into());
                        app.last_txid = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.last_txid = None;
                }
            }
            Task::none()
        }
        // Transaction history handlers
        Message::HistoryAddressChanged(v) => {
            app.history_address = v;
            Task::none()
        }
        Message::FetchTransactionHistory(address) => {
            let cfg = ApiConfig {
                base_url: app.base_url.clone(),
                api_key: Some(app.api_key.clone()),
            };
            Task::perform(spawn_on_tokio(fetch_address_transactions(cfg, address)), Message::TransactionHistoryLoaded)
        }
        Message::TransactionHistoryLoaded(res) => {
            match res {
                Ok(api) => {
                    if api.success {
                        app.transaction_history = api.data;
                        app.status = "Transaction history loaded".into();
                    } else {
                        app.status = api.error.unwrap_or_else(|| "Error loading transaction history".into());
                        app.transaction_history = None;
                    }
                }
                Err(e) => {
                    app.status = e;
                    app.transaction_history = None;
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
        Message::CopyToClipboard(text) => {
            let text_clone = text.clone();
            app.status = "Copying to clipboard...".to_string();
            Task::perform(
                async move {
                    let mut clipboard = arboard::Clipboard::new().ok()?;
                    clipboard.set_text(text_clone).ok()?;
                    Some(())
                },
                |result| Message::ClipboardCopied(result.is_some())
            )
        }
        Message::ClipboardCopied(success) => {
            app.status = if success {
                "✓ Copied to clipboard!".to_string()
            } else {
                "✗ Failed to copy to clipboard".to_string()
            };
            Task::none()
        }
    }
}

