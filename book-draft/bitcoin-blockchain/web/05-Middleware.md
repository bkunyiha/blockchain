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
   - [05: Middleware](05-Middleware.md) - Middleware layer ← *You are here*
   - [06: Data Models](06-Data-Models.md) - Request/response models
   - [07: Error Handling](07-Error-Handling.md) - Error management
   - [08: OpenAPI](08-OpenAPI.md) - API documentation
   - [09: Security](09-Security.md) - Security architecture
   - [10: Best Practices](10-Best-Practices.md) - Design patterns
   - [Axum Framework Guide](Axum.md) - Framework reference
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

# Chapter 3.5: Middleware Layer

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.5: Middleware Layer** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Middleware Layer

Middleware provides cross-cutting concerns like authentication, CORS, logging, and error handling. Let's explore each middleware component.

### Authentication Middleware

Authentication middleware in `middleware/auth.rs` protects routes by checking API keys:

**The `require_role()` function in `middleware/auth.rs`:**

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

**How Authentication Works:**

1. **Extract API Key**: Gets the `X-API-Key` header from the request
2. **Determine Role**: Checks if the key matches admin or wallet keys
3. **Check Authorization**: Verifies the role has permission for the route
4. **Attach Role**: Stores the role in request extensions for handlers to use
5. **Continue or Reject**: Either continues to the handler or returns an error

**Role Hierarchy:**

The `require_role()` function in `middleware/auth.rs` implements role hierarchy: Admin users can access both admin and wallet routes, but wallet users can only access wallet routes. This provides a clear security model.

**Convenience Functions:**

The `require_admin()` and `require_wallet()` functions in `middleware/auth.rs` are convenience wrappers around `require_role()`:

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

**Key Validation:**

The `is_admin_key()` and `is_wallet_key()` functions in `middleware/auth.rs` validate API keys:

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

**Security Considerations:**

- Keys are read from environment variables
- Default values are provided for development
- In production, always set strong keys via environment variables
- Consider using a key management service for production deployments

### CORS Middleware

CORS (Cross-Origin Resource Sharing) middleware in `middleware/cors.rs` allows web browsers to make requests from different origins:

**The `create_cors_layer()` function in `middleware/cors.rs`:**

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

**CORS Configuration:**

- **`allow_origin(Any)`**: Allows requests from any origin (development only)
- **`allow_methods(Any)`**: Allows all HTTP methods (GET, POST, etc.)
- **`allow_headers(Any)`**: Allows all request headers
- **`expose_headers(Any)`**: Exposes all response headers to JavaScript
- **`max_age`**: Caches preflight requests for 24 hours

See [CORS Configuration in Axum](Axum.md#cors-configuration) for detailed technical information on CORS setup, production configuration, and security considerations. For more details on Tower's CORS layer, see [CORS Middleware in Tower](Tower.md#cors-middleware). For information on how logging works with Tower's TraceLayer, see [Integration with Tower](Tracing.md#integration-with-tower) in the Tracing Framework Guide.

**Production CORS:**

For production, use the `create_cors_layer_with_origins()` function in `middleware/cors.rs` to restrict origins:

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

Rate limiting middleware in `middleware/rate_limit.rs` (currently a placeholder) will prevent abuse:

**The `RateLimitConfig` struct and `create_rate_limit_layer()` function in `middleware/rate_limit.rs`:**

```rust
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

pub fn create_rate_limit_layer(
    _config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    // Placeholder - returns no-op layer
    tower::layer::util::Identity::new()
}
```

**Future Implementation:**

When implemented, rate limiting will:
- Track requests per IP address or API key
- Enforce limits based on configuration
- Return `429 Too Many Requests` when limits are exceeded
- Use a token bucket algorithm for burst handling

### Error Handling Middleware

Error handling middleware in `server.rs` (the `handle_errors()` function) catches and formats errors:

```rust
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;

    // Log error response body if status indicates an error
    if response.status().is_server_error() || response.status().is_client_error() {
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

**Error Handling Flow:**

1. **Run Next Middleware**: Processes the request through the handler
2. **Check Status**: Determines if the response indicates an error
3. **Log Errors**: Logs error details for debugging
4. **Format Errors**: Converts internal server errors to user-friendly responses
5. **Preserve Errors**: Other errors pass through unchanged

See [Error Handling in Axum](Axum.md#error-handling) for detailed technical information on error handling patterns, error conversion, and error middleware implementation. For comprehensive error handling strategies, see [Error Handling](07-Error-Handling.md). For Tower middleware details, see [Tower Framework Guide](Tower.md). For middleware layer composition and execution order, see [Middleware Layers in Axum](Axum.md#middleware-layers).

---

## Navigation

- **[← Previous: Request Handlers](04-Handlers.md)** - Processing requests and implementing business logic
- **[Next: Data Models →](06-Data-Models.md)** - Request and response structures with type safety
- **[Error Handling →](07-Error-Handling.md)** - Comprehensive error management strategies
- **[Web API Index](README.md)** - Overview and navigation
- **[Axum Framework Guide](Axum.md)** - Detailed Axum feature explanations

---

<div align="center">

**📚 [← Previous: Handlers](04-Handlers.md)** | **Chapter 3.5: Middleware Layer** | **[Next: Data Models →](06-Data-Models.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers the middleware layer. Continue to [Data Models](06-Data-Models.md) to learn about request and response structures.*
