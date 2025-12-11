# Hash Functions: SHA-256 in Blockchain

Hash functions are fundamental to blockchain operations. They create fixed-size fingerprints of arbitrary data, enabling efficient identification and verification. In this section, we explore how SHA-256 is used throughout our blockchain implementation for transaction IDs, block hashes, Merkle trees, and address generation.

## Table of Contents

1. [Overview: Hash Functions in Blockchain](#overview-hash-functions-in-blockchain)
2. [SHA-256 Digest: General-Purpose Hashing](#sha-256-digest-general-purpose-hashing)
3. [Taproot Hash: P2TR Address Hashing](#taproot-hash-p2tr-address-hashing)
4. [Usage in Transaction System](#usage-in-transaction-system)
5. [Usage in Block System](#usage-in-block-system)
6. [Usage in Proof-of-Work Mining](#usage-in-proof-of-work-mining)
7. [Hash Function Properties](#hash-function-properties)
8. [Why Two Hash Functions?](#why-two-hash-functions)

---

## Overview: Hash Functions in Blockchain

Hash functions serve multiple critical roles in blockchain systems:

1. **Transaction Identification**: Every transaction gets a unique ID by hashing its contents
2. **Block Identification**: Blocks are identified by their hash, calculated from header data
3. **Merkle Tree Construction**: Transaction hashes are combined into a Merkle tree root
4. **Address Generation**: Public keys are hashed to create addresses
5. **Proof-of-Work**: Miners hash block data repeatedly to find valid nonces

Our implementation uses SHA-256 (Secure Hash Algorithm 256-bit) for all these purposes. SHA-256 produces a 32-byte (256-bit) hash output, providing sufficient security for blockchain operations.

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

**In `Transaction::hash()`:**

```rust
// From bitcoin/src/primitives/transaction.rs
pub fn hash(&self) -> Result<Vec<u8>> {
    let tx_copy = Transaction {
        id: vec![],
        vin: self.vin.clone(),
        vout: self.vout.clone(),
    };
    Ok(sha256_digest(tx_copy.serialize()?.as_slice()))
}
```

**Process:**
1. Create a copy of the transaction without the ID field
2. Serialize the transaction to bytes
3. Hash the serialized bytes using SHA-256
4. Return the 32-byte hash as the transaction ID

**Why Hash the Transaction?**

- **Uniqueness**: Each transaction gets a unique identifier
- **Integrity**: Any change to transaction data changes the hash
- **Efficiency**: 32-byte IDs are compact and efficient for storage
- **Verification**: Easy to verify transaction integrity by re-hashing

**Example Usage:**

```rust
// Create a transaction
let mut tx = Transaction::new_utxo_transaction(...)?;

// Generate transaction ID
tx.id = tx.hash()?;

// Transaction ID is now a 32-byte Vec<u8>
// Can be converted to hex for display: HEXLOWER.encode(&tx.id)
```

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

Merkle trees enable efficient verification of transaction inclusion in blocks. The root hash represents all transactions in the block.

**Merkle Tree Structure:**

```
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
// Simplified Merkle tree construction
fn calculate_merkle_root(transactions: &[Transaction]) -> Vec<u8> {
    if transactions.is_empty() {
        return vec![0u8; 32]; // Empty tree hash
    }
    
    if transactions.len() == 1 {
        return transactions[0].hash()?;
    }
    
    // Hash pairs of transactions
    let mut level = transactions.iter()
        .map(|tx| tx.hash()?)
        .collect::<Vec<_>>();
    
    // Build tree bottom-up
    while level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in level.chunks(2) {
            if chunk.len() == 2 {
                let combined = [chunk[0].as_slice(), chunk[1].as_slice()].concat();
                next_level.push(sha256_digest(&combined));
            } else {
                next_level.push(chunk[0].clone());
            }
        }
        level = next_level;
    }
    
    level[0].clone()
}
```

---

## Taproot Hash: P2TR Address Hashing

The `taproot_hash` function provides SHA-256 hashing specifically for Taproot (P2TR) addresses using the `sha2` crate. It's used for public key hashing in modern Bitcoin addresses.

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

**In `hash_pub_key()`:**

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

Ideally, the codebase should use a single SHA-256 implementation for:
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

- Continue to [Digital Signatures](02-Digital-Signatures.md) to learn about transaction signing
- Explore [Key Pair Generation](03-Key-Pair-Generation.md) to see how keys are generated
- Review [Address Encoding](04-Address-Encoding.md) to understand address generation
- Check [Security and Performance](05-Security-and-Performance.md) for performance considerations

---

## Navigation

- **[← Previous: Cryptography Index](README.md)** - Cryptographic primitives overview
- **[Next: Digital Signatures →](02-Digital-Signatures.md)** - Transaction signing and verification
- **[Cryptography Index](README.md)** - Complete guide overview
- **[Key Pair Generation](03-Key-Pair-Generation.md)** - Key generation and derivation
- **[Address Encoding](04-Address-Encoding.md)** - Base58 encoding
- **[Security and Performance](05-Security-and-Performance.md)** - Performance considerations

**Related Guides:**
- **[Rust Language Guide](../../rust/README.md)** - Rust language features
- **[Web API Architecture](../web/README.md)** - Cryptographic operations in APIs

---

<div align="center">

**📚 [← Cryptography Index](README.md)** | **Hash Functions** | **[Next: Digital Signatures →](02-Digital-Signatures.md)** 📚

</div>

---

*This section covers hash functions used in our blockchain implementation. Continue to [Digital Signatures](02-Digital-Signatures.md) to learn about transaction signing and verification.*
