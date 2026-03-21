# Glossary

This glossary defines terms used throughout the book. Each entry includes the chapter where the term is first introduced or most thoroughly explained.

---

## Bitcoin and Blockchain Terms

**Address** — A shortened, human-readable representation of a public key hash used to receive Bitcoin. Our implementation derives addresses from ECDSA public keys using SHA-256 and RIPEMD-160 hashing. *(Ch 14)*

**BIP (Bitcoin Improvement Proposal)** — A design document for introducing features or information to Bitcoin. Referenced BIPs include BIP-32 (HD wallets), BIP-39 (mnemonic seeds), and BIP-44 (multi-account hierarchy). *(Ch 14, Further Reading)*

**Block** — A data structure containing a header (previous hash, timestamp, nonce, Merkle root) and an ordered list of transactions. Represented as the `Block` struct. *(Ch 6)*

**Block hash** — The SHA-256 hash of a block's header, used as the block's unique identifier. Stored as `Vec<u8>` in our implementation. *(Ch 6, 8)*

**Block height** — The position of a block in the chain, counting from the genesis block (height 0). *(Ch 9)*

**Blockchain** — An append-only sequence of blocks where each block references the hash of its predecessor, forming a tamper-evident chain. *(Ch 2)*

**Canonical chain (tip)** — The longest valid chain of blocks. The tip is the most recent block on that chain. Our storage layer tracks it via the `"tip_block_hash"` key. *(Ch 9, 11)*

**Coinbase transaction** — The first transaction in every block, which creates new coins as the miner's reward. It has no inputs (no coins are being spent). *(Ch 9)*

**Consensus** — The mechanism by which nodes agree on which chain of blocks is valid. In our implementation, consensus follows the longest-chain rule with proof-of-work. *(Ch 10)*

**Difficulty Target** — A 256-bit number that a block's hash must be less than for the block to be valid. Adjusted periodically to maintain consistent block times. *(Ch 10)*

**Difficulty** — A measure of how hard it is to mine a new block. Our implementation uses a simplified fixed difficulty target. *(Ch 10)*

**Double-spend** — An attempt to spend the same UTXO in two different transactions. Prevented by the consensus layer rejecting blocks that reference already-spent outputs. *(Ch 3, 10)*

**Hash function** — A deterministic function that maps arbitrary-length input to a fixed-length output (the hash or digest). Bitcoin uses SHA-256 and RIPEMD-160 for block hashing, transaction IDs, and address derivation. A cryptographic hash function is collision-resistant and one-way. *(Ch 8)*

**Fork** — When two valid blocks reference the same parent, creating a temporary branch in the chain. Resolved when one branch becomes longer than the other. *(Ch 10)*

**Genesis block** — The hardcoded first block in the chain (height 0). It has no parent hash. *(Ch 9)*

**Mempool (Memory Pool)** — The set of unconfirmed transactions waiting to be included in a block. Each node maintains its own mempool, prioritizing transactions by fee rate. *(Ch 13)*

**Merkle root** — A single hash that summarizes all transactions in a block, computed by recursively hashing pairs of transaction hashes into a binary tree. *(Ch 9)*

**Mining** — The process of finding a nonce that makes a block's hash satisfy the difficulty target. The first miner to find a valid nonce earns the coinbase reward and the right to append their block to the chain. *(Ch 10, 13)*

**Nonce** — A 32-bit field in the block header that miners increment to search for a valid block hash. The nonce is the variable that makes mining a trial-and-error process. *(Ch 10)*

**Outpoint** — A reference to a specific output of a previous transaction, consisting of a transaction ID and an output index. Used as the "pointer" in transaction inputs. *(Ch 6)*

**Peer** — Another node in the network. Our implementation connects peers via TCP and exchanges typed messages (`Package` structs). *(Ch 12)*

**Peer-to-peer (P2P)** — A network architecture where nodes communicate directly with each other rather than through a central server. Each node in our blockchain acts as both client and server, relaying blocks and transactions. *(Ch 2, 12)*

**Proof-of-work (PoW)** — The consensus mechanism where miners must find a nonce that makes the block hash fall below a target. Proves computational effort was spent. *(Ch 3, 10)*

**SPV (Simplified Payment Verification)** — A method that allows lightweight clients to verify transactions without downloading the entire blockchain, using Merkle proofs to confirm inclusion in a block. *(Ch 3)*

**Transaction** — A data structure that transfers value by consuming UTXOs (inputs) and creating new UTXOs (outputs). Represented as the `Transaction` struct. *(Ch 6)*

**Transaction ID (txid)** — The SHA-256 hash of a serialized transaction, used as its unique identifier. *(Ch 6, 8)*

**UTXO (Unspent Transaction Output)** — An output of a previous transaction that has not yet been consumed as an input. The UTXO set represents all spendable coins at any point in time. *(Ch 3, 9)*

**Wallet** — A component that manages private keys, derives addresses, signs transactions, and tracks balances by scanning the UTXO set. Our project includes both Iced and Tauri wallet implementations with encrypted local storage. *(Ch 14, 18, 19)*

---

## Rust Language Terms

**`async`/`await`** — Rust's syntax for asynchronous programming. Functions marked `async fn` return a `Future` that is lazily executed. Used throughout the networking, node, and web API layers. *(Ch 10, 12)*

