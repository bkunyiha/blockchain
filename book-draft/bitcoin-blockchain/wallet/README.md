<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
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
14. **Chapter 14: Wallet System** ← *You are here*
15. <a href="../web/README.md">Chapter 15: Web API Architecture</a>
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
# Wallet System — Keys, Addresses, and Local Wallet Persistence

**Part I: Foundations & Core Implementation** | **Chapter 14: Wallet System**

<div align="center">

**[← Chapter 13: Node Orchestration](../node/README.md)** | **[Chapter 14: Wallet System](README.md)** | **[Chapter 15: Web API Architecture →](../web/README.md)**
</div>

---

> **Prerequisites:**: This chapter depends on the cryptographic primitives from Chapter 8 (key generation, hashing, signing) and the UTXO model from Chapter 9. You do not need to have read the network or node chapters — the wallet module is a standalone library that the node and UI layers consume.

> **What you will learn in this chapter:**
> - Generate cryptographic key pairs and create wallet addresses
> - Sign transactions to authorize spending from a wallet
> - Persist wallet data securely across application restarts
> - Understand the key management lifecycle from creation through usage to storage

> **Scope:** This chapter covers single-key wallets with basic send and receive functionality. We do not cover BIP-32 hierarchical deterministic (HD) wallets, multi-signature schemes, hardware wallet integration, or Lightning Network payment channels.

---

## Overview

The wallet system (`bitcoin/src/wallet`) is the part of our Rust Bitcoin implementation that turns **keys** into a stable **address**, validates that address format, and persists a set of wallets locally on disk.

The key types are `Wallet` (key material and address derivation), `WalletAddress` (validation logic), and `WalletService` (persistence and lifecycle). The full method listings appear in the companion chapter below.

---

## What this wallet module actually implements (scope)

In `bitcoin/src/wallet`, we implement three concrete things:

- **Keypair creation**: `Wallet::new()` generates a Schnorr keypair (Taproot-style).
- **Address derivation + validation**: `Wallet::get_address()` and `WalletAddress::validate(...)` implement a Base58 payload format with a version byte + hash + checksum.
- **Local persistence**: `WalletService` loads/saves a map of wallets to a single file (default `wallets.dat`) using `bincode`.

What we do **not** implement in this module:

- **Transaction signing** is handled elsewhere (crypto and transaction logic). This wallet module focuses on keys, addresses, and persistence of wallet material.

> **Warning:** Never reuse a private key across wallets. If a key is compromised in one wallet, all funds associated with that key are at risk regardless of which wallet holds them.

---

## Diagram: address payload structure used here

This wallet implementation uses a payload that matches the “classic” Base58Check idea (version + data + checksum), but note the version byte and hashing choices are specific to our implementation. This is intentional for teaching purposes; Bitcoin's actual address format is slightly different.

```text
payload bytes:
  [ version: 1 byte ] [ pub_key_hash: N bytes ] [ checksum: 4 bytes ]

encoded as:
  Base58(payload)
```

The address pipeline flows through `Wallet::get_address` (constructs the payload), `get_pub_key_hash` (hashes the public key), and `WalletAddress::validate` (verifies checksum and version on decode). `convert_address` extracts the public key hash from a Base58 address string.

---

## Exercises

1. **Multi-Wallet Transfer** — Generate two wallets, fund the first with a coinbase transaction, then send coins from the first wallet to the second. Verify the UTXO set reflects the transfer. Attempt to spend more than the first wallet's balance and confirm the transaction is rejected.

2. **Key Persistence Verification** — Create a wallet, generate an address, and save the wallet. Restart the application, reload the wallet, and verify the same key pair and address are available. This confirms the persistence layer correctly serializes and deserializes cryptographic keys.

---

## Summary

- We built the wallet system that generates key pairs, creates addresses, and signs transactions to authorize spending.
- We implemented wallet persistence so keys and addresses survive across application restarts.
- We traced the key management lifecycle from generation through address derivation, transaction signing, and secure storage.

In the next chapter, we expose the blockchain's capabilities through a REST API, creating the interface that desktop and web frontends will consume.

---

Chapter 14.A: Code Walkthrough
<div align="center">

**[← Chapter 13: Node Orchestration](../node/README.md)** | **[Chapter 14: Wallet System](README.md)** | **[Chapter 14.A: Wallet System — Code Walkthrough →](01-Wallet-System-Code-Walkthrough.md)**
</div>

---

### Further Reading

- **BIP-32 (HD Wallets)** — Defines hierarchical deterministic key derivation, allowing an entire tree of key pairs to be generated from a single seed. The `bitcoin` crate in the Rust ecosystem implements this.
- **BIP-39 (Mnemonic Seed Phrases)** — Specifies how a random seed is encoded as a human-readable list of 12 or 24 words. The `bip39` crate provides a Rust implementation.
- **BIP-44 (Multi-Account Hierarchy)** — Builds on BIP-32 to define a standard derivation path (`m/44'/0'/0'/...`) for organizing keys by coin type and account.
- **[rust-bitcoin wallet module](https://docs.rs/bitcoin)** — The community's production-grade Bitcoin library for Rust. Studying its wallet implementation reveals the gap between our teaching implementation and production requirements.
