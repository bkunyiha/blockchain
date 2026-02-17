<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. Chapter 1.4: Bitcoin Whitepaper In Rust - Bitcoin Whitepaper In Rust
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. Chapter 2.5: Storage Layer - Persistent storage implementation
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
12. Chapter 2.7: Network Layer - Peer-to-peer networking and protocol
13. Chapter 2.8: Node Orchestration - Node context and coordination
14. **Chapter 2.9: Wallet System** ← *You are here*
15. Chapter 3: Web API Architecture - REST API implementation
16. Chapter 4: Desktop Admin Interface - Iced framework architecture
17. Chapter 5: Wallet User Interface - Wallet UI implementation
18. Chapter 6: Embedded Database & Persistence - SQLCipher integration
19. Chapter 7: Web Admin Interface - React/TypeScript web UI

### Part II: Deployment & Operations

20. Chapter 8: Docker Compose Deployment - Docker Compose guide
21. Chapter 9: Kubernetes Deployment - Kubernetes production guide
22. Chapter 10: Rust Language Guide - Rust programming language reference

</details>

</div>

---
# Wallet System

**Part I: Core Blockchain Implementation** | **Chapter 2.9: Wallet System**

<div align="center">

**[📚 ← Chapter 2.8: Node Orchestration](../node/README.md)** | **[Chapter 2.9: Wallet System](README.md)** | **[Chapter 3: Web API Architecture →](../web/README.md)** 📚

</div>

---

## Overview

The wallet system (`bitcoin/src/wallet`) provides functionality for creating, managing, and using cryptocurrency wallets. This module handles key pair generation, address creation, transaction signing, and wallet persistence.

Following Bitcoin Core's wallet architecture, this module provides the cryptographic and key management functionality needed for users to securely store and spend their cryptocurrency.

## Key Components

### Wallet

The `Wallet` structure represents an individual wallet:

**Wallet Components:**
- **Private Key**: Secret key for signing transactions
- **Public Key**: Public key derived from private key
- **Address**: Wallet address derived from public key
- **Key Management**: Secure key storage and access

**Key Operations:**
- Wallet creation
- Address generation
- Transaction signing
- Key pair management

### WalletService

The `WalletService` manages multiple wallets:

**Service Responsibilities:**
- Wallet creation and management
- Wallet persistence (file-based storage)
- Address listing and retrieval
- Wallet loading and saving

**Storage:**
- File-based wallet storage (`wallets.dat`)
- Wallet serialization
- Secure key storage patterns

### Address Management

Address-related utilities:

**Address Operations:**
- Address encoding/decoding
- Address validation
- Public key hash extraction
- Address checksum verification

## Relationship to Bitcoin Core

This module aligns with Bitcoin Core's wallet architecture:

- **Bitcoin Core's `wallet/`**: Wallet functionality
- **Bitcoin Core's key management**: Private/public key handling
- **Bitcoin Core's address format**: Address encoding/decoding

## Topics to Cover

### Core Concepts

1. **Wallet Architecture**
   - Wallet data structures
   - Key pair management
   - Address generation
   - Wallet lifecycle

2. **Cryptographic Operations**
   - Key pair generation
   - Address derivation (Base58 encoding)
   - Transaction signing
   - Signature verification

3. **Wallet Service**
   - Multi-wallet management
   - Wallet persistence
   - File-based storage
   - Wallet loading/saving

### Implementation Details

4. **Key Management**
   - Private key generation
   - Public key derivation
   - Key storage security
   - Key recovery mechanisms

5. **Address System**
   - Address encoding (Base58)
   - Address validation
   - Checksum verification
   - Address format standards

6. **Transaction Signing**
   - Signing process
   - Signature creation
   - Signature verification
   - Transaction authorization

### Advanced Topics

7. **Security Considerations**
   - Private key protection
   - Secure storage patterns
   - Key derivation functions
   - Best practices

8. **Wallet Persistence**
   - File format design
   - Serialization patterns
   - Encryption considerations
   - Backup strategies

9. **Address Formats**
   - Base58 encoding
   - Checksum algorithms
   - Address versioning
   - Multi-format support

## Related Chapters

- **Cryptography**: Cryptographic primitives used in wallets
- **Transaction ID Format**: Transaction signing and creation
- **Node Orchestration**: Wallet integration with node
- **Web API Architecture**: Wallet API endpoints

## Code Examples

**Creating a Wallet:**

```rust
use blockchain::wallet::{Wallet, WalletService};

// Create new wallet
let mut wallet_service = WalletService::new()?;
let address = wallet_service.create_wallet()?;

// Get wallet by address
let wallet = wallet_service.get_wallet(&address)?;
```

**Address Operations:**

```rust
use blockchain::wallet::{WalletAddress, convert_address, get_pub_key_hash};

// Get address from wallet
let address = wallet.get_address()?;

// Convert address to public key hash
let pub_key_hash = get_pub_key_hash(&address)?;

// Validate address
let is_valid = convert_address(&address).is_ok();
```

**Transaction Signing:**

```rust
use blockchain::wallet::Wallet;

// Sign transaction
let signed_tx = wallet.sign_transaction(transaction, utxos)?;

// Get public key for verification
let pub_key = wallet.get_public_key();
```

---

<div align="center">

**[📚 ← Chapter 2.8: Node Orchestration](../node/README.md)** | **[Chapter 2.9: Wallet System](README.md)** | **[Chapter 3: Web API Architecture →](../web/README.md)** 📚

</div>

---

*This chapter has examined the wallet system that provides functionality for creating, managing, and using cryptocurrency wallets. We've explored how key pairs are generated, how addresses are created using Base58 encoding, how transactions are signed, and how wallets are persisted securely. The wallet module follows Bitcoin Core's wallet architecture, providing the cryptographic and key management functionality needed for users to securely store and spend cryptocurrency. In the next chapter, we'll explore Chapter 3: Web API Architecture to understand how the REST API interface enables clients to interact with the blockchain through HTTP endpoints.*
