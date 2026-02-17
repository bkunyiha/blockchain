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
8. **Chapter 2.3: Cryptography** ← *You are here*
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. Chapter 2.5: Storage Layer - Persistent storage implementation
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
12. Chapter 2.7: Network Layer - Peer-to-peer networking and protocol
13. Chapter 2.8: Node Orchestration - Node context and coordination
14. Chapter 2.9: Wallet System - Wallet implementation and key management
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
<div align="right">

**[← Back to Main Book](../../README.md)** | **[← Back to Transaction ID Format](../primitives/02-Transaction-ID-Format.md)**

</div>

---

# Cryptography in Blockchain

**Part I: Core Blockchain Implementation** | **Section 2.3: Cryptography (Cryptographic Primitives and Libraries)**

<div align="center">

**[← Utilities](../util/README.md)** | **Cryptography** | **[Blockchain (Technical Foundations) →](../chain/README.md)** 📚

</div>

---

## Introduction

Cryptography is the foundation of blockchain security. Every transaction must be signed, every address must be derived from keys, and every block must be hashed. In this section, we explore how our blockchain implementation uses cryptographic primitives to ensure security, authenticity, and integrity.
This section examines the cryptographic libraries we use, why we chose them, and how they're applied throughout the blockchain. We'll see how Rust's type system and memory safety 
enable secure cryptographic operations, and how different libraries serve different purposes in our implementation.

In the whitepaper, Satoshi’s system relies on a few cryptographic building blocks:

- **Hashing** makes data tamper-evident and powers proof-of-work.
- **Digital signatures** let a spender prove ownership of a key without revealing it.
- **Key generation** gives users the material needed to sign.
- **Address encoding** turns binary identifiers into human-usable strings.

In this project, the cryptography layer lives in `bitcoin/src/crypto/`.

Cryptography is the part of a Bitcoin implementation that answers three practical questions:

- **How do we name things?** We use hashes as stable, tamper-evident identifiers (for transactions, blocks, and proof-of-work candidates).
- **How do we prove authorization?** We use digital signatures so only the key holder can authorize spends.
- **How do we represent these ideas as bytes?** We standardize key formats, signature formats, and address encodings so nodes can interoperate.

Some of this code is used by consensus-critical paths today (hashing and Schnorr verification). Some pieces exist as learning scaffolding or forward-looking building blocks (for example, alternative signature schemes), and we call that out explicitly where it matters.

## Section map

In this section, we provide a high-level tour of the cryptography layer. For detailed implementations and deeper explanations, we go part-by-part here:

### Section 1: Hash Functions

**01-Hash-Functions** — SHA-256 for transaction IDs, block hashes, and proof-of-work.

### Section 2: Digital Signatures

**02-Digital-Signatures** — Schnorr signatures (primary) and ECDSA helpers (contrast/legacy).

### Section 3: Key Pair Generation

**03-Key-Pair-Generation** — generating private keys and deriving public keys (secp256k1 for Schnorr).

### Section 4: Address Encoding

**04-Address-Encoding** — Base58 encoding/decoding building blocks and how address formats are constructed.

### Section 5: Security and Performance

**05-Security-and-Performance** — threat model, safe practices, and performance tradeoffs.

## How the crypto layer is structured

In Rust, a “module” is a namespace boundary. We use it here to keep cryptographic concerns isolated and easy to audit: application code calls a small set of cryptographic functions, and the crypto module owns the details of which library implements what.

`bitcoin/src/crypto/mod.rs` is the entry point for this section’s code. It defines four submodules, each representing a concrete cryptographic responsibility:

- **`hash`**: hashing bytes into fixed-size digests (identifiers and PoW work units)
- **`signature`**: producing and verifying signatures (authorization)
- **`keypair`**: generating private keys and deriving public keys (identity material)
- **`address`**: converting binary payloads to and from human-facing strings (encoding/decoding)

We keep the crypto surface area explicit and small by re-exporting the functions we want the rest of the crate to call directly:

```rust
pub mod address;
pub mod hash;
pub mod keypair;
pub mod signature;

pub use address::{base58_decode, base58_encode};
pub use hash::{sha256_digest, taproot_hash};
pub use keypair::{get_schnorr_public_key, new_key_pair, new_schnorr_key_pair};
pub use signature::{
    ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify, schnorr_sign_digest,
    schnorr_sign_verify,
};
```

Because `bitcoin/src/lib.rs` re-exports the crypto module (`pub use crypto::*;`), most call sites inside the crate simply use `crate::sha256_digest(...)` or `crate::schnorr_sign_verify(...)`. This is a deliberate ergonomics choice: it keeps cryptographic call sites readable while still letting us centralize cryptographic decisions inside `bitcoin/src/crypto/`.

