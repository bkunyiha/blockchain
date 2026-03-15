<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. **Chapter 4: Desktop Admin Interface** ← *You are here*
17. <a href="../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---

# Chapter 4.A — Desktop Admin Interface: Boot, Runtime, Types, State, and API Client

This walkthrough prints the “supporting scaffolding” for the desktop admin UI:

- how the app boots (`main.rs`)
- how we run async work (`runtime.rs`)
- how we model navigation and events (`types.rs`)
- what state we render (`app.rs`)
- how we call the admin API (`api.rs`)

> **Methods involved**
>
> - `main`
> - `init_runtime`, `spawn_on_tokio`
> - `AdminApp::{new, clear_related_data}`
> - all `Message` variants
> - `fetch_*` / `create_wallet_admin` / `send_transaction` API helpers

---

## Diagram: file-to-responsibility map (what lives where)

```
bitcoin-desktop-ui-iced/src/
  main.rs      -> boot (logging, runtime, iced application)
  runtime.rs   -> Tokio runtime kept alive in a background thread
  types.rs     -> navigation enums + Message (the “event vocabulary”)
  app.rs       -> AdminApp state (the “model” in MVU)
  api.rs       -> async wrappers around AdminClient HTTP calls
  update.rs    -> message dispatcher (sync), spawns async work and applies results
  view.rs      -> rendering (pure-ish), emits Message on user interaction
```

> **Methods involved**
>
> - `main`
> - `init_runtime`, `spawn_on_tokio`
> - `Message` (event vocabulary)

---

## How to read this chapter (so the code “clicks”)

- **If you’re new to Iced**: read `main.rs` and `runtime.rs` first, then jump to Chapter 4.B (`update.rs`) to see how messages trigger async calls.
- **If you already know MVU**: skim `main.rs`, then focus on `types.rs` (`Message`) and `app.rs` (`AdminApp`)—that’s the “contract” between update and view.
- **If you’re debugging an API call**: start at `api.rs`, then find where the matching `Message::FetchXxx` / `Message::XxxLoaded` is handled in Chapter 4.B.

## Mental model (one sentence)

This UI is a synchronous MVU loop (Iced) that **outsources I/O to Tokio**, and only ever changes state by handling a `Message`.

---

## Code Listing 4A-1.1 — App entrypoint (`bitcoin-desktop-ui-iced/src/main.rs`)

```rust
mod api;
mod app;
mod runtime;
mod types;
mod update;
mod view;

use app::AdminApp;
use iced::{Theme, application};
use runtime::init_runtime;
use update::update;
use view::view;

fn main() -> iced::Result {
    // 1) Configure structured logging (controlled by `RUST_LOG`).
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // 2) Start a Tokio runtime in the background so our async HTTP calls have an executor.
    init_runtime();

    // 3) Start the Iced application: `update` handles Messages, `view` renders UI.
    application("Bitcoin Admin UI", update, view)
        .theme(|_| Theme::Dark)
        // `run_with` calls `AdminApp::new()` exactly once to create the initial state.
        .run_with(AdminApp::new)
}
```

### Walkthrough: what `main` wires together

- **Modules**: `update` and `view` are the MVU “engine”; everything else exists to support them.
- **`init_runtime()`**: starts an async executor so later HTTP tasks don’t block the UI thread.
- **`application(...).run_with(AdminApp::new)`**: creates initial state once, then enters the event loop:
  - `view(&AdminApp)` draws the UI
  - user interaction emits `Message`
  - `update(&mut AdminApp, Message)` mutates state and optionally schedules work

---

## Code Listing 4A-1.2 — Tokio runtime bridge (`bitcoin-desktop-ui-iced/src/runtime.rs`)

```rust
use std::sync::OnceLock;

// Store the Tokio runtime handle globally so tasks can access it
static TOKIO_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

pub fn init_runtime() {
    // Create a Tokio runtime for async operations.
    // This runtime must outlive the UI event loop, otherwise spawned tasks will stall.
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    // Store the runtime handle globally so `update.rs` can schedule futures from the UI thread.
    TOKIO_HANDLE
        .set(rt.handle().clone())
        .expect("Failed to set Tokio handle");

    // Keep the runtime alive in a background thread.
    // Why a thread? Iced’s update/view loop is synchronous; we want Tokio running alongside it.
    std::thread::spawn(move || {
        rt.block_on(async {
            // Keep the runtime alive indefinitely.
            std::future::pending::<()>().await;
        });
    });
}

// Helper function to wrap a future to ensure it runs on Tokio runtime
pub fn spawn_on_tokio<F>(fut: F) -> impl std::future::Future<Output = F::Output> + Send
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    // Get a clone of the stored runtime handle (panics if `init_runtime()` was never called).
    let handle = TOKIO_HANDLE
        .get()
        .expect("Tokio runtime not initialized")
        .clone();
    // Return a future that, when awaited by Iced, awaits the spawned Tokio task’s result.
    async move { handle.spawn(fut).await.unwrap() }
}
```

