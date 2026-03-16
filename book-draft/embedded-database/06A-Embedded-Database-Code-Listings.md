<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a>
3. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a>
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a>
5. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a>
6. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a>
7. <a href="../bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a>
8. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a>
9. <a href="../bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a>
10. <a href="../bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a>
11. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance</a>
12. <a href="../bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a>
13. <a href="../bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a>
14. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a>
15. <a href="../bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a>
16. <a href="../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 4.1: Desktop Admin (Iced)</a>
17. <a href="../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">4.1A: Code Walkthrough</a>
18. <a href="../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">4.1B: Update Loop</a>
19. <a href="../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">4.1C: View Layer</a>
20. <a href="../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 4.2: Desktop Admin (Tauri)</a>
21. <a href="../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">4.2A: Rust Backend</a>
22. <a href="../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">4.2B: Frontend Infrastructure</a>
23. <a href="../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">4.2C: Frontend Pages</a>
24. <a href="../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 5.1: Wallet UI (Iced)</a>
25. <a href="../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">5.1A: Code Listings</a>
26. <a href="../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 5.2: Wallet UI (Tauri)</a>
27. <a href="../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">5.2A: Rust Backend</a>
28. <a href="../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">5.2B: Frontend Infrastructure</a>
29. <a href="../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">5.2C: Frontend Pages</a>
30. <a href="06-Embedded-Database.md">Chapter 6: Embedded Database</a>
31. **6A: Code Listings** ← *You are here*
32. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a>
33. <a href="../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">7A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a>
35. <a href="../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">8A: Code Listings</a>
36. <a href="../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a>
37. <a href="../ci/kubernetes/01A-Kubernetes-Code-Listings.md">9A: Code Listings</a>

### Part III: Language Reference

38. <a href="../rust/README.md">Chapter 10: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../README.md)**

</div>

---

## Chapter 6A: Embedded Database — Complete Code Listings

This companion chapter contains **complete, verbatim listings** of the SQLCipher persistence module from both wallet applications. Chapter 6 contains the **annotated walkthroughs** with explanatory comments. The listings here are reproduced unmodified so you can cross-check against the exact implementation.

The database module exists in two places:

- **Iced wallet**: `bitcoin-wallet-ui-iced/src/database.rs` — single file
- **Tauri wallet**: `bitcoin-wallet-ui-tauri/src-tauri/src/database/mod.rs` + `tests.rs` — module directory

Both are presented below. The code is nearly identical; we include both so the reader can compare without opening the repository.

---

## Listing 6.1: Iced Wallet `src/database.rs`

> **Methods involved**
> - `init_database`
> - `get_connection`
> - `get_database_path`
> - `create_tables`
> - `run_migrations`
> - `load_settings`
> - `save_settings`
> - `WalletAddress::new`
> - `save_wallet_address`
> - `load_wallet_addresses`

