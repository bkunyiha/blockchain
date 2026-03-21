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

# Chapter 15.10: Security Architecture

**Part I: Foundations & Core Implementation** | **Web API Architecture**

<div align="center">

**[← OpenAPI](09-OpenAPI.md)** | **[Chapter 15.10: Security Architecture](10-Security.md)** | **[Best Practices →](11-Best-Practices.md)**

</div>

---

## Security Architecture

Security is built into the web layer from the ground up. We explore the security measures below.

### Authentication

**API Key Authentication:**

- Keys are passed via `X-API-Key` header
- Admin and wallet keys are separate
- Keys are validated against environment variables
- Default keys are provided for development only

**Role-Based Access Control:**

- Admin role: Full access to all endpoints
- Wallet role: Limited access to wallet operations
- Unauthenticated: Only health check endpoints

For detailed implementation, see the [Authentication Middleware](05-Middleware.md#authentication-middleware) section.

### CORS Configuration

**Development:**

- Allows all origins (for local development)
- Allows all methods and headers

**Production:**

- Restrict to specific origins
- Limit methods and headers
- Set appropriate cache times

For detailed CORS implementation, see the [CORS Middleware](05-Middleware.md#cors-middleware) section and [CORS Configuration in Axum](Axum.md#cors-configuration) for comprehensive technical information on CORS setup, security considerations, and production configuration.

### Error Handling Security

**Error Sanitization:**

- Internal errors don't leak stack traces
- Sensitive information is filtered
- Generic error messages for clients
- Detailed errors in logs only

For more details, see Error Handling and [Error Handling Middleware](05-Middleware.md#error-handling-middleware).

### Rate Limiting

**Future Implementation:**

- Per-IP rate limiting
- Per-API-key rate limiting
- Configurable limits
- Burst handling

For current implementation status, see [Rate Limiting Middleware](05-Middleware.md#rate-limiting-middleware).

---

## Navigation

- **← Previous: OpenAPI Documentation** - Automatic API documentation generation
- **Next: Best Practices and Patterns →** - Design patterns and conventions
- **[Authentication Middleware](05-Middleware.md#authentication-middleware)** - Detailed authentication implementation
- **[CORS Middleware](05-Middleware.md#cors-middleware)** - CORS configuration details
- **Web API Index** - Overview and navigation
- **Tower Framework Guide** - CORS middleware details
- **Axum Framework Guide** - Detailed Axum feature explanations

---

<div align="center">

**[← Previous: OpenAPI](09-OpenAPI.md)** | **[Chapter 15.10: Security Architecture](10-Security.md)** | **[Next: Best Practices →](11-Best-Practices.md)**

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers security architecture. Continue to Best Practices and Patterns to learn about design patterns and conventions.*
