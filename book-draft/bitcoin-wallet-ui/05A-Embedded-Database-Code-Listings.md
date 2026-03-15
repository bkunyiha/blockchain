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
16. <a href="../bitcoin-desktop-ui/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../README.md)**

</div>

---

## Chapter 6A: Embedded Database — Complete Code Listings

This companion chapter contains **complete, verbatim listings** of the `bitcoin-wallet-ui` embedded persistence module (`src/database.rs`), as referenced by Chapter 6.

Chapter 6 contains **annotated walkthroughs** (with inline explanatory comments) of key methods. The listing here remains unmodified so you can cross-check against the exact implementation.

---

## Listing 6.1: `src/database.rs`

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

<div align="center">

**Reading order**

**[← Previous: Embedded Database & Persistence](05-Embedded-Database.md)** | **[Next: Web Admin Interface →](../bitcoin-web-ui/06-Web-Admin-UI.md)**

</div>

---
