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
   - [07: Error Handling](07-Error-Handling.md) - Error management ← *You are here*
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

# Chapter 3.7: Error Handling

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.7: Error Handling** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

</div>

---

## Error Handling

Error handling in the web layer follows a consistent pattern that provides clear feedback to clients while maintaining security.

### Error Flow

1. **Handler Error**: Handler encounters an error
2. **Status Code Mapping**: Error is mapped to appropriate HTTP status code
3. **Error Response**: Error is formatted into `ErrorResponse`
4. **Logging**: Error is logged for debugging
5. **Client Response**: Formatted error is sent to client

### Common Error Patterns

**Not Found Errors:**

```rust
match block {
    Some(block) => Ok(Json(ApiResponse::success(response))),
    None => Err(StatusCode::NOT_FOUND),
}
```

**Validation Errors:**

```rust
if !request.validate().is_ok() {
    return Err(StatusCode::BAD_REQUEST);
}
```

**Internal Server Errors:**

```rust
let result = operation().await
    .map_err(|e| {
        error!("Operation failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
```

### Error Middleware

The `handle_errors()` function in `server.rs` catches unhandled errors and formats them:

```rust
if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    let error_response = ErrorResponse {
        error: "Internal Server Error".to_string(),
        message: "An unexpected error occurred".to_string(),
        status_code: 500,
        timestamp: chrono::Utc::now(),
    };
    // Return formatted error
}
```

**Security Consideration:**

Internal server errors are sanitized before being sent to clients. This prevents leaking sensitive information like stack traces or internal paths.

For more details on error handling middleware, see the [Middleware Layer](05-Middleware.md#error-handling-middleware) chapter. For Axum-specific error handling patterns, see [Error Handling in Axum](Axum.md#error-handling) for detailed technical information on error conversion, error types, and error middleware implementation.

---

## Navigation

- **[← Previous: Data Models](06-Data-Models.md)** - Request and response structures with type safety
- **[Next: OpenAPI Documentation →](08-OpenAPI.md)** - Automatic API documentation generation
- **[Middleware Layer](05-Middleware.md)** - Error handling middleware implementation
- **[Web API Index](README.md)** - Overview and navigation
- **[Axum Framework Guide](Axum.md)** - Detailed Axum feature explanations
- **[Serde Framework Guide](Serde.md)** - Error serialization details
- **[Tracing Framework Guide](Tracing.md)** - Error logging and diagnostics

---

<div align="center">

**📚 [← Previous: Data Models](06-Data-Models.md)** | **Chapter 3.7: Error Handling** | **[Next: OpenAPI →](08-OpenAPI.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers error handling. Continue to [OpenAPI Documentation](08-OpenAPI.md) to learn about automatic API documentation generation.*
