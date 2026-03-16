# Appendix: Source Code Reference

This appendix provides an annotated directory structure for each module covered in the book. Rather than printing verbatim listings (see companion chapters A/B/C), we map out the repository's organization so you can navigate to the exact files discussed in each chapter.

> The complete source code for this book is available at the repository. Each section below maps to a companion chapter that contains the full source listings. Use the directory trees to orient yourself in the repository and find the files discussed in each chapter.

---

## Chapter 16: Desktop Admin UI (Iced)

**Repository path:** `bitcoin-desktop-ui-iced/`

This is a single-window Rust desktop application using the Iced GUI toolkit. It follows an Elm-style MVU (Model-View-Update) architecture where all state lives in `AdminApp`, all user actions become `Message` events, and async work (HTTP requests) runs on a background Tokio runtime.

```text
bitcoin-desktop-ui-iced/
├── Cargo.toml                 # Dependencies (iced, tokio, tracing, bitcoin-api)
└── src/
    ├── main.rs                # Application entrypoint, logging, Tokio init
    ├── runtime.rs             # Tokio runtime bridge (keeps async executor alive)
    ├── types.rs               # Message enum (event vocabulary) + Menu navigation
    ├── app.rs                 # AdminApp struct (the "model") + defaults
    ├── api.rs                 # Async HTTP helpers (AdminClient wrappers)
    ├── update.rs              # Message dispatcher (state mutations + tasks)
    └── view.rs                # UI rendering (pure-ish, emits Message on click)
```

**Key patterns:**
- **MVU loop**: `view` → user click → `Message` → `update` → new `AdminApp` → re-render
- **Async I/O**: Each HTTP call is wrapped as `Task::perform(spawn_on_tokio(...), Message::XxxLoaded)`
- **Navigation**: `Menu` enum + section sub-enums drive which panel renders on screen

**Methods involved:** `main`, `init_runtime`, `spawn_on_tokio`, `AdminApp::new`, `AdminApp::clear_related_data`, all `Message` variants, `update`, `view`

---

## Chapter 17: Desktop Admin UI (Tauri)

**Repository path:** `bitcoin-desktop-ui-tauri/`

This is a cross-platform desktop app using Tauri 2 (Rust backend) + React/TypeScript frontend. The Rust backend exposes command handlers that the frontend invokes via IPC. State is split: Rust holds `ApiConfig` (server URL + key), React holds UI state via Zustand.

