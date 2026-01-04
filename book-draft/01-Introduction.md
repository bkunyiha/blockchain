<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. **Chapter 1: Introduction & Overview** ← *You are here*
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](ci/kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](rust/README.md) - Rust programming language reference

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
├── bitcoin-desktop-ui/        # Desktop admin interface (Iced)
├── bitcoin-wallet-ui/          # Wallet user interface (Iced)
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

