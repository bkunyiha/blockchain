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
24. **Chapter 24: Web API Architecture** ← *You are here*
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
# Chapter 24: Web API Architecture

**Part I: Foundations & Core Implementation** | **Chapter 24: Web API Architecture**

<div align="center">

**[← Chapter 32: Wallet System](../wallet/README.md)** | **Chapter 24: Web API Architecture** | **[Chapter 25: Desktop Admin UI (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
</div>

---

## Introduction

This section provides a comprehensive guide to the web API layer that powers our blockchain node. The web API serves as the interface that enables clients—desktop applications, web UIs, and other services—to interact with the blockchain through HTTP endpoints.

Built using Rust's Axum framework, the web layer implements a complete system for handling authentication, request validation, error handling, and API documentation. This section explores how we've structured the REST API, how requests flow through the system, and the architectural decisions that make it secure, scalable, and maintainable.

> **What you will learn in this chapter:**
> - Structure REST endpoints using the Axum web framework
> - Implement request handlers, middleware, and error handling for a blockchain API
> - Configure authentication, CORS, and security layers
> - Generate OpenAPI documentation automatically with Utoipa

> **Scope:** This chapter covers the REST API for a development/learning environment. We do not cover WebSocket subscriptions, GraphQL APIs, gRPC interfaces, or production rate limiting and DDoS protection.

This page serves as the index for the Web API section. For the full architecture overview and design principles, begin with **Introduction & Architecture Overview** (Section 01).

---

## Table of Contents

### Part 1: Core Concepts

1. **01: Introduction & Architecture Overview** — Architecture overview, design principles, and component organization
2. **02: Server Setup and Configuration** — Server initialization, configuration, and lifecycle management
3. **03: Routing System** — Route definitions, nesting, and endpoint organization

### Part 2: Request Processing

4. **04: Request Handlers** — Handler patterns, business logic, and request processing
5. **05: Middleware Layer** — Authentication, CORS, logging, and cross-cutting concerns
6. **06: Data Models** — Request/response structures, validation, and type safety

### Part 3: Advanced Topics

7. **07: Error Handling** — Error management strategies and patterns
8. **08: Rate Limiting Implementation** — Rate limiting algorithms, implementation, and configuration
9. **09: OpenAPI Documentation** — Automatic API documentation generation
10. **10: Security Architecture** — Authentication, authorization, and security measures
11. **11: Best Practices and Patterns** — Design patterns and conventions

### Reference Materials

- **Axum Framework Guide** — Comprehensive Axum framework reference
- **Tower Framework Guide** — Middleware framework and tower_http components
- **Serde Framework Guide** — Serialization and deserialization framework
- **Utoipa Framework Guide** — OpenAPI documentation generation
- **Tracing Framework Guide** — Structured logging and diagnostics
- **Tokio Runtime Guide** — Async runtime framework
- **Chapter 33: Rust Language Guide** — Comprehensive Rust language reference

> **Tip:** The generated OpenAPI specification is available at `/api-docs` when the server is running. Open it in Swagger UI to interactively test endpoints without writing any client code.

> **Warning:** The default development configuration has no rate limiting. Before exposing the API to external traffic, add rate limiting middleware to prevent denial-of-service attacks.

---

## Architecture Principles

The web layer enforces separation of concerns: routes, handlers, middleware, and models live in distinct modules. Rust's type system provides compile-time validation of request/response shapes. The entire stack is async, built on Tokio and Axum, with authentication, CORS, and error handling wired in at the middleware level. OpenAPI documentation is generated automatically via Utoipa.

**Figure 15-1: API Request Flow**

```text
 HTTP Request
      │
      ▼
 ┌──────────┐
 │  CORS    │  Middleware Layer
 │  Auth    │  (Tower)
 │  Logging │
 └────┬─────┘
      │
      ▼
 ┌──────────┐
 │  Router  │  Axum Route Matching
 │ /api/v1/ │
 └────┬─────┘
      │
      ▼
 ┌──────────┐
 │ Handler  │  Extract params, call logic
 └────┬─────┘
      │
      ▼
 ┌──────────┐
 │  Node    │  Blockchain operations
 │ Context  │
 └────┬─────┘
      │
      ▼
 ┌──────────┐
 │ Response │  Serialize to JSON
 │  (JSON)  │
 └──────────┘
```

### Technology Stack

- **Axum**: Modern web framework for Rust - See Axum Framework Guide
- **Tokio**: Async runtime - See Tokio Runtime Guide
- **Tower**: Middleware and service traits - See Tower Framework Guide
- **Serde**: Serialization/deserialization - See Serde Framework Guide
- **Utoipa**: OpenAPI/Swagger documentation - See Utoipa Framework Guide
- **Tracing**: Structured logging and diagnostics - See Tracing Framework Guide

---

## Code Examples

All code examples in this section are taken from the actual implementation:

- **Server Configuration**: `bitcoin/src/web/server.rs`
- **Route Definitions**: `bitcoin/src/web/routes/`
- **Request Handlers**: `bitcoin/src/web/handlers/`
- **Middleware**: `bitcoin/src/web/middleware/`
- **Data Models**: `bitcoin/src/web/models/`

---

## Summary

- We structured a REST API using Axum with typed handlers, extractors, and shared state for blockchain operations.
- We implemented middleware layers for authentication, CORS, logging, and comprehensive error handling.
- We built request handlers that translate HTTP operations into blockchain actions through the NodeContext.
- We generated OpenAPI documentation automatically with Utoipa, providing a self-documenting API surface.

In the next chapter, we build the first of four user interfaces: a desktop admin panel using the Iced framework's pure Rust MVU architecture.

---

## Exercises

1. **Add a New Endpoint** — Implement a new `GET /api/v1/difficulty` endpoint that returns the current mining difficulty target. Write the handler function, add the route to the router, create the response data model, and generate OpenAPI documentation with Utoipa. Test with curl or the Swagger UI.

2. **Middleware Investigation** — Examine the middleware stack (authentication, CORS, logging) and trace how a request passes through each layer. Temporarily disable CORS and observe what changes in browser-based API calls. What security implications does this have?

---

<div align="center">

**[← Previous: Wallet System](../wallet/README.md)** | **[Chapter 24: Web API Architecture](README.md)** | **[Next: Introduction & Architecture Overview →](01-Introduction.md)**
</div>

---

> **Checkpoint:** With the Web API in place, you can start the node and hit the admin endpoint from your browser or command line: `curl http://localhost:8080/api/admin/blockchain-info | python3 -m json.tool`. You should get a JSON response showing the chain height, tip hash, peer count, and mempool size. This confirms that the entire Part I stack — from primitives through the REST API — is working end-to-end.

---

### Further Reading

- **[Axum documentation](https://docs.rs/axum)** — The official API reference for the web framework used in this project, including routing, extractors, middleware, and state management.
- **[Tower ecosystem](https://docs.rs/tower)** — The middleware framework that Axum is built on. Understanding Tower services, layers, and the `Service` trait helps when writing custom middleware.
- **[Serde JSON](https://docs.rs/serde_json)** — The JSON serialization library used for request/response bodies. Its `Value` type is useful for dynamic JSON handling.