```text
bitcoin-desktop-ui-tauri/
├── Cargo.toml                 # Rust workspace (root level)
├── src-tauri/
│   ├── Cargo.toml             # Backend dependencies (tauri, tokio, bitcoin-api)
│   ├── tauri.conf.json        # Tauri config (window, build, security)
│   ├── capabilities/
│   │   └── main.json          # Tauri security scopes for main window
│   ├── build.rs               # Tauri build script
│   └── src/
│       ├── main.rs            # Tauri builder setup, command registration
│       ├── lib.rs             # Re-exports for command handlers
│       ├── config/
│       │   └── mod.rs         # ApiConfig struct (base_url + api_key)
│       ├── models/
│       │   └── mod.rs         # Serializable DTO structs (BlockchainInfo, etc.)
│       ├── services/
│       │   └── bitcoin_api.rs  # BitcoinApiService (25 HTTP call wrappers)
│       ├── database/
│       │   ├── mod.rs         # Database setup + persistence helpers
│       │   └── tests.rs       # Database unit tests
│       └── commands/
│           ├── mod.rs         # Command module aggregator
│           ├── blockchain.rs  # Blockchain info, blocks, mining endpoints
│           ├── wallet.rs      # Wallet CRUD and balance operations
│           ├── transactions.rs # Mempool and transaction queries
│           ├── mining.rs      # Mining info + block generation
│           ├── health.rs      # Health, liveness, readiness checks
│           └── settings.rs    # Settings update command
└── src/ (Frontend - React/TypeScript)
    ├── package.json           # Frontend deps (@tauri-apps/api, react, tanstack)
    ├── vite.config.ts         # Vite bundler config
    ├── tailwind.config.js     # Tailwind CSS config
    ├── tsconfig.json          # TypeScript config
    ├── index.css              # Global styles + Tailwind directives
    ├── main.tsx               # React entry point (ReactDOM.render)
    ├── App.tsx                # 18 routes + provider nesting
    ├── types/
    │   └── index.ts           # Shared TypeScript interfaces (BlockchainInfo, etc.)
    ├── lib/
    │   ├── commands.ts        # Tauri invoke() wrappers (22 commands)
    │   └── utils.ts           # Helpers (truncateHash, formatDate, etc.)
    ├── store/
    │   ├── useAppStore.ts     # Zustand store (theme, menu, status)
    │   └── toastStore.ts      # Toast notifications state
    ├── hooks/
    │   ├── useInvoke.ts       # Custom hook for command execution + loading state
    │   └── useClipboard.ts    # Clipboard copy helper
    ├── components/
    │   ├── AppLayout.tsx      # Root layout (sidebar + main area)
    │   ├── Sidebar.tsx        # Navigation menu with route links
    │   ├── TopBar.tsx         # Header with theme toggle + status
    │   ├── ConnectionBadge.tsx # Visual indicator of server connection
    │   ├── DataCard.tsx       # Key-value card display component
    │   ├── DataTable.tsx      # Sortable table with rows/columns
    │   ├── JsonViewer.tsx     # Collapsible JSON renderer
    │   ├── ConfirmDialog.tsx  # Modal for action confirmation
    │   ├── SettingsDialog.tsx # Modal for URL/key config
    │   ├── CommandPalette.tsx # Keyboard-driven command launcher
    │   ├── ThemeProvider.tsx  # Dark/light theme context
    │   ├── ShortcutProvider.tsx # Global keyboard shortcut handler
    │   └── Toast.tsx          # Notification toast display
    └── pages/
        ├── blockchain/
        │   ├── BlockchainInfoPage.tsx     # Auto-fetch key-value display
        │   ├── LatestBlocksPage.tsx       # Table of recent blocks
        │   ├── AllBlocksPage.tsx          # JSON viewer for all blocks
        │   └── BlockByHashPage.tsx        # Search form + JSON result
        ├── wallet/
        │   ├── CreateWalletPage.tsx       # Form to create new wallet
        │   ├── WalletListPage.tsx         # Table of saved wallets
        │   ├── WalletInfoPage.tsx         # Search + key-value display
        │   ├── BalancePage.tsx            # Search + card display
        │   ├── SendBitcoinPage.tsx        # Form with confirmation dialog
        │   ├── TxHistoryPage.tsx          # Search + JSON result
        │   └── AddressListPage.tsx        # Table of wallet addresses
        ├── transactions/
        │   ├── MempoolPage.tsx            # Auto-fetch mempool summary
        │   ├── MempoolTxPage.tsx          # Search mempool transaction
        │   ├── AllTransactionsPage.tsx    # JSON viewer for all txs
        │   └── AddressTxPage.tsx          # Search address transactions
        ├── mining/
        │   ├── MiningInfoPage.tsx         # Auto-fetch mining statistics
        │   └── GenerateBlocksPage.tsx     # Form to mine blocks
        └── health/
            └── HealthPage.tsx             # Multi-query health status
```

**Key patterns:**
- **Tauri IPC**: Frontend calls `invoke('command_name', args)` → Rust handler → HTTP → result back to JS
- **State split**: Rust stores API config globally, React stores UI state (menu, theme, errors) via Zustand
- **Three page patterns**: Auto-fetch (useQuery), Search (useInvoke + form), Form submission (react-hook-form + zod + confirm)
- **Shared components**: DataCard, DataTable, JsonViewer used across all 18 pages

**Methods involved:** `main` (Tauri setup), `BitcoinApiService::*` (25 HTTP helpers), `commands::*::*` (22 Tauri commands), `invoke()` wrappers, `useAppStore`, `useInvoke`, page components

---

## Chapter 18: Wallet UI (Iced)

**Repository path:** `bitcoin-wallet-ui-iced/`

A second Iced desktop application focused on wallet operations (create, list, send, balance, history). Like the Admin UI, it uses MVU with Tokio, but adds encrypted SQLite persistence (Chapter 20) for wallet address storage.

```text
bitcoin-wallet-ui-iced/
├── Cargo.toml                 # Dependencies (iced, tokio, sqlcipher, bitcoin-api)
└── src/
    ├── main.rs                # Entry point, logging, Tokio init, database init
    ├── runtime.rs             # Tokio bridge (same pattern as Admin UI)
    ├── types.rs               # Message enum (wallet-focused events) + Menu
    ├── app.rs                 # WalletApp state (wallets, selected address, etc.)
    ├── api.rs                 # HTTP wrappers for wallet endpoints
    ├── database.rs            # SQLCipher integration (load/save wallets)
    ├── update.rs              # Message dispatcher (state + persistence)
    └── view.rs                # UI rendering (wallet list, forms, displays)
```

