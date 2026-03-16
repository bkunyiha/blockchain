<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
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

# Chapter 15.9: OpenAPI Documentation

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Rate Limiting](08-Rate-Limiting.md)** | **[Chapter 15.9: OpenAPI Documentation](09-OpenAPI.md)** | **[Security →](10-Security.md)** 

</div>

---

## OpenAPI Documentation

The web layer includes automatic OpenAPI/Swagger documentation generation using the `utoipa` crate. For detailed explanations of Utoipa features, see the Utoipa Framework Guide.

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
        (
            url = "http://localhost:8080",
            description = "Local development server"
        )
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
pub fn create_app(
    &self,
) -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new()
        .merge(create_all_api_routes())
        .merge(create_wallet_only_routes())
        .merge(create_web_routes())  // ← Includes Swagger UI
        .with_state(self.node.clone());

    // ... middleware layers ...
    Ok(app)
}
```

**3. Server Startup:**

When the server starts, the complete router (including Swagger UI) is served:

```rust
// In server.rs - start_with_shutdown() method
pub async fn start_with_shutdown(
    &self,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = self.create_app()?;  // ← Includes openapi.rs routes

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    Ok(())
}
```

**Complete Flow:**

```text
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
// ← ToSchema enables OpenAPI generation
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

> **See the complete implementation**: For details on how data models are structured, see the Data Models chapter. For Utoipa framework details, see the Utoipa Framework Guide.

---

## Navigation

- **← Previous: Rate Limiting** - Rate limiting implementation
- **Next: Security Architecture →** - Authentication, authorization, and security
- **Web API Index** - Overview and navigation
- **Utoipa Framework Guide** - Detailed Utoipa feature explanations
- **Serde Framework Guide** - Schema serialization details

---

<div align="center">

**[← Previous: Rate Limiting](08-Rate-Limiting.md)** | **[Chapter 15.9: OpenAPI Documentation](09-OpenAPI.md)** | **[Next: Security →](10-Security.md)** 

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers OpenAPI documentation. Continue to Security Architecture to learn about authentication, authorization, and security measures.*
