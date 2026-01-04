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

# Chapter 3.9: OpenAPI Documentation

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Rate Limiting](08-Rate-Limiting.md)** | **Chapter 3.9: OpenAPI Documentation** | **[Security →](10-Security.md)** 📚

</div>

---

## OpenAPI Documentation

The web layer includes automatic OpenAPI/Swagger documentation generation using the `utoipa` crate. For detailed explanations of Utoipa features, see the [Utoipa Framework Guide](Utoipa.md).

### OpenAPI Definition

The OpenAPI definition in `openapi.rs` uses the `utoipa` crate:

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        blockchain::get_blockchain_info,
        wallet::create_wallet,
        // ... all endpoints
    ),
    components(
        schemas(
            BlockchainInfoResponse,
            WalletResponse,
            // ... all models
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Blockchain", description = "Blockchain data and information"),
        // ... all tags
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

### Swagger UI

The `create_swagger_ui()` function in `openapi.rs` serves Swagger UI automatically at `/swagger-ui`:

```rust
pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
```

**Benefits:**

- **Interactive Documentation**: Test endpoints directly from the browser
- **Automatic Updates**: Documentation stays in sync with code
- **Type Safety**: OpenAPI schema is generated from Rust types
- **Client Generation**: Can generate client libraries from the schema

### Where and How openapi.rs is Called

The `openapi.rs` file is integrated into the web server through a specific call chain. Understanding this integration is crucial for understanding how OpenAPI documentation is served.

**Location**: `bitcoin/src/web/openapi.rs`

The `openapi.rs` file defines the complete OpenAPI specification and references all data models in the `components(schemas(...))` section:

```rust
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // All handler functions are listed here
        health::health_check,
        blockchain::get_blockchain_info,
        wallet::create_wallet,
        // ... other endpoints
    ),
    components(
        schemas(
            // Response models - referenced by ToSchema derive
            crate::web::models::responses::ApiResponse,
            crate::web::models::responses::BlockchainInfoResponse,
            crate::web::models::responses::BlockResponse,
            crate::web::models::responses::WalletResponse,
            crate::web::models::responses::BalanceResponse,
            // ... other response models
            
            // Request models - referenced by ToSchema derive
            crate::web::models::requests::CreateWalletRequest,
            crate::web::models::requests::SendTransactionRequest,
            // ... other request models
            
            // Error models - referenced by ToSchema derive
            crate::web::models::errors::ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Blockchain", description = "Blockchain data and information"),
        (name = "Wallet", description = "Wallet management and operations"),
        // ... other tags
    ),
    info(
        title = "Blockchain API",
        version = "0.1.0",
        description = "A comprehensive blockchain API"
    )
)]
pub struct ApiDoc;

/// Create Swagger UI router
pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
```

**Call Chain:**

**1. In `routes/web.rs`:**

The `create_swagger_ui()` function from `openapi.rs` is called and merged into the web routes:

```rust
// In routes/web.rs
use crate::web::openapi::create_swagger_ui;

pub fn create_web_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        // ... React app serving routes ...
        .merge(create_swagger_ui())  // ← openapi.rs is called here
}
```

**2. In `server.rs`:**

The web routes (which include Swagger UI) are merged into the main application router:

```rust
// In server.rs - create_app() method
pub fn create_app(&self) -> Router {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())  // ← Includes Swagger UI from openapi.rs
        .with_state(self.node.clone());
    
    // ... middleware layers ...
    app
}
```

**3. Server Startup:**

When the server starts, the complete router (including Swagger UI) is served:

```rust
// In server.rs - start_with_shutdown() method
pub async fn start_with_shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
    let app = self.create_app();  // ← Includes openapi.rs routes
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await?;
    
    Ok(())
}
```

**Complete Flow:**

```
Server Startup
    ↓
server.rs::start_with_shutdown()
    ↓
server.rs::create_app()
    ↓
routes/web.rs::create_web_routes()
    ↓
openapi.rs::create_swagger_ui()  ← openapi.rs is called here
    ↓
Swagger UI available at /swagger-ui
OpenAPI JSON available at /api-docs/openapi.json
```

### How Data Models Generate OpenAPI Schemas

Data models play a crucial role in OpenAPI documentation generation. The `ToSchema` derive macro on each model enables Utoipa to automatically generate OpenAPI schemas, which are then referenced in `openapi.rs` to create comprehensive API documentation.

When a data model has the `ToSchema` derive macro, Utoipa automatically generates an OpenAPI schema:

**Example: WalletResponse**

```rust
// In models/responses.rs
#[derive(Debug, Serialize, Deserialize, ToSchema)]  // ← ToSchema enables OpenAPI generation
pub struct WalletResponse {
    pub address: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
}
```

**Generated OpenAPI Schema:**

When `openapi.rs` references `WalletResponse` in the `components(schemas(...))` section, Utoipa generates:

```json
{
  "WalletResponse": {
    "type": "object",
    "required": ["address", "public_key", "created_at"],
    "properties": {
      "address": {
        "type": "string"
      },
      "public_key": {
        "type": "string"
      },
      "created_at": {
        "type": "string",
        "format": "date-time"
      }
    }
  }
}
```

The Swagger UI is automatically generated from the data models referenced in `openapi.rs`, ensuring documentation stays synchronized with the code.

### Accessing OpenAPI Documentation

Once the server is running, the OpenAPI documentation is available at:

- **Swagger UI**: `http://localhost:8080/swagger-ui` - Interactive API documentation
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json` - Raw OpenAPI specification

> **See the complete implementation**: For details on how data models are structured, see the [Data Models chapter](06-Data-Models.md). For Utoipa framework details, see the [Utoipa Framework Guide](Utoipa.md).

---

## Navigation

- **[← Previous: Rate Limiting](08-Rate-Limiting.md)** - Rate limiting implementation
- **[Next: Security Architecture →](10-Security.md)** - Authentication, authorization, and security
- **[Web API Index](README.md)** - Overview and navigation
- **[Utoipa Framework Guide](Utoipa.md)** - Detailed Utoipa feature explanations
- **[Serde Framework Guide](Serde.md)** - Schema serialization details

---

<div align="center">

**📚 [← Previous: Rate Limiting](08-Rate-Limiting.md)** | **Chapter 3.9: OpenAPI Documentation** | **[Next: Security →](10-Security.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers OpenAPI documentation. Continue to [Security Architecture](10-Security.md) to learn about authentication, authorization, and security measures.*
