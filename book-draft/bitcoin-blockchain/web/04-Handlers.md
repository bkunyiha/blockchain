<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. **Chapter 3: Web API Architecture** ← *You are here*
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 3.4: Request Handlers

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**[📚 ← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.4: Request Handlers** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Request Handlers

Handlers contain the business logic for processing requests. Each handler is an async function that extracts data from the request, processes it, and returns a response.

### Handler Pattern

All handlers in the `handlers/` directory follow a consistent pattern:

```rust
pub async fn handler_name(
    State(node): State<Arc<NodeContext>>,
    // ... other extractors (Path, Query, Json)
) -> Result<Json<ApiResponse<T>>, StatusCode> {
    // 1. Extract and validate input
    // 2. Process the request using NodeContext
    // 3. Build response
    // 4. Return success or error
}
```

**Key Components:**

1. **State Extraction**: `State(node)` gives us access to the blockchain node - see [State Injection in Axum](Axum.md#state-injection) for detailed technical information on state injection, type requirements, and shared ownership patterns
2. **Input Extraction**: `Path`, `Query`, `Json` extract data from the request - see [Request Extractors in Axum](Axum.md#request-extractors) for comprehensive details on all extractor types, type-safe extraction, and custom extractors. The `Json` extractor uses Serde for deserialization - see [Request Deserialization in Serde](Serde.md#request-deserialization)
3. **Return Type**: `Result<Json<ApiResponse<T>>, StatusCode>` provides type-safe responses - see [Response Types in Axum](Axum.md#response-types) for detailed information on response types, JSON responses, and error responses. The `Json` wrapper uses Serde for serialization - see [Response Serialization in Serde](Serde.md#response-serialization)
4. **Error Handling**: Errors are converted to appropriate HTTP status codes - see [Error Handling in Axum](Axum.md#error-handling) for comprehensive error handling patterns, error conversion, and error middleware
5. **Async/Await**: All handlers are async functions - see [Async/Await in Axum](Axum.md#asyncawait) for details on async handler patterns and concurrency

### Blockchain Handlers

Blockchain handlers in `handlers/blockchain.rs` provide access to blockchain data:

**Get Blockchain Info:**

The `get_blockchain_info()` function in `handlers/blockchain.rs`:

```rust
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    let height = node.get_blockchain_height().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let last_block = node.blockchain().get_last_block().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mempool_size = node.get_mempool_size()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let info = BlockchainInfoResponse {
        height,
        difficulty: 1,
        total_blocks: height,
        total_transactions: /* ... */,
        mempool_size,
        last_block_hash: /* ... */,
        last_block_timestamp: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(info)))
}
```

**What This Handler Does:**

1. **Extracts blockchain data**: Gets height, last block, mempool size
2. **Calculates statistics**: Computes total transactions
3. **Builds response**: Creates a structured response with all the information
4. **Error handling**: Converts errors to HTTP status codes

**Get Block by Hash:**

The `get_block_by_hash()` function in `handlers/blockchain.rs`:

```rust
pub async fn get_block_by_hash(
    State(node): State<Arc<NodeContext>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    let block = node.get_block_by_hash(&hash).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match block {
        Some(block) => {
            let response = BlockResponse {
                hash: block.get_hash().to_string(),
                previous_hash: block.get_previous_hash().to_string(),
                // ... other fields
            };
            Ok(Json(ApiResponse::success(response)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
```

**Path Parameter Extraction:**

The `Path(hash)` extractor automatically extracts the `{hash}` parameter from the URL. Axum handles the parsing and validation for us. See [Request Extractors in Axum](Axum.md#request-extractors) for detailed technical information on path extraction, type-safe path parameters, and extraction patterns.

### Wallet Handlers

Wallet handlers in `handlers/wallet.rs` manage wallet operations:

**Create Wallet:**

The `create_wallet()` function in `handlers/wallet.rs`:

```rust
pub async fn create_wallet(
    State(_node): State<Arc<NodeContext>>,
    Json(_request): Json<CreateWalletRequest>,
) -> Result<Json<ApiResponse<WalletResponse>>, StatusCode> {
    let mut wallet_service = WalletService::new()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let address = wallet_service.create_wallet()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let wallet = wallet_service.get_wallet(&address)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = WalletResponse {
        address: address.as_string(),
        public_key: HEXLOWER.encode(wallet.get_public_key()),
        created_at: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(response)))
}
```

> **📘 Response Conversion**: The `WalletResponse` struct is converted to an HTTP response through Axum's `IntoResponse` trait. See [Example: Converting WalletResponse to HTTP Response](Axum.md#example-converting-walletresponse-to-http-response) in the Axum Framework Guide for a detailed step-by-step explanation of how `WalletResponse` → `ApiResponse<WalletResponse>` → `Json<ApiResponse<WalletResponse>>` → HTTP `Response`.

**Get Balance:**

The `get_balance()` function in `handlers/wallet.rs`:

```rust
pub async fn get_balance(
    State(node): State<Arc<NodeContext>>,
    Path(address): Path<WalletAddress>,
) -> Result<Json<ApiResponse<BalanceResponse>>, StatusCode> {
    let balance = node.get_balance(&address).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let utxo_set = UTXOSet::new(node.blockchain().clone());
    let utxo_count = utxo_set.utxo_count(&address).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = BalanceResponse {
        address: address.as_string(),
        balance,
        utxo_count,
        updated_at: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(response)))
}
```

**Type-Safe Path Parameters:**

Notice that `Path(address)` extracts a `WalletAddress` directly, not a `String`. This is because Axum can deserialize path parameters into custom types, providing compile-time type safety. See [Request Extractors in Axum](Axum.md#request-extractors) for detailed technical information on type-safe extraction, custom type deserialization, and extractor composition.

### Transaction Handlers

Transaction handlers in `handlers/transaction.rs` manage transaction operations:

**Send Transaction:**

The `send_transaction()` function in `handlers/transaction.rs`:

```rust
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    let txid = node.btc_transaction(
        &request.from_address,
        &request.to_address,
        request.amount
    ).await.map_err(|e| {
        error!("Failed to create transaction: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    info!("Transaction {} submitted successfully", txid);

    let response = SendBitCoinResponse {
        txid,
        timestamp: chrono::Utc::now(),
    };

    Ok(Json(ApiResponse::success(response)))
}
```

**Error Handling in Handlers:**

When `btc_transaction` fails, we:
1. Log the error for debugging
2. Map it to `BAD_REQUEST` (400) status code
3. Return early with the error

This provides clear feedback to the client about what went wrong.

### Mining Handlers

Mining handlers in `handlers/mining.rs` provide mining operations:

**Get Mining Info:**

The `get_mining_info()` function in `handlers/mining.rs`:

```rust
pub async fn get_mining_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<MiningInfoResponse>>, StatusCode> {
    let height = node.get_blockchain_height().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mempool_size = node.get_mempool_size()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = MiningInfoResponse {
        blocks: height as u64,
        currentblocksize: 0,
        currentblocktx: mempool_size as u32,
        difficulty: 1.0,
        networkhashps: 0.0,
        pooledtx: mempool_size as u32,
    };

    Ok(Json(ApiResponse::success(response)))
}
```

**Generate to Address:**

The `generate_to_address()` function in `handlers/mining.rs`:

```rust
pub async fn generate_to_address(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<GenerateToAddressRequest>,
) -> Result<Json<ApiResponse<GenerateToAddressResponse>>, StatusCode> {
    // Validate address
    let address = WalletAddress::from_string(&request.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Generate blocks
    let block_hashes = (0..request.nblocks)
        .map(|_| {
            broadcast_new_block(&node, &address).await
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = GenerateToAddressResponse {
        block_hashes,
    };

    Ok(Json(ApiResponse::success(response)))
}
```

### Health Check Handlers

Health check handlers in `handlers/health.rs` provide system status:

**Health Check:**

The `health_check()` function in `handlers/health.rs`:

```rust
pub async fn health_check(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let height = node.get_blockchain_height().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let connected_peers = node.get_peer_count().unwrap_or(0);

    let health_response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: /* ... */,
        blockchain_height: height,
        connected_peers,
        memory_usage_mb: 0.0,
    };

    Ok(Json(ApiResponse::success(health_response)))
}
```

**Liveness Probe:**

The `liveness()` function in `handlers/health.rs`:

```rust
pub async fn liveness() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("alive".to_string())))
}
```

**Readiness Probe:**

The `readiness()` function in `handlers/health.rs`:

```rust
pub async fn readiness(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match node.get_blockchain_height().await {
        Ok(_) => Ok(Json(ApiResponse::success("ready".to_string()))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
```

**Why Three Health Endpoints?**

- **`/health`**: Comprehensive health check with detailed metrics
- **`/health/live`**: Simple liveness check (is the process running?)
- **`/health/ready`**: Readiness check (can the service handle requests?)

Kubernetes uses `/health/live` to determine if a pod should be restarted, and `/health/ready` to determine if traffic should be routed to the pod.

---

## Navigation

- **← Previous: Routing System** - Organizing endpoints and route definitions
- **Next: Middleware Layer →** - Cross-cutting concerns: authentication, CORS, logging
- **Web API Index** - Overview and navigation
- **Axum Framework Guide** - Detailed Axum feature explanations
- **Serde Framework Guide** - Serialization framework details

---

<div align="center">

**[📚 ← Previous: Routing](03-Routing.md)** | **[Chapter 3.4: Request Handlers](04-Handlers.md)** | **[Next: Middleware →](05-Middleware.md)** 📚

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers request handlers. Continue to Middleware Layer to learn about authentication, CORS, and error handling.*
