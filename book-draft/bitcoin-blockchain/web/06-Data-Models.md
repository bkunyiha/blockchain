<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../wallet/README.md) - Wallet implementation and key management
15. **Chapter 3: Web API Architecture** ← *You are here*
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../../ci/kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 3.6: Data Models

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.6: Data Models** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Data Models

Data models define the structure of requests and responses. They provide type safety and automatic serialization/deserialization. These models use Serde for JSON serialization and Utoipa for OpenAPI schema generation. 

**Axum Integration**: Data models are used with Axum's `Json` extractor for request deserialization and `Json` response wrapper for serialization. See [Request Extractors in Axum](Axum.md#request-extractors) for details on JSON extraction and [Response Types in Axum](Axum.md#response-types) for JSON response handling.

For detailed information on serialization frameworks, see the [Serde Framework Guide](Serde.md) and [Utoipa Framework Guide](Utoipa.md).

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
    pub utxo_count: usize,
    pub updated_at: DateTime<Utc>,
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
    
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
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

- **[← Previous: Middleware Layer](05-Middleware.md)** - Cross-cutting concerns: authentication, CORS, logging
- **[Next: Error Handling →](07-Error-Handling.md)** - Comprehensive error management strategies
- **[Web API Index](README.md)** - Overview and navigation
- **[Serde Framework Guide](Serde.md)** - Detailed Serde feature explanations
- **[Utoipa Framework Guide](Utoipa.md)** - OpenAPI schema generation details

---

<div align="center">

**📚 [← Previous: Middleware](05-Middleware.md)** | **Chapter 3.6: Data Models** | **[Next: Error Handling →](07-Error-Handling.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers data models. Continue to [Error Handling](07-Error-Handling.md) to learn about comprehensive error management strategies.*