### Walkthrough: why we need `spawn_on_tokio`

Iced’s update loop is **synchronous**: it should be fast and never block on network I/O. The `spawn_on_tokio(...)` helper gives us a clean “async boundary”:

- `update.rs` can return a `Task<Message>` immediately (UI stays responsive)
- Tokio runs the future in the background
- the result is converted into a `Message::XxxLoaded(...)` and re-enters the update loop

---

## Code Listing 4A-2.1 — Navigation + events (`bitcoin-desktop-ui-iced/src/types.rs`)

```rust
use bitcoin_api::{
    ApiResponse, BlockSummary, BlockchainInfo, CreateWalletResponse, SendTransactionResponse,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletSection {
    GetWalletInfo,
    GetBalance,
    Create,
    Send,
    History,
    Addresses,
}

impl WalletSection {
    pub const ALL: [WalletSection; 6] = [
        WalletSection::GetWalletInfo,
        WalletSection::GetBalance,
        WalletSection::Create,
        WalletSection::Send,
        WalletSection::History,
        WalletSection::Addresses,
    ];
}

impl core::fmt::Display for WalletSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            WalletSection::GetWalletInfo => "Get Wallet Info",
            WalletSection::GetBalance => "Get Balance",
            WalletSection::Create => "Create Wallet",
            WalletSection::Send => "Send Bitcoin",
            WalletSection::History => "Transaction History",
            WalletSection::Addresses => "All Addresses",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionSection {
    Mempool,
    MempoolTx,
    AllTransactions,
    AddressTransactions,
}

impl TransactionSection {
    pub const ALL: [TransactionSection; 4] = [
        TransactionSection::Mempool,
        TransactionSection::MempoolTx,
        TransactionSection::AllTransactions,
        TransactionSection::AddressTransactions,
    ];
}

impl core::fmt::Display for TransactionSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            TransactionSection::Mempool => "Mempool",
            TransactionSection::MempoolTx => "Mempool Transaction",
            TransactionSection::AllTransactions => "All Transactions",
            TransactionSection::AddressTransactions => "Address Transactions",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiningSection {
    Info,
    Generate,
}

impl MiningSection {
    pub const ALL: [MiningSection; 2] = [MiningSection::Info, MiningSection::Generate];
}

impl core::fmt::Display for MiningSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            MiningSection::Info => "Mining Info",
            MiningSection::Generate => "Generate Blocks",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthSection {
    Health,
    Liveness,
    Readiness,
}

impl HealthSection {
    pub const ALL: [HealthSection; 3] = [
        HealthSection::Health,
        HealthSection::Liveness,
        HealthSection::Readiness,
    ];
}

impl core::fmt::Display for HealthSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            HealthSection::Health => "Health Check",
            HealthSection::Liveness => "Liveness Check",
            HealthSection::Readiness => "Readiness Check",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockchainSection {
    Info,
    LatestBlocks,
    AllBlocks,
    BlockByHash,
}

impl BlockchainSection {
    pub const ALL: [BlockchainSection; 4] = [
        BlockchainSection::Info,
        BlockchainSection::LatestBlocks,
        BlockchainSection::AllBlocks,
        BlockchainSection::BlockByHash,
    ];
}

impl core::fmt::Display for BlockchainSection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            BlockchainSection::Info => "Get Block Info",
            BlockchainSection::LatestBlocks => "Latest Blocks",
            BlockchainSection::AllBlocks => "All Blocks",
            BlockchainSection::BlockByHash => "Block by Hash",
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
    CreateWalletAdmin,
    CreateWalletAdminDone(Result<ApiResponse<CreateWalletResponse>, String>),
    FetchAddressesAdmin,
    AddressesAdminLoaded(Result<ApiResponse<Value>, String>),
    FetchWalletInfoAdmin(String),
    WalletInfoAdminLoaded(Result<ApiResponse<Value>, String>),
    FetchBalanceAdmin(String),
    BalanceAdminLoaded(Result<ApiResponse<Value>, String>),
    // Send transaction
    SendFromChanged(String),
    SendToChanged(String),
    SendAmountChanged(String),
    SendTx,
    TxSent(Result<ApiResponse<SendTransactionResponse>, String>),
    // Transaction history
    HistoryAddressChanged(String),
    FetchTransactionHistory(String),
    TransactionHistoryLoaded(Result<ApiResponse<Value>, String>),
    // Wallet section navigation
    WalletSectionChanged(WalletSection),
    // Transaction section navigation
    TransactionSectionChanged(TransactionSection),
    // Blockchain section navigation
    BlockchainSectionChanged(BlockchainSection),
    // Blockchain menu hover
    BlockchainMenuHovered(bool),
    WalletMenuHovered(bool),
    TransactionMenuHovered(bool),
    MiningMenuHovered(bool),
    HealthMenuHovered(bool),
    // Mining section navigation
    MiningSectionChanged(MiningSection),
    // Health section navigation
    HealthSectionChanged(HealthSection),
    // Clipboard
    CopyToClipboard(String),
    ClipboardCopied(bool), // true = success, false = failed
    // Text editor edit handlers for JSON displays
    TransactionsEditorAction(iced::widget::text_editor::Action),
    MempoolEditorAction(iced::widget::text_editor::Action),
    MempoolTxEditorAction(iced::widget::text_editor::Action),
    AddressTransactionsEditorAction(iced::widget::text_editor::Action),
    WalletInfoEditorAction(iced::widget::text_editor::Action),
    WalletBalanceEditorAction(iced::widget::text_editor::Action),
    TransactionHistoryEditorAction(iced::widget::text_editor::Action),
    BlocksAllEditorAction(iced::widget::text_editor::Action),
    BlockByHashEditorAction(iced::widget::text_editor::Action),
    BlockchainInfoEditorAction(iced::widget::text_editor::Action),
    LatestBlocksEditorAction(iced::widget::text_editor::Action),
    CreatedWalletAddressEditorAction(iced::widget::text_editor::Action),
}
```