```rust
use rusqlite::{Connection, Result as SqliteResult, params};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;

/// Table column information from PRAGMA table_info
type TableColumnInfo = (
    String,         // name
    String,         // type
    Option<String>, // notnull
    Option<String>, // default_value
    Option<String>, // pk
    bool,           // placeholder (unused)
);

// Database file location
const DB_FILENAME: &str = "bitcoin-wallet.db";

// Global database connection (thread-safe)
static DB_CONN: OnceLock<Mutex<Connection>> = OnceLock::new();

// Database schema version for migrations
const SCHEMA_VERSION: i32 = 2;

/// Initialize the database connection with SQLCipher encryption
pub fn init_database(password: &str) -> SqliteResult<()> {
    let db_path = get_database_path();

    // Ensure database directory exists (creates if missing, no-op if exists)
    // create_dir_all() returns Ok(()) if directory already exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Failed to create database directory: {}", e)),
            )
        })?;
    }

    // Open or create the database
    let conn = Connection::open(&db_path)?;

    // Set SQLCipher key
    conn.pragma_update(None, "key", password)?;

    // Enable foreign keys
    conn.pragma_update(None, "foreign_keys", "ON")?;

    // Create tables
    create_tables(&conn)?;

    // Run migrations
    run_migrations(&conn)?;

    // Store connection globally
    DB_CONN.set(Mutex::new(conn)).map_err(|_| {
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
            Some("Database already initialized".to_string()),
        )
    })?;

    Ok(())
}

/// Get the database connection
fn get_connection() -> SqliteResult<std::sync::MutexGuard<'static, Connection>> {
    DB_CONN
        .get()
        .ok_or_else(|| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                Some("Database not initialized".to_string()),
            )
        })?
        .lock()
        .map_err(|_| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                Some("Failed to acquire database lock".to_string()),
            )
        })
}

/// Get the database file path
fn get_database_path() -> PathBuf {
    // Use application data directory
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("bitcoin-wallet-ui");
    path.push(DB_FILENAME);
    path
}

/// Create database tables
fn create_tables(conn: &Connection) -> SqliteResult<()> {
    // Settings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            base_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:8080',
            api_key TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Insert default settings if table is empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;

    if count == 0 {
        conn.execute(
            "INSERT INTO settings (id, base_url, api_key) VALUES (1, 'http://127.0.0.1:8080', ?)",
            params![
                std::env::var("BITCOIN_API_WALLET_KEY")
                    .unwrap_or_else(|_| "wallet-secret".to_string())
            ],
        )?;
    }

    // Wallet addresses table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallet_addresses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            address TEXT NOT NULL UNIQUE,
            label TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Create index on address for faster lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_wallet_addresses_address ON wallet_addresses(address)",
        [],
    )?;

    // User table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            first_name TEXT,
            last_name TEXT,
            profile_picture BLOB,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Schema version table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        )",
        [],
    )?;

    // Set initial schema version if not exists
    let version_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM schema_version", [], |row| row.get(0))?;

    if version_count == 0 {
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (?)",
            params![SCHEMA_VERSION],
        )?;
    }

    Ok(())
}

/// Run database migrations
fn run_migrations(conn: &Connection) -> SqliteResult<()> {
    let current_version: i32 = conn
        .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    // Migration 1 -> 2: Change profile_picture_path (TEXT) to profile_picture (BLOB)
    if current_version < 2 {
        // Check if old column exists
        let table_info: Vec<TableColumnInfo> = conn
            .prepare("PRAGMA table_info(users)")?
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(1)?, // name
                    row.get::<_, String>(2)?, // type
                    row.get(3)?,              // notnull
                    row.get(4)?,              // default_value
                    row.get(5)?,              // pk
                    false,                    // placeholder
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let has_old_column = table_info
            .iter()
            .any(|(name, _, _, _, _, _)| name == "profile_picture_path");
        let has_new_column = table_info
            .iter()
            .any(|(name, _, _, _, _, _)| name == "profile_picture");

        if has_old_column && !has_new_column {
            // Migrate: rename column and change type
            // SQLite doesn't support ALTER COLUMN, so we need to recreate the table
            conn.execute_batch(
                "BEGIN TRANSACTION;
                 CREATE TABLE users_new (
                     id INTEGER PRIMARY KEY CHECK (id = 1),
                     first_name TEXT,
                     last_name TEXT,
                     profile_picture BLOB,
                     created_at TEXT NOT NULL DEFAULT (datetime('now')),
                     updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                 );
                 INSERT INTO users_new (id, first_name, last_name, created_at, updated_at)
                 SELECT id, first_name, last_name, created_at, updated_at FROM users;
                 DROP TABLE users;
                 ALTER TABLE users_new RENAME TO users;
                 COMMIT;",
            )?;
        } else if !has_new_column {
            // Table exists but doesn't have profile_picture column yet
            // Add the new column (SQLite will set it to NULL for existing rows)
            conn.execute("ALTER TABLE users ADD COLUMN profile_picture BLOB", [])?;

            // Drop old column if it exists (SQLite doesn't support DROP COLUMN directly)
            if has_old_column {
                // Recreate table without old column
                conn.execute_batch(
                    "BEGIN TRANSACTION;
                     CREATE TABLE users_new (
                         id INTEGER PRIMARY KEY CHECK (id = 1),
                         first_name TEXT,
                         last_name TEXT,
                         profile_picture BLOB,
                         created_at TEXT NOT NULL DEFAULT (datetime('now')),
                         updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                     );
                     INSERT INTO users_new (id, first_name, last_name, created_at, updated_at)
                     SELECT id, first_name, last_name, created_at, updated_at FROM users;
                     DROP TABLE users;
                     ALTER TABLE users_new RENAME TO users;
                     COMMIT;",
                )?;
            }
        }

        // Update schema version
        conn.execute("UPDATE schema_version SET version = ?", params![2])?;
    }

    // Update to latest version if needed
    if current_version < SCHEMA_VERSION {
        conn.execute(
            "UPDATE schema_version SET version = ?",
            params![SCHEMA_VERSION],
        )?;
    }

    Ok(())
}

/// Settings structure
#[derive(Debug, Clone)]
pub struct Settings {
    pub base_url: String,
    pub api_key: String,
}

/// Load settings from database
pub fn load_settings() -> SqliteResult<Settings> {
    let conn = get_connection()?;

    conn.query_row(
        "SELECT base_url, api_key FROM settings WHERE id = 1",
        [],
        |row| {
            Ok(Settings {
                base_url: row.get(0)?,
                api_key: row.get(1)?,
            })
        },
    )
}

/// Save settings to database
pub fn save_settings(settings: &Settings) -> SqliteResult<()> {
    let conn = get_connection()?;

    conn.execute(
        "UPDATE settings SET base_url = ?, api_key = ?, updated_at = datetime('now') WHERE id = 1",
        params![settings.base_url, settings.api_key],
    )?;

    Ok(())
}

/// Wallet address structure
#[derive(Debug, Clone)]
pub struct WalletAddress {
    pub address: String,
    pub label: Option<String>,
    pub created_at: String,
}

impl WalletAddress {
    /// Create a new WalletAddress with just an address (for new addresses)
    pub fn new(address: String, label: Option<String>) -> Self {
        Self {
            address,
            label,
            created_at: String::new(), // Will be set by database
        }
    }
}

/// Save a wallet address to database
pub fn save_wallet_address(wallet: &WalletAddress) -> SqliteResult<WalletAddress> {
    let conn = get_connection()?;

    // Try to insert, or update if exists
    match conn.execute(
        "INSERT INTO wallet_addresses (address, label, updated_at) VALUES (?, ?, datetime('now'))",
        params![wallet.address, wallet.label],
    ) {
        Ok(_) => {
            // Get the inserted record with all fields
            conn.query_row(
                "SELECT address, label, created_at FROM wallet_addresses WHERE address = ?",
                params![wallet.address],
                |row| {
                    Ok(WalletAddress {
                        address: row.get(0)?,
                        label: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                },
            )
        }
        Err(rusqlite::Error::SqliteFailure(err, _))
            if err.code == rusqlite::ErrorCode::ConstraintViolation =>
        {
            // Address already exists, update it
            conn.execute(
                "UPDATE wallet_addresses SET label = ?, updated_at = datetime('now') WHERE address = ?",
                params![wallet.label, wallet.address],
            )?;
            // Return updated record
            conn.query_row(
                "SELECT address, label, created_at FROM wallet_addresses WHERE address = ?",
                params![wallet.address],
                |row| {
                    Ok(WalletAddress {
                        address: row.get(0)?,
                        label: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                },
            )
        }
        Err(e) => Err(e),
    }
}

/// Load all wallet addresses from database
pub fn load_wallet_addresses() -> SqliteResult<Vec<WalletAddress>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT address, label, created_at FROM wallet_addresses ORDER BY created_at DESC",
    )?;

    let addresses = stmt.query_map([], |row| {
        Ok(WalletAddress {
            address: row.get(0)?,
            label: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?;

    let mut result = Vec::new();
    for address in addresses {
        result.push(address?);
    }

    Ok(result)
}
```

