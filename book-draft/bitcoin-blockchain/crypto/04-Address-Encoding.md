<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="../web/README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
# Address Encoding: Base58 for Human-Readable Addresses

Address encoding is where raw key material becomes something a human can copy, paste, and exchange safely. In production Bitcoin, address formats vary (Base58Check for legacy scripts, Bech32/Bech32m for SegWit and Taproot). This project uses a **Base58Check-style** format for its wallet addresses: `version byte + public key hash + checksum`, then Base58-encode the bytes. The goal is to show the exact byte-level pipeline you would implement in a blockchain codebase.

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

An address is a compact, checksummed identifier derived from a public key. It is *not* the public key itself. In this project, we hash the public key, attach a version byte and checksum, then encode the result into a Base58 string.

### Address Properties

- **Human-Readable**: Easy to read and type
- **Compact**: More compact than hexadecimal
- **Error-Resistant**: Avoids ambiguous characters
- **Checksummed**: Includes checksum for error detection

### Address Lifecycle

```text
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

### Figure: Address payload layout (version + hash + checksum)

In this project, we build an address by constructing a payload and then Base58-encoding it:

```text
payload = version (1) || pub_key_hash (32) || checksum (4)

┌────────┬──────────────────┬───────────────────────┐
│ ver(1) │  hash (32)       │   checksum (4)        │
├────────┼──────────────────┼───────────────────────┤
│ 0x01   │ SHA256(pubkey)   │ first4(SHA256²(v||h)) │
└────────┴──────────────────┴───────────────────────┘
```

This layout maps directly to `bitcoin/src/wallet/wallet_impl.rs`, where:
- `VERSION` is `0x01`
- `hash_pub_key(pubkey)` produces `pub_key_hash`
- `checksum(payload)` produces 4 bytes
- `base58_encode(payload)` produces the address string

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

```text
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

In this project, addresses consist of three main components:

1. **Version Byte**: Network identifier (mainnet/testnet)
2. **Hash**: Public key hash (20 bytes for P2PKH, 32 bytes for P2TR)
3. **Checksum**: 4-byte checksum for error detection

### Address Format

```text
[Version Byte (1 byte)] [Hash (20-32 bytes)] [Checksum (4 bytes)]
```

### Version Bytes

The implementation uses a single version byte to tag the address format:

- **`0x01`**: Project-specific **P2TR-style address tag** (Base58Check-style)

For context, legacy Bitcoin Base58Check uses `0x00` (P2PKH mainnet) and `0x6f` (P2PKH testnet). Taproot addresses in production Bitcoin are Bech32m rather than Base58Check.

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

P2TR (Pay-to-Taproot) addresses are generated from Schnorr public keys in this project, then encoded using the Base58Check-style format described above.

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

// 4. Calculate checksum (double SHA-256, first 4 bytes)
let checksum = checksum(&address_data);
address_data.extend_from_slice(&checksum);

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

```text
Version: 0x01
Hash: [32 bytes of public key hash]
Checksum: [4 bytes]
Encoded: "<Base58 string>"
```

---

## Address Validation

Address validation ensures addresses are correctly formatted and have valid checksums.

### Validation Process

```rust
// In validate_address()
pub fn validate_address(address: &str) -> Result<bool> {
    // 1. Decode Base58
    let payload = base58_decode(address)?;

    // 2. Extract components
    let actual_checksum = payload[payload.len() - 4..].to_vec();
    let version = payload[0];
    let pub_key_hash = payload[1..payload.len() - 4].to_vec();

    // 3. Recompute checksum from version + hash
    let mut target_vec = vec![version];
    target_vec.extend(pub_key_hash);
    let target_checksum = checksum(target_vec.as_slice());

    // 4. Compare checksums
    Ok(actual_checksum.eq(target_checksum.as_slice()))
}
```

### Validation Steps

1. **Decode Base58**: Convert address string to bytes
2. **Extract Components**: Split version, hash, and checksum
3. **Recompute Checksum**: Build `version || hash`, then double-SHA-256
4. **Compare Checksums**: Return true if the checksum matches

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

- Continue to Security and Performance to learn about security best practices
- Review Key Pair Generation to see how keys are generated
- Explore Hash Functions for address hashing details
- Check Digital Signatures to understand signature operations

---

## Navigation

- **← Previous: Key Pair Generation** - Key generation and derivation
- **Next section: Security and Performance →** - Security best practices
- **Cryptography Index** - Complete guide overview
- **Hash Functions** - Address hashing details
- **Digital Signatures** - Signature operations
- **Key Pair Generation** - Key generation details

**Related Guides:**
- **Rust Language Guide** - Rust language features
- **Web API Architecture** - Cryptographic operations in APIs

---

<div align="center">

**[← Previous: Key Pair Generation](03-Key-Pair-Generation.md)** | **[Address Encoding](04-Address-Encoding.md)** | **[Next section: Security and Performance →](05-Security-and-Performance.md)**

</div>

---

*In the next part of this section, we zoom out from formats to operational reality: safe key handling, side channels, and performance. Continue to Security and Performance to learn best practices for cryptographic code in production.*

---

## References

In this section, we provide references for the canonical address/encoding conventions used in Bitcoin:

- **[Bitcoin Wiki: Base58Check encoding](https://en.bitcoin.it/wiki/Base58Check_encoding)** (version byte + checksum convention)
- **[Bitcoin Wiki: Address](https://en.bitcoin.it/wiki/Address)** (address types and high-level structure)