## Technical foundations (what this code is based on)

This section provides the “why” behind each primitive, at the level you need to implement and review the code safely. We keep it grounded in the whitepaper’s model, but we also call out where our implementation currently differs from Bitcoin Core for simplicity.

### Hashing: what properties we rely on

**SHA-256** (Secure Hash Algorithm, 256-bit) is a *hash function*: a deterministic algorithm that takes any input bytes and produces a fixed 32-byte output (“digest”). You can think of it as a fingerprint of the input: easy to compute, hard to reverse, and extremely sensitive to changes.

When we say “SHA-256,” we are relying on specific properties:

- **Determinism**: same bytes in, same bytes out. That is what allows every node to agree on identifiers.
- **Preimage resistance**: given \(h = \text{SHA256}(m)\), it is computationally infeasible to recover \(m\).
- **Second-preimage resistance**: given \(m\), it is infeasible to find \(m' \ne m\) where \(\text{SHA256}(m) = \text{SHA256}(m')\).
- **Collision resistance**: it is infeasible to find *any* \(m_1 \ne m_2\) with the same hash.
- **Avalanche effect**: flipping 1 bit of input tends to flip ~50% of output bits, which makes tampering obvious.

**What the whitepaper uses hashing for**:

- Block headers link together by including the previous block hash.
- Proof-of-work is “try nonces until the hash is below a target.”
- Transactions are identified by their hashes so that later spends can refer to them.

**How our code maps to this**:

- `bitcoin/src/crypto/hash.rs` provides `sha256_digest(...)` and `taproot_hash(...)`.
- `sha256_digest` is used by transaction ID generation and proof-of-work loops.

#### Important implementation note: “Bitcoin-style” double hashing vs our current code

In Bitcoin Core, many identifiers are computed using **double SHA-256** (often written as \(\text{SHA256}(\text{SHA256}(\cdot))\)). In this project, some places currently use a **single** SHA-256 for simplicity.

This is not a moral failing; it is a deliberate scope choice. But it is consensus-relevant: if you want byte-for-byte compatibility with Bitcoin’s formats, you will need to align those hashing rules.

### Proof-of-work: “hard to compute, easy to verify”

The whitepaper’s proof-of-work idea depends on a crucial asymmetry:

- **Mining**: requires many hash evaluations (brute force search).
- **Validation**: requires one hash evaluation + a comparison.

This is why `sha256_digest` shows up in the PoW module: PoW is simply repeated hashing over candidate header bytes.

### Digital signatures: what a signature means in this system

A **digital signature** is a mathematical proof that “someone who knows a private key approved these bytes.” It lets the network verify authorization without learning the private key.

A signature scheme gives us three things:

- **Authenticity**: “the holder of this private key authorized this message.”
- **Integrity**: if the message changes, verification fails.
- **Unforgeability**: without the private key, producing a valid signature is computationally infeasible.

In Bitcoin’s model, a spend is authorized by proving you can satisfy the previous output’s locking condition (usually “produce a valid signature under this public key / public key hash”).

### Curves and key formats: secp256k1, compressed keys, and x-only keys

An **elliptic curve** is the mathematical structure behind Bitcoin’s public/private keys. Bitcoin’s standard curve is **secp256k1** (defined in SEC 2). In practice, “using secp256k1” means we represent keys as specific byte formats and use curve operations to derive a public key from a private key.

Key material is commonly represented as:

- **Private key**: 32 bytes
- **Compressed public key**: 33 bytes (prefix byte 0x02/0x03 + 32-byte X coordinate)
- **X-only public key** (Taproot/Schnorr context): 32 bytes

Our Schnorr verification path converts a compressed 33-byte public key into an x-only 32-byte representation for verification, because that is what Schnorr verification expects.

### Schnorr vs ECDSA (and a project-specific caveat)

**ECDSA** (Elliptic Curve Digital Signature Algorithm) and **Schnorr** are two different digital signature algorithms. Both can run on secp256k1 keys, but they have different properties and formats.

Bitcoin historically used **ECDSA over secp256k1**. Modern Bitcoin (Taproot, BIP-340) uses **Schnorr over secp256k1**, which has advantages like fixed-size signatures and better composability.

In this project:

- **Schnorr (secp256k1)** is the primary path used by transaction signing/verification.
- We also expose **ECDSA helpers** for experimentation and contrast.