---

## Listing 6.2: Tauri Wallet `src-tauri/src/database/mod.rs`

> **Methods involved**
> - `init_database`, `get_connection`, `get_database_path`
> - `create_tables`, `run_migrations`
> - `load_settings`, `save_settings_db`
> - `WalletAddress::new`, `save_wallet_address`, `load_wallet_addresses`
> - `delete_wallet_address_db`, `update_wallet_label_db`

The Tauri version adds `Serialize`/`Deserialize` on the data types (needed for Tauri IPC), renames `save_settings` to `save_settings_db`, and adds `delete_wallet_address_db` and `update_wallet_label_db` operations.

```rust
use rusqlite::{Connection, Result as SqliteResult, params};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;
use serde::{Serialize, Deserialize};

type TableColumnInfo = (
    String, String, Option<String>, Option<String>, Option<String>, bool,
);

const DB_FILENAME: &str = "bitcoin-wallet.db";
static DB_CONN: OnceLock<Mutex<Connection>> = OnceLock::new();
const SCHEMA_VERSION: i32 = 2;

pub fn init_database(password: &str) -> SqliteResult<()> {
    let db_path = get_database_path();

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Failed to create database directory: {}", e)),
            )
        })?;
    }

    let conn = Connection::open(&db_path)?;
    conn.pragma_update(None, "key", password)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    create_tables(&conn)?;
    run_migrations(&conn)?;

    DB_CONN.set(Mutex::new(conn)).map_err(|_| {
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
            Some("Database already initialized".to_string()),
        )
    })?;

    Ok(())
}

fn get_connection() -> SqliteResult<std::sync::MutexGuard<'static, Connection>> {
    DB_CONN
        .get()
        .ok_or_else(|| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                Some("Database not initialized".to_string()),
            )
        })?
        .lock()
        .map_err(|_| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                Some("Failed to acquire database lock".to_string()),
            )
        })
}

fn get_database_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("bitcoin-wallet-ui");
    path.push(DB_FILENAME);
    path
}

fn create_tables(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            base_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:8080',
            api_key TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO settings (id, base_url, api_key) VALUES (1, 'http://127.0.0.1:8080', ?)",
            params![
                std::env::var("BITCOIN_API_WALLET_KEY")
                    .unwrap_or_else(|_| "wallet-secret".to_string())
            ],
        )?;
    }

    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallet_addresses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            address TEXT NOT NULL UNIQUE,
            label TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_wallet_addresses_address ON wallet_addresses(address)",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            first_name TEXT,
            last_name TEXT,
            profile_picture BLOB,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        )",
        [],
    )?;

    let version_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM schema_version", [], |row| row.get(0))?;
    if version_count == 0 {
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (?)",
            params![SCHEMA_VERSION],
        )?;
    }

    Ok(())
}

fn run_migrations(conn: &Connection) -> SqliteResult<()> {
    let current_version: i32 = conn
        .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| row.get(0))
        .unwrap_or(0);

    if current_version < 2 {
        let table_info: Vec<TableColumnInfo> = conn
            .prepare("PRAGMA table_info(users)")?
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    false,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let has_old_column = table_info.iter().any(|(name, _, _, _, _, _)| name == "profile_picture_path");
        let has_new_column = table_info.iter().any(|(name, _, _, _, _, _)| name == "profile_picture");

        if has_old_column && !has_new_column {
            conn.execute_batch(
                "BEGIN TRANSACTION;
                 CREATE TABLE users_new (
                     id INTEGER PRIMARY KEY CHECK (id = 1),
                     first_name TEXT,
                     last_name TEXT,
                     profile_picture BLOB,
                     created_at TEXT NOT NULL DEFAULT (datetime('now')),
                     updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                 );
                 INSERT INTO users_new (id, first_name, last_name, created_at, updated_at)
                 SELECT id, first_name, last_name, created_at, updated_at FROM users;
                 DROP TABLE users;
                 ALTER TABLE users_new RENAME TO users;
                 COMMIT;",
            )?;
        } else if !has_new_column {
            conn.execute("ALTER TABLE users ADD COLUMN profile_picture BLOB", [])?;
            if has_old_column {
                conn.execute_batch(
                    "BEGIN TRANSACTION;
                     CREATE TABLE users_new (
                         id INTEGER PRIMARY KEY CHECK (id = 1),
                         first_name TEXT,
                         last_name TEXT,
                         profile_picture BLOB,
                         created_at TEXT NOT NULL DEFAULT (datetime('now')),
                         updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                     );
                     INSERT INTO users_new (id, first_name, last_name, created_at, updated_at)
                     SELECT id, first_name, last_name, created_at, updated_at FROM users;
                     DROP TABLE users;
                     ALTER TABLE users_new RENAME TO users;
                     COMMIT;",
                )?;
            }
        }

        conn.execute("UPDATE schema_version SET version = ?", params![2])?;
    }

    if current_version < SCHEMA_VERSION {
        conn.execute("UPDATE schema_version SET version = ?", params![SCHEMA_VERSION])?;
    }

    Ok(())
}

// ============== Public Types ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub base_url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub label: Option<String>,
    pub created_at: String,
}

impl WalletAddress {
    pub fn new(address: String, label: Option<String>) -> Self {
        Self { address, label, created_at: String::new() }
    }
}

// ============== CRUD Operations ==============

pub fn load_settings() -> SqliteResult<Settings> {
    let conn = get_connection()?;
    conn.query_row(
        "SELECT base_url, api_key FROM settings WHERE id = 1",
        [],
        |row| Ok(Settings { base_url: row.get(0)?, api_key: row.get(1)? }),
    )
}

pub fn save_settings_db(settings: &Settings) -> SqliteResult<()> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE settings SET base_url = ?, api_key = ?, updated_at = datetime('now') WHERE id = 1",
        params![settings.base_url, settings.api_key],
    )?;
    Ok(())
}

pub fn save_wallet_address(wallet: &WalletAddress) -> SqliteResult<WalletAddress> {
    let conn = get_connection()?;
    match conn.execute(
        "INSERT INTO wallet_addresses (address, label, updated_at) VALUES (?, ?, datetime('now'))",
        params![wallet.address, wallet.label],
    ) {
        Ok(_) => {
            conn.query_row(
                "SELECT address, label, created_at FROM wallet_addresses WHERE address = ?",
                params![wallet.address],
                |row| Ok(WalletAddress { address: row.get(0)?, label: row.get(1)?, created_at: row.get(2)? }),
            )
        }
        Err(rusqlite::Error::SqliteFailure(err, _))
            if err.code == rusqlite::ErrorCode::ConstraintViolation =>
        {
            conn.execute(
                "UPDATE wallet_addresses SET label = ?, updated_at = datetime('now') WHERE address = ?",
                params![wallet.label, wallet.address],
            )?;
            conn.query_row(
                "SELECT address, label, created_at FROM wallet_addresses WHERE address = ?",
                params![wallet.address],
                |row| Ok(WalletAddress { address: row.get(0)?, label: row.get(1)?, created_at: row.get(2)? }),
            )
        }
        Err(e) => Err(e),
    }
}

pub fn load_wallet_addresses() -> SqliteResult<Vec<WalletAddress>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT address, label, created_at FROM wallet_addresses ORDER BY created_at DESC",
    )?;
    let addresses = stmt.query_map([], |row| {
        Ok(WalletAddress { address: row.get(0)?, label: row.get(1)?, created_at: row.get(2)? })
    })?;
    let mut result = Vec::new();
    for address in addresses {
        result.push(address?);
    }
    Ok(result)
}

pub fn delete_wallet_address_db(address: &str) -> SqliteResult<()> {
    let conn = get_connection()?;
    conn.execute("DELETE FROM wallet_addresses WHERE address = ?", params![address])?;
    Ok(())
}

pub fn update_wallet_label_db(address: &str, label: &str) -> SqliteResult<()> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE wallet_addresses SET label = ?, updated_at = datetime('now') WHERE address = ?",
        params![label, address],
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
```

