# Bitcoin Wallet UI - Architecture and Implementation Details

## Overview

The Bitcoin Wallet UI has been refactored from a single-file monolithic structure into a modular, professional architecture that matches the design patterns of the desktop admin UI. This document explains the architecture, data flow, and implementation details.

## Architecture Overview

The application follows the **Model-View-Update (MVU)** pattern used by the Iced framework, with clear separation of concerns across multiple modules:

```
bitcoin-wallet-ui/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Application state (Model)
│   ├── types.rs         # Type definitions (Menu, Message enums)
│   ├── update.rs        # Business logic (Update function)
│   ├── view.rs          # UI rendering (View function)
│   ├── api.rs           # Async API client functions
│   └── runtime.rs       # Tokio runtime management
```

## Module-by-Module Breakdown

### 1. `main.rs` - Application Entry Point

**Purpose**: Initializes the application and wires everything together.

**Key Components**:

```rust
fn main() -> iced::Result {
    // Initialize Tokio runtime for async operations
    init_runtime();

    // Run the application
    application("Bitcoin Wallet UI", update, view)
        .theme(|_| Theme::Dark)
        .run_with(WalletApp::new)
}
```

**How it works**:
1. **`init_runtime()`**: Sets up a global Tokio runtime that will handle all async operations (HTTP requests). This must be done before the Iced application starts.
2. **`application()`**: Creates an Iced application with:
   - Title: "Bitcoin Wallet UI"
   - Update function: `update` (from `update.rs`)
   - View function: `view` (from `view.rs`)
   - Theme: Dark mode
3. **`run_with(WalletApp::new)`**: Starts the application with the initial state provided by `WalletApp::new()`

**Data Flow**: 
```
main() 
  → init_runtime() [sets up Tokio]
  → application() [creates Iced app]
  → WalletApp::new() [creates initial state]
  → Iced event loop starts
```

---

### 2. `runtime.rs` - Tokio Runtime Management

**Purpose**: Manages the Tokio async runtime that powers all HTTP requests.

**Key Components**:

#### Global Runtime Handle Storage
```rust
static TOKIO_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();
```

**Why `OnceLock`?**
- `OnceLock` ensures the runtime handle is initialized exactly once
- Thread-safe: can be accessed from any thread after initialization
- Prevents multiple runtime creation (which would cause errors)

#### Runtime Initialization
```rust
pub fn init_runtime() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    TOKIO_HANDLE.set(rt.handle().clone())
        .expect("Failed to set Tokio handle");
    
    // Keep the runtime alive in a background thread
    std::thread::spawn(move || {
        rt.block_on(async {
            std::future::pending::<()>().await; // Run forever
        });
    });
}
```

**How it works**:
1. Creates a new Tokio runtime
2. Stores the runtime handle globally (so any thread can access it)
3. Spawns a background thread that keeps the runtime alive indefinitely
   - The runtime must stay alive for async operations to work
   - `std::future::pending::<()>().await` creates a future that never completes, keeping the thread alive

#### Spawning Tasks on Tokio
```rust
pub fn spawn_on_tokio<F>(fut: F) -> impl std::future::Future<Output = F::Output> + Send
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    let handle = TOKIO_HANDLE.get().expect("Tokio runtime not initialized").clone();
    async move { handle.spawn(fut).await.unwrap() }
}
```

**How it works**:
1. Retrieves the global Tokio handle
2. Spawns the provided future onto the Tokio runtime
3. Returns a new future that will yield the result when the spawned task completes

**Why this is needed**: 
- Iced's `Task::perform` runs futures on its own executor
- But `reqwest` (HTTP client) requires a Tokio runtime
- `spawn_on_tokio` ensures HTTP requests run on the Tokio runtime

**Data Flow**:
```
Iced Task::perform
  → spawn_on_tokio(future)
  → Tokio runtime spawns task
  → HTTP request executes
  → Result returned to Iced
```

---

### 3. `types.rs` - Type Definitions

**Purpose**: Defines all enums and types used throughout the application.

#### Menu Enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Wallet,
    Send,
    History,
    Settings,
}
```

**Purpose**: Represents the different sections/screens of the application.

**Implementation Details**:
- `Copy`: Can be copied cheaply (no heap allocation)
- `PartialEq, Eq`: Can be compared for equality
- `Display` trait: Converts to string for UI display

#### Message Enum
```rust
#[derive(Debug, Clone)]
pub enum Message {
    // UI interaction messages
    MenuChanged(Menu),
    BaseUrlChanged(String),
    ApiKeyChanged(String),
    FromChanged(String),
    ToChanged(String),
    AmountChanged(String),
    
