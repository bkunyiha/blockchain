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
# Wallet System — Keys, Addresses, and Local Wallet Persistence

**Part I: Core Blockchain Implementation** | **Chapter 2.9: Wallet System**

<div align="center">

**[📚 ← Chapter 2.8: Node Orchestration](../node/README.md)** | **[Chapter 2.9: Wallet System](README.md)** | **[Chapter 3: Web API Architecture →](../web/README.md)** 📚

</div>

---

## Overview

The wallet system (`bitcoin/src/wallet`) is the part of our Rust Bitcoin implementation that turns **keys** into a stable **address**, validates that address format, and persists a set of wallets locally on disk.

This chapter is written as a code walkthrough. You should be able to read it without having the project open:

- every referenced method is printed in full in the walkthrough chapter below (or explicitly marked “defined earlier” and linked)
- every section has a consistent **Methods involved** box
- diagrams show the exact on-wire / on-disk formats this module produces

> **Methods involved**
>
> - `Wallet::{new, get_address, get_public_key, get_private_key}`
> - `WalletAddress::{validate}` (and its internal validation)
> - `convert_address`, `get_pub_key_hash`
> - `WalletService::{new, create_wallet, get_addresses, get_wallet, load_from_file, get_wallet_file_path}`

---

## What this wallet module actually implements (scope)

In `bitcoin/src/wallet`, we implement three concrete things:

- **Keypair creation**: `Wallet::new()` generates a Schnorr keypair (Taproot-style).
- **Address derivation + validation**: `Wallet::get_address()` and `WalletAddress::validate(...)` implement a Base58 payload format with a version byte + hash + checksum.
- **Local persistence**: `WalletService` loads/saves a map of wallets to a single file (default `wallets.dat`) using `bincode`.

What we **do not** implement in this module:

- **Transaction signing** is handled elsewhere (crypto / transaction logic). This wallet module focuses on keys, addresses, and persistence of wallet material.

---

## Diagram: address payload structure used here

This wallet implementation uses a payload that matches the “classic” Base58Check idea (version + data + checksum), but note the version byte and hashing choices are specific to our implementation.

```
payload bytes:
  [ version: 1 byte ] [ pub_key_hash: N bytes ] [ checksum: 4 bytes ]

encoded as:
  Base58(payload)
```

> **Methods involved**
>
> - `Wallet::get_address()`
> - `WalletAddress::validate(...)`
> - `convert_address(...)`
> - `get_pub_key_hash(...)`

---

Chapter 2.9.A: Code Walkthrough
<div align="center">

**[📚 ← Chapter 2.8: Node Orchestration](../node/README.md)** | **[Chapter 2.9: Wallet System](README.md)** | **[Chapter 2.9.A: Wallet System — Code Walkthrough →](01-Wallet-System-Code-Walkthrough.md)** 📚

</div>

---

*Next: Chapter 3 shows how the web layer exposes wallet and node operations through HTTP routes, request models, and handlers.*
