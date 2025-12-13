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
   - [07: Error Handling](07-Error-Handling.md) - Error management
   - [08: OpenAPI](08-OpenAPI.md) - API documentation
   - [09: Security](09-Security.md) - Security architecture ← *You are here*
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

# Chapter 3.9: Security Architecture

**Part I: Core Blockchain Implementation** | **Web API Architecture**

<div align="center">

**📚 [← Chapter 2.2: Transaction ID Format](../primitives/02-Transaction-ID-Format.md)** | **Chapter 3.9: Security Architecture** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)** 📚

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

- **[← Previous: OpenAPI Documentation](08-OpenAPI.md)** - Automatic API documentation generation
- **[Next: Best Practices and Patterns →](10-Best-Practices.md)** - Design patterns and conventions
- **[Authentication Middleware](05-Middleware.md#authentication-middleware)** - Detailed authentication implementation
- **[CORS Middleware](05-Middleware.md#cors-middleware)** - CORS configuration details
- **[Web API Index](README.md)** - Overview and navigation
- **[Tower Framework Guide](Tower.md)** - CORS middleware details
- **[Axum Framework Guide](Axum.md)** - Detailed Axum feature explanations

---

<div align="center">

**📚 [← Previous: OpenAPI](08-OpenAPI.md)** | **Chapter 3.9: Security Architecture** | **[Next: Best Practices →](10-Best-Practices.md)** 📚

**[← Web API Index](README.md)** | **[Introduction & Architecture Overview](01-Introduction.md)**

</div>

---

*This chapter covers security architecture. Continue to [Best Practices and Patterns](10-Best-Practices.md) to learn about design patterns and conventions.*