---

## Walkthrough: `Message` is the UI’s event vocabulary

Treat `Message` as a **strict interface** between the view and the update loop:

- **Input messages** (`BaseUrlChanged`, `MiningAddressChanged`, …): pure state changes.
- **Request messages** (`FetchInfo`, `FetchBlocks`, `CreateWalletAdmin`, …): these trigger `Task::perform(...)` in Chapter 4.B.
- **Result messages** (`InfoLoaded`, `BlocksLoaded`, `TxSent`, …): these store JSON into `AdminApp` and update status/editor buffers.
- **UI mechanics** (hover + section routing): `*MenuHovered(bool)` and `*SectionChanged(...)` exist to drive popups and the active panel.

This design keeps the view layer simple: it never “does” network calls—it only emits intent.

---

## Code Listing 4A-3.1 — Application state (`bitcoin-desktop-ui-iced/src/app.rs`)

```rust
use crate::types::{
    BlockchainSection, DataSection, HealthSection, Menu, Message, MiningSection,
    TransactionSection, WalletSection,
};
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
    pub addresses: Vec<String>,
    pub wallet_info: Option<Value>,
    pub wallet_balance: Option<Value>,
    pub created_wallet_address: Option<String>,
    // Send transaction state
    pub send_from_address: String,
    pub send_to_address: String,
    pub send_amount: String,
    pub last_txid: Option<String>,
    // Transaction history state
    pub history_address: String,
    pub transaction_history: Option<Value>,
    // Wallet section navigation
    pub wallet_section: WalletSection,
    // Transaction section navigation
    pub transaction_section: TransactionSection,
    // Blockchain section navigation
    pub blockchain_section: BlockchainSection,
    // Mining section navigation
    pub mining_section: MiningSection,
    // Health section navigation
    pub health_section: HealthSection,
    // Blockchain menu hover state
    pub blockchain_menu_hovered: bool,
    pub wallet_menu_hovered: bool,
    pub transaction_menu_hovered: bool,
    pub mining_menu_hovered: bool,
    pub health_menu_hovered: bool,
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
    // Text editor states for selectable JSON display
    pub transactions_editor: iced::widget::text_editor::Content,
    pub mempool_editor: iced::widget::text_editor::Content,
    pub mempool_tx_editor: iced::widget::text_editor::Content,
    pub address_transactions_editor: iced::widget::text_editor::Content,
    pub wallet_info_editor: iced::widget::text_editor::Content,
    pub wallet_balance_editor: iced::widget::text_editor::Content,
    pub transaction_history_editor: iced::widget::text_editor::Content,
    pub blocks_all_editor: iced::widget::text_editor::Content,
    pub block_by_hash_editor: iced::widget::text_editor::Content,
    pub blockchain_info_editor: iced::widget::text_editor::Content,
    pub latest_blocks_editor: iced::widget::text_editor::Content,
    pub created_wallet_address_editor: iced::widget::text_editor::Content,
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
            addresses: Vec::new(),
            wallet_info: None,
            wallet_balance: None,
            created_wallet_address: None,
            send_from_address: String::new(),
            send_to_address: String::new(),
            send_amount: String::new(),
            last_txid: None,
            history_address: String::new(),
            transaction_history: None,
            wallet_section: WalletSection::Create,
            transaction_section: TransactionSection::Mempool,
            blockchain_section: BlockchainSection::Info,
            mining_section: MiningSection::Info,
            health_section: HealthSection::Health,
            blockchain_menu_hovered: false,
            wallet_menu_hovered: false,
            transaction_menu_hovered: false,
            mining_menu_hovered: false,
            health_menu_hovered: false,
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
            transactions_editor: iced::widget::text_editor::Content::new(),
            mempool_editor: iced::widget::text_editor::Content::new(),
            mempool_tx_editor: iced::widget::text_editor::Content::new(),
            address_transactions_editor: iced::widget::text_editor::Content::new(),
            wallet_info_editor: iced::widget::text_editor::Content::new(),
            wallet_balance_editor: iced::widget::text_editor::Content::new(),
            transaction_history_editor: iced::widget::text_editor::Content::new(),
            blocks_all_editor: iced::widget::text_editor::Content::new(),
            block_by_hash_editor: iced::widget::text_editor::Content::new(),
            blockchain_info_editor: iced::widget::text_editor::Content::new(),
            latest_blocks_editor: iced::widget::text_editor::Content::new(),
            created_wallet_address_editor: iced::widget::text_editor::Content::new(),
        }
    }
}

impl AdminApp {
    pub fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                menu: Menu::Blockchain,
                // Base URL for our Rust Bitcoin implementation’s HTTP server.
                base_url: "http://127.0.0.1:8080".into(),
                // Admin key is read from env; the UI also allows overriding the value in the toolbar.
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
                addresses: Vec::new(),
                wallet_info: None,
                wallet_balance: None,
                created_wallet_address: None,
                send_from_address: String::new(),
                send_to_address: String::new(),
                send_amount: String::new(),
                last_txid: None,
                history_address: String::new(),
                transaction_history: None,
                wallet_section: WalletSection::Create,
                transaction_section: TransactionSection::Mempool,
                blockchain_section: BlockchainSection::Info,
                mining_section: MiningSection::Info,
                health_section: HealthSection::Health,
                blockchain_menu_hovered: false,
                wallet_menu_hovered: false,
                transaction_menu_hovered: false,
                mining_menu_hovered: false,
                health_menu_hovered: false,
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
                transactions_editor: iced::widget::text_editor::Content::new(),
                mempool_editor: iced::widget::text_editor::Content::new(),
                mempool_tx_editor: iced::widget::text_editor::Content::new(),
                address_transactions_editor: iced::widget::text_editor::Content::new(),
                wallet_info_editor: iced::widget::text_editor::Content::new(),
                wallet_balance_editor: iced::widget::text_editor::Content::new(),
                transaction_history_editor: iced::widget::text_editor::Content::new(),
                blocks_all_editor: iced::widget::text_editor::Content::new(),
                block_by_hash_editor: iced::widget::text_editor::Content::new(),
                blockchain_info_editor: iced::widget::text_editor::Content::new(),
                latest_blocks_editor: iced::widget::text_editor::Content::new(),
                created_wallet_address_editor: iced::widget::text_editor::Content::new(),
            },
            iced::Task::none(),
        )
    }

    /// Clear previously loaded data for a specific section
    pub fn clear_related_data(&mut self, section: DataSection) {
        // “Hygiene rule”: when we load one dataset, clear adjacent datasets so the UI
        // doesn’t render stale JSON next to fresh JSON.
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
```

