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

# Chapter 3.10: Security Architecture

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← OpenAPI](09-OpenAPI.md)** | **Chapter 3.10: Security Architecture** | **[Best Practices →](11-Best-Practices.md)** 📚

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

For more details, see [Error Handling](07-Error-Handling.md) and [Error Handling Middleware](05-Middleware.md#error-handling-middleware).

### Rate Limiting

**Future Implementation:**

- Per-IP rate limiting
- Per-API-key rate limiting
- Configurable limits
- Burst handling

For current implementation status, see [Rate Limiting Middleware](05-Middleware.md#rate-limiting-middleware).

---

## Navigation

- **[← Previous: OpenAPI Documentation](09-OpenAPI.md)** - Automatic API documentation generation
- **[Next: Best Practices and Patterns →](11-Best-Practices.md)** - Design patterns and conventions
- **[Authentication Middleware](05-Middleware.md#authentication-middleware)** - Detailed authentication implementation
- **[CORS Middleware](05-Middleware.md#cors-middleware)** - CORS configuration details
- **[Web API Index](README.md)** - Overview and navigation
- **[Tower Framework Guide](Tower.md)** - CORS middleware details
- **[Axum Framework Guide](Axum.md)** - Detailed Axum feature explanations

---

<div align="center">

**📚 [← Previous: OpenAPI](09-OpenAPI.md)** | **Chapter 3.10: Security Architecture** | **[Next: Best Practices →](11-Best-Practices.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers security architecture. Continue to [Best Practices and Patterns](11-Best-Practices.md) to learn about design patterns and conventions.*