    // Action messages
    CreateWallet,
    SendTx,
    
    // Async result messages
    WalletCreated(Result<ApiResponse<CreateWalletResponse>, String>),
    TxSent(Result<ApiResponse<SendTransactionResponse>, String>),
    
    // Clipboard messages
    CopyToClipboard(String),
    ClipboardCopied(bool),
    
    // Text editor messages
    WalletAddressEditorAction(iced::widget::text_editor::Action),
    TransactionIdEditorAction(iced::widget::text_editor::Action),
}
```

**Purpose**: All possible events/actions in the application. This is the communication mechanism between the View and Update functions.

**Message Categories**:
1. **Input Messages**: User typing in text fields (`BaseUrlChanged`, `FromChanged`, etc.)
2. **Action Messages**: User clicking buttons (`CreateWallet`, `SendTx`)
3. **Async Result Messages**: HTTP requests completing (`WalletCreated`, `TxSent`)
4. **Clipboard Messages**: Copy operations
5. **Editor Messages**: Text selection/copying in text editors

**Data Flow**:
```
User Action → View generates Message → Update processes Message → State changes → View re-renders
```

---

### 4. `app.rs` - Application State (Model)

**Purpose**: Holds all application state/data.

#### WalletApp Structure
```rust
pub struct WalletApp {
    // Navigation
    pub menu: Menu,
    
    // Configuration
    pub base_url: String,
    pub api_key: String,
    
    // Send transaction form
    pub from: String,
    pub to: String,
    pub amount: String,
    
    // Status and results
    pub status: String,
    pub new_address: Option<String>,
    pub last_txid: Option<String>,
    