---

## Walkthrough: what lives in `AdminApp` (and why)

`AdminApp` is intentionally “wide”: it contains both **domain-ish data** and **UI presentation state**.

- **Configuration**: `base_url`, `api_key` (what server we talk to).
- **Navigation**: `menu` + section enums (which panel to render).
- **Inputs**: text fields like `block_hash_input`, `send_amount` (what the user typed).
- **Responses**: `*_data: Option<Value>` are cached JSON payloads to display.
- **Editors**: `*_editor: text_editor::Content` exist so JSON can be selectable/copyable and scrollable.

The `clear_related_data(...)` helper is a small UX rule: when you fetch one dataset, clear nearby datasets so stale JSON doesn’t linger on screen.

---

## Code Listing 4A-4.1 — Async API client helpers (`bitcoin-desktop-ui-iced/src/api.rs`)

```rust
use bitcoin_api::{
    AdminClient, ApiConfig, ApiResponse, BlockSummary, BlockchainInfo, CreateWalletRequest,
    CreateWalletResponse, SendTransactionRequest, SendTransactionResponse,
};
use serde_json::Value;

pub async fn fetch_info(cfg: ApiConfig) -> Result<ApiResponse<BlockchainInfo>, String> {
    // A new client is constructed per call; config contains base_url + optional API key.
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_blockchain_info()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_blocks(cfg: ApiConfig) -> Result<ApiResponse<Vec<BlockSummary>>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_latest_blocks().await.map_err(|e| e.to_string())
}

// Additional admin endpoints
pub async fn fetch_blocks_all(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_blocks().await.map_err(|e| e.to_string())
}

pub async fn fetch_block_by_hash(
    cfg: ApiConfig,
    hash: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_block_by_hash(&hash)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_mining_info(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_mining_info().await.map_err(|e| e.to_string())
}

pub async fn generate_to_address(
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

pub async fn fetch_health(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.health().await.map_err(|e| e.to_string())
}

pub async fn fetch_liveness(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.liveness().await.map_err(|e| e.to_string())
}

pub async fn fetch_readiness(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.readiness().await.map_err(|e| e.to_string())
}

pub async fn fetch_mempool(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_mempool().await.map_err(|e| e.to_string())
}

pub async fn fetch_mempool_tx(cfg: ApiConfig, txid: String) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_mempool_transaction(&txid)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_transactions(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client.get_transactions().await.map_err(|e| e.to_string())
}

pub async fn fetch_address_transactions(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_address_transactions_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

// Wallet admin functions
pub async fn create_wallet_admin(
    cfg: ApiConfig,
    req: CreateWalletRequest,
) -> Result<ApiResponse<CreateWalletResponse>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .create_wallet_admin(&req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_addresses_admin(cfg: ApiConfig) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_addresses_admin()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_wallet_info_admin(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_wallet_info_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_balance_admin(
    cfg: ApiConfig,
    address: String,
) -> Result<ApiResponse<Value>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        .get_balance_admin(&address)
        .await
        .map_err(|e| e.to_string())
}

// Send transaction using admin client
pub async fn send_transaction(
    cfg: ApiConfig,
    req: SendTransactionRequest,
) -> Result<ApiResponse<SendTransactionResponse>, String> {
    let client = AdminClient::new(cfg).map_err(|e| e.to_string())?;
    client
        // Admin endpoint that creates + broadcasts a transaction (server-side).
        .send_transaction_admin(&req)
        .await
        .map_err(|e| e.to_string())
}
```