---

## Listing 6.3: Tauri Wallet `src-tauri/src/database/tests.rs`

> **Methods involved**
> - `setup_test_db` (test helper)
> - `test_schema_creation`
> - `test_settings_crud`
> - `test_wallet_address_crud`
> - `test_wallet_address_uniqueness`
> - `test_wallet_list_ordering`
> - `test_schema_version`
> - `test_settings_singleton_constraint`

```rust
#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    /// Helper: create an in-memory SQLCipher DB with the same schema
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        // No encryption for test DB (in-memory)
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                base_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:8080',
                api_key TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO settings (id, base_url, api_key) VALUES (1, 'http://127.0.0.1:8080', 'test-key')",
            [],
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wallet_addresses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address TEXT NOT NULL UNIQUE,
                label TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        ).unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_wallet_addresses_address ON wallet_addresses(address)",
            [],
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                first_name TEXT,
                last_name TEXT,
                profile_picture BLOB,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            )",
            [],
        ).unwrap();

        conn.execute("INSERT INTO schema_version (version) VALUES (2)", []).unwrap();

        conn
    }

    #[test]
    fn test_schema_creation() {
        let conn = setup_test_db();

        // Verify all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"settings".to_string()));
        assert!(tables.contains(&"wallet_addresses".to_string()));
        assert!(tables.contains(&"users".to_string()));
        assert!(tables.contains(&"schema_version".to_string()));
    }

    #[test]
    fn test_settings_crud() {
        let conn = setup_test_db();

        // Read default settings
        let (base_url, api_key): (String, String) = conn
            .query_row("SELECT base_url, api_key FROM settings WHERE id = 1", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();

        assert_eq!(base_url, "http://127.0.0.1:8080");
        assert_eq!(api_key, "test-key");

        // Update settings
        conn.execute(
            "UPDATE settings SET base_url = ?, api_key = ? WHERE id = 1",
            rusqlite::params!["http://localhost:9090", "new-key"],
        ).unwrap();

        let (base_url, api_key): (String, String) = conn
            .query_row("SELECT base_url, api_key FROM settings WHERE id = 1", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();

        assert_eq!(base_url, "http://localhost:9090");
        assert_eq!(api_key, "new-key");
    }

    #[test]
    fn test_wallet_address_crud() {
        let conn = setup_test_db();

        // Insert wallet
        conn.execute(
            "INSERT INTO wallet_addresses (address, label) VALUES (?, ?)",
            rusqlite::params!["addr_test_123", "My Wallet"],
        ).unwrap();

        // Read wallet
        let (address, label): (String, Option<String>) = conn
            .query_row(
                "SELECT address, label FROM wallet_addresses WHERE address = ?",
                rusqlite::params!["addr_test_123"],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();

        assert_eq!(address, "addr_test_123");
        assert_eq!(label, Some("My Wallet".to_string()));

        // Update label
        conn.execute(
            "UPDATE wallet_addresses SET label = ? WHERE address = ?",
            rusqlite::params!["Renamed Wallet", "addr_test_123"],
        ).unwrap();

        let label: Option<String> = conn
            .query_row(
                "SELECT label FROM wallet_addresses WHERE address = ?",
                rusqlite::params!["addr_test_123"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(label, Some("Renamed Wallet".to_string()));

        // Delete wallet
        conn.execute(
            "DELETE FROM wallet_addresses WHERE address = ?",
            rusqlite::params!["addr_test_123"],
        ).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM wallet_addresses", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_wallet_address_uniqueness() {
        let conn = setup_test_db();

        conn.execute(
            "INSERT INTO wallet_addresses (address, label) VALUES (?, ?)",
            rusqlite::params!["addr_unique_1", "First"],
        ).unwrap();

        // Duplicate should fail
        let result = conn.execute(
            "INSERT INTO wallet_addresses (address, label) VALUES (?, ?)",
            rusqlite::params!["addr_unique_1", "Duplicate"],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_wallet_list_ordering() {
        let conn = setup_test_db();

        conn.execute(
            "INSERT INTO wallet_addresses (address, label, created_at) VALUES (?, ?, ?)",
            rusqlite::params!["addr_1", "First", "2025-01-01 00:00:00"],
        ).unwrap();

        conn.execute(
            "INSERT INTO wallet_addresses (address, label, created_at) VALUES (?, ?, ?)",
            rusqlite::params!["addr_2", "Second", "2025-06-01 00:00:00"],
        ).unwrap();

        conn.execute(
            "INSERT INTO wallet_addresses (address, label, created_at) VALUES (?, ?, ?)",
            rusqlite::params!["addr_3", "Third", "2025-03-01 00:00:00"],
        ).unwrap();

        let wallets: Vec<(String, String)> = conn
            .prepare("SELECT address, label FROM wallet_addresses ORDER BY created_at DESC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get::<_, String>(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(wallets.len(), 3);
        assert_eq!(wallets[0].0, "addr_2"); // Most recent first
        assert_eq!(wallets[1].0, "addr_3");
        assert_eq!(wallets[2].0, "addr_1");
    }

    #[test]
    fn test_schema_version() {
        let conn = setup_test_db();

        let version: i32 = conn
            .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| row.get(0))
            .unwrap();

        assert_eq!(version, 2);
    }

    #[test]
    fn test_settings_singleton_constraint() {
        let conn = setup_test_db();

        // Try to insert a second settings row
        let result = conn.execute(
            "INSERT INTO settings (id, base_url, api_key) VALUES (2, 'test', 'test')",
            [],
        );

        // Should fail due to CHECK(id = 1)
        assert!(result.is_err());
    }
}
```

---

## Listing 6.4: `generate_database_password` (shared between both wallets)

> **Methods involved**
> - `generate_database_password` (Iced: `src/main.rs`, Tauri: `src-tauri/src/main.rs`)

This function is identical in both wallets. It appears in each wallet's `main.rs`:

```rust
/// Generate a secure database password
/// Uses a combination of machine-specific and user-specific data
/// This ensures the database is encrypted but doesn't require user input
fn generate_database_password() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();

    // Use username
    if let Ok(username) = std::env::var("USER") {
        username.hash(&mut hasher);
    } else if let Ok(username) = std::env::var("USERNAME") {
        username.hash(&mut hasher);
    }

    // Use home directory
    if let Some(home) = dirs::home_dir() {
        home.to_string_lossy().hash(&mut hasher);
    }

    // Use application name
    "bitcoin-wallet-ui".hash(&mut hasher);

    // Convert to hex string
    format!("{:x}", hasher.finish())
}
```

---

<div align="center">

**Reading order**

**[← Previous: Embedded Database & Persistence](06-Embedded-Database.md)** | **[Next: Web Admin Interface →](../bitcoin-web-ui/06-Web-Admin-UI.md)**

</div>

---
