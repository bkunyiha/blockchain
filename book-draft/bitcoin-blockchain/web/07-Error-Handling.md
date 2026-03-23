<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
<div align="right">

**[← Back to Web API Index](README.md)** | **[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 24.7: Error Handling

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← Chapter 24: Web API Architecture](README.md)** | **Chapter 24.7: Error Handling** | **[Next: Chapter 25: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**

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

**[← Previous: Data Models](06-Data-Models.md)** | **[Chapter 24.7: Error Handling](07-Error-Handling.md)** | **[Next: Rate Limiting →](08-Rate-Limiting.md)**

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers error handling. Continue to Rate Limiting to learn about rate limiting implementation.*
