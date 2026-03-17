<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. <a href="../node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 15.6: Data Models

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.6: Data Models** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)** 

</div>

---

## Data Models

Data models define the structure of requests and responses. They provide type safety and automatic serialization/deserialization. These models use Serde for JSON serialization and Utoipa for OpenAPI schema generation. 

**Axum Integration**: Data models are used with Axum's `Json` extractor for request deserialization and `Json` response wrapper for serialization. See [Request Extractors in Axum](Axum.md#request-extractors) for details on JSON extraction and [Response Types in Axum](Axum.md#response-types) for JSON response handling.

For detailed information on serialization frameworks, see the Serde Framework Guide and Utoipa Framework Guide.

### Response Models

**Generic API Response:**

All API responses use a consistent wrapper defined in `models/responses.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}
```

The `success()` and `error()` methods in `models/responses.rs`:

```rust
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}
```

**Why This Structure?**

- **Consistent Format**: All responses follow the same structure
- **Type Safety**: Generic `T` ensures compile-time type checking
- **Error Handling**: Errors are clearly distinguished from success
- **Timestamps**: Every response includes a timestamp for debugging

**Example Response:**

```json
{
  "success": true,
  "data": {
    "height": 100,
    "total_blocks": 100,
    "mempool_size": 5
  },
  "error": null,
  "timestamp": "2024-01-01T12:00:00Z"
}
```

**Blockchain Info Response:**

Defined in `models/responses.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BlockchainInfoResponse {
    pub height: usize,
    pub difficulty: u32,
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub mempool_size: usize,
    pub last_block_hash: String,
    pub last_block_timestamp: DateTime<Utc>,
}
```

**Block Response:**

Defined in `models/responses.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BlockResponse {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub height: usize,
    pub nonce: u64,
    pub difficulty: u32,
    pub transaction_count: usize,
    pub merkle_root: String,
    pub size_bytes: usize,
}
```

**Transaction Response:**

Defined in `models/responses.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub txid: String,
    pub is_coinbase: bool,
    pub input_count: usize,
    pub output_count: usize,
    pub total_input_value: i32,
    pub total_output_value: i32,
    pub fee: i32,
    pub timestamp: DateTime<Utc>,
    pub size_bytes: usize,
}
```

**Wallet Response:**

Defined in `models/responses.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletResponse {
    pub address: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
}
```

**Balance Response:**

Defined in `models/responses.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: i32,
    pub unconfirmed_balance: i32,
    pub utxo_count: usize,
    pub last_updated: DateTime<Utc>,
}
```

### Request Models

Request models in `models/requests.rs` define the structure of incoming requests and include validation:

**Create Wallet Request:**

Defined in `models/requests.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateWalletRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Wallet name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
}
```

**Send Transaction Request:**

Defined in `models/requests.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct SendTransactionRequest {
    pub from_address: WalletAddress,
    pub to_address: WalletAddress,
    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i32,
}
```

**Block Query:**

Defined in `models/requests.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct BlockQuery {
    #[validate(range(min = 0, message = "Page must be 0 or greater"))]
    pub page: Option<u32>,

    #[validate(
        range(
            min = 1,
            max = 100,
            message = "Limit must be between 1 and 100"
        )
    )]
    pub limit: Option<u32>,
    
    pub hash: Option<String>,
}
```

**Validation:**

The `validator` crate provides compile-time and runtime validation:
- **Length validation**: Ensures strings are within bounds
- **Range validation**: Ensures numbers are within bounds
- **Custom validation**: Can be added for complex rules

### Error Models

Error models in `models/errors.rs` provide structured error responses:

**ErrorResponse struct:**

Defined in `models/errors.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
    pub timestamp: DateTime<Utc>,
}
```

**WebError Enum:**

Defined in `models/errors.rs`:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum WebError {
    ValidationError(String),
    NotFound(String),
    InternalError(String),
    Unauthorized(String),
    RateLimitExceeded,
    InvalidRequest(String),
    ServiceUnavailable(String),
}
```

**Error to Status Code Mapping:**

The `status_code()` method in `models/errors.rs` maps errors to HTTP status codes:

```rust
impl WebError {
    pub fn status_code(&self) -> u16 {
        match self {
            WebError::ValidationError(_) => 400,
            WebError::NotFound(_) => 404,
            WebError::InternalError(_) => 500,
            WebError::Unauthorized(_) => 401,
            WebError::RateLimitExceeded => 429,
            WebError::InvalidRequest(_) => 400,
            WebError::ServiceUnavailable(_) => 503,
        }
    }
}
```

---

## Navigation

- **← Previous: Middleware Layer** - Cross-cutting concerns: authentication, CORS, logging
- **Next: Error Handling →** - Comprehensive error management strategies
- **Web API Index** - Overview and navigation
- **Serde Framework Guide** - Detailed Serde feature explanations
- **Utoipa Framework Guide** - OpenAPI schema generation details

---

<div align="center">

**[← Previous: Middleware](05-Middleware.md)** | **[Chapter 15.6: Data Models](06-Data-Models.md)** | **[Next: Error Handling →](07-Error-Handling.md)** 

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers data models. Continue to Error Handling to learn about comprehensive error management strategies.*
