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

# Chapter 3.7: Error Handling

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**[📚 ← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.7: Error Handling** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md)** 📚

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

**[📚 ← Previous: Data Models](06-Data-Models.md)** | **[Chapter 3.7: Error Handling](07-Error-Handling.md)** | **[Next: Rate Limiting →](08-Rate-Limiting.md)** 📚

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers error handling. Continue to Rate Limiting to learn about rate limiting implementation.*
