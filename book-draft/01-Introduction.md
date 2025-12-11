<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. **Chapter 1: Introduction & Overview** (this chapter) ← *You are here*
2. [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 2.1: Cryptography](bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
4. [Chapter 2.2: Transaction System](bitcoin-blockchain/02-Transaction-System.md) - Transaction ID format guide
5. [Chapter 3: Web API Architecture](bitcoin-blockchain/web/README.md) - REST API implementation
6. [Chapter 4: Desktop Admin Interface](bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
7. [Chapter 5: Wallet User Interface](bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
8. [Chapter 6: Embedded Database & Persistence](bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
9. [Chapter 7: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

9. [Chapter 8: Docker Compose Deployment](ci/docker-compose/01-Introduction.md) - Docker Compose guide
10. [Chapter 9: Kubernetes Deployment](ci/kubernetes/README.md) - Kubernetes production guide

</details>

</div>

<div align="right">

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

### What We'll Build Together

In this book, we'll build a complete Bitcoin blockchain implementation from scratch using Rust. Our journey will cover:

- **Blockchain Fundamentals**: We'll explore transaction structure, validation, the UTXO model, and script execution—the core concepts that make blockchain technology work.

- **Cryptographic Primitives**: We'll dive into hash functions, digital signatures, and key management, understanding how cryptography secures our blockchain.

- **Network Protocols**: We'll implement peer-to-peer communication, message handling, and synchronization, learning how nodes communicate and maintain consensus.

- **User Interfaces**: We'll build three different interfaces—a desktop admin UI using Iced, a wallet UI also using Iced, and a modern web admin UI using React and TypeScript. Each serves different purposes and teaches different patterns.

- **Database Design**: We'll integrate SQLCipher for encrypted storage, exploring persistence strategies that keep user data secure.

- **Deployment**: Finally, we'll deploy our system using Docker Compose for development and Kubernetes for production, learning how to scale and operate a blockchain network.

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

#### Web UI (TypeScript/React)
- **React 18**: UI framework
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool and dev server
- **React Query**: Server state management
- **React Router**: Client-side routing
- **Tailwind CSS**: Utility-first CSS framework
- **Axios**: HTTP client

### Learning Paths

We've designed this book to accommodate different learning goals and experience levels. Choose the path that best fits where you are in your journey:

#### Path 1: Complete Beginner - Building Understanding from Scratch

**If you're new to blockchain development**, this path will guide you step by step:

1. **Start with [Chapter 1: Introduction & Overview](01-Introduction.md)** - We'll introduce you to the project structure and technical stack, giving you a roadmap for what's ahead.

2. **Continue with [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md)** - Here, we'll explore what Bitcoin and blockchain are, their origins, advantages, and real-world use cases. This provides the foundational knowledge for understanding blockchain technology.

3. **Learn [Chapter 2.1: Cryptography](bitcoin-blockchain/crypto/README.md)** - We'll dive into the cryptographic primitives that secure blockchain: hash functions, digital signatures, key pairs, and address encoding.

4. **Explore [Chapter 2.2: Transaction System](bitcoin-blockchain/02-Transaction-System.md)** - Here, we'll explore core blockchain concepts: transaction structure, the UTXO model, and validation. This is where you'll understand how blockchain fundamentally works.

5. **Understand [Chapter 3: Web API Architecture](bitcoin-blockchain/web/README.md)** - We'll dive into the REST API layer, understanding how HTTP requests are handled, how authentication works, and how the web layer connects clients to the blockchain.

4. **Explore UI Development** - [Chapter 4: Desktop Admin Interface](bitcoin-desktop-ui/03-Desktop-Admin-UI.md) will show you desktop UI architecture using the Iced framework, while [Chapter 5: Wallet User Interface](bitcoin-wallet-ui/04-Wallet-UI.md) demonstrates user-facing application design and state management.

5. **Learn About Data Persistence** - [Chapter 6: Embedded Database & Persistence](bitcoin-wallet-ui/05-Embedded-Database.md) teaches you about encrypted data storage and persistence strategies.

6. **Discover Modern Web Development** - [Chapter 7: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) explores modern web development patterns with React and TypeScript.

#### Path 2: Developer Focus - Implementation Details

**If you're an experienced developer** looking to understand implementation patterns:

1. **Start with [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md)** - Understand blockchain fundamentals, use cases, and technical foundations.

2. **Learn [Chapter 2.1: Cryptography](bitcoin-blockchain/crypto/README.md)** - Explore cryptographic primitives: hash functions, signatures, and key pairs.

3. **Dive into [Chapter 2.2: Transaction System](bitcoin-blockchain/02-Transaction-System.md)** - We'll explore transaction ID format, storage patterns, and performance considerations.

4. **Explore [Chapter 3: Web API Architecture](bitcoin-blockchain/web/README.md)** - We'll understand how the REST API is built, how authentication works, and how handlers process requests.

3. **Study UI Architecture** - [Chapter 4: Desktop Admin Interface](bitcoin-desktop-ui/03-Desktop-Admin-UI.md) covers the MVU pattern, async operations, and UI component design. [Chapter 5: Wallet User Interface](bitcoin-wallet-ui/04-Wallet-UI.md) shows wallet architecture, API integration, and user experience design.

4. **Understand Data Security** - [Chapter 6: Embedded Database & Persistence](bitcoin-wallet-ui/05-Embedded-Database.md) explores SQLCipher integration, schema design, and security considerations.

5. **Explore Modern Web Patterns** - [Chapter 7: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) demonstrates React patterns, TypeScript type safety, and API authentication.

#### Path 3: Operations & Deployment - Production Ready

**If your goal is to deploy and operate** the blockchain network:

1. **Start with [Chapter 8: Docker Compose Deployment](ci/docker-compose/01-Introduction.md)** - We'll walk through local development and single-host deployments. Follow the sections in order: Introduction → Architecture → Execution Flow → Network Configuration → Sequential Startup → Port Mapping → Scaling → Deployment Scenarios → Accessing Webserver → Deployment Guide → Execution Walkthrough → DNS Resolution.

2. **Move to Production** - [Chapter 9: Kubernetes Deployment](ci/kubernetes/README.md) covers production-grade orchestration with autoscaling.

**Prerequisites:** Basic understanding of Docker and containerization concepts will be helpful, but we'll explain everything as we go.

#### Path 4: Quick Reference - Specific Topics

**If you need information about specific topics**, you can jump directly to:

- **Transaction Storage:** [Chapter 2: Transaction System](bitcoin-blockchain/02-Transaction-System.md) - We explore `Vec<u8>` vs String, bytes vs hex representations
- **Web API Architecture:** [Chapter 3: Web API Architecture](bitcoin-blockchain/web/README.md) - REST API implementation, handlers, middleware, and security
- **Desktop UI Patterns:** [Chapter 4: Desktop Admin Interface](bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - MVU architecture and component design
- **Wallet Implementation:** [Chapter 5: Wallet User Interface](bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet architecture and user experience
- **Database Security:** [Chapter 6: Embedded Database](bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher and encryption strategies
- **Web Development:** [Chapter 7: Web Admin Interface](bitcoin-web-ui/06-Web-Admin-UI.md) - React patterns and TypeScript usage
- **Local Deployment:** [Chapter 8: Docker Compose](ci/docker-compose/01-Introduction.md) - Quick start and deployment scenarios
- **Production Deployment:** [Chapter 9: Kubernetes](ci/kubernetes/README.md) - Production deployment and autoscaling
- **Rust Language:** [Rust Language Guide](rust/README.md) - Rust programming language features and examples

---

## Navigation

- **[← Back to Main Book](README.md)** - Book index and table of contents
- **[Next: Introduction to Bitcoin & Blockchain →](bitcoin-blockchain/README.md)** - Bitcoin and blockchain fundamentals
- **[Cryptography →](bitcoin-blockchain/crypto/README.md)** - Cryptographic primitives
- **[Transaction System →](bitcoin-blockchain/02-Transaction-System.md)** - Transaction ID format and storage
- **[Web API Architecture →](bitcoin-blockchain/web/README.md)** - REST API implementation

**Related Guides:**
- **[Rust Language Guide](rust/README.md)** - Rust programming language features
- **[Tokio Runtime Guide](bitcoin-blockchain/Tokio.md)** - Async runtime framework

---

<div align="center">

**📚 [← Back to Main Book](README.md)** | **Introduction & Overview** | **[Next: Introduction to Bitcoin & Blockchain →](bitcoin-blockchain/README.md)** 📚

</div>

---

*This chapter has introduced the book and project structure, providing an overview of what we'll build together and how the book is organized. We've explored the technical stack, project structure, and learning paths designed for different experience levels. Understanding this foundation is essential for navigating the subsequent chapters, as it provides context for how all the pieces of our blockchain implementation fit together. Whether you're a first-time reader following the sequential path or an experienced developer jumping to specific topics, this chapter establishes the framework for understanding the complete system. In the next chapter, we'll begin exploring [Chapter 2: Introduction to Bitcoin & Blockchain](bitcoin-blockchain/README.md) to understand the fundamental concepts of Bitcoin and blockchain technology before diving into implementation details.*