**Key patterns:**
- **Persistence layer**: All wallet addresses saved to encrypted SQLite (password derived from username + home dir)
- **Wallet CRUD**: Create button → form → API call → save address to DB → update list
- **Balance tracking**: Query balance for selected address, cache in state
- **Transaction history**: Search by address, display paginated results

**Methods involved:** `main`, `generate_database_password`, `database::init_database`, `WalletApp::new`, `Message` variants, `update`, `view`

---

## Chapter 19: Wallet UI (Tauri)

**Repository path:** `bitcoin-wallet-ui-tauri/`

A Tauri version of the Wallet UI (Rust backend + React frontend). Same functional scope as Iced wallet but uses the IPC architecture. Includes encrypted SQLite in the Rust backend, with state persistence.

```text
bitcoin-wallet-ui-tauri/
├── Cargo.toml                 # Workspace (root)
├── src-tauri/
│   ├── Cargo.toml             # Backend deps (tauri, tokio, sqlcipher, bitcoin-api)
│   ├── tauri.conf.json        # Tauri config
│   ├── capabilities/
│   │   └── main.json          # Security scopes
│   ├── build.rs               # Build script
│   └── src/
│       ├── main.rs            # Tauri builder + command registration
│       ├── lib.rs             # Re-exports
│       ├── config/
│       │   └── mod.rs         # ApiConfig (server URL + key)
│       ├── models/
│       │   └── mod.rs         # Wallet, Address, Balance, etc. DTOs
│       ├── services/
│       │   └── bitcoin_api.rs  # HTTP client wrappers
│       ├── database/
│       │   ├── mod.rs         # SQLCipher setup, wallet persistence
│       │   └── tests.rs       # Unit tests
│       └── commands/
│           ├── mod.rs         # Command aggregator
│           ├── wallet.rs      # Create, list, get info, delete wallet
│           ├── settings.rs    # Config persistence
│           └── health.rs      # Health status
└── src/ (Frontend - React/TypeScript)
    ├── package.json           # Frontend deps (react, tauri api, zustand)
    ├── vite.config.ts         # Vite config
    ├── tailwind.config.js     # Tailwind config
    ├── tsconfig.json          # TypeScript config
    ├── index.css              # Styles
    ├── main.tsx               # React entry point
    ├── App.tsx                # Routes: WalletList, CreateWallet, WalletInfo, etc.
    ├── types/
    │   └── index.ts           # Wallet, Address, Balance, Transaction types
    ├── hooks/
    │   └── useCommands.ts     # Tauri invoke() wrappers (7 commands)
    ├── lib/
    │   └── utils.ts           # Helpers (formatAddress, etc.)
    ├── store/
    │   ├── walletStore.ts     # Zustand (wallets, selected address, balance)
    │   └── toastStore.ts      # Toast notifications
    ├── components/
    │   ├── AppLayout.tsx      # Root layout
    │   ├── WalletCard.tsx     # Card displaying a single wallet
    │   ├── JsonViewer.tsx     # JSON display helper
    │   └── ToastContainer.tsx # Toast renderer
    └── pages/
        └── wallet/
            ├── CreateWalletPage.tsx      # Form to create wallet
            ├── WalletListPage.tsx        # List of saved wallets
            ├── WalletInfoPage.tsx        # Search + key-value display
            ├── BalancePage.tsx           # Query and display balance
            ├── SendPage.tsx              # Form to send transaction
            ├── HistoryPage.tsx           # Transaction history
            └── SettingsPage.tsx          # API config + options
```

**Key patterns:**
- **Wallet CRUD via IPC**: Frontend form → Tauri command → Rust DB operation → result back
- **Persistence in backend**: SQLCipher database lives in Rust; frontend just reads/writes through commands
- **Limited scope**: 5 core wallet pages (create, list, info, balance, send, history, settings)

**Methods involved:** Tauri command handlers for wallet CRUD, database persistence, Zustand store, React pages

---

## Chapter 20: Embedded Database

**Repository path:** `embedded-database/`

This chapter covers encrypted SQLite integration in the Wallet UIs (both Iced and Tauri). The database stores wallet addresses, metadata, and transaction history locally.

```text
embedded-database/
├── README.md (Chapter main content, architecture discussion)
├── 06A-Embedded-Database-Code-Listings.md
└── (Source code resides in each wallet UI's database module:)
    ├── bitcoin-wallet-ui-iced/src/database.rs
    │   └── Implements SQLCipher encryption with machine-specific key derivation
    │
    └── bitcoin-wallet-ui-tauri/src-tauri/src/database/
        ├── mod.rs       # Database connection pool, migrations, queries
        └── tests.rs     # Unit tests for database operations
```