    // Text editor states (for selectable displays)
    pub wallet_address_editor: iced::widget::text_editor::Content,
    pub transaction_id_editor: iced::widget::text_editor::Content,
}
```

**Key Fields Explained**:

1. **`menu`**: Current active section (Wallet, Send, History, Settings)
2. **`base_url`, `api_key`**: API configuration
3. **`from`, `to`, `amount`**: Form inputs for sending transactions
4. **`status`**: User-facing status messages
5. **`new_address`**: Wallet address after creation (None if not created yet)
6. **`last_txid`**: Transaction ID after sending (None if not sent yet)
7. **`wallet_address_editor`**: Text editor content for the wallet address display
   - Allows users to select and copy the address
   - Maintains selection state
8. **`transaction_id_editor`**: Text editor content for transaction ID display

#### Initialization
```rust
pub fn new() -> (Self, iced::Task<crate::types::Message>) {
    (
        Self {
            menu: Menu::Wallet,
            base_url: "http://127.0.0.1:8080".into(),
            api_key: std::env::var("BITCOIN_API_WALLET_KEY")
                .unwrap_or_else(|_| "wallet-secret".into()),
            // ... other fields initialized
        },
        iced::Task::none(), // No initial task
    )
}
```

**Returns**: A tuple of (initial state, initial task)
- The task can be used to perform async operations on startup (we don't need any)

---

### 5. `update.rs` - Business Logic (Update Function)

**Purpose**: Processes all messages and updates application state. This is the "brain" of the application.

#### Function Signature
```rust
pub fn update(app: &mut WalletApp, message: Message) -> Task<Message>
```

**Parameters**:
- `app`: Mutable reference to application state (will be modified)
- `message`: The event/action to process

**Returns**: `Task<Message>` - An async task that may produce a future message

#### Message Handling Patterns

##### 1. Simple State Updates
```rust
Message::MenuChanged(m) => {
    app.menu = m;
    Task::none() // No async work needed
}
```

**Flow**: User clicks menu button → View sends `MenuChanged` → Update changes `app.menu` → View re-renders with new section

##### 2. Input Field Updates
```rust
Message::BaseUrlChanged(v) => {
    app.base_url = v;
    Task::none()
}
```

**Flow**: User types in text field → View sends `BaseUrlChanged` with new value → Update stores it in state

##### 3. Async Operations (Creating Wallet)
```rust
Message::CreateWallet => {
    let cfg = ApiConfig {
        base_url: app.base_url.clone(),
        api_key: Some(app.api_key.clone()),
    };
    let req = CreateWalletRequest { label: None };
    Task::perform(
        spawn_on_tokio(api::create_wallet(cfg, req)), 
        Message::WalletCreated
    )
}
```

**Flow**:
1. User clicks "Create Wallet" button
2. Update function creates API config and request
3. `Task::perform` starts an async operation:
   - `spawn_on_tokio(api::create_wallet(...))` - Runs HTTP request on Tokio
   - `Message::WalletCreated` - Message to send when request completes
4. HTTP request executes in background
5. When complete, Iced sends `Message::WalletCreated(Result<...>)` back to update

##### 4. Handling Async Results
```rust
Message::WalletCreated(res) => {
    match res {
        Ok(api) => {
            if api.success {
                app.new_address = api.data.map(|d| d.address.clone());
                // Update text editor with new address
                if let Some(addr) = &app.new_address {
                    app.wallet_address_editor =
                        iced::widget::text_editor::Content::with_text(addr);
                }
                app.status = "Wallet created successfully".into();
            } else {
                app.status = api.error.unwrap_or_else(|| "Error creating wallet".into());
                app.wallet_address_editor = iced::widget::text_editor::Content::new();
            }
        }
        Err(e) => {
            app.status = format!("Error: {}", e);
            app.wallet_address_editor = iced::widget::text_editor::Content::new();
        }
    }
    Task::none()
}
```

**Flow**:
1. HTTP request completes (success or failure)
2. Update function receives result
3. Updates state:
   - Stores wallet address in `app.new_address`
   - Populates text editor with address (for selectable display)
   - Sets status message
4. View re-renders with new data

##### 5. Clipboard Operations
```rust
Message::CopyToClipboard(text) => {
    let text_clone = text.clone();
    Task::perform(
        async move {
            // Platform-specific clipboard copy
            #[cfg(target_os = "macos")]
            {
                use std::process::Command;
                let mut cmd = Command::new("pbcopy");
                // ... copy logic
            }
            // ... other platforms
            true // or false on failure
        },
        Message::ClipboardCopied,
    )
}
```

**How it works**:
1. User clicks copy button
2. Update spawns async task to copy to clipboard
3. Uses platform-specific command (`pbcopy` on macOS, `xclip` on Linux, `clip` on Windows)
4. When complete, sends `Message::ClipboardCopied(bool)` back
5. Update sets status message based on success/failure

##### 6. Text Editor Actions
```rust
Message::WalletAddressEditorAction(action) => {
    app.wallet_address_editor.perform(action);
    Task::none()
}
```

**Purpose**: Handles text selection, copying, and other editor interactions. The `action` contains information about what the user did (selected text, copied, etc.).

**Data Flow Summary**:
```
User Action
  ↓
View generates Message
  ↓
Update processes Message
  ↓
[If async needed] Task::perform spawns async operation
  ↓
State updated (app modified)
  ↓
[If async] Result comes back as new Message
  ↓
Update processes result Message
  ↓
State updated again
  ↓
View re-renders with new state
```

---

### 6. `api.rs` - Async API Client Functions

**Purpose**: Contains all HTTP API calls. These are pure async functions that make requests to the Bitcoin node.

#### Example: Creating a Wallet
```rust
pub async fn create_wallet(
    cfg: ApiConfig,
    req: CreateWalletRequest,
) -> Result<ApiResponse<CreateWalletResponse>, String> {
    let client = WalletClient::new(cfg).map_err(|e| e.to_string())?;
    client.create_wallet(&req).await.map_err(|e| e.to_string())
}
```

**How it works**:
1. Creates a `WalletClient` with the provided config (base URL, API key)
2. Calls `create_wallet` async method
3. Returns `Result` - either success with response data, or error as String

**Error Handling**:
- `map_err(|e| e.to_string())`: Converts API errors to user-friendly strings
- Errors bubble up to the update function as `Err(String)` in the Result

**Data Flow**:
```
update() calls api::create_wallet()
  ↓
WalletClient created
  ↓
HTTP POST request to /api/wallet/create
  ↓
Server processes request
  ↓
Response received
  ↓
Parsed into ApiResponse<CreateWalletResponse>
  ↓
