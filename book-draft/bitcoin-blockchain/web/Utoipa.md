<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../README.md)
2. [Chapter 2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)
3. **Chapter 3: Web API Architecture** ← *You are here*
   - [Web API Index](README.md) - Overview and navigation
   - [01: Introduction](01-Introduction.md) - Architecture overview
   - [02: Server Setup](02-Server-Setup.md) - Server configuration
   - [03: Routing](03-Routing.md) - Route definitions
   - [04: Handlers](04-Handlers.md) - Request handlers
   - [05: Middleware](05-Middleware.md) - Middleware layer
   - [06: Data Models](06-Data-Models.md) - Request/response models
   - [07: Error Handling](07-Error-Handling.md) - Error management
   - [08: OpenAPI](08-OpenAPI.md) - API documentation
   - [09: Security](09-Security.md) - Security architecture
   - [10: Best Practices](10-Best-Practices.md) - Design patterns
   - [Axum Framework Guide](Axum.md) - Framework reference
   - [Tower Framework Guide](Tower.md) - Middleware framework
   - [Serde Framework Guide](Serde.md) - Serialization framework
   - [Utoipa Framework Guide](Utoipa.md) - OpenAPI framework ← *You are here*
4. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
5. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md)
6. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md)
7. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

8. [Chapter 8: Docker Compose Deployment](../../ci/docker-compose/01-Introduction.md)
9. [Chapter 9: Kubernetes Deployment](../../ci/kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Utoipa Framework Guide

**Part I: Core Blockchain Implementation** | **Technical Reference: Utoipa OpenAPI Framework**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Utoipa Framework Guide** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

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

> **📘 See the full implementation**: This guide explains Utoipa concepts. To see how Utoipa is used in our complete web API architecture, see the [OpenAPI Documentation](08-OpenAPI.md) chapter and the [Data Models](06-Data-Models.md) chapter.

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

The `#[derive(OpenApi)]` macro generates the complete OpenAPI specification for your API.

### Implementation in `openapi.rs`

Our OpenAPI definition in `openapi.rs`:

```rust
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health endpoints
        health::health_check,
        health::liveness,
        health::readiness,
        // Blockchain endpoints
        blockchain::get_blockchain_info,
        blockchain::get_blocks,
        // ... more endpoints
    ),
    components(
        schemas(
            // Response schemas
            crate::web::models::responses::HealthResponse,
            crate::web::models::responses::BlockchainInfoResponse,
            // ... more schemas
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Blockchain", description = "Blockchain data and information"),
        // ... more tags
    ),
    info(
        title = "Blockchain API",
        version = "0.1.0",
        description = "A comprehensive blockchain API"
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server")
    )
)]
pub struct ApiDoc;
```

**What the macro does:**
- **`paths(...)`**: Lists all API endpoints to document
- **`components(schemas(...))`**: Lists all data models used in the API
- **`tags(...)`**: Defines tags for organizing endpoints
- **`info(...)`**: Provides API metadata (title, version, description)
- **`servers(...)`**: Lists API server URLs

### OpenAPI Structure

The generated OpenAPI specification includes:

1. **Info Section**: API title, version, description
2. **Servers**: Base URLs for the API
3. **Paths**: All endpoints with methods, parameters, responses
4. **Components**: Reusable schemas, parameters, responses
5. **Tags**: Organization and grouping of endpoints

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

The `ToSchema` macro generates an OpenAPI schema like:

```json
{
  "BlockchainInfoResponse": {
    "type": "object",
    "properties": {
      "height": {
        "type": "integer",
        "format": "int64"
      },
      "difficulty": {
        "type": "integer",
        "format": "int32"
      },
      "total_blocks": {
        "type": "integer",
        "format": "int64"
      },
      "total_transactions": {
        "type": "integer",
        "format": "int64"
      },
      "mempool_size": {
        "type": "integer",
        "format": "int64"
      },
      "last_block_hash": {
        "type": "string"
      },
      "last_block_timestamp": {
        "type": "string",
        "format": "date-time"
      }
    },
    "required": ["height", "difficulty", "total_blocks", "total_transactions", "mempool_size", "last_block_hash", "last_block_timestamp"]
  }
}
```

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

Path documentation uses the `#[utoipa::path(...)]` attribute to document individual endpoints.

### Basic Path Documentation

In our `handlers/blockchain.rs`:

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
        (status = 200, description = "Blockchain information retrieved successfully", body = ApiResponse<BlockchainInfoResponse>),
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

For endpoints with path parameters:

```rust
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks/{hash}",
    tag = "Blockchain",
    params(
        ("hash" = String, Path, description = "Block hash")
    ),
    responses(
        (status = 200, description = "Block retrieved successfully", body = ApiResponse<BlockResponse>),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_block_by_hash(
    State(node): State<Arc<NodeContext>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockResponse>>, StatusCode> {
    // ... handler implementation
}
```

**Path Parameters:**
- **`params(...)`**: Documents path parameters
- **`("hash" = String, Path, ...)`**: Parameter name, type, location, description

### Query Parameters

For endpoints with query parameters:

```rust
#[utoipa::path(
    get,
    path = "/api/v1/transactions",
    tag = "Transaction",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Transactions retrieved successfully", body = ApiResponse<Vec<TransactionResponse>>)
    )
)]
pub async fn get_transactions(
    State(node): State<Arc<NodeContext>>,
    Query(params): Query<BlockQuery>,
) -> Result<Json<ApiResponse<Vec<TransactionResponse>>>, StatusCode> {
    // ... handler implementation
}
```

---

## Schema Documentation

Schemas can be documented with descriptions and examples using attributes.

### Schema Descriptions

Add descriptions to schemas:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(description = "Comprehensive blockchain information including statistics and status")]
pub struct BlockchainInfoResponse {
    /// Current blockchain height (number of blocks)
    #[schema(description = "Current blockchain height")]
    pub height: usize,
    