**Key concepts:**
- **Key derivation**: Password generated from username + home directory (no user prompt)
- **Encrypted storage**: SQLCipher (SQLite + OpenSSL) encrypts the database file
- **Wallet persistence**: Addresses and labels stored locally, survive app restart
- **Query patterns**: Functions to insert, load, delete wallet addresses

---

## Chapter 21: Web Admin UI

**Repository path:** `bitcoin-web-ui/`

A browser-based admin interface (React/TypeScript). It provides the same blockchain query capabilities as the desktop UIs but runs in a web browser and communicates directly with the HTTP API (no desktop framework wrapping).

```text
bitcoin-web-ui/
├── package.json               # Dependencies (react, vite, tailwind, axios)
├── vite.config.ts             # Vite config
├── tailwind.config.js         # Tailwind CSS
├── tsconfig.json              # TypeScript config
├── index.html                 # HTML entry point
├── src/
│   ├── main.tsx               # React entrypoint (ReactDOM.createRoot)
│   ├── App.tsx                # Routes + provider nesting
│   ├── index.css              # Global styles
│   ├── types/
│   │   └── api.ts             # Shared type definitions
│   ├── services/
│   │   └── api.ts             # Axios HTTP client + endpoint helpers
│   ├── contexts/
│   │   └── ApiConfigContext.tsx  # React context for URL/key config
│   ├── hooks/
│   │   └── useApi.ts          # Custom hook for API calls
│   ├── utils/
│   │   └── date.ts            # Date formatting utilities
│   ├── components/
│   │   ├── Layout/
│   │   │   ├── Layout.tsx      # Root layout wrapper
│   │   │   ├── Navbar.tsx      # Top navigation bar
│   │   │   └── Sidebar.tsx     # Side navigation menu
│   │   ├── common/
│   │   │   ├── JsonViewer.tsx  # JSON display component
│   │   │   ├── LoadingSpinner.tsx # Loading indicator
│   │   │   ├── ErrorMessage.tsx   # Error display
│   │   │   └── StatCard.tsx    # Key-value stat card
│   │   ├── Dashboard/
│   │   │   └── Dashboard.tsx   # Landing page with key metrics
│   │   ├── Blockchain/
│   │   │   ├── BlockchainInfo.tsx
│   │   │   ├── LatestBlocks.tsx
│   │   │   ├── AllBlocks.tsx
│   │   │   └── BlockByHash.tsx
│   │   ├── Wallet/
│   │   │   ├── CreateWallet.tsx
│   │   │   ├── WalletInfo.tsx
│   │   │   ├── Balance.tsx
│   │   │   ├── SendTransaction.tsx
│   │   │   ├── TransactionHistory.tsx
│   │   │   └── AllAddresses.tsx
│   │   └── Transactions/
│   │       ├── Mempool.tsx
│   │       ├── MempoolTx.tsx
│   │       ├── AllTransactions.tsx
│   │       └── AddressTransactions.tsx
│   └── pages/
│       └── Home.tsx           # Home page router
```

**Key patterns:**
- **HTTP-only**: Direct calls to Bitcoin API (no desktop IPC)
- **Context API**: Stores base URL + API key globally
- **React Query** (implicit): useApi hook likely wraps react-query for caching
- **Same UIs as desktop**: Most components mirror the Tauri pages (blockchain, wallet, transactions)

---

## Chapter 22: Docker Compose Deployment

**Repository path:** `ci/docker-compose/`

Configuration files and scripts for running the Bitcoin node and all UIs (Tauri backend, web UI) in Docker containers, orchestrated with Docker Compose.

```text
ci/docker-compose/
├── configs/
│   ├── docker-compose.yml           # Main service definition (node, miners, web)
│   ├── docker-compose.scale.sh      # Script to run multiple containers
│   ├── docker-compose.miner.yml     # Miner-only compose config
│   ├── docker-compose.webserver.yml # Web UI + API server config
│   ├── Dockerfile                   # Image build (Bitcoin node + Tauri backend)
│   ├── docker-entrypoint.sh         # Container startup script
│   ├── wait-for-node.sh             # Health check + wait utility
│   ├── scale-up.sh                  # Scale replica count
│   ├── scale-down.sh                # Remove replicas
│   ├── generate-compose-ports.sh    # Dynamic port assignment
│   └── Settings.toml                # Bitcoin node config (network, mining)
└── 01A-Docker-Compose-Code-Listings.md
```