**Axum** — A web framework built on top of Tower and Hyper that provides routing, extractors, and middleware for async HTTP services. Powers the blockchain node's REST API. *(Ch 15)*

**Borrow checker** — The Rust compiler's system for enforcing ownership rules at compile time. Prevents data races and use-after-free bugs without runtime overhead. *(Ch 24)*

**Cargo Workspace** — A set of related Rust packages (crates) that share a common Cargo.lock and output directory. This project uses a workspace to organize its many crates. *(Ch 5)*

**Channel (`mpsc`)** — A multi-producer, single-consumer message-passing primitive from Tokio. Used in the node module for routing messages between subsystems. *(Ch 13)*

**Derive macro** — A `#[derive(...)]` attribute that auto-generates trait implementations. Our primitives derive `Serialize`, `Deserialize`, `Clone`, and `Debug`. *(Ch 6)*

**`enum` (algebraic data type)** — A type that can hold one of several variants. Used for message types (`Package`), error types, and UI messages in the Iced framework. *(Ch 12, 16)*

**Feature Flags** — Conditional compilation markers in Cargo.toml that enable or disable optional dependencies and code paths at build time. *(Ch 5)*

**Lifetime** — An annotation (e.g., `'a`) that tells the borrow checker how long a reference is valid. Most lifetimes in this book are elided (inferred by the compiler). *(Ch 24)*

**Monomorphization** — The process by which the Rust compiler generates specialized versions of generic functions for each concrete type. Produces zero-cost abstractions. *(Ch 24)*

**`Option<T>`** — A type representing either `Some(value)` or `None`. Used instead of null pointers. *(Ch 24)*

**Ownership** — Rust's core memory management concept: every value has a single owner, and the value is dropped when the owner goes out of scope. *(Ch 24)*

**Pin** — A wrapper type that prevents a value from being moved in memory. Required for certain async patterns where futures hold self-referential pointers. *(Ch 24)*

**`Result<T, E>`** — A type representing either `Ok(value)` or `Err(error)`. The primary error-handling mechanism in Rust. *(Ch 24)*

**secp256k1** — The elliptic curve used in Bitcoin's digital signature scheme. The `secp256k1` crate provides key generation, signing, and verification. Our wallet derives addresses from secp256k1 public keys. *(Ch 8, 14)*

**Serde** — The Rust ecosystem's standard serialization/deserialization framework. All primitive types derive `Serialize` and `Deserialize` for JSON and binary encoding. *(Ch 6)*

**Tokio** — The async runtime used throughout this project. Provides task spawning (`tokio::spawn`), TCP networking, timers, and synchronization primitives. *(Ch 10, 12)*

**Trait** — Rust's mechanism for defining shared behavior (similar to interfaces). Key traits in this project include `Serialize`, `Deserialize`, `Clone`, and custom traits for the API layer. *(Ch 24)*

**Trait Object** — A dynamically dispatched reference to a type implementing a trait, written as `dyn Trait`. Used when the concrete type is not known at compile time. *(Ch 24)*

**`Vec<u8>`** — A growable byte vector. Used throughout the primitives module for hashes, transaction IDs, and serialized data. Chosen over fixed-size arrays for flexibility. *(Ch 6)*

---

## Deployment & Infrastructure Terms

**Container** — A lightweight, isolated environment that packages application code, dependencies, and configuration. Containers are created from images and managed by Docker or Kubernetes. *(Ch 22, 23)*

**Container Registry** — A centralized storage for Docker images (e.g., Docker Hub, AWS ECR). Registries allow teams to push and pull images for deployment. *(Ch 22, 23)*

**Headless Service** — A Kubernetes Service without a ClusterIP, used for StatefulSets where pods need stable DNS names without load balancing. Each pod is directly addressable. *(Ch 23)*

**Volume** — A Docker storage mechanism for persisting data beyond container lifetime. Volumes are managed by Docker and mounted into containers at a path. *(Ch 22)*

**PersistentVolumeClaim (PVC)** — A Kubernetes request for storage. PVCs abstract underlying storage systems, allowing pods to request storage without knowing implementation details. *(Ch 23)*

---

## Project-Specific Terms

**bitcoin-api** — The shared API client crate consumed by all frontend applications (Iced, Tauri, and web). Provides a framework-agnostic interface to the blockchain node's REST API. *(Ch 15, 16, 17)*

**BlockchainFileSystem** — The storage abstraction that wraps the Sled embedded database, providing persistence for blocks, chain state, and UTXO data. *(Ch 11)*

**`ChainState`** — The in-memory representation of the current blockchain state: the canonical chain, the UTXO set, and the tip hash. *(Ch 9)*

**Companion Chapter** — A supplementary chapter (suffixed A, B, or C) containing complete, unabridged code listings for its parent chapter. Excluded from the print edition. *(Throughout)*

**NodeContext** — The central orchestration struct that coordinates blockchain state, mempool, network operations, mining, and validation behind a unified API. *(Ch 13)*

**`Package`** — The typed message enum used in the network layer. Variants include `NewBlock`, `NewTransaction`, `RequestBlock`, and peer-management messages. *(Ch 12)*

---

<div align="center">

**[← Back to Main Book](README.md)**

</div>
