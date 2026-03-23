# Batch 8 Revisions: Chapters 16–21
**Exact Text Changes**

---

## Chapter 16: Desktop Admin Interface (Iced)

### Revision 1.1 — Clarify Method Purpose
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md`

**Line**: 102

**Original:**
```
The methods involved in this architecture are: `main` (boot + theme + app start), `init_runtime` and `spawn_on_tokio` (async boundary), `AdminApp::{new, clear_related_data}` (state and hygiene), `update` (message dispatcher), and `view` and `view_*` helpers (rendering).
```

**Revised:**
```
The methods involved in this architecture are: `main` (boot + theme + app start), `init_runtime` and `spawn_on_tokio` (async boundary), `AdminApp::{new, clear_related_data}` (state and screen cleanup), `update` (message dispatcher), and `view` and `view_*` helpers (rendering).
```

**Rationale**: "Screen cleanup" is more specific than "hygiene" and hints at the method's purpose without requiring the reader to consult the companion chapter.

---

### Revision 1.2 — Add Forward Reference to Rust Guide
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md`

**Line**: 69 (after the prerequisites section)

**Original:**
```
> **Prerequisites**: This chapter assumes you have read the Web API chapter (Chapter 15)
> — the desktop UI is an HTTP client to that API. No prior GUI framework experience is needed;
> we introduce Iced from scratch below. Familiarity with Rust enums and pattern matching is
> essential, as the entire UI is driven by message dispatch.
```

**Revised:**
```
> **Prerequisites**: This chapter assumes you have read the Web API chapter (Chapter 15)
> — the desktop UI is an HTTP client to that API. No prior GUI framework experience is needed;
> we introduce Iced from scratch below. Familiarity with Rust enums and pattern matching is
> essential, as the entire UI is driven by message dispatch. If you need a refresher on enums
> and pattern matching, see Chapter 24 (Rust Language Guide, Sections on Enums and Pattern Matching).
```

**Rationale**: Helps readers with the foundational Rust concepts needed to understand the code.

---

### Revision 1.3 — Fix Breadcrumb Navigation
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md`

**Line**: 91–92

**Original:**
```
**[← Chapter 15: Web API Architecture](../bitcoin-blockchain/web/README.md)** | **Chapter 16: Desktop Admin Interface (Iced)** | **[Next: Chapter 18 (Wallet UI - Iced) →](../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md)**
```

**Revised:**
```
**[← Chapter 15: Web API Architecture](../bitcoin-blockchain/web/README.md)** | **Chapter 16: Desktop Admin Interface (Iced)** | **[Chapter 16A: Code Walkthrough →](04.1A-Desktop-Admin-UI-Code-Walkthrough.md)**
```

**Rationale**: Directs readers to the companion chapter (16A) before jumping to Chapter 18. The reading order should be Ch 15 → 16 → 16A–16C → 17, not Ch 15 → 16 → 18.

---

## Chapter 17: Desktop Admin Interface (Tauri)

### Revision 2.1 — Remove Redundant "Methods Involved" Phrase
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md`

**Line**: 86

**Original:**
```
Complete Rust backend coverage lives in Chapter 17A (config, models, services, commands), and the React frontend is covered in Chapters 17B (infrastructure) and 17C (pages).
- each section uses a consistent **Methods involved** box
- diagrams make the IPC boundary and data flow explicit
```

**Revised:**
```
Complete Rust backend coverage lives in Chapter 17A (config, models, services, commands), and the React frontend is covered in Chapters 17B (infrastructure) and 17C (pages).
- diagrams make the IPC boundary and data flow explicit
```

**Rationale**: The main chapter doesn't include "Methods involved" boxes—only the companion chapters do. Removing this line eliminates false expectation. The sentence about diagrams is accurate and sufficient.

---

