<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. **Chapter 1: Introduction & Overview** ← *You are here*
2. <a href="bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
<div align="middle">

**[← Back to Main Book](README.md)** | **[Next: Introduction to Bitcoin & Blockchain →](bitcoin-blockchain/README.md)**

</div>

---

<div align="center">

# 📚 Building a Full-Stack Bitcoin Blockchain With Rust

## Chapter 1: Introduction & Overview

**[← Back to Main Book](README.md)** | **Introduction & Overview** | **[Next: Introduction to Bitcoin & Blockchain →](bitcoin-blockchain/README.md)** 📚

</div>

---

## Chapter 1: Introduction & Overview

### Project Structure

```
blockchain/
├── bitcoin/                    # Core blockchain implementation
│   ├── src/
│   │   ├── primitives/        # Transaction, Block, UTXO structures
│   │   ├── node/              # Node context and networking
│   │   ├── store/             # Blockchain storage (file system, database)
│   │   └── web/               # REST API server (Axum)
│   └── ...
├── bitcoin-desktop-ui-iced/        # Desktop admin interface (Iced)
├── bitcoin-wallet-ui-iced/          # Wallet user interface (Iced)
├── bitcoin-web-ui/             # Web admin interface (React/TypeScript)
├── ci/
│   ├── docker-compose/        # Docker Compose deployment
│   └── kubernetes/            # Kubernetes deployment
└── book-draft/                 # This documentation
```

### Technical Stack

#### Backend (Rust)
- **Tokio**: Async runtime for non-blocking I/O
- **SQLCipher**: Encrypted SQLite database
- **Rusqlite**: Rust bindings for SQLite
- **Serde**: Serialization framework
- **Axum**: Web framework for REST APIs
- **Reqwest**: HTTP client

#### Desktop UIs (Rust)
- **Iced**: Cross-platform GUI framework
- **Model-View-Update (MVU)**: Architecture pattern
- **Desktop Admin Interface**: Admin UI for blockchain management (Iced)
- **Wallet User Interface**: User-facing wallet application (Iced)

#### Web UI (TypeScript/React)
- **React 18**: UI framework
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool and dev server
- **React Query**: Server state management
- **React Router**: Client-side routing
- **Tailwind CSS**: Utility-first CSS framework
- **Axios**: HTTP client

---

## Navigation

- **[← Back to Main Book](README.md)** - Book index and table of contents
- **[Next: Introduction to Bitcoin & Blockchain →](bitcoin-blockchain/README.md)** - Bitcoin and blockchain fundamentals
- **[Cryptography →](bitcoin-blockchain/crypto/README.md)** - Cryptographic primitives
- **[Transaction ID Format →](bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)** - Transaction ID format and storage
- **[Web API Architecture →](bitcoin-blockchain/web/README.md)** - REST API implementation

**Related Guides:**
- **[Chapter 10: Rust Language Guide](rust/README.md)** - Comprehensive Rust language reference (Part III)
- **[Tokio Runtime Guide](bitcoin-blockchain/Tokio.md)** - Async runtime framework

---

<div align="center">

**📚 [← Back to Main Book](README.md)** | **Introduction & Overview** | **[Next: Introduction to Bitcoin & Blockchain →](bitcoin-blockchain/README.md)** 📚

</div>

---

*This chapter has introduced the book and project structure, providing an overview of what we'll build together and how the book is organized. We've explored the technical stack, project structure, and learning paths designed for different experience levels. Understanding this foundation is essential for navigating the subsequent chapters, as it provides context for how all the pieces of our blockchain implementation fit together. Whether you're a first-time reader following the sequential path or an experienced developer jumping to specific topics, this chapter establishes the framework for understanding the complete system. In the next chapter, we'll begin exploring [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md) to understand the fundamental concepts of Bitcoin and blockchain technology before diving into implementation details.*
*This chapter has introduced the project structure and technical stack. In the next chapter, we begin exploring [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md) to understand the fundamental concepts of Bitcoin and blockchain technology before diving into implementation details.*