    /// Mining difficulty
    #[schema(description = "Current mining difficulty")]
    pub difficulty: u32,
    
    // ... other fields
}
```

### Schema Examples

Provide examples for schemas:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "height": 100,
    "difficulty": 1,
    "total_blocks": 100,
    "total_transactions": 250,
    "mempool_size": 5
}))]
pub struct BlockchainInfoResponse {
    // ... fields
}
```

### Field Documentation

Document individual fields:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SendTransactionRequest {
    /// Source wallet address
    #[schema(description = "Source wallet address for the transaction")]
    pub from_address: WalletAddress,
    
    /// Destination wallet address
    #[schema(description = "Destination wallet address for the transaction")]
    pub to_address: WalletAddress,
    
    /// Transaction amount in satoshis
    #[schema(description = "Transaction amount in satoshis", minimum = 1)]
    pub amount: i32,
}
```

---

## Swagger UI Integration

Swagger UI provides interactive API documentation. Utoipa integrates Swagger UI seamlessly.

### Creating Swagger UI

In our `openapi.rs`:

```rust
use utoipa_swagger_ui::SwaggerUi;

pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
```

**How it works:**
1. **`SwaggerUi::new("/swagger-ui")`**: Creates Swagger UI at `/swagger-ui` path
2. **`.url(...)`**: Points to the OpenAPI JSON specification
3. **`ApiDoc::openapi()`**: Generates the OpenAPI spec from our `ApiDoc` struct

### Accessing Swagger UI

Once configured, Swagger UI is available at:
- **URL**: `http://localhost:8080/swagger-ui`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

### Swagger UI Features

Swagger UI provides:
- **Interactive Testing**: Test endpoints directly from the browser
- **Schema Exploration**: Browse request/response schemas
- **Example Generation**: See example requests and responses
- **Authentication**: Test authenticated endpoints
- **Response Validation**: See actual API responses

---

## Tags and Organization

Tags help organize endpoints in the OpenAPI documentation.

### Defining Tags

In our `openapi.rs`:

```rust
tags(
    (name = "Health", description = "Health check endpoints"),
    (name = "Blockchain", description = "Blockchain data and information"),
    (name = "Wallet", description = "Wallet management and operations"),
    (name = "Transaction", description = "Transaction creation and management"),
    (name = "Mining", description = "Mining operations and status"),
)
```

**Tag Structure:**
- **`name`**: Tag identifier (used in path documentation)
- **`description`**: Human-readable description

### Using Tags in Paths

Assign tags to endpoints:

```rust
#[utoipa::path(
    get,
    path = "/api/v1/blockchain",
    tag = "Blockchain",  // ← Tag assignment
    // ...
)]
```

**Benefits:**
- **Organization**: Endpoints grouped by functionality
- **Navigation**: Easy to find related endpoints
- **Clarity**: Clear API structure

---

## Request/Response Documentation

Utoipa documents request bodies and response types automatically.

### Request Body Documentation

For POST endpoints with request bodies:

```rust
#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transaction",
    request_body = SendTransactionRequest,  // ← Request body schema
    responses(
        (status = 202, description = "Transaction accepted", body = ApiResponse<SendBitCoinResponse>),
        (status = 400, description = "Bad request")
    )
)]
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    // ... handler implementation
}
```

**Request Body:**
- **`request_body = SendTransactionRequest`**: References the request schema
- Swagger UI shows the request body structure
- Users can test with example data

### Response Documentation

Document multiple response types:

```rust
#[utoipa::path(
    get,
    path = "/api/v1/blockchain/blocks/{hash}",
    tag = "Blockchain",
    params(
        ("hash" = String, Path, description = "Block hash")
    ),
    responses(
        (status = 200, description = "Block retrieved successfully", body = ApiResponse<BlockResponse>),
        (status = 404, description = "Block not found", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    )
)]
```

**Response Documentation:**
- **Status Codes**: Document all possible status codes
- **Response Bodies**: Specify the response body type for each status
- **Descriptions**: Human-readable descriptions

---

## Error Documentation

Document error responses to help API consumers understand failure cases.

### Error Response Schema

Our `ErrorResponse` in `models/errors.rs`:

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
    pub timestamp: DateTime<Utc>,
}
```

### Documenting Error Responses

In path documentation:

```rust
#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transaction",
    request_body = SendTransactionRequest,
    responses(
        (status = 202, description = "Transaction accepted", body = ApiResponse<SendBitCoinResponse>),
        (status = 400, description = "Bad request - invalid addresses or amount", body = ApiResponse<ErrorResponse>),
        (status = 401, description = "Unauthorized - missing or invalid API key", body = ApiResponse<ErrorResponse>),
        (status = 500, description = "Internal server error", body = ApiResponse<ErrorResponse>)
    )
)]
```

**Error Documentation Benefits:**
- **Clear Expectations**: Users know what errors to expect
- **Error Handling**: Clients can handle errors appropriately
- **Debugging**: Easier to troubleshoot issues

---

## Best Practices

### 1. Document All Endpoints

Every endpoint should have `#[utoipa::path(...)]` documentation:

```rust
#[utoipa::path(
    get,
    path = "/api/v1/endpoint",
    tag = "TagName",
    responses(/* ... */)
)]
```

### 2. Provide Descriptions

Add descriptions to schemas and fields:

```rust
#[schema(description = "Clear description of what this represents")]
pub struct Response {
    #[schema(description = "What this field represents")]
    pub field: String,
}
```

### 3. Document All Response Codes

Include all possible status codes:

```rust
responses(
    (status = 200, /* success */),
    (status = 400, /* bad request */),
    (status = 404, /* not found */),
    (status = 500, /* server error */)
)
```

### 4. Use Meaningful Tags

Organize endpoints with clear tags:

```rust
tags(
    (name = "Blockchain", description = "Blockchain data operations"),
    (name = "Wallet", description = "Wallet management"),
)
```

### 5. Provide Examples

Include examples in schemas when helpful:

```rust
#[schema(example = json!({
    "field1": "value1",
    "field2": 100
}))]
```

### 6. Keep Documentation Updated

Since Utoipa generates docs from code:
- Update code → documentation updates automatically
- Change types → schemas update automatically
- Add endpoints → appear in documentation automatically

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
- **[OpenAPI Documentation](08-OpenAPI.md)**: How we use Utoipa in our API
- **[Data Models](06-Data-Models.md)**: How schemas are generated from our models
- **[Serde Framework Guide](Serde.md)**: Serialization framework that Utoipa integrates with
- **[Axum Framework Guide](Axum.md)**: Web framework that Utoipa documents
- **[Tower Framework Guide](Tower.md)**: Middleware framework used in our API
- **[Tokio Runtime Guide](../Tokio.md)**: Async runtime that powers async operations
- **[Rust Language Guide](../../rust/README.md)**: Rust language features used throughout

---

<div align="center">

**📚 [← Web API Index](README.md)** | **Utoipa Framework Guide** | **[Introduction & Architecture Overview →](01-Introduction.md)** | **[Axum](Axum.md)** | **[Tower](Tower.md)** | **[Serde](Serde.md)** | **[Tracing](Tracing.md)** | **[Tokio](../Tokio.md)** 📚

</div>

---

*This guide provides detailed explanations of Utoipa framework features used in our blockchain API. For implementation details, see the [OpenAPI Documentation](08-OpenAPI.md) chapter.*
