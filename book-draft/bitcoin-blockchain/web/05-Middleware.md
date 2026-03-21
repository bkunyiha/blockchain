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

# Chapter 15.5: Middleware Layer

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.5: Middleware Layer** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

</div>

---

## Middleware Layer

Middleware provides cross-cutting concerns like authentication, CORS, logging, and error handling. We explore each middleware component below.

### Authentication Middleware

We protect routes by checking API keys in authentication middleware in `middleware/auth.rs`:

**The `require_role()` function in `middleware/auth.rs`**

```rust
pub async fn require_role(
    mut req: axum::http::Request<axum::body::Body>,
    required: Role,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let key = req.headers().get("X-API-Key")
        .and_then(|h| h.to_str().ok());

    let caller_role = match key {
        Some(k) if is_admin_key(k) => Role::Admin,
        Some(k) if is_wallet_key(k) => Role::Wallet,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Admin can access wallet routes too
    let allowed = caller_role == required
        || (caller_role == Role::Admin && required == Role::Wallet);

    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }

    // Attach role to extensions for handlers
    req.extensions_mut().insert(caller_role);

    Ok(next.run(req).await)
}
```

**How Authentication Works**

1. **Extract API Key**: Gets the `X-API-Key` header from the request
2. **Determine Role**: Checks if the key matches admin or wallet keys
3. **Check Authorization**: Verifies the role has permission for the route
4. **Attach Role**: Stores the role in request extensions for handlers to use
5. **Continue or Reject**: Either continues to the handler or returns an error

**Role Hierarchy**

We implement role hierarchy in the `require_role()` function: admin users can access both admin and wallet routes, but wallet users can only access wallet routes. This provides a clear security model.

**Convenience Functions**

We provide convenience wrappers around `require_role()` through the `require_admin()` and `require_wallet()` functions in `middleware/auth.rs`:

```rust
pub async fn require_admin(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    require_role(req, Role::Admin, next).await
}

pub async fn require_wallet(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    require_role(req, Role::Wallet, next).await
}
```

**Key Validation**

We validate API keys in the `is_admin_key()` and `is_wallet_key()` functions in `middleware/auth.rs`:

```rust
fn is_admin_key(k: &str) -> bool {
    let expected = std::env::var("BITCOIN_API_ADMIN_KEY")
        .unwrap_or_else(|_| "admin-secret".to_string());
    k == expected
}

fn is_wallet_key(k: &str) -> bool {
    let expected = std::env::var("BITCOIN_API_WALLET_KEY")
        .unwrap_or_else(|_| "wallet-secret".to_string());
    k == expected
}
```

**Security Considerations**

- We read keys from environment variables
- Default values are provided for development
- In production, we always set strong keys via environment variables
- We consider using a key management service for production deployments

### CORS Middleware

We allow web browsers to make requests from different origins through CORS (Cross-Origin Resource Sharing) middleware in `middleware/cors.rs`:

**The `create_cors_layer()` function in `middleware/cors.rs`**

```rust
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400)) // 24 hours
}
```

**CORS Configuration**

- **`allow_origin(Any)`**: Allows requests from any origin (development only)
- **`allow_methods(Any)`**: Allows all HTTP methods (GET, POST, etc.)
- **`allow_headers(Any)`**: Allows all request headers
- **`expose_headers(Any)`**: Exposes all response headers to JavaScript
- **`max_age`**: Caches preflight requests for 24 hours

See [CORS Configuration in Axum](Axum.md#cors-configuration) for detailed technical information on CORS setup, production configuration, and security considerations. For more details on Tower's CORS layer, see [CORS Middleware in Tower](Tower.md#cors-middleware). For information on how logging works with Tower's TraceLayer, see [Integration with Tower](Tracing.md#integration-with-tower) in the Tracing Framework Guide.

**Production CORS**

For production, we use the `create_cors_layer_with_origins()` function in `middleware/cors.rs` to restrict origins:

```rust
pub fn create_cors_layer_with_origins(origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(std::time::Duration::from_secs(86400));

    for origin in origins {
        if let Ok(parsed_origin) = origin.parse::<axum::http::HeaderValue>() {
            cors = cors.allow_origin(parsed_origin);
        }
    }

    cors
}
```

### Rate Limiting Middleware

We prevent abuse by limiting the number of requests clients can make within a specified time window through rate limiting middleware in `middleware/rate_limit.rs`.

**The `RateLimitConfig` struct and `create_rate_limit_layer()` function in `middleware/rate_limit.rs`**

```rust
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

pub fn create_rate_limit_layer(
    config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    // Returns rate limiting layer
}
```

**Rate Limiting Features**

- **Token Bucket Algorithm**: Allows bursts up to `burst_size` while maintaining average rate
- **Per-IP Limiting**: Tracks requests by client IP address
- **Per-API-Key Limiting**: Supports API key-based rate limiting for granular control
- **Automatic Cleanup**: Removes inactive entries to manage memory
- **Standard Headers**: Returns `X-RateLimit-Limit`, `X-RateLimit-Remaining`, and `Retry-After` headers
- **429 Response**: Returns `429 Too Many Requests` when limits are exceeded

For detailed implementation guide, code examples, and advanced configurations, see Rate Limiting Implementation Guide.

### Error Handling Middleware

We catch and format errors through error handling middleware in `server.rs` (the `handle_errors()` function):

```rust
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;

    // Log error response body if status indicates an error
    let is_error = response.status().is_server_error()
        || response.status().is_client_error();
    if is_error {
        let (parts, body) = response.into_parts();
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();
        let body_str = String::from_utf8_lossy(&body_bytes);

        tracing::error!(
            "[handle_errors]: Error response ({}): {}",
            parts.status,
            body_str
        );

        // Reconstruct response
        let response = axum::response::Response::from_parts(
            parts,
            axum::body::Body::from(body_bytes)
        );

        if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
            let error_response = ErrorResponse {
                error: "Internal Server Error".to_string(),
                message: "An unexpected error occurred".to_string(),
                status_code: 500,
                timestamp: chrono::Utc::now(),
            };

            return Ok(Json(ApiResponse::<()>::error(
                serde_json::to_string(&error_response)
                    .unwrap_or_else(|_| "Unknown error".to_string()),
            )).into_response());
        }

        Ok(response)
    } else {
        Ok(response)
    }
}
```

**Error Handling Flow**

1. **Run Next Middleware**: Processes the request through the handler
2. **Check Status**: Determines if the response indicates an error
3. **Log Errors**: Logs error details for debugging
4. **Format Errors**: Converts internal server errors to user-friendly responses
5. **Preserve Errors**: Other errors pass through unchanged

See [Error Handling in Axum](Axum.md#error-handling) for detailed technical information on error handling patterns, error conversion, and error middleware implementation. For comprehensive error handling strategies, see Error Handling. For Tower middleware details, see Tower Framework Guide. For middleware layer composition and execution order, see [Middleware Layers in Axum](Axum.md#middleware-layers).

---

## Navigation

- **← Previous: Request Handlers** - Processing requests and implementing business logic
- **Next: Data Models →** - Request and response structures with type safety
- **Error Handling →** - Comprehensive error management strategies
- **Rate Limiting →** - Detailed rate limiting implementation guide
- **Web API Index** - Overview and navigation
- **Axum Framework Guide** - Detailed Axum feature explanations

---

<div align="center">

**[← Previous: Handlers](04-Handlers.md)** | **[Chapter 15.5: Middleware Layer](05-Middleware.md)** | **[Next: Data Models →](06-Data-Models.md)**

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers the middleware layer. Continue to Data Models to learn about request and response structures.*