**Caveat**: our ECDSA helpers use **P-256** via `ring`, not secp256k1. That makes them *not Bitcoin-standard*. Keep them as an educational/alternative module unless/until you swap them for secp256k1 ECDSA.

### Address encoding: bytes humans can safely copy/paste

An **address** is not a key. It is a human-friendly string that encodes a binary payload (for example, a version byte plus a hash plus a checksum). In Bitcoin, the encoding choice is designed to reduce transcription errors and make addresses shorter than raw hex.

Humans do not want to copy 32 raw bytes. Encodings like **Base58** are used to:

- avoid ambiguous characters (0/O, I/l),
- shorten representation vs hex,
- support error-detection via a **checksum** (in Bitcoin’s “Base58Check” formats). A checksum is a few bytes derived from the payload; if you mistype the address, the checksum usually won’t match.

**Important distinction**: our `bitcoin/src/crypto/address.rs` currently provides *raw* `base58_encode` / `base58_decode`. Those are building blocks. The full Bitcoin “address format” includes versioning + payload + checksum rules, which live in the wallet/address layer in a full implementation.

## Hashing: making data tamper-evident (and powering mining)

### `sha256_digest(data: &[u8]) -> Vec<u8>`

This is the “workhorse hash.” A hash function is a one-way compression function: it maps arbitrary-length input bytes to a fixed-length output (here, 32 bytes). In Bitcoin, that fixed-length output becomes a stable identifier and a proof-of-work work unit.

In the running code, this hash shows up in several places:

- **Transaction IDs**: `Transaction::hash()` serializes a trimmed copy of the transaction and hashes it.
- **Mining / proof-of-work**: the PoW loop hashes candidate block header bytes repeatedly.
- **Merkle-ish aggregation**: the block code hashes transaction hashes when computing a root/summary.

In other words: hashing is how we create *stable identifiers*, and those identifiers are what we chain together.

### `taproot_hash(data: &[u8]) -> Vec<u8>`

This function is also SHA-256, but implemented via a different library (`sha2` instead of `ring`). Conceptually, there is no new cryptographic primitive here; it is the same digest algorithm with a different implementation.

In this project, `taproot_hash` is used by the wallet address pipeline (via `wallet_impl.rs` → `hash_pub_key`), where we intentionally use a 32-byte public-key hash as part of a Taproot-style address construction.

That mismatch is useful in a book: it lets us discuss dependency choices and how a project evolves. But in production code, we usually want **one** SHA-256 implementation for consistency and smaller dependency surface area.

## Signatures: authorizing spends without a trusted party

### The important idea

In Bitcoin’s model, an output is “locked” to a condition, and an input “unlocks” it by providing a valid signature. Verification is what lets every node agree on which spends are allowed.

### What we actually use right now

In the current implementation, the active signature path is **Schnorr over secp256k1**, exposed as:

- `schnorr_sign_digest(private_key, message) -> Result<Vec<u8>>`
- `schnorr_sign_verify(public_key, signature, message) -> bool`

These are used by the transaction code:

- **Signing**: when a transaction is created, we build a “trimmed copy,” compute its hash, and sign that hash.
- **Verification**: we rebuild the same trimmed copy and verify each input signature against the corresponding public key material.

One subtle (but important) implementation detail: `schnorr_sign_digest` hashes the provided `message` again internally (SHA-256), so today we effectively sign **SHA256(tx_hash_bytes)**. That is not “wrong” in itself, but it is a design choice worth calling out and revisiting as the project matures.

### Where signatures connect to the rest of the code

To keep the mental model tight, here is the dependency chain you should have in your head while reading the transaction code:

- **Transaction signing** (in `bitcoin/src/primitives/transaction.rs`) depends on:
  - `sha256_digest` to compute the transaction ID/hash of a trimmed copy
  - `schnorr_sign_digest` to authorize each input
- **Transaction verification** depends on:
  - reconstructing the same trimmed copy
  - `schnorr_sign_verify` to validate each signature

That is the “verify the math” heart of the whitepaper: validation is deterministic and local, even though creation is distributed.

### ECDSA helpers (present, but not used)

We also expose ECDSA signing/verification via:

- `ecdsa_p256_sha256_sign_digest(...)`
- `ecdsa_p256_sha256_sign_verify(...)`

These are **not used by the chain today**. They exist mainly as a comparison point and for experimentation. (Note: Bitcoin’s historical ECDSA is also over secp256k1; our ECDSA helpers use P-256 via `ring`, which is not Bitcoin-standard.)

## Key generation and address encoding (how bytes become usable artifacts)

