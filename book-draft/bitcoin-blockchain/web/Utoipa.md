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

# Utoipa Framework Guide

**Part I: Foundations & Core Implementation** | **Technical Reference: Utoipa OpenAPI Framework**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Utoipa Framework Guide** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

</div>

---

## Overview

This guide provides detailed explanations of the Utoipa framework and how it's used to generate OpenAPI documentation for our blockchain web API. Utoipa is a Rust framework for generating OpenAPI (formerly Swagger) specifications from Rust code. It integrates seamlessly with Axum and Serde to automatically generate comprehensive API documentation.

In our blockchain API, Utoipa is used for:
- **OpenAPI Specification**: Generating complete OpenAPI 3.0 specifications
- **Swagger UI**: Providing interactive API documentation
- **Schema Generation**: Automatically generating schemas from Rust types
- **Path Documentation**: Documenting endpoints with request/response details
- **Type Safety**: Ensuring documentation matches implementation

> **See the full implementation:**: This guide explains Utoipa concepts. To see how Utoipa is used in our complete web API architecture, see the OpenAPI Documentation chapter and the Data Models chapter.

---

## Table of Contents

1. [What is Utoipa?](#what-is-utoipa) - Understanding Utoipa's role
2. [OpenApi Derive Macro](#openapi-derive-macro) - Generating OpenAPI specs
3. [ToSchema Derive Macro](#toschema-derive-macro) - Generating schemas
4. [Path Documentation](#path-documentation) - Documenting endpoints
5. [Schema Documentation](#schema-documentation) - Documenting data models
6. [Swagger UI Integration](#swagger-ui-integration) - Interactive documentation
7. [Tags and Organization](#tags-and-organization) - Organizing endpoints
8. [Request/Response Documentation](#requestresponse-documentation) - Documenting API contracts
9. [Error Documentation](#error-documentation) - Documenting error responses
10. [Best Practices](#best-practices) - Effective documentation patterns

---

## What is Utoipa?

Utoipa is a Rust framework for generating OpenAPI specifications from Rust code. It uses Rust's type system and procedural macros to automatically generate accurate, up-to-date API documentation.

### OpenAPI Specification

OpenAPI (formerly Swagger) is a specification for describing REST APIs. It provides:
- **API Structure**: Endpoints, methods, paths
- **Request/Response Schemas**: Data structures
- **Authentication**: Security requirements
- **Examples**: Sample requests and responses

### Utoipa's Approach

Utoipa generates OpenAPI specifications by:
1. **Analyzing Rust Code**: Reads handler functions and data models
2. **Extracting Metadata**: Uses attributes to get documentation
3. **Generating Specs**: Creates OpenAPI 3.0 JSON/YAML
4. **Type Safety**: Ensures documentation matches implementation

### Why Utoipa?

- **Automatic**: Documentation generated from code
- **Type-Safe**: Compile-time checks ensure accuracy
- **Always Up-to-Date**: Documentation matches code automatically
- **Rich Metadata**: Supports descriptions, examples, tags
- **Swagger UI**: Interactive documentation out of the box

---

## OpenApi Derive Macro

The `#[derive(OpenApi)]` macro generates complete OpenAPI specifications.

### Implementation in `openapi.rs`

```rust
#[derive(OpenApi)]
#[openapi(
    paths(health::health_check, blockchain::get_blockchain_info),
    components(schemas(HealthResponse, BlockchainInfoResponse)),
    tags((name = "Health", description = "Health check endpoints")),
    info(title = "Blockchain API", version = "0.1.0"),
    servers((url = "http://localhost:8080"))
)]
pub struct ApiDoc;
```

The OpenAPI spec includes: Info (title, version), Servers (URLs), Paths (endpoints), Components (schemas), Tags (organization)

---

## ToSchema Derive Macro

The `#[derive(ToSchema)]` macro generates OpenAPI schemas from Rust types. It works alongside Serde's `Serialize` and `Deserialize`.

### Basic Usage

In our `models/responses.rs`:

```rust
use utoipa::ToSchema;

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

**What `ToSchema` does:**
- Analyzes the struct's fields
- Generates OpenAPI schema definition
- Includes field types, names, and optional descriptions
- Works with nested types automatically

### Generated Schema

The `ToSchema` macro generates OpenAPI schemas with proper type mappings for JSON serialization.

### Generic Types

`ToSchema` works with generic types. Our `ApiResponse<T>`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}
```

When used with a concrete type like `ApiResponse<BlockchainInfoResponse>`, Utoipa generates the appropriate schema with `T` replaced by `BlockchainInfoResponse`.

---

## Path Documentation

Use `#[utoipa::path(...)]` to document individual endpoints.

### Basic Path Documentation

```rust
/// Get blockchain information
///
/// Returns comprehensive blockchain statistics including height, difficulty,
/// total blocks, transactions, and mempool status.
#[utoipa::path(
    get,
    path = "/api/v1/blockchain",
    tag = "Blockchain",
    responses(
        (
            status = 200,
            description = "Blockchain information retrieved successfully",
            body = ApiResponse<BlockchainInfoResponse>
        ),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_blockchain_info(
    State(node): State<Arc<NodeContext>>,
) -> Result<Json<ApiResponse<BlockchainInfoResponse>>, StatusCode> {
    // ... handler implementation
}
```

**Path Attribute Parameters:**
- **`get`**: HTTP method (can be `get`, `post`, `put`, `delete`, etc.)
- **`path = "/api/v1/blockchain"`**: The endpoint path
- **`tag = "Blockchain"`**: Tag for organizing endpoints
- **`responses(...)`**: Documented response types and status codes

### Path Parameters

```rust
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks/{hash}",
    tag = "Blockchain",
    params(("hash" = String, Path, description = "Block hash")),
    responses((status = 200, body = ApiResponse<BlockResponse>))
)]
pub async fn get_block_by_hash(
    State(node): State<Arc<NodeContext>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    // ...
}
```

### Query Parameters

```rust
#[utoipa::path(
    get,
    path = "/api/v1/transactions",
    tag = "Transaction",
    params(("page" = Option<u32>, Query), ("limit" = Option<u32>, Query)),
    responses((status = 200, body = ApiResponse<Vec<TransactionResponse>>))
)]
pub async fn get_transactions(
    State(node): State<Arc<NodeContext>>,
    Query(params): Query<BlockQuery>,
) -> Result<Json<ApiResponse<Vec<TransactionResponse>>>, StatusCode> {
    // ...
}
```

---

## Schema Documentation

Schemas can be documented with descriptions and examples using attributes.

### Schema Descriptions

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(description = "Blockchain information")]
pub struct BlockchainInfoResponse {
    #[schema(description = "Blockchain height")]
    pub height: usize,
    #[schema(description = "Mining difficulty")]
    pub difficulty: u32,
}
```

### Schema Examples

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"height": 100, "difficulty": 1}))]
pub struct BlockchainInfoResponse {
    // ...
}
```

### Field Documentation

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SendTransactionRequest {
    #[schema(description = "Source address")]
    pub from_address: WalletAddress,
    #[schema(description = "Destination address")]
    pub to_address: WalletAddress,
    #[schema(description = "Amount in satoshis", minimum = 1)]
    pub amount: i32,
}
```

---

## Swagger UI Integration

Swagger UI provides interactive API documentation. Utoipa integrates Swagger UI seamlessly.

### Creating Swagger UI

```rust
use utoipa_swagger_ui::SwaggerUi;

pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
```

### Accessing Swagger UI

Once configured, Swagger UI is available at:
- **URL**: `http://localhost:8080/swagger-ui`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

Swagger UI provides interactive testing, schema exploration, example generation, and response validation.

---

## Tags and Organization

Use tags to organize endpoints:

```rust
tags(
    (name = "Health", description = "Health check endpoints"),
    (name = "Blockchain", description = "Blockchain data operations"),
)
```

Assign in path documentation with `tag = "TagName"`.

---

## Request/Response Documentation

### Request Body Documentation

```rust
#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transaction",
    request_body = SendTransactionRequest,
    responses((status = 202, body = ApiResponse<SendBitCoinResponse>))
)]
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    // ...
}
```

### Response Documentation

```rust
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks/{hash}",
    tag = "Blockchain",
    params(("hash" = String, Path)),
    responses(
        (status = 200, body = ApiResponse<BlockResponse>),
        (status = 404, description = "Not found"),
        (status = 500, description = "Server error")
    )
)]
```

---

## Error Documentation

Document error responses:

```rust
#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transaction",
    request_body = SendTransactionRequest,
    responses(
        (status = 202, body = ApiResponse<SendBitCoinResponse>),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Server error")
    )
)]
```

---

## Best Practices

1. **Document All Endpoints**: Use `#[utoipa::path(...)]`
2. **Provide Descriptions**: Add to schemas and fields
3. **Document All Response Codes**: Include all status codes (200, 400, 404, 500)
4. **Use Meaningful Tags**: Organize with clear tags
5. **Provide Examples**: Include schema examples
6. **Keep Updated**: Code changes automatically update docs

---

## Integration with Axum

Utoipa integrates seamlessly with Axum through the `utoipa-axum` crate.

### Route Integration

Utoipa automatically discovers Axum routes:

```rust
// In openapi.rs
#[openapi(
    paths(
        blockchain::get_blockchain_info,  // ← References Axum handler
        blockchain::get_block_by_hash,
        // ...
    ),
)]
```

### Type Integration

Utoipa works with Axum's type system:

```rust
// Handler signature
pub async fn handler() -> Result<Json<ApiResponse<Response>>, StatusCode>

// Utoipa understands:
// - Json wrapper → JSON response
// - ApiResponse<T> → Response wrapper schema
// - Response → Inner response schema
// - StatusCode → HTTP status codes
```

---

## Summary

Utoipa provides automatic OpenAPI documentation generation for our blockchain API:

- **OpenApi Macro**: Generates complete OpenAPI specifications
- **ToSchema Macro**: Generates schemas from Rust types
- **Path Documentation**: Documents endpoints with metadata
- **Swagger UI**: Interactive API documentation
- **Type Safety**: Documentation matches implementation
- **Always Current**: Documentation updates with code changes

Utoipa's integration with Axum and Serde creates a seamless documentation workflow where code and documentation stay in sync automatically.

---

## Additional Resources

- **[Utoipa Documentation](https://docs.rs/utoipa/)**: Official Utoipa crate documentation
- **[OpenAPI Specification](https://swagger.io/specification/)**: OpenAPI 3.0 specification
- **[Swagger UI](https://swagger.io/tools/swagger-ui/)**: Interactive API documentation
- **OpenAPI Documentation**: How we use Utoipa in our API
- **Data Models**: How schemas are generated from our models
- **Serde Framework Guide**: Serialization framework that Utoipa integrates with
- **Axum Framework Guide**: Web framework that Utoipa documents
- **Tower Framework Guide**: Middleware framework used in our API
- **Tokio Runtime Guide**: Async runtime that powers async operations
- **Rust Language Guide**: Rust language features used throughout

---

<div align="center">

**[← Web API Index](README.md)** | **Utoipa Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **Axum** | **Tower** | **Serde** | **Tracing** | **Tokio**

</div>

---

*This guide provides detailed explanations of Utoipa framework features used in our blockchain API. For implementation details, see the OpenAPI Documentation chapter.*
