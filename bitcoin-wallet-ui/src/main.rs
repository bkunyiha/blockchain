use iced::{application, Element, Task, Theme, widget::{column, row, button, text, text_input, pick_list, container, Space}};
use bitcoin_api::{WalletClient, ApiConfig, ApiResponse, CreateWalletRequest, CreateWalletResponse, SendTransactionRequest, SendTransactionResponse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Menu {
    Wallet,
    Send,
    History,
    Settings,
}

impl Menu {
    const ALL: [Menu; 4] = [Menu::Wallet, Menu::Send, Menu::History, Menu::Settings];
}

impl core::fmt::Display for Menu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Menu::Wallet => "Wallet",
            Menu::Send => "Send",
            Menu::History => "History",
            Menu::Settings => "Settings",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
enum Message {
    MenuChanged(Menu),
    BaseUrlChanged(String),
    ApiKeyChanged(String),
    FromChanged(String),
    ToChanged(String),
    AmountChanged(String),
    CreateWallet,
    SendTx,
    WalletCreated(Result<ApiResponse<CreateWalletResponse>, String>),
    TxSent(Result<ApiResponse<SendTransactionResponse>, String>),
}

#[derive(Debug)]
struct WalletApp {
    menu: Menu,
    base_url: String,
    api_key: String,
    from: String,
    to: String,
    amount: String,
    status: String,
    new_address: Option<String>,
    last_txid: Option<String>,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self {
            menu: Menu::Wallet,
            base_url: String::new(),
            api_key: String::new(),
            from: String::new(),
            to: String::new(),
            amount: String::new(),
            status: String::new(),
            new_address: None,
            last_txid: None,
        }
    }
}

impl WalletApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                menu: Menu::Wallet,
                base_url: "http://127.0.0.1:8080".into(),
                api_key: std::env::var("BITCOIN_API_WALLET_KEY").unwrap_or_else(|_| "wallet-secret".into()),
                from: String::new(),
                to: String::new(),
                amount: String::new(),
                status: String::new(),
                new_address: None,
                last_txid: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MenuChanged(m) => { self.menu = m; Task::none() }
            Message::BaseUrlChanged(v) => { self.base_url = v; Task::none() }
            Message::ApiKeyChanged(v) => { self.api_key = v; Task::none() }
            Message::FromChanged(v) => { self.from = v; Task::none() }
            Message::ToChanged(v) => { self.to = v; Task::none() }
            Message::AmountChanged(v) => { self.amount = v; Task::none() }
            Message::CreateWallet => {
                let cfg = ApiConfig { base_url: self.base_url.clone(), api_key: Some(self.api_key.clone()) };
                let req = CreateWalletRequest { label: None };
                Task::perform(create_wallet(cfg, req), Message::WalletCreated)
            }
            Message::SendTx => {
                let amount_sat = self.amount.trim().parse::<u64>().unwrap_or(0);
                let cfg = ApiConfig { base_url: self.base_url.clone(), api_key: Some(self.api_key.clone()) };
                let req = SendTransactionRequest { from_address: self.from.clone(), to_address: self.to.clone(), amount_satoshis: amount_sat };
                Task::perform(send_tx(cfg, req), Message::TxSent)
            }
            Message::WalletCreated(res) => {
                match res {
                    Ok(api) => {
                        if api.success { self.new_address = api.data.map(|d| d.address); self.status = "Wallet created".into(); }
                        else { self.status = api.error.unwrap_or_else(|| "Error".into()); }
                    }
                    Err(e) => { self.status = e; }
                }
                Task::none()
            }
            Message::TxSent(res) => {
                match res {
                    Ok(api) => {
                        if api.success { self.last_txid = api.data.map(|d| d.txid); self.status = "Transaction sent".into(); }
                        else { self.status = api.error.unwrap_or_else(|| "Error".into()); }
                    }
                    Err(e) => { self.status = e; }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let toolbar = row![
            pick_list(&Menu::ALL[..], Some(self.menu), Message::MenuChanged),
            Space::with_width(iced::Length::Fill),
            text("Bitcoin Wallet").size(20),
            Space::with_width(iced::Length::Fill),
        ].spacing(10);

        let section: Element<Message> = match self.menu {
            Menu::Wallet => {
                let addr = self.new_address.as_deref().unwrap_or("<none>");
                let addr_el: Element<Message> = text(format!("Address: {}", addr)).into();
                column![
                    text("Wallet Management").size(18),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    button("Create New Wallet").on_press(Message::CreateWallet),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    addr_el,
                ].spacing(8).into()
            }
            Menu::Send => {
                column![
                    text("Send Bitcoin").size(18),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    text_input("From Address", &self.from).on_input(Message::FromChanged).width(300),
                    text_input("To Address", &self.to).on_input(Message::ToChanged).width(300),
                    text_input("Amount (satoshis)", &self.amount).on_input(Message::AmountChanged).width(200),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    button("Send Transaction").on_press(Message::SendTx),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    text(format!("Last TX: {}", self.last_txid.as_deref().unwrap_or("<none>"))),
                ].spacing(8).into()
            }
            Menu::History => {
                column![
                    text("Transaction History").size(18),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    text("History view (coming soon)"),
                ].spacing(8).into()
            }
            Menu::Settings => {
                column![
                    text("Settings").size(18),
                    Space::with_height(iced::Length::Fixed(10.0)),
                    text_input("Base URL", &self.base_url).on_input(Message::BaseUrlChanged).width(300),
                    text_input("API Key", &self.api_key).on_input(Message::ApiKeyChanged).width(300),
                ].spacing(8).into()
            }
        };

        let status_bar: Element<Message> = if !self.status.is_empty() {
            container(text(&self.status)).padding(8).into()
        } else {
            Space::with_height(iced::Length::Fixed(0.0)).into()
        };

        column![
            toolbar,
            Space::with_height(iced::Length::Fixed(20.0)),
            section,
            Space::with_height(iced::Length::Fill),
            status_bar,
        ].spacing(12).into()
    }
}

async fn create_wallet(cfg: ApiConfig, req: CreateWalletRequest) -> Result<ApiResponse<CreateWalletResponse>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client.create_wallet(&req).await.map_err(|e| e.to_string())
}

async fn send_tx(cfg: ApiConfig, req: SendTransactionRequest) -> Result<ApiResponse<SendTransactionResponse>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client.send_transaction(&req).await.map_err(|e| e.to_string())
}

fn main() -> iced::Result {
    application("Bitcoin Wallet UI", WalletApp::update, WalletApp::view)
        .theme(|_| Theme::Dark)
        .run_with(WalletApp::new)
}
