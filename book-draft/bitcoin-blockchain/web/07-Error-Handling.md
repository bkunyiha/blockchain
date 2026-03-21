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

# Chapter 15.7: Error Handling

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.7: Error Handling** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

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

- **← Previous: Data Models** - Request and response structures with type safety
- **Next: Rate Limiting →** - Rate limiting implementation
- **Middleware Layer** - Error handling middleware implementation
- **Web API Index** - Overview and navigation
- **Axum Framework Guide** - Detailed Axum feature explanations
- **Serde Framework Guide** - Error serialization details
- **Tracing Framework Guide** - Error logging and diagnostics

---

<div align="center">

**[← Previous: Data Models](06-Data-Models.md)** | **[Chapter 15.7: Error Handling](07-Error-Handling.md)** | **[Next: Rate Limiting →](08-Rate-Limiting.md)**

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers error handling. Continue to Rate Limiting to learn about rate limiting implementation.*
