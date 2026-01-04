<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../util/README.md) - Utility functions and helpers
8. **Chapter 2.3: Cryptography** ← *You are here*
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../../ci/kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
# Address Encoding: Base58 for Human-Readable Addresses

Bitcoin addresses are encoded using Base58, a human-readable encoding scheme that avoids ambiguous characters. In this section, we explore how Base58 encoding converts cryptographic hashes into human-readable addresses, how addresses are structured, and how address validation works.

## Table of Contents

1. [Overview: Address Encoding in Blockchain](#overview-address-encoding-in-blockchain)
2. [Base58 Encoding Algorithm](#base58-encoding-algorithm)
3. [Base58 Decoding Algorithm](#base58-decoding-algorithm)
4. [Address Structure](#address-structure)
5. [P2TR Address Generation](#p2tr-address-generation)
6. [Address Validation](#address-validation)
7. [Error Handling](#error-handling)

---

## Overview: Address Encoding in Blockchain

Addresses serve as identifiers for wallet recipients in the blockchain. They are derived from public keys but don't expose the public key directly, providing privacy and security.

### Address Properties

- **Human-Readable**: Easy to read and type
- **Compact**: More compact than hexadecimal
- **Error-Resistant**: Avoids ambiguous characters
- **Checksummed**: Includes checksum for error detection

### Address Lifecycle

```
Public Key (33 bytes)
    ↓
Hash Public Key (32 bytes)
    ↓
Add Version Byte (1 byte)
    ↓
Calculate Checksum (4 bytes)
    ↓
Base58 Encode
    ↓
Address (Base58 string)
```

---

## Base58 Encoding Algorithm

Base58 encoding converts binary data into a human-readable string, excluding characters that could be confused (0, O, I, l).

### Implementation

```rust
pub fn base58_encode(data: &[u8]) -> Result<String> {
    Ok(bs58::encode(data).into_string())
}
```

**Function Signature:**
- **Input**: `&[u8]` - Reference to byte slice
- **Output**: `Result<String>` - Base58-encoded string

**Process:**
1. Take input byte slice
2. Convert to Base58 using `bs58` crate
3. Return encoded string

### Why Base58?

Base58 provides:

- **Human Readability**: Easy to read and type
- **Error Prevention**: Avoids ambiguous characters (0/O, I/l)
- **Compactness**: More compact than hexadecimal
- **Bitcoin Standard**: Used by Bitcoin for addresses

### Base58 Character Set

```
123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
```

**Excluded Characters:**
- `0` (zero) - Can be confused with `O` (capital O)
- `O` (capital O) - Can be confused with `0` (zero)
- `I` (capital I) - Can be confused with `l` (lowercase L)
- `l` (lowercase L) - Can be confused with `I` (capital I)

### Example Encoding

```rust
let data = b"Hello, Blockchain!";
let encoded = base58_encode(data)?;
// Result: "72k1xXWGvuF1TBpZ"
```

---

## Base58 Decoding Algorithm

Base58 decoding converts human-readable addresses back to binary data.

### Implementation

```rust
pub fn base58_decode(data: &str) -> Result<Vec<u8>> {
    bs58::decode(data)
        .into_vec()
        .map_err(|e| BtcError::AddressDecodingError(e.to_string()))
}
```

**Function Signature:**
- **Input**: `&str` - Base58-encoded string
- **Output**: `Result<Vec<u8>>` - Decoded byte vector

**Process:**
1. Take input string
2. Decode using `bs58` crate
3. Return decoded bytes or error

### Error Handling

Base58 decoding can fail if:
- Invalid characters are present
- String format is incorrect
- Decoding fails

All errors are properly handled and returned as `BtcError::AddressDecodingError`.

### Example Decoding

```rust
let encoded = "72k1xXWGvuF1TBpZ";
let decoded = base58_decode(encoded)?;
// Result: b"Hello, Blockchain!"
```

---

## Address Structure

Bitcoin addresses consist of three main components:

1. **Version Byte**: Network identifier (mainnet/testnet)
2. **Hash**: Public key hash (20 bytes for P2PKH, 32 bytes for P2TR)
3. **Checksum**: 4-byte checksum for error detection

### Address Format

```
[Version Byte (1 byte)] [Hash (20-32 bytes)] [Checksum (4 bytes)]
```

### Version Bytes

- **0x00**: P2PKH (Pay-to-Public-Key-Hash) mainnet
- **0x01**: P2TR (Pay-to-Taproot) mainnet
- **0x6f**: P2PKH testnet
- **0x01**: P2TR testnet

### Checksum Calculation

The checksum is calculated using double SHA-256:

```rust
// Calculate checksum
let payload = [version_byte, ...hash];
let hash = sha256_digest(&sha256_digest(&payload));
let checksum = &hash[..4]; // First 4 bytes
```

**Process:**
1. Hash the payload (version + hash) using SHA-256
2. Hash the result again using SHA-256
3. Take first 4 bytes as checksum

**Why Double SHA-256?**

- **Security**: Provides additional security
- **Error Detection**: Detects transmission errors
- **Bitcoin Standard**: Matches Bitcoin's checksum algorithm

---

## P2TR Address Generation

P2TR (Pay-to-Taproot) addresses are generated for modern Bitcoin addresses.

### Address Generation Flow

```rust
// 1. Generate key pair
let private_key = new_schnorr_key_pair()?;
let public_key = get_schnorr_public_key(&private_key)?;

// 2. Hash public key
let pub_key_hash = taproot_hash(&public_key);

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

### Step-by-Step Process

1. **Hash Public Key**: Use `taproot_hash` to hash public key
2. **Add Version Byte**: Prepend version byte (0x01 for P2TR)
3. **Calculate Checksum**: Double SHA-256 hash, take first 4 bytes
4. **Append Checksum**: Add checksum to address data
5. **Base58 Encode**: Convert to Base58 string

### Example Address

```
Version: 0x01
Hash: [32 bytes of public key hash]
Checksum: [4 bytes]
Encoded: "bc1p..." (Base58)
```

---

## Address Validation

Address validation ensures addresses are correctly formatted and have valid checksums.

### Validation Process

```rust
// In validate_address()
pub fn validate_address(address: &str) -> Result<()> {
    // 1. Decode Base58
    let decoded = base58_decode(address)?;

    // 2. Verify structure
    if decoded.len() < 5 {
        return Err(BtcError::InvalidAddress);
    }

    // 3. Extract components
    let version = decoded[0];
    let hash = &decoded[1..decoded.len()-4];
    let checksum = &decoded[decoded.len()-4..];

    // 4. Verify checksum
    let payload = &decoded[..decoded.len()-4];
    let expected_checksum = sha256_digest(&sha256_digest(payload)[..4]);
    if checksum != &expected_checksum[..4] {
        return Err(BtcError::InvalidAddress);
    }

    // 5. Verify version
    if version != 0x01 {
        return Err(BtcError::InvalidAddress);
    }

    Ok(())
}
```

### Validation Steps

1. **Decode Base58**: Convert address string to bytes
2. **Check Length**: Ensure minimum length (5 bytes: version + hash + checksum)
3. **Extract Components**: Separate version, hash, and checksum
4. **Verify Checksum**: Recalculate and compare checksum
5. **Verify Version**: Ensure version byte is valid

### Why Validate?

- **Error Detection**: Catches transmission errors
- **Security**: Prevents invalid addresses
- **User Experience**: Provides clear error messages
- **Integrity**: Ensures address integrity

---

## Error Handling

Address operations can fail for various reasons, and proper error handling is essential.

### Decoding Errors

```rust
pub fn base58_decode(data: &str) -> Result<Vec<u8>> {
    bs58::decode(data)
        .into_vec()
        .map_err(|e| BtcError::AddressDecodingError(e.to_string()))
}
```

**Error Cases:**
- Invalid Base58 characters
- Malformed address string
- Decoding failures

### Validation Errors

```rust
pub fn validate_address(address: &str) -> Result<()> {
    // Various validation checks...
    // Returns BtcError::InvalidAddress on failure
}
```

**Error Cases:**
- Invalid length
- Invalid checksum
- Invalid version byte
- Invalid hash length

### Error Types

- **AddressDecodingError**: Base58 decoding failed
- **InvalidAddress**: Address validation failed
- **InvalidVersion**: Invalid version byte
- **InvalidChecksum**: Checksum mismatch

---

## Summary

Address encoding is essential for blockchain usability:

1. **Base58 Encoding**: Human-readable address format
2. **Address Structure**: Version byte, hash, and checksum
3. **P2TR Addresses**: Modern Bitcoin address format
4. **Address Validation**: Ensures address integrity
5. **Error Handling**: Proper error handling for invalid addresses

**Key Takeaways:**

- Base58 provides human-readable addresses
- Addresses include checksums for error detection
- P2TR addresses use 32-byte public key hashes
- Validation ensures address integrity

**Next Steps:**

- Continue to [Security and Performance](05-Security-and-Performance.md) to learn about security best practices
- Review [Key Pair Generation](03-Key-Pair-Generation.md) to see how keys are generated
- Explore [Hash Functions](01-Hash-Functions.md) for address hashing details
- Check [Digital Signatures](02-Digital-Signatures.md) to understand signature operations

---

## Navigation

- **[← Previous: Key Pair Generation](03-Key-Pair-Generation.md)** - Key generation and derivation
- **[Next: Security and Performance →](05-Security-and-Performance.md)** - Security best practices
- **[Cryptography Index](README.md)** - Complete guide overview
- **[Hash Functions](01-Hash-Functions.md)** - Address hashing details
- **[Digital Signatures](02-Digital-Signatures.md)** - Signature operations
- **[Key Pair Generation](03-Key-Pair-Generation.md)** - Key generation details

**Related Guides:**
- **[Rust Language Guide](../../rust/README.md)** - Rust language features
- **[Web API Architecture](../web/README.md)** - Cryptographic operations in APIs

---

<div align="center">

**📚 [← Previous: Key Pair Generation](03-Key-Pair-Generation.md)** | **Address Encoding** | **[Next: Security and Performance →](05-Security-and-Performance.md)** 📚

</div>

---

*This section covers address encoding used in our blockchain implementation. Continue to [Security and Performance](05-Security-and-Performance.md) to learn about security best practices.*