---

## Walkthrough: how `api.rs` supports the update loop

This module is intentionally boring: each function is a **single HTTP call** wrapped in an `async fn` with a UI-friendly error type.

- **Inputs**: `ApiConfig` is built from `AdminApp { base_url, api_key }` in Chapter 4.B.
- **Return type**: `Result<ApiResponse<T>, String>`
  - `Err(String)` means: “we could not even complete the request” (client construction, network error, decode error, etc.).
  - `Ok(ApiResponse<T>)` means: “we got a response envelope”; it can still represent application-level failure via `api.success == false`.
- **Why build a client per call?** It keeps the UI side stateless and makes each request self-contained. In Chapter 4.B you can read each `Task::perform(spawn_on_tokio(api_call), Message::XxxLoaded)` arm as a direct mapping: *message → API call → loaded message*.

If you want to understand one feature end-to-end, pick one helper here (for example `create_wallet_admin`), then jump to Chapter 4.B and find:

- `Message::CreateWalletAdmin` (request)
- `Message::CreateWalletAdminDone` (result handling)

---

<div align="center">

**📚 [← Chapter 4: Desktop Admin Interface](03-Desktop-Admin-UI.md)** | **Chapter 4.A** | **[Next: Chapter 4.B (Update Loop) →](03B-Desktop-Admin-UI-Update-Loop.md)** 📚

</div>

---