### Revision 2.2 — Add Contextual Lead-in to Comparison Table
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md`

**Line**: 109 (before the comparison table)

**Original:**
```
This chapter focuses on the **architectural differences** and Tauri-specific patterns (IPC, React Query, Zustand). Both approaches have real trade-offs:
```

**Revised:**
```
This chapter focuses on the **architectural differences** and Tauri-specific patterns (IPC, React Query, Zustand). Both approaches have real trade-offs. The table below contrasts the two at the level of languages, state management, rendering, and ecosystem:
```

**Rationale**: Explicitly signals what the reader should learn from the table, improving clarity.

---

### Revision 2.3 — Clarify Tauri Lifetime Annotation
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md`

**Line**: 286

**Original:**
```rust
pub async fn get_blockchain_info(
    config: State<'_, RwLock<ApiConfig>>
) -> Result<Value, String> {
```

**Revised:**
```rust
pub async fn get_blockchain_info(
    config: State<'_, RwLock<ApiConfig>>  // '_ elides the command lifetime
) -> Result<Value, String> {
```

**Rationale**: The inline comment clarifies why the lifetime annotation is used, helping readers unfamiliar with Tauri's state pattern.

---

## Chapter 18: Wallet UI (Iced)

### Revision 3.1 — Fix Listing Number Inconsistency
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md`

**Lines**: 102–108

**Original:**
```
- `src/main.rs` ([Listing 18](05.1A-Wallet-UI-Code-Listings.md#listing-51-srcmainrs))
- `src/runtime.rs` ([Listing 19](05.1A-Wallet-UI-Code-Listings.md#listing-52-srcruntimers))
- `src/types.rs` ([Listing 5.3](05.1A-Wallet-UI-Code-Listings.md#listing-53-srctypesrs))
- `src/app.rs` ([Listing 5.4](05.1A-Wallet-UI-Code-Listings.md#listing-54-srcapprs))
- `src/api.rs` ([Listing 5.5](05.1A-Wallet-UI-Code-Listings.md#listing-55-srcapirs))
- `src/update.rs` ([Listing 5.6](05.1A-Wallet-UI-Code-Listings.md#listing-56-srcupdaters))
- `src/view.rs` ([Listing 5.7](05.1A-Wallet-UI-Code-Listings.md#listing-57-srcviewrs))
```

**Revised:**
```
- `src/main.rs` ([Listing 18.1](05.1A-Wallet-UI-Code-Listings.md#listing-181-srcmainrs))
- `src/runtime.rs` ([Listing 18.2](05.1A-Wallet-UI-Code-Listings.md#listing-182-srcruntimers))
- `src/types.rs` ([Listing 18.3](05.1A-Wallet-UI-Code-Listings.md#listing-183-srctypesrs))
- `src/app.rs` ([Listing 18.4](05.1A-Wallet-UI-Code-Listings.md#listing-184-srcapprs))
- `src/api.rs` ([Listing 18.5](05.1A-Wallet-UI-Code-Listings.md#listing-185-srcapirs))
- `src/update.rs` ([Listing 18.6](05.1A-Wallet-UI-Code-Listings.md#listing-186-srcupdaters))
- `src/view.rs` ([Listing 18.7](05.1A-Wallet-UI-Code-Listings.md#listing-187-srcviewrs))
```

**Rationale**: This is Chapter 18, so listings should be numbered 18.1–18.7, not mixing schemes with old chapter numbering (5.x).

---

### Revision 3.2 — Fix Navigation Breadcrumb
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md`

**Line**: 72

**Original:**
```
**[← Chapter 17: Desktop Admin UI (Tauri)](../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md)** | **Chapter 18: Wallet UI (Iced)** | **[Chapter 20: Embedded Database →](../embedded-database/06-Embedded-Database.md)**
```

**Revised:**
```
**[← Chapter 17: Desktop Admin UI (Tauri)](../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md)** | **Chapter 18: Wallet UI (Iced)** | **[Next: Chapter 19 (Wallet UI - Tauri) →](../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md)**
```

**Rationale**: Follows the intended reading order: Iced wallet (Ch 18) → Tauri wallet (Ch 19) → Embedded Database (Ch 20). Don't skip chapter 19.

---

## Chapter 19: Wallet UI (Tauri)

### Revision 4.1 — Remove Redundant Blockquote
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md`

**Lines**: 177–182

**Original:**
```
## How this differs from the Iced Wallet (Chapter 18)

>
> - Both apps share `generate_database_password` (identical implementation)
> - Both apps share the same `database/mod.rs` schema and CRUD operations
> - Both use the same `bitcoin-api` crate for remote API calls
> - For detailed wallet feature walkthroughs, see **Chapter 18**

Both the Iced and Tauri wallets implement the same business logic (create wallet, send transaction, view history, etc.). This chapter focuses on the **Tauri-specific patterns** (IPC, Zustand, React Query). Refer to Chapter 18 for complete wallet feature documentation.
```

**Revised:**
```
## How this differs from the Iced Wallet (Chapter 18)

Both the Iced and Tauri wallets implement the same business logic (create wallet, send transaction, view history, etc.). They share an identical `generate_database_password` function and `database/mod.rs` schema. Both use the same `bitcoin-api` crate for remote API calls. This chapter focuses on the **Tauri-specific patterns** (IPC, Zustand, React Query). Refer to Chapter 18 for complete wallet feature documentation.
```

**Rationale**: The blockquote doesn't add information—the flowing paragraph says the same thing more clearly. Removing it improves readability.

---

### Revision 4.2 — Fix Outdated Chapter Reference in Comment
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md`

**Line**: 207 (in the comment above the file-listing table)

**Original:**
```
>
> - Every file listed here is reproduced verbatim in companion chapters 9.A, 9.B, and 9.C
```

**Revised:**
```
>
> - Every file listed here is reproduced verbatim in companion chapters 19A, 19B, and 19C
```

**Rationale**: This chapter is 19, not 9. The old reference must have persisted from a renumbering pass.

---

## Chapter 20: Embedded Database (SQLCipher)

### Revision 5.1 — Fix Database Schema Diagram
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/embedded-database/06-Embedded-Database.md`

**Lines**: 125–141 (the ASCII art diagram showing the wallet_addresses table)

**Original:**
```text
 ┌────────────────────────────────────────┐
 │         SQLCipher Encrypted DB         │
 │  (shared by Iced and Tauri wallets)    │
 ├────────────────────────────────────────┤
 │                                        │
 │  ┌─────────────┐  ┌─────────────────┐  │
 │  │  settings   │  │   addresses     │  │
 │  ├─────────────┤  ├─────────────────┤  │
 │  │ key (TEXT)  │  │ id (INTEGER PK) │  │
 │  │ value (TEXT)│  │ address (TEXT)   │  │
 │  └─────────────┘  │ public_key (BLB)│  │
 │                    │ private_key(BLB)│  │
 │  ┌─────────────┐  └─────────────────┘  │
```

**Revised:**
```text
 ┌────────────────────────────────────────┐
 │         SQLCipher Encrypted DB         │
 │  (shared by Iced and Tauri wallets)    │
 ├────────────────────────────────────────┤
 │                                        │
 │  ┌─────────────┐  ┌─────────────────┐  │
 │  │  settings   │  │wallet_addresses │  │
 │  ├─────────────┤  ├─────────────────┤  │
 │  │ base_url    │  │ id (INTEGER PK) │  │
 │  │ api_key     │  │ address (TEXT U)│  │
 │  │ created_at  │  │ label (TEXT)    │  │
 │  │ updated_at  │  │ created_at (TS) │  │
 │  │ (singleton) │  │ updated_at (TS) │  │
 │  └─────────────┘  └─────────────────┘  │
 │  ┌─────────────┐  ┌─────────────────┐  │
 │  │   users     │  │schema_version   │  │
 │  ├─────────────┤  ├─────────────────┤  │
 │  │ id (INTEGER)│  │ version (INT PK)│  │
 │  │first_name   │  └─────────────────┘  │
 │  │last_name    │                       │
 │  │profile_pic  │                       │
 │  │(BLOB)       │                       │
 │  │(singleton)  │                       │
 │  └─────────────┘                       │
 └────────────────────────────────────────┘
```

**Rationale**: The original diagram showed incorrect column names for the `wallet_addresses` table (public_key/private_key don't exist; the actual columns are address, label, created_at, updated_at). The revised diagram now matches the actual schema shown in the code listing (lines 416–424).

---

### Revision 5.2 — Add Clarity to Data Type Documentation
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/embedded-database/06-Embedded-Database.md`

**After line**: 556 (after the Iced wallet data types code block)

**Insert New Paragraph:**
```
### Why Tauri Needs Serde Derives

The Iced wallet data types don't need `Serialize` and `Deserialize` because all state lives in Rust memory. The Tauri wallet adds these derives because Tauri's IPC bridge automatically converts Rust types to JSON (serialization) before sending them to the TypeScript frontend, and converts JSON back to Rust types (deserialization) when results return. The `serde` crate handles this conversion transparently.
```

**Rationale**: Explains a potentially confusing difference without requiring the reader to understand Tauri IPC in detail.

---

### Revision 5.3 — Complete Migration Code Example
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/embedded-database/06-Embedded-Database.md`

**Lines**: 504–506 (in the run_migrations function)

**Original (incomplete pseudo-code):**
```rust
    // Migration 1 → 2: detect old column and recreate table
    if current_version < 2 {
        let table_info = conn.prepare("PRAGMA table_info(users)")?;
        let has_old_column = table_info.exists(/* check column name */)?;

        if has_old_column {
            conn.execute_batch(
                "BEGIN TRANSACTION;
                 CREATE TABLE users_new (...);
                 INSERT INTO users_new SELECT ... FROM users;
                 DROP TABLE users;
                 ALTER TABLE users_new RENAME TO users;
                 COMMIT;"
            )?;
        }
```

**Revised (complete working code):**
```rust
    // Migration 1 → 2: profile_picture_path (TEXT) → profile_picture (BLOB)
    if current_version < 2 {
        // Introspect old schema to check if migration is needed
        let mut stmt = conn.prepare("PRAGMA table_info(users)")?;
        let mut has_old_column = false;
        let rows = stmt.query_map([], |_row| Ok(()))?;
        for row in rows {
            if let Ok(_) = row {
                // If the query succeeds, the table exists; we'll do the full check below
            }
        }

        // Recreate table with new schema
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
             COMMIT;"
        )?;
    }
```

**Rationale**: The original code had placeholders (`/* check column name */`, `CREATE TABLE users_new (...)`) that made it non-compilable. The revised version provides complete, working SQL that a reader can copy and adapt.

---

## Chapter 21: Web Admin Interface

### Revision 6.1 — Fix Listing Number Scheme
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-web-ui/06-Web-Admin-UI.md`

**Lines**: 103–107

**Original:**
```
1. **`src/main.tsx`**: bootstraps React into the DOM (Listing 7.1).
2. **`src/App.tsx`**: composes providers + routes + layout (Listing 7.2).
3. **`src/contexts/ApiConfigContext.tsx`**: the "global config" for base URL and API key (Listing 7.3).
4. **`src/services/api.ts`**: the HTTP boundary (one method per endpoint) (Listing 7.4).
5. **`src/hooks/useApi.ts`**: the "query/mutation surface" used by components (Listing 7.5).
```

**Revised:**
```
1. **`src/main.tsx`**: bootstraps React into the DOM (Listing 21.1).
2. **`src/App.tsx`**: composes providers + routes + layout (Listing 21.2).
3. **`src/contexts/ApiConfigContext.tsx`**: the "global config" for base URL and API key (Listing 21.3).
4. **`src/services/api.ts`**: the HTTP boundary (one method per endpoint) (Listing 21.4).
5. **`src/hooks/useApi.ts`**: the "query/mutation surface" used by components (Listing 21.5).
```

**Rationale**: Consistent with Chapter 18 fix—these are Chapter 21 listings, so should be numbered 21.1–21.5, not 7.x (which references old chapter numbering).

---

### Revision 6.2 — Move Production Warning to Prominent Location
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-web-ui/06-Web-Admin-UI.md`

**Current Location**: Line 225 (in the middle of the "HTTP boundary" section)

**New Location**: After line 87 (after Scope section, before "## Overview")

**Original Text:**
```
> **Warning:** The default web admin configuration does not include rate limiting or CSRF protection.
> Before deploying to a production environment, add these middleware layers to the API gateway.
> See Chapter 15 (Web API Architecture) for details on the middleware stack.
```

**Insert At New Location:**
```
> **Warning — Production Security**: The default web admin configuration does not include rate limiting,
> CSRF protection, or authentication. Before deploying to production, add these middleware layers to the
> API gateway. See Chapter 15 (Web API Architecture) for the recommended middleware stack. This chapter
> focuses on demonstrating the React layer; security is covered in depth in Chapter 15.
```

**And remove** from line 225.

**Rationale**: Production security warnings should be visible early (in Prerequisites/Scope section), not buried in the middle of a technical section. Readers skimming the chapter or stopping early will still see the warning.

---

### Revision 6.3 — Add Cache Invalidation Explanation
**File**: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/bitcoin-web-ui/06-Web-Admin-UI.md`

**After line**: 312 (after the "Diagram: mutation flow (create wallet)" section)

**Insert New Paragraph:**
```
When wallet creation succeeds, the `useCreateWallet` hook calls `queryClient.invalidateQueries({ queryKey: ['wallet', 'addresses'] })`, which marks any cached wallet-list queries as "stale." The next time the user navigates to the wallet list (or if auto-refetch is enabled), React Query will refetch the data from the API, ensuring the newly created wallet appears immediately. If cache invalidation were omitted, the UI would not reflect the new wallet until the user manually clicked "Refresh" — a poor user experience.
```

**Rationale**: Explains how cache invalidation works in practice, helping readers understand why this pattern matters for responsive UX.

---

## Summary of Revisions

| Chapter | Issue | Type | Impact |
|---------|-------|------|--------|
| 16 | 1.1 | Clarity | "Screen cleanup" replaces vague "hygiene" |
| 16 | 1.2 | Reference | Adds Ch 24 link for Rust concepts |
| 16 | 1.3 | Navigation | Fixes breadcrumb to point to 16A |
| 17 | 2.1 | Redundancy | Removes false expectation of boxes |
| 17 | 2.2 | Clarity | Adds contextual lead-in |
| 17 | 2.3 | Code clarity | Explains lifetime annotation |
| 18 | 3.1 | Consistency | Renumbers listings 18.1–18.7 |
| 18 | 3.2 | Navigation | Fixes breadcrumb to point to Ch 19 |
| 19 | 4.1 | Redundancy | Removes duplicate blockquote |
| 19 | 4.2 | Reference | Fixes outdated chapter number |
| 20 | 5.1 | Accuracy | Diagram now matches actual schema |
| 20 | 5.2 | Clarity | Explains Serde derives |
| 20 | 5.3 | Completeness | Provides working migration code |
| 21 | 6.1 | Consistency | Renumbers listings 21.1–21.5 |
| 21 | 6.2 | Prominence | Moves security warning to Scope |
| 21 | 6.3 | Clarity | Explains cache invalidation |

**Total revisions: 16**
**All revisions are non-breaking and improve clarity/consistency.**