Returned to update() as Result
```

---

### 7. `view.rs` - UI Rendering (View Function)

**Purpose**: Renders the UI based on current application state. This is a pure function - it doesn't modify state, only displays it.

#### Main View Function
```rust
pub fn view(app: &WalletApp) -> Element<Message> {
    // Build UI components
    let menu_buttons = ...;
    let config_toolbar = ...;
    let section = match app.menu { ... };
    let status_bar = ...;
    
    // Combine into final layout
    column![config_toolbar, menu_buttons, section, status_bar]
        .spacing(15)
        .padding(20)
        .into()
}
```

**How it works**:
1. Takes immutable reference to `WalletApp` (read-only)
2. Builds UI components based on state
3. Returns an `Element<Message>` - the rendered UI

**Key UI Components**:

##### 1. Menu Buttons
```rust
let menu_buttons: Element<Message> = row(
    Menu::ALL
        .iter()
        .map(|&menu_item| {
            let menu_label = menu_item.to_string();
            button(text(menu_label))
                .on_press(Message::MenuChanged(menu_item))
                .into()
        })
        .collect::<Vec<_>>(),
)
.spacing(10)
.into();
```

**How it works**:
- Iterates over all menu items
- Creates a button for each
- `.on_press(Message::MenuChanged(menu_item))`: When clicked, sends message to update function
- Arranges buttons in a horizontal row

##### 2. Wallet Address Display (Selectable)
```rust
if app.new_address.is_some() {
    column![
        row![
            text("Wallet Address:").size(14),
            button("📋 Copy")
                .on_press(Message::CopyToClipboard(
                    app.new_address.as_ref().unwrap().clone()
                ))
                .style(...), // Professional styling
        ],
        scrollable(
            container(
                text_editor(&app.wallet_address_editor)
                    .on_action(Message::WalletAddressEditorAction)
                    .height(iced::Length::Fixed(80.0))
            )
            .padding(12)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.95, g: 0.95, b: 0.95, a: 1.0, // Light gray
                })),
                border: iced::Border {
                    radius: 6.0.into(), // Rounded corners
                    width: 1.0,
                    color: iced::Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 },
                },
                ..container::Style::default()
            })
        )
    ]
}
```

**Key Features**:
- **`text_editor`**: Makes the address selectable and copyable
- **`.on_action(...)`**: Handles text selection events
- **Styled container**: Light gray background with border for visual distinction
- **Copy button**: Quick copy functionality
- **Scrollable**: Handles long addresses gracefully

**Professional Styling Details**:
- **Background Color**: RGB(0.95, 0.95, 0.95) - Light gray, easy to read on dark theme
- **Border**: Subtle gray border with 6px radius for modern look
- **Padding**: 12px for comfortable spacing
- **Height**: Fixed 80px - enough to display address without taking too much space

##### 3. Button Styling
```rust
button("Create New Wallet")
    .on_press(Message::CreateWallet)
    .style(|_theme, _status| {
        iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.2, g: 0.6, b: 0.9, a: 1.0, // Blue
            })),
            text_color: iced::Color::WHITE,
            border: iced::Border {
                radius: 6.0.into(),
                width: 1.0,
                color: iced::Color { r: 0.3, g: 0.7, b: 1.0, a: 1.0 },
            },
            ..iced::widget::button::Style::default()
        }
    })
    .padding([10, 20]) // Vertical: 10px, Horizontal: 20px
```

**Color Scheme**:
- **Create Wallet**: Blue (RGB 0.2, 0.6, 0.9) - Primary action
- **Send Transaction**: Green (RGB 0.2, 0.7, 0.3) - Success/action
- **Copy Button**: Gray (RGB 0.3, 0.3, 0.3) - Secondary action

##### 4. Status Bar
```rust
let status_bar: Element<Message> = if !app.status.is_empty() {
    container(text(&app.status).size(14))
        .padding(12)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.2, g: 0.2, b: 0.2, a: 1.0, // Dark gray
            })),
            border: iced::Border {
                radius: 4.0.into(),
                width: 1.0,
                color: iced::Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 },
            },
            ..container::Style::default()
        })
        .width(iced::Length::Fill)
        .into()
} else {
    container(text("")).height(iced::Length::Fixed(0.0)).into()
};
```

**Features**:
- Only displays when `app.status` is not empty
- Dark background with border for visibility
- Full width to match layout
- Hidden (0 height) when no status message

---

## Complete Data Flow Example: Creating a Wallet

Let's trace through what happens when a user creates a wallet:

### Step 1: User Clicks Button
```
User clicks "Create New Wallet" button in UI
```

### Step 2: View Generates Message
```rust
// In view.rs
button("Create New Wallet")
    .on_press(Message::CreateWallet)  // ← Message sent here
```

### Step 3: Iced Routes Message to Update
```
Iced framework receives Message::CreateWallet
  ↓
