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
8. <a href="README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../net/README.md">Chapter 12: Network Layer</a>
13. <a href="../node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../wallet/README.md">Chapter 14: Wallet System</a>
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
# Hash Functions: SHA-256 in Blockchain

Hash functions are fundamental to blockchain operations. They create fixed-size fingerprints of arbitrary data, enabling efficient identification and verification. In this section, we explore how SHA-256 is used throughout our blockchain implementation for transaction IDs, block hashes, Merkle trees, and address generation.

## Table of Contents

1. [Overview: Hash Functions in Blockchain](#overview-hash-functions-in-blockchain)
2. [SHA-256 Digest: General-Purpose Hashing](#sha-256-digest-general-purpose-hashing)
3. [Taproot-related hashing: P2TR Address Hashing](#taproot-related-hashing-p2tr-address-hashing)
4. [Usage in Transaction ID Format](#usage-in-transaction-system)
5. [Usage in Block System](#usage-in-block-system)
6. [Usage in Proof-of-Work Mining](#usage-in-proof-of-work-mining)
7. [Hash Function Properties](#hash-function-properties)
8. [Why Two Hash Functions?](#why-two-hash-functions)

---

## Overview: Hash Functions in Blockchain

Before we talk about “hash functions,” it helps to define what a **hash** is in plain terms.

A **hash** (also called a *digest*) is the fixed-size output of a hash function. You can think of it as a compact “fingerprint” of some input bytes:

- **Same input → same hash** (deterministic)
- **Small change in input → very different hash** (avalanche effect)
- **Hash is easy to compute, but hard to reverse** (you can’t recover the original input from the hash in any practical way)

In Bitcoin and in this project, hashes are used as *identifiers* (“this transaction / this block”), as *tamper-evidence* (“if the bytes change, the hash changes”), and as the work unit for proof-of-work (“find a hash below a target”).

Hash functions serve multiple critical roles in blockchain systems:

1. **Transaction Identification**: Every transaction gets a unique ID by hashing its contents
2. **Block Identification**: Blocks are identified by their hash, calculated from header data
3. **Merkle Tree Construction**: Transaction hashes are combined into a Merkle tree root
4. **Address Generation**: Public keys are hashed to create addresses
5. **Proof-of-Work**: Miners hash block data repeatedly to find valid nonces

Our implementation uses SHA-256 (Secure Hash Algorithm 256-bit) for all these purposes. SHA-256 produces a 32-byte (256-bit) hash output, providing sufficient security for blockchain operations.

### Figure: Where hashing shows up in the system

```text
                 ┌───────────────────────────────┐
                 │        Block header           │
                 │ (prev_hash, merkle-ish root,  │
                 │  timestamp, nonce, ...)       │
                 └──────────────┬────────────────┘
                                │
                                │  sha256_digest(...)
                                ▼
                         block header hash
                                │
                ┌───────────────┴────────────────┐
                │                                │
                ▼                                ▼
          proof-of-work loop                block linking
   (search for nonce/target)      (next block references prev)

  Transactions:
  TX (minus id) ──serialize──▶ bytes ──sha256_digest──▶ txid

  Wallet/address pipeline:
  pubkey ──taproot_hash──▶ pubkey_hash ──Base58Check──▶ address string
```

### Hash Function Requirements

Blockchain hash functions must satisfy several properties:

- **Deterministic**: Same input always produces same output
- **Avalanche Effect**: Small input changes produce completely different outputs
- **Fixed Output Size**: Always produces 32-byte output
- **One-Way**: Cannot reverse hash to recover original input
- **Collision Resistant**: Extremely difficult to find two inputs with same hash
- **Fast Computation**: Efficient enough for high-throughput systems

SHA-256 satisfies all these requirements, making it ideal for blockchain use.

---

## SHA-256 Digest: General-Purpose Hashing

The `sha256_digest` function provides general-purpose SHA-256 hashing using the `ring` crate. It's used throughout the blockchain for transaction IDs, block hashes, and Merkle tree calculations.

### Implementation

```rust
use ring::digest::{Context, SHA256};

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}
```

**Function Signature:**
- **Input**: `&[u8]` - Reference to byte slice (any length)
- **Output**: `Vec<u8>` - 32-byte SHA-256 hash

**Process:**
1. Create a new SHA-256 context
2. Update context with input data
3. Finalize and extract digest
4. Convert to vector of bytes

### Why `ring` for SHA-256?

The `ring` crate is a comprehensive cryptographic library based on BoringSSL. It provides:

- **Security**: Well-audited, production-tested cryptographic primitives
- **Performance**: Optimized C implementations (BoringSSL)
- **Compatibility**: Used by other parts of our codebase (ECDSA signatures)
- **Reliability**: Battle-tested in production systems

### Usage in Transaction ID Generation

Every transaction gets a unique identifier by hashing its serialized contents. This ID is used throughout the blockchain to reference transactions.

**Why we use a transaction hash (instead of trusting the transaction’s own `id` field):**
- **The `id` field is derived data, not authoritative**: the real “identity” of a transaction is its content (inputs, outputs, amounts, and the data being authorized). If we allowed an arbitrary `id` field to define identity, a sender could change `id` without changing the transaction’s meaning.
- **Avoids a circular definition**: if the transaction ID were “the value stored in `tx.id`,” you immediately face a self-reference problem when defining “the bytes of the transaction,” because the transaction would have to include its own ID in its serialized form. In our code, we solve this by hashing a copy with `id: vec![]`.
- **Content-addressed security model**: nodes do not need to *trust* the sender’s claimed ID. They recompute the hash locally and can reject any transaction where the claimed ID doesn’t match the bytes.
- **Stable references**: later spends refer to earlier transactions by ID. Those references are only meaningful if “ID” is bound to the transaction bytes; otherwise, you could rewrite history by swapping IDs.

> **Implementation note (Bitcoin Core vs this project): txid, wtxid, and “what exactly is hashed?”**
>
> In Bitcoin Core, there are two important identifiers:
> - **txid**: computed from the transaction **without witness data** (historically using double-SHA256 over the legacy serialization).
> - **wtxid**: computed from the transaction **including witness data** (also using double-SHA256), introduced with SegWit.
>
> This project uses a simplified model: we compute an “ID” by hashing a serialized copy with `id: []`. That is great for learning and for internal consistency, but it is not byte-for-byte compatible with Bitcoin Core’s txid/wtxid rules.

**In `Transaction::hash()` (internal helper):**

```rust
// From bitcoin/src/primitives/transaction.rs
fn hash(&mut self) -> Result<Vec<u8>> {
    // IMPORTANT: we must NOT hash the transaction including its own id,
    // otherwise the definition becomes circular (the id would depend on
    // itself).
    let tx_copy = Transaction {
        // Exclude the id from the bytes we hash by setting it to empty.
        id: vec![],
        vin: self.vin.clone(),
        vout: self.vout.clone(),
    };
    // The transaction ID is the SHA-256 hash of the serialized transaction
    // (with id excluded).
    Ok(sha256_digest(tx_copy.serialize()?.as_slice()))
}
```

**Process:**
1. Create a copy of the transaction without the ID field
2. Serialize the transaction to bytes
3. Hash the serialized bytes using SHA-256
4. Return the 32-byte hash as the transaction ID

**Important note about the signature:**
- In our code, `hash` is an internal helper (`fn`, not `pub fn`) and it takes `&mut self` because it is used during transaction construction/signing while updating the in-memory transaction ID.

### Figure: How the transaction ID is computed and then stored

The key idea is: we compute the ID from the transaction’s content, but we do **not** include the ID field itself in the bytes being hashed.

```text
Transaction in memory:
  tx = { id: ?, vin: [...], vout: [...] }

Step 1: build a copy with an empty id
  tx_copy_for_hash = { id: [], vin: tx.vin, vout: tx.vout }

Step 2: serialize + hash
  txid = SHA256( serialize(tx_copy_for_hash) )

Step 3: store the derived value back onto the transaction
  tx.id = txid
```

This pattern is used both when a new transaction is constructed and during signing/verification flows where we need to recompute the ID of a “trimmed copy” deterministically.

**Why Hash the Transaction?**

- **Uniqueness**: Each transaction gets a unique identifier
- **Integrity**: Any change to transaction data changes the hash
- **Efficiency**: 32-byte IDs are compact and efficient for storage
- **Verification**: Easy to verify transaction integrity by re-hashing

**Example Usage:**

```rust
// Conceptual example (pseudocode):
// In this project, `Transaction::new_utxo_transaction(...)` is async and
// computes `tx.id`
// internally after assembling inputs/outputs.
let tx = Transaction::new_utxo_transaction(...).await?;

// Transaction ID is now a 32-byte Vec<u8>
// Can be converted to hex for display: HEXLOWER.encode(&tx.id)
```

**Where this happens in the project:**
- **Transaction construction**: `Transaction::new_utxo_transaction(...)` computes `tx.id = tx.hash()?` after assembling inputs/outputs (internally, by hashing a copy with `id: vec![]`).
- **Signing and verification**: the transaction code computes `tx_copy.id = tx_copy.hash()?` on a trimmed copy while preparing the exact bytes that will be signed/verified.

### Usage in Block Hashing

Blocks are identified by their hash, calculated from header data. This hash serves as the block's immutable fingerprint.

**In `Block::hash_transactions()`:**

```rust
// From bitcoin/src/primitives/block.rs
pub fn hash_transactions(&self) -> Vec<u8> {
    let mut txhashs = vec![];
    for transaction in &self.transactions {
        txhashs.extend(transaction.get_id());
    }
    crate::sha256_digest(txhashs.as_slice())
}
```

**Process:**
1. Collect all transaction IDs from the block
2. Concatenate them into a single byte vector
3. Hash the concatenated transaction IDs
4. Return the hash as the transactions hash

**Why Hash Transaction IDs?**

- **Merkle Tree**: Forms the basis of Merkle tree construction
- **Efficiency**: Single hash represents all transactions
- **Verification**: Easy to verify transaction inclusion
- **Integrity**: Any transaction change affects the hash

### Usage in Merkle Tree Construction

**What a Merkle tree is (succinctly):**
A **Merkle tree** is a binary hash tree: **leaves** are hashes of items (e.g., transaction IDs or transaction hashes), and each **internal node** is the hash of its two child hashes. The **Merkle root** is the final hash at the top and serves as a compact *commitment* to the entire set and ordering of leaves. Given a leaf and its **Merkle proof** (the sibling hashes along the path), any node can verify inclusion by recomputing hashes up to the root in \(O(\log n)\) time.

If you are implementing this yourself, the important thing to remember is: you do *pairwise hashing bottom-up* until you get one hash, and that one hash is what you commit to in the block header.
Merkle trees enable efficient verification of transaction inclusion in blocks. The root hash represents all transactions in the block.

**Merkle Tree Structure:**

```text
        Root Hash
       /         \
   Hash 1-2    Hash 3-4
   /     \     /     \
 Hash 1 Hash 2 Hash 3 Hash 4
  |      |      |      |
 TX1    TX2    TX3    TX4
```

**Benefits:**
- **Efficient Verification**: Verify transaction inclusion with O(log n) hashes
- **Compact Proof**: Merkle proofs are small compared to full transaction list
- **Parallel Processing**: Can compute hashes in parallel
- **Scalability**: Works efficiently even with thousands of transactions

**Implementation Pattern:**

```rust
// Conceptual Merkle root construction (pseudocode):
// 1) start with leaf hashes (transaction IDs or transaction hashes)
// 2) hash pairs bottom-up until one hash remains
fn calculate_merkle_root(leaf_hashes: &[Vec<u8>]) -> Vec<u8> {
    if leaf_hashes.is_empty() {
        return vec![0u8; 32];
    }

    let mut level = leaf_hashes.to_vec();
    while level.len() > 1 {
        let mut next = Vec::new();
        for pair in level.chunks(2) {
            // duplicate last if odd
            let right = if pair.len() == 2 { &pair[1] } else { &pair[0] };
            let combined = [pair[0].as_slice(), right.as_slice()].concat();
            next.push(sha256_digest(&combined));
        }
        level = next;
    }
    level[0].clone()
}
```

> **Implementation note (Bitcoin Core vs this project): Merkle root construction details**
>
> Bitcoin’s Merkle root is built from **double-SHA256** hashes of transaction IDs, and when a level has an odd number of nodes, the last hash is **duplicated** before hashing the pair. The pseudocode above shows the “duplicate last if odd” rule.
>
> In this project, `Block::hash_transactions()` currently computes a simplified “Merkle-ish” root by concatenating all transaction IDs and hashing once. That is useful for learning, but it is not the same as Bitcoin’s Merkle root construction.

---

## Taproot-related hashing: P2TR Address Hashing

### What Taproot is (and why it exists)

Taproot is easiest to understand if we separate three ideas that often get mixed together:

- **Output types** (how an output is locked)
- **Signature algorithms** (how authorization is proven)
- **Protocol upgrades** (new rules that introduce new output types and signature checks)

In practical terms: **for Taproot (P2TR) spends, Bitcoin uses Schnorr instead of the legacy ECDSA (Elliptic Curve Digital Signature Algorithm) signature scheme** (legacy output types still use ECDSA).

#### What an output type is

An **output type** is a standard pattern for the *locking condition* of a transaction output—i.e., the rule that says what data must be provided later to spend the output. In implementation terms, an output contains a **locking script** (`scriptPubKey`). Different standard `scriptPubKey` patterns are what developers call output types.
For example: 
    - **P2PKH** (Pay-to-PubKey-Hash): locks to a public-key hash; typically spent with an ECDSA signature + public key.
    - **P2SH** (Pay-to-Script-Hash): locks to a script hash; spending reveals the redeem script plus required data (often signatures).
    - **P2WPKH** (Pay-to-Witness-PubKey-Hash, SegWit v0): P2PKH-style lock, but signatures move to witness (fixes malleability for these spends).
    - **P2TR** (Pay-to-Taproot, SegWit v1): Taproot output; typically spent with Schnorr (key-path) or via Tapscript (script-path).

#### What Taproot / P2TR is

**Taproot** is a Bitcoin protocol upgrade that introduced a new **output type** (**P2TR**, Pay-to-Taproot) and a new signature scheme (**Schnorr**, BIP 340). Conceptually, Taproot changes how spending conditions are committed to and revealed:
- **Key-path spending**: authorize with a single Schnorr signature, making complex policies look like a simple “single-sig” spend on-chain.
- **Script-path spending (Tapscript)**: if a script is needed, reveal only the branch you used (via a Merkle commitment over script branches—often described as MAST), which improves privacy and can reduce on-chain data.

**Was Taproot in the original Bitcoin whitepaper?** No. The 2008 whitepaper describes ECDSA-style signatures for authorization and does not define Taproot/P2TR/Schnorr/MAST. Taproot was introduced later and is specified primarily by the following **BIP**(*Bitcoin Improvement Proposal*):
- **BIP 340** (Schnorr signatures)
- **BIP 341** (Taproot/P2TR output construction)
- **BIP 342** (Tapscript validation rules)

#### Signature algorithms by output type (what is “ECDSA-based” vs “Schnorr-based”?)

Output types are not “ECDSA” or “Schnorr” by themselves, but standard spend paths tend to require one or the other:

- **ECDSA-based spend types (legacy + SegWit v0)**:
  - **P2PKH**: spend provides an ECDSA signature + public key
  - **P2WPKH (SegWit v0)**: spend provides an ECDSA signature + public key (in witness)
  - **P2SH / P2WSH (SegWit v0)**: the redeem/witness script can require many conditions; common scripts (e.g., multisig) require ECDSA signatures

- **Schnorr-based spend types (Taproot / SegWit v1)**:
  - **P2TR (Taproot)**:
    - **Key-path**: spend provides a Schnorr signature
    - **Script-path (Tapscript)**: signature checks use Schnorr where signatures apply

**Why Taproot was introduced (high level):**
- **Privacy**: complex spending policies can be hidden behind the key-path, and script-path spends reveal less information.
- **Efficiency**: many spends become smaller on-chain (less script data, fixed-size Schnorr signatures).
- **Upgradeability**: Taproot provides a cleaner path to extend scripting rules (Tapscript) while preserving backward compatibility.

**Important terminology note (to avoid confusion):** Taproot is *not* a hash function. The phrase “Taproot-related hashing” in this section refers to **SHA-256 hashing used in Taproot/P2TR-related code paths**.

In our codebase, the `taproot_hash` function is simply a SHA-256 digest implemented via the `sha2` crate. We use it in the wallet/address pipeline (via `hash_pub_key`) to produce the `pub_key_hash` bytes that are later encoded into an address payload.

### Table: Legacy (ECDSA) vs Taproot (P2TR/Schnorr) at a glance

| Dimension | Legacy / SegWit v0 (mostly ECDSA-based) | Taproot / SegWit v1 (P2TR, Schnorr-based) |
|---|---|---|
| **Output types (examples)** | P2PK, P2PKH, P2SH, P2WPKH, P2WSH, nested SegWit (P2SH-P2WPKH/P2WSH) | P2TR (Taproot) |
| **Signature algorithm** | ECDSA over secp256k1 (typical) | Schnorr over secp256k1 (BIP 340) |
| **What “authorization” looks like on-chain** | Script patterns often reveal the spending policy (e.g., multisig script and pubkeys) | Key-path spends often look like a single key + single signature; script-path reveals only the used branch (MAST-style) |
| **Witness version** | Legacy (no witness) or **SegWit v0** witness programs | **SegWit v1** witness programs (Taproot) |
| **Script system** | Legacy Script + SegWit v0 rules (e.g., BIP 141/143) | Taproot + Tapscript rules (BIP 341/342) |
| **Hashing used in the spend rules** | Transaction signatures sign a *sighash* derived from the transaction; txids and many identifiers use hashing | Taproot introduces new hashing constructions (e.g., tagged hashes in BIP 340/341) and commits scripts via Merkle roots |
| **Address encoding (Bitcoin network)** | Base58Check (legacy), bech32 (SegWit v0) | bech32m (Taproot / SegWit v1) |
| **Addresses/encoding in this project** | We implement Base58 encoding utilities and a simplified wallet address payload format | Our wallet currently encodes a Taproot-style pubkey hash into a Base58 payload (educational/simplified, not bech32m) |
| **Why it exists** | Original design + incremental improvements | Privacy, efficiency, and a cleaner upgrade path for scripts |

### Implementation

```rust
use sha2::{Digest as Sha2Digest, Sha256 as Sha2Hash};

pub fn taproot_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha2Hash::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
```

**Function Signature:**
- **Input**: `&[u8]` - Reference to byte slice (typically public key)
- **Output**: `Vec<u8>` - 32-byte SHA-256 hash

**Process:**
1. Create a new SHA-256 hasher
2. Update hasher with input data
3. Finalize and extract hash
4. Convert to vector of bytes

### Why `sha2` for Taproot?

The `sha2` crate is a focused hashing library that:

- **Simplicity**: Lightweight, focused on hashing algorithms
- **Taproot Compatibility**: Matches Bitcoin's Taproot upgrade requirements
- **Modern Standard**: Uses SHA-256 instead of RIPEMD160 for better security
- **Pure Rust**: No C dependencies, easier to audit

### Usage in P2TR Address Generation

Public keys are hashed to create Taproot addresses. This provides privacy (public keys aren't directly exposed) and enables address validation.

**In `hash_pub_key()` (wallet address pipeline):**

```rust
// From bitcoin/src/wallet/wallet_impl.rs
pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    taproot_hash(pub_key)
}
```

**Process:**
1. Take a public key (33 bytes compressed)
2. Hash it using `taproot_hash`
3. Return the 32-byte hash

**Where the result is used next:**
- `Wallet::get_address()` and `convert_address(...)` build `version || pub_key_hash || checksum` and then call `crate::base58_encode(...)` to produce the human-readable address string.

**Why Hash Public Keys?**

- **Privacy**: Public keys aren't directly exposed in addresses
- **Security**: Reduces attack surface (can't derive private key from hash)
- **Efficiency**: 32-byte hashes are compact
- **Bitcoin Compatibility**: Matches Bitcoin's Taproot standard

**Address Generation Flow:**

```rust
// 1. Generate key pair
let private_key = new_schnorr_key_pair()?;
let public_key = get_schnorr_public_key(&private_key)?;

// 2. Hash public key
let pub_key_hash = hash_pub_key(&public_key);

// 3. Create address payload
let version_byte = 0x01; // P2TR version
let mut address_data = vec![version_byte];
address_data.extend_from_slice(&pub_key_hash);

// 4. Calculate checksum
let checksum = sha256_digest(&sha256_digest(&address_data)[..4]);
address_data.extend_from_slice(&checksum[..4]);

// 5. Encode to Base58
let address = base58_encode(&address_data)?;
```

### Usage in Transaction Input Validation

Transaction inputs validate that they can be unlocked by checking if the public key hash matches the output's public key hash.

**In `TXInput::uses_key()`:**

```rust
// From bitcoin/src/primitives/transaction.rs
pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
    let locking_hash = hash_pub_key(self.pub_key.as_slice());
    locking_hash.eq(pub_key_hash)
}
```

**Process:**
1. Hash the input's public key
2. Compare with the provided public key hash
3. Return true if they match

**Why This Validation?**

- **Authorization**: Ensures only the owner can spend outputs
- **Security**: Prevents unauthorized spending
- **Efficiency**: Fast hash comparison

---

## Usage in Proof-of-Work Mining

Proof-of-work mining uses hash functions to find valid block hashes. Miners repeatedly hash block data with different nonces until they find a hash below the target.

### Mining Process

**In `ProofOfWork::run()`:**

```rust
// From bitcoin/src/pow.rs
pub fn run(&self) -> (i64, String) {
    let mut nonce = 0;
    let mut hash = Vec::new();
    
    while nonce < MAX_NONCE {
        let data = self.prepare_data(nonce);
        hash = crate::sha256_digest(data.as_slice());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
        
        if hash_int.lt(self.target.borrow()) {
            break; // Found valid hash!
        } else {
            nonce += 1; // Try next nonce
        }
    }
    
    (nonce, HEXLOWER.encode(hash.as_slice()))
}
```

**Process:**
1. Start with nonce = 0
2. Prepare block data with current nonce
3. Hash the block data
4. Convert hash to integer
5. Check if hash < target
6. If valid, return nonce and hash
7. If not, increment nonce and repeat

**Why Hash in Mining?**

- **Difficulty**: Finding valid hashes requires computational work
- **Security**: Prevents easy block creation
- **Consensus**: All miners compete to find valid hashes
- **Fairness**: Probability of finding valid hash is proportional to hash rate

**Data Preparation:**

```rust
fn prepare_data(&self, nonce: i64) -> Vec<u8> {
    let pre_block_hash = self.block.get_pre_block_hash();
    let transactions_hash = self.block.hash_transactions();
    let timestamp = self.block.get_timestamp();
    
    let mut data_bytes = vec![];
    data_bytes.extend(pre_block_hash.as_bytes());
    data_bytes.extend(transactions_hash);
    data_bytes.extend(timestamp.to_be_bytes());
    data_bytes.extend(TARGET_BITS.to_be_bytes());
    data_bytes.extend(nonce.to_be_bytes());
    data_bytes
}
```

**Components Hashed:**
- Previous block hash
- Transactions hash (Merkle root)
- Timestamp
- Target bits (difficulty)
- Nonce (variable)

---

## Hash Function Properties

Both hash functions exhibit the essential properties of cryptographic hash functions:

### 1. Deterministic

Same input always produces same output:

```rust
let data = b"Hello, Blockchain!";
let hash1 = sha256_digest(data);
let hash2 = sha256_digest(data);
assert_eq!(hash1, hash2); // Always true
```

**Why Important:**
- Transaction IDs are consistent
- Block hashes are reproducible
- Address generation is deterministic

### 2. Avalanche Effect

Small input changes produce completely different outputs:

```rust
let data1 = b"Hello, Blockchain!";
let data2 = b"Hello, Blockchain?"; // One character difference
let hash1 = sha256_digest(data1);
let hash2 = sha256_digest(data2);
// hash1 and hash2 are completely different
```

**Why Important:**
- Prevents similar inputs from having similar hashes
- Ensures transaction modifications are detectable
- Provides security against collision attacks

### 3. Fixed Output Size

Always produces 32-byte (256-bit) output:

```rust
let small_data = b"a";
let large_data = vec![0u8; 10000];
let hash1 = sha256_digest(small_data);
let hash2 = sha256_digest(&large_data);
assert_eq!(hash1.len(), 32); // Always 32 bytes
assert_eq!(hash2.len(), 32); // Always 32 bytes
```

**Why Important:**
- Predictable storage requirements
- Efficient database indexing
- Consistent API responses

### 4. One-Way Function

Cannot reverse hash to recover original input:

```rust
let data = b"Secret message";
let hash = sha256_digest(data);
// Cannot recover "Secret message" from hash
// This is computationally infeasible
```

**Why Important:**
- Public keys can't be derived from address hashes
- Transaction contents can't be recovered from IDs
- Provides privacy and security

### 5. Collision Resistant

Extremely difficult to find two inputs with same hash:

```rust
// Finding two different inputs with same hash
// is computationally infeasible
let data1 = b"Input 1";
let data2 = b"Input 2";
let hash1 = sha256_digest(data1);
let hash2 = sha256_digest(data2);
// Probability of hash1 == hash2 is ~1/2^256
```

**Why Important:**
- Prevents transaction ID collisions
- Ensures block hash uniqueness
- Provides security guarantees

---

## Why Two Hash Functions?

The codebase uses two different libraries for SHA-256 hashing:

### Historical Reasons

1. **Started with `ring`**: Codebase began with `ring` for general hashing
2. **Added `sha2` for Taproot**: When Taproot support was added, `sha2` was chosen
3. **Different Requirements**: Taproot has specific requirements

### Technical Differences

| Aspect | `sha256_digest` (ring) | `taproot_hash` (sha2) |
|--------|------------------------|------------------------|
| **Library** | `ring` (BoringSSL) | `sha2` (pure Rust) |
| **Implementation** | C code (BoringSSL) | Pure Rust |
| **Use Case** | General hashing | Taproot addresses |
| **Dependencies** | BoringSSL | None |
| **Performance** | Optimized C | Good Rust performance |

### Output Compatibility

Both functions produce **identical SHA-256 output**:

```rust
let data = b"Test data";
let hash1 = sha256_digest(data);
let hash2 = taproot_hash(data);
assert_eq!(hash1, hash2); // Always true
```

### Recommendation for Future Refactoring

In a production system, we typically want a single SHA-256 implementation for:
- **Consistency**: Single implementation to maintain
- **Reduced Dependencies**: Fewer dependencies to manage
- **Improved Maintainability**: Less code to maintain

However, both implementations are correct and produce identical results, so the current approach works well.

---

## Summary

Hash functions are fundamental to blockchain operations:

1. **Transaction IDs**: Unique identifiers for transactions
2. **Block Hashes**: Immutable fingerprints of blocks
3. **Merkle Trees**: Efficient verification of transaction inclusion
4. **Address Generation**: Public key hashing for addresses
5. **Proof-of-Work**: Mining requires repeated hashing

**Key Takeaways:**

- SHA-256 provides all necessary security properties
- Both `ring` and `sha2` produce identical results
- Hash functions enable efficient blockchain operations
- Merkle trees provide scalable verification

**Next Steps:**

- Continue to Digital Signatures to learn about transaction signing

---

## References

In this section, we provide references that describe the standards and Bitcoin conventions behind the code:

- **[FIPS 180-4: Secure Hash Standard (SHA-256)](https://csrc.nist.gov/publications/detail/fips/180/4/final)**
- **[Bitcoin: A Peer-to-Peer Electronic Cash System (whitepaper)](https://bitcoin.org/bitcoin.pdf)** (sections on PoW and block chaining)
- **[Bitcoin Wiki: Proof of Work](https://en.bitcoin.it/wiki/Proof_of_work)**
- Explore Key Pair Generation to see how keys are generated
- Review Address Encoding to understand address generation
- Check Security and Performance for performance considerations

---

## Navigation

- **← Previous: Cryptography Index** - Cryptographic primitives overview
- **Next section: Digital Signatures →** - Transaction signing and verification
- **Cryptography Index** - Complete guide overview
- **Key Pair Generation** - Key generation and derivation
- **Address Encoding** - Base58 encoding
- **Security and Performance** - Performance considerations

**Related Guides:**
- **Rust Language Guide** - Rust language features
- **Web API Architecture** - Cryptographic operations in APIs

---

<div align="center">

**[← Cryptography Index](README.md)** | **[Hash Functions](01-Hash-Functions.md)** | **[Next section: Digital Signatures →](02-Digital-Signatures.md)** 

</div>

---

*In the next part of this section, we move from tamper-evident identifiers (hashes) to authorization (signatures). Continue to Digital Signatures to learn about transaction signing and verification.*