Two more files exist because a Bitcoin implementation needs a bridge from “math” to “usable artifacts”:

- **`keypair.rs`**: key generation and derivation. A private key is a 32-byte secret; the public key is derived from it and shared with the network.
- **`address.rs`**: encoding and decoding. An address is not “a key”; it is a human-friendly encoding of a binary payload (version + hash + checksum, depending on format).

These functions are used primarily through the wallet layer today (for example, Base58 encoding/decoding and Taproot-style public-key hashing). As we expand wallet and script functionality, this is where we will tighten format compatibility and add richer address types.

## Where to go next

- If you want the “hashing and mining” story: **01-Hash-Functions** then **Chain (PoW)**.
- If you want the “ownership and authorization” story: **02-Digital-Signatures** then the transaction code in `bitcoin/src/primitives/transaction.rs`.
- If you want the “wallet pipeline” story: **03-Key-Pair-Generation** and **04-Address-Encoding**.

## Additional resources

- **Internal docs**:
  - **01-Hash-Functions**
  - **02-Digital-Signatures**
  - **03-Key-Pair-Generation**
  - **04-Address-Encoding**
  - **05-Security-and-Performance**

## References and further reading

In this section, we provide credible references you can use to validate the concepts behind the code and go deeper when you need more detail.

### Bitcoin foundations

- **[Bitcoin: A Peer-to-Peer Electronic Cash System (whitepaper)](https://bitcoin.org/bitcoin.pdf)**
- **[Bitcoin Core Developer Guide](https://developer.bitcoin.org/devguide/)**

### Hashing and proof-of-work

- **[FIPS 180-4: Secure Hash Standard (SHA-256)](https://csrc.nist.gov/publications/detail/fips/180/4/final)** (the SHA-2 specification)
- **[Bitcoin Wiki: Proof of Work](https://en.bitcoin.it/wiki/Proof_of_work)** (high-level PoW intuition and terminology)

### Schnorr signatures and Taproot

- **[BIP 340: Schnorr Signatures for secp256k1](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)** (x-only keys, signature format, tagged hash details)
- **[BIP 341: Taproot](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)** (Taproot output construction and spending paths)
- **[BIP 342: Validation of Taproot Scripts](https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki)** (Tapscript semantics)
- **[secp256k1 library (Bitcoin Core)](https://github.com/bitcoin-core/secp256k1)** (reference implementation used widely in the ecosystem)

### ECDSA (background / contrast)

- **[FIPS 186-5: Digital Signature Standard (DSS)](https://csrc.nist.gov/publications/detail/fips/186/5/final)** (ECDSA definition and security considerations)
- **[SEC 2: Recommended Elliptic Curve Domain Parameters](https://www.secg.org/sec2-v2.pdf)** (includes secp256k1 parameters used by Bitcoin)

### Address formats and encodings

- **[Bitcoin Wiki: Base58Check encoding](https://en.bitcoin.it/wiki/Base58Check_encoding)** (version + payload + checksum conventions)
- **[Bitcoin Wiki: Address](https://en.bitcoin.it/wiki/Address)** (address types and high-level structure)

### Rust and implementation practice

- **[The Rust Programming Language (“The Rust Book”)](https://doc.rust-lang.org/book/)** (language fundamentals)
- **[ring crate documentation](https://docs.rs/ring/latest/ring/)** (the `ring` APIs used for SHA-256 and P-256 ECDSA helpers in this project)
- **[secp256k1 crate documentation](https://docs.rs/secp256k1/latest/secp256k1/)** (the Rust wrapper used for secp256k1 and Schnorr in this project)

---

---

<div align="center">

**[📚 ← Utilities](../util/README.md)** | **Section 2.3: Cryptography** | **[Start Reading: Hash Functions →](01-Hash-Functions.md)** 📚

</div>

---

**📖 Continue Reading Cryptography Documentation:**

- **01: Hash Functions** - SHA-256 hashing for transaction IDs, block hashes, and Merkle trees
- **02: Digital Signatures** - Schnorr and ECDSA signatures for transaction authorization
- **03: Key Pair Generation** - Secure key pair generation and public key derivation
- **04: Address Encoding** - Base58 encoding for human-readable addresses
- **05: Security and Performance** - Security best practices and performance considerations

---

*In this section, we connected the whitepaper’s security model to real Rust code. We saw where hashing and Schnorr verification show up in the running system today, and we identified which crypto helpers are currently “book scaffolding” versus production dependencies. Next, we dive into Hash Functions and trace how SHA-256 shows up in transaction IDs, block linking, and proof-of-work.*
