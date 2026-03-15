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
16. <a href="../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
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

# Chapter 3.10: Security Architecture

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**[📚 ← OpenAPI](09-OpenAPI.md)** | **[Chapter 3.10: Security Architecture](10-Security.md)** | **[Best Practices →](11-Best-Practices.md)** 📚

</div>

---

## Security Architecture

Security is built into the web layer from the ground up. Let's explore the security measures.

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

**[📚 ← Previous: OpenAPI](09-OpenAPI.md)** | **[Chapter 3.10: Security Architecture](10-Security.md)** | **[Next: Best Practices →](11-Best-Practices.md)** 📚

**[← Web API Index](README.md)** | **Introduction & Architecture Overview**

</div>

---

*This chapter covers security architecture. Continue to Best Practices and Patterns to learn about design patterns and conventions.*
