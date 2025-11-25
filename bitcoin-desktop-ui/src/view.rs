use crate::app::AdminApp;
use crate::types::{Menu, Message};
use iced::Element;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};

pub fn view(app: &AdminApp) -> Element<Message> {
    let toolbar = row![
        pick_list(&Menu::ALL[..], Some(app.menu), Message::MenuChanged),
        text_input("Base URL", &app.base_url)
            .on_input(Message::BaseUrlChanged)
            .width(250),
        text_input("Admin API Key", &app.api_key)
            .on_input(Message::ApiKeyChanged)
            .width(250),
    ]
    .spacing(10);

    let section: Element<Message> = match app.menu {
        Menu::Blockchain => view_blockchain(app),
        Menu::Wallet => view_wallet(app),
        Menu::Transactions => view_transactions(app),
        Menu::Mining => view_mining(app),
        Menu::Health => view_health(app),
    };

    column![toolbar, text(&app.status), section]
        .spacing(12)
        .into()
}

fn view_blockchain(app: &AdminApp) -> Element<Message> {
    let hash_input = text_input("Block hash", &app.block_hash_input)
        .on_input(Message::BlockHashChanged)
        .width(300);
    let info: Element<Message> = match &app.info {
        Some(i) => text(format!(
            "height: {} blocks:{} difficulty:{} last_block:{}",
            i.height, i.total_blocks, i.difficulty, i.last_block_hash
        ))
        .into(),
        None => text("no info").into(),
    };
    // Display latest blocks in a scrollable list
    let blocks_display: Element<Message> = if app.blocks.is_empty() {
        text("No blocks loaded").into()
    } else {
        container(
            scrollable(
                column(
                    app.blocks
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
    let all_blocks_display = json_data_display(&app.blocks_all_data, 200.0);

    // Display block by hash data
    let block_hash_display = json_data_display(&app.block_by_hash_data, 200.0);

    column![
        row![
            button("Get Block Info").on_press(Message::FetchInfo),
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
                .on_press(Message::FetchBlockByHash(app.block_hash_input.clone())),
        ]
        .spacing(10),
        text("Block by Hash").size(14),
        block_hash_display,
    ]
    .width(iced::Length::Fill)
    .spacing(8)
    .into()
}

fn view_wallet(app: &AdminApp) -> Element<Message> {
    // Create wallet section
    let create_section = column![
        text("Create Wallet").size(16),
        row![
            text_input("Label (optional)", &app.wallet_label_input)
                .on_input(Message::WalletLabelChanged)
                .width(250),
            button("Create Wallet").on_press(Message::CreateWalletAdmin),
        ]
        .spacing(10),
        if let Some(addr) = &app.created_wallet_address {
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
            let addresses_list: Element<Message> = if app.addresses.is_empty() {
                text("No addresses loaded").into()
            } else {
                scrollable(
                    column(
                        app.addresses
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
            text_input("Wallet Address", &app.wallet_address_input)
                .on_input(Message::WalletAddressChanged)
                .width(350),
            button("Get Wallet Info").on_press(Message::FetchWalletInfoAdmin(
                app.wallet_address_input.clone()
            )),
            button("Get Balance").on_press(Message::FetchBalanceAdmin(
                app.wallet_address_input.clone()
            )),
        ]
        .spacing(10),
        {
            let info_display: Element<Message> = if let Some(ref info) = app.wallet_info {
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
            let balance_display: Element<Message> = if let Some(ref balance) = app.wallet_balance {
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

fn view_transactions(app: &AdminApp) -> Element<Message> {
    let txid_input = text_input("Txid", &app.txid_input)
        .on_input(Message::TxidChanged)
        .width(300);
    let addr_input = text_input("Address", &app.addr_tx_input)
        .on_input(Message::AddrTxChanged)
        .width(300);
    
    let mempool_display = json_data_display(&app.mempool_data, 200.0);
    let mempool_tx_display = json_data_display(&app.mempool_tx_data, 200.0);
    let transactions_display = json_data_display(&app.transactions_data, 200.0);
    let addr_tx_display = json_data_display(&app.address_transactions_data, 200.0);

    column![
        row![
            button("Mempool").on_press(Message::FetchMempool),
            txid_input,
            button("Mempool Tx")
                .on_press(Message::FetchMempoolTx(app.txid_input.clone())),
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
            button("Wallet Address Transactions").on_press(Message::FetchAddressTransactions(
                app.addr_tx_input.clone()
            )),
        ]
        .spacing(10),
        text("Wallet Address Transactions").size(14),
        addr_tx_display,
    ]
    .width(iced::Length::Fill)
    .spacing(8)
    .into()
}

fn view_mining(app: &AdminApp) -> Element<Message> {
    let addr = text_input("Mining reward address", &app.mining_address_input)
        .on_input(Message::MiningAddressChanged)
        .width(320);
    let nblocks = text_input("Blocks", &app.mining_nblocks_input)
        .on_input(Message::MiningNBlocksChanged)
        .width(100);
    let maxtries = text_input("Max tries (opt)", &app.mining_maxtries_input)
        .on_input(Message::MiningMaxTriesChanged)
        .width(140);
    let parsed_nblocks = app.mining_nblocks_input.trim().parse::<u32>().unwrap_or(1);
    let parsed_maxtries = match app.mining_maxtries_input.trim() {
        "" => None,
        s => s.parse::<u32>().ok(),
    };

    let mining_info_display = json_data_display(&app.mining_info_data, 200.0);
    let generate_display = json_data_display(&app.generate_result, 150.0);

    column![
        row![button("Mining Info").on_press(Message::FetchMiningInfo),].spacing(10),
        text("Mining Info").size(14),
        mining_info_display,
        row![
            addr,
            nblocks,
            maxtries,
            button("Generate").on_press(Message::GenerateToAddress {
                address: app.mining_address_input.clone(),
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

fn view_health(app: &AdminApp) -> Element<Message> {
    let health_display = json_data_display(&app.health_data, 200.0);
    let liveness_display = json_data_display(&app.liveness_data, 150.0);
    let readiness_display = json_data_display(&app.readiness_data, 150.0);

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

// Helper function to create a scrollable JSON data display with copy functionality
fn json_data_display(data: &Option<serde_json::Value>, height: f32) -> Element<Message> {
    if let Some(data) = data {
        let json_string = serde_json::to_string_pretty(data).unwrap_or_else(|_| "Error formatting".into());
        column![
            row![
                button("📋 Copy")
                    .on_press(Message::CopyToClipboard(json_string.clone())),
            ]
            .spacing(8),
            container(
                scrollable(
                    // Use text widget for proper scrolling - text is selectable on most platforms
                    text(json_string.clone())
                        .size(10)
                        .width(iced::Length::Fill)
                )
                .height(iced::Length::Fixed(height))
                .width(iced::Length::Fill)
            )
            .padding(8)
            .width(iced::Length::Fill)
        ]
        .spacing(4)
        .width(iced::Length::Fill)
        .into()
    } else {
        text("").into()
    }
}