**Key concepts:**
- **Multi-container**: Separate services for node, miner, web server
- **Network**: Services communicate via container DNS
- **Volumes**: Persistent blockchain data, config files
- **Scaling**: Scripts to add/remove miner replicas
- **Health checks**: Wait-for-node.sh ensures readiness before starting web UI

---

## Chapter 23: Kubernetes Deployment

**Repository path:** `ci/kubernetes/`

Kubernetes manifests for deploying the Bitcoin node and all services in a K8s cluster (scaling, health checks, network policies, autoscaling).

```text
ci/kubernetes/
├── manifests/
│   ├── deploy.sh                    # Script to apply all manifests
│   ├── undeploy.sh                  # Script to remove all resources
│   ├── kustomization.yaml           # Kustomize aggregation of all resources
│   ├── 01-namespace.yaml            # Namespace for all resources
│   ├── 02-configmap.yaml            # ConfigMap (Bitcoin settings)
│   ├── 03-secrets.yaml              # Secret (API keys, passwords)
│   ├── 04-pvc-miner.yaml            # PVC for miner data
│   ├── 05-pvc-webserver.yaml        # PVC for web UI state
│   ├── 06-statefulset-miner.yaml    # StatefulSet for miners (ordered names)
│   ├── 07-deployment-webserver.yaml # Deployment for web server
│   ├── 08-service-miner.yaml        # Headless service for miner StatefulSet
│   ├── 08-service-miner-headless.yaml
│   ├── 09-service-webserver.yaml    # ClusterIP service for web UI
│   ├── 09-service-webserver-headless.yaml
│   ├── 10-hpa-webserver.yaml        # Horizontal Pod Autoscaler (web)
│   ├── 11-hpa-miner.yaml            # Horizontal Pod Autoscaler (miners)
│   ├── 12-pod-disruption-budget.yaml # Ensure availability during evictions
│   ├── 13-network-policy.yaml       # Network segmentation (pod-to-pod rules)
│   ├── 14-configmap-rate-limit.yaml # Rate limiting config
│   ├── 15-redis.yaml                # Redis cache for sessions/rate limiting
│   └── 01A-Kubernetes-Code-Listings.md
```

**Key concepts:**
- **StatefulSet**: Miners use ordered, persistent identities (miner-0, miner-1, etc.)
- **Deployment**: Web servers are stateless, can scale freely
- **Headless services**: Miner DNS (miner-N.miner-headless.default.svc.cluster.local)
- **HPA**: Auto-scale pods based on CPU/memory metrics
- **Network policies**: Restrict which pods can talk to which
- **ConfigMap + Secrets**: Externalize Bitcoin settings and credentials
- **PVCs**: Persistent volumes for data across pod restarts

---

## Whitepaper Supplement: nBits / Target Expansion

**Repository path:** `bitcoin-blockchain/whitepaper-rust/04A-nBits-Target-Expansion.md`

This is a focused mathematical deep-dive (16 lines of introduction + 150+ lines of proofs and examples) explaining how Bitcoin's compact difficulty encoding (`nBits`) expands into a 256-bit Proof-of-Work target. It supports Chapter 9 (Blockchain) and the PoW discussion in the whitepaper section.

**Key topics:**
- Compact encoding definition (exponent + mantissa)
- Expansion formula: `target = mantissa * 256^(exp-15)`
- Proof that exp equals the expanded target's byte-width
- Equivalence to bit-shifting: `mantissa << (8 * (exp - 3))`

**Not a code section** — this is pure mathematics supporting the cryptography chapters. No source tree.

---

## Summary

| Chapter | Module | Framework | Repo Path |
|---------|--------|-----------|-----------|
| 16 | Desktop Admin | Iced (Rust GUI) | `bitcoin-desktop-ui-iced/` |
| 17 | Desktop Admin | Tauri (Rust + React) | `bitcoin-desktop-ui-tauri/` |
| 18 | Wallet UI | Iced (Rust GUI) | `bitcoin-wallet-ui-iced/` |
| 19 | Wallet UI | Tauri (Rust + React) | `bitcoin-wallet-ui-tauri/` |
| 6 | Embedded DB | SQLCipher | `(wallet UIs)` |
| 7 | Web Admin | React/TypeScript | `bitcoin-web-ui/` |
| 8 | Docker Compose | Docker | `ci/docker-compose/` |
| 9 | Kubernetes | K8s manifests | `ci/kubernetes/` |
| — | Whitepaper | Math/Encoding | `bitcoin-blockchain/whitepaper-rust/` |

Each companion chapter (A/B/C suffixed files) contains complete source listings keyed to these directory trees.

