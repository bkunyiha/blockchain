<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
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
15. **Chapter 15: Web API Architecture** ← *You are here*
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
# Web API Architecture

**Part I: Foundations & Core Implementation** | **Chapter 15: Web API Architecture**

<div align="center">

**[← Chapter 14: Wallet](../wallet/README.md)** | **Chapter 15: Web API Architecture** | **[Chapter 16: Desktop Admin UI (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
</div>

---

## Introduction

This section provides a comprehensive guide to the web API layer that powers our blockchain node. The web API serves as the interface that enables clients—desktop applications, web UIs, and other services—to interact with the blockchain through HTTP endpoints.

Built using Rust's Axum framework, the web layer implements a complete system for handling authentication, request validation, error handling, and API documentation. This section explores how we've structured the REST API, how requests flow through the system, and the architectural decisions that make it secure, scalable, and maintainable.

This page serves as the index for the Web API section. For the full architecture overview and design principles, begin with **Introduction & Architecture Overview** (Section 01).

---

## Table of Contents

### Part 1: Core Concepts

1. **01: Introduction & Architecture Overview** - Architecture overview, design principles, and component organization
2. **02: Server Setup and Configuration** - Server initialization, configuration, and lifecycle management
3. **03: Routing System** - Route definitions, nesting, and endpoint organization

### Part 2: Request Processing

4. **04: Request Handlers** - Handler patterns, business logic, and request processing
5. **05: Middleware Layer** - Authentication, CORS, logging, and cross-cutting concerns
6. **06: Data Models** - Request/response structures, validation, and type safety

### Part 3: Advanced Topics

7. **07: Error Handling** - Error management strategies and patterns
8. **08: Rate Limiting Implementation** - Rate limiting algorithms, implementation, and configuration
9. **09: OpenAPI Documentation** - Automatic API documentation generation
10. **10: Security Architecture** - Authentication, authorization, and security measures
11. **11: Best Practices and Patterns** - Design patterns and conventions

### Reference Materials

- **Axum Framework Guide** - Comprehensive Axum framework reference
- **Tower Framework Guide** - Middleware framework and tower_http components
- **Serde Framework Guide** - Serialization and deserialization framework
- **Utoipa Framework Guide** - OpenAPI documentation generation
- **Tracing Framework Guide** - Structured logging and diagnostics
- **Tokio Runtime Guide** - Async runtime framework
- **Chapter 24: Rust Language Guide** - Comprehensive Rust language reference

---

## Architecture Principles

The web layer enforces separation of concerns: routes, handlers, middleware, and models live in distinct modules. Rust's type system provides compile-time validation of request/response shapes. The entire stack is async, built on Tokio and Axum, with authentication, CORS, and error handling wired in at the middleware level. OpenAPI documentation is generated automatically via Utoipa.

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

<div align="center">

**[← Previous: Wallet System](../wallet/README.md)** | **[Chapter 15: Web API Architecture](README.md)** | **[Next: Introduction & Architecture Overview →](01-Introduction.md)** 
</div>

---

---

> **Checkpoint:** With the Web API in place, you can start the node and hit the admin endpoint from your browser or command line: `curl http://localhost:8080/api/admin/blockchain-info | python3 -m json.tool`. You should get a JSON response showing the chain height, tip hash, peer count, and mempool size. This confirms that the entire Part I stack — from primitives through the REST API — is working end-to-end.

---

### Further Reading

- **[Axum documentation](https://docs.rs/axum)** — The official API reference for the web framework used in this project, including routing, extractors, middleware, and state management.
- **[Tower ecosystem](https://docs.rs/tower)** — The middleware framework that Axum is built on. Understanding Tower services, layers, and the `Service` trait helps when writing custom middleware.
- **[Serde JSON](https://docs.rs/serde_json)** — The JSON serialization library used for request/response bodies. Its `Value` type is useful for dynamic JSON handling.
