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

**Difficulty** — A measure of how hard it is to mine a new block. Our implementation uses a simplified fixed difficulty target. *(Ch 10)*

**Double-spend** — An attempt to spend the same UTXO in two different transactions. Prevented by the consensus layer rejecting blocks that reference already-spent outputs. *(Ch 3, 10)*

**Fork** — When two valid blocks reference the same parent, creating a temporary branch in the chain. Resolved when one branch becomes longer than the other. *(Ch 10)*

**Genesis block** — The hardcoded first block in the chain (height 0). It has no parent hash. *(Ch 9)*

**Mempool** — The set of unconfirmed transactions that a node has received but not yet included in a block. Implemented as an in-memory collection in the node module. *(Ch 13)*

**Merkle root** — A single hash that summarizes all transactions in a block, computed by recursively hashing pairs of transaction hashes into a binary tree. *(Ch 9)*

**Nonce** — A number that miners increment to produce a block hash below the difficulty target. The core mechanism of proof-of-work. *(Ch 10)*

**Outpoint** — A reference to a specific output of a previous transaction, consisting of a transaction ID and an output index. Used as the "pointer" in transaction inputs. *(Ch 6)*

**Peer** — Another node in the network. Our implementation connects peers via TCP and exchanges typed messages (`Package` structs). *(Ch 12)*

**Proof-of-work (PoW)** — The consensus mechanism where miners must find a nonce that makes the block hash fall below a target. Proves computational effort was spent. *(Ch 3, 10)*

**SPV (Simplified Payment Verification)** — A method for lightweight clients to verify transactions without downloading the full blockchain, using Merkle proofs. Not implemented in this book. *(Ch 3)*

**Transaction** — A data structure that transfers value by consuming UTXOs (inputs) and creating new UTXOs (outputs). Represented as the `Transaction` struct. *(Ch 6)*

**Transaction ID (txid)** — The SHA-256 hash of a serialized transaction, used as its unique identifier. *(Ch 6, 8)*

**UTXO (Unspent Transaction Output)** — An output of a previous transaction that has not yet been consumed as an input. The UTXO set represents all spendable coins at any point in time. *(Ch 3, 9)*

---

## Rust Language Terms

**`async`/`await`** — Rust's syntax for asynchronous programming. Functions marked `async fn` return a `Future` that is lazily executed. Used throughout the networking, node, and web API layers. *(Ch 12, 10)*

**Borrow checker** — The Rust compiler's system for enforcing ownership rules at compile time. Prevents data races and use-after-free bugs without runtime overhead. *(Ch 24)*

**Channel (`mpsc`)** — A multi-producer, single-consumer message-passing primitive from Tokio. Used in the node module for routing messages between subsystems. *(Ch 13)*

**Derive macro** — A `#[derive(...)]` attribute that auto-generates trait implementations. Our primitives derive `Serialize`, `Deserialize`, `Clone`, and `Debug`. *(Ch 6)*

**`enum` (algebraic data type)** — A type that can hold one of several variants. Used for message types (`Package`), error types, and UI messages in the Iced framework. *(Ch 12, 16)*

**Lifetime** — An annotation (e.g., `'a`) that tells the borrow checker how long a reference is valid. Most lifetimes in this book are elided (inferred by the compiler). *(Ch 24)*

**Monomorphization** — The process by which the Rust compiler generates specialized versions of generic functions for each concrete type. Produces zero-cost abstractions. *(Ch 24)*

**`Option<T>`** — A type representing either `Some(value)` or `None`. Used instead of null pointers. *(Ch 24)*

**Ownership** — Rust's core memory management concept: every value has a single owner, and the value is dropped when the owner goes out of scope. *(Ch 24)*

**`Result<T, E>`** — A type representing either `Ok(value)` or `Err(error)`. The primary error-handling mechanism in Rust. *(Ch 24)*

**Serde** — The Rust ecosystem's standard serialization/deserialization framework. All primitive types derive `Serialize` and `Deserialize` for JSON and binary encoding. *(Ch 6)*

**Tokio** — The async runtime used throughout this project. Provides task spawning (`tokio::spawn`), TCP networking, timers, and synchronization primitives. *(Ch 12, 10)*

**Trait** — Rust's mechanism for defining shared behavior (similar to interfaces). Key traits in this project include `Serialize`, `Deserialize`, `Clone`, and custom traits for the API layer. *(Ch 24)*

**`Vec<u8>`** — A growable byte vector. Used throughout the primitives module for hashes, transaction IDs, and serialized data. Chosen over fixed-size arrays for flexibility. *(Ch 6)*

---

## Project-Specific Terms

**`bitcoin-api` crate** — The shared API layer that both the Iced and Tauri admin UIs consume. Wraps the node's admin endpoints into a Rust client library. *(Ch 15, 16, 17)*

**`BlockchainFileSystem`** — The storage abstraction that wraps sled and provides methods like `create_blockchain`, `get_tip_hash`, and `update_blocks_tree`. *(Ch 11)*

**`ChainState`** — The in-memory representation of the current blockchain state: the canonical chain, the UTXO set, and the tip hash. *(Ch 9)*

**Companion chapter** — A chapter suffixed with A, B, or C (e.g., 11.A, 17B) containing complete code listings. These are available online but excluded from the print edition. *(Throughout)*

**`NodeContext`** — The central coordination struct in the node module. Routes inbound messages from the network to the appropriate subsystem (mempool, chainstate, mining, relay). *(Ch 13)*

**`Package`** — The typed message enum used in the network layer. Variants include `NewBlock`, `NewTransaction`, `RequestBlock`, and peer-management messages. *(Ch 12)*

---

<div align="center">

**[← Back to Main Book](README.md)**

</div>