Calls update(&mut app, Message::CreateWallet)
```

### Step 4: Update Function Processes Message
```rust
// In update.rs
Message::CreateWallet => {
    let cfg = ApiConfig { ... };
    let req = CreateWalletRequest { label: None };
    Task::perform(
        spawn_on_tokio(api::create_wallet(cfg, req)), 
        Message::WalletCreated
    )
}
```

**What happens**:
- Creates API configuration from app state
- Creates request object
- Spawns async task that will:
  1. Run `api::create_wallet()` on Tokio runtime
  2. When complete, send `Message::WalletCreated(result)` back

### Step 5: HTTP Request Executes
```rust
// In api.rs
pub async fn create_wallet(...) -> Result<...> {
    let client = WalletClient::new(cfg)?;
    client.create_wallet(&req).await  // ← HTTP POST request
        .map_err(|e| e.to_string())
}
```

**What happens**:
- Creates HTTP client
- Makes POST request to `{base_url}/api/wallet/create`
- Server processes request and returns response
- Response parsed into `ApiResponse<CreateWalletResponse>`

### Step 6: Result Sent Back to Update
```
HTTP request completes
  ↓
Result<ApiResponse<CreateWalletResponse>, String> created
  ↓
Iced sends Message::WalletCreated(result) to update()
```

### Step 7: Update Processes Result
```rust
// In update.rs
Message::WalletCreated(res) => {
    match res {
        Ok(api) => {
            if api.success {
                app.new_address = api.data.map(|d| d.address.clone());
                // Update text editor
                if let Some(addr) = &app.new_address {
                    app.wallet_address_editor =
                        iced::widget::text_editor::Content::with_text(addr);
                }
                app.status = "Wallet created successfully".into();
            }
        }
    }
    Task::none()
}
```

**What happens**:
- Extracts wallet address from response
- Stores in `app.new_address`
- Populates text editor with address (for selectable display)
- Sets success status message

### Step 8: View Re-renders
```
State changed (app.new_address is now Some(...))
  ↓
Iced calls view(&app) again
  ↓
View sees app.new_address.is_some() == true
  ↓
Renders wallet address display with text_editor
```

### Step 9: User Sees Result
```
User sees:
- "Wallet created successfully" in status bar
- Wallet address displayed in selectable text editor
- Copy button available
```

---

## Key Design Patterns

### 1. Separation of Concerns
- **State** (`app.rs`): What the app knows
- **Logic** (`update.rs`): How the app responds to events
- **Presentation** (`view.rs`): How the app looks
- **API** (`api.rs`): How the app communicates with server
- **Runtime** (`runtime.rs`): How async operations are managed

### 2. Immutability in View
- View function takes `&WalletApp` (immutable reference)
- View never modifies state directly
- All state changes go through `update()` function

### 3. Message-Driven Architecture
- All user actions become Messages
- All async results become Messages
- Update function is a pure message processor
- Makes the application predictable and testable

### 4. Async Task Pattern
```rust
Task::perform(
    spawn_on_tokio(async_operation()),
    Message::ResultHandler
)
```
- `spawn_on_tokio`: Ensures operation runs on Tokio runtime
- `Message::ResultHandler`: Message type to send when operation completes
- Result comes back as `Message::ResultHandler(Result<...>)`

### 5. Professional Styling Pattern
- Consistent color scheme (blue for primary, green for success, gray for secondary)
- Rounded corners (6px radius)
- Proper padding and spacing (12-20px)
- Light gray backgrounds for text displays (RGB 0.95, 0.95, 0.95)
- Dark theme with good contrast

---

## Benefits of This Architecture

1. **Maintainability**: Each module has a single, clear responsibility
2. **Testability**: Update function can be tested with different messages
3. **Scalability**: Easy to add new features by adding new messages and handlers
4. **Type Safety**: Rust's type system ensures messages are handled correctly
5. **Performance**: Async operations don't block the UI
6. **User Experience**: Professional styling and selectable text editors

---

## Summary

The refactored Bitcoin Wallet UI follows modern Rust GUI patterns with:
- **Modular architecture** for maintainability
- **MVU pattern** for predictable state management
- **Async runtime integration** for non-blocking HTTP requests
- **Professional styling** matching the desktop admin UI
- **Selectable text displays** for better user experience
- **Clear data flow** from user actions to state updates

This architecture makes the codebase easier to understand, maintain, and extend.

