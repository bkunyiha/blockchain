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
# Digital Signatures: Transaction Authorization

Digital signatures prove ownership and authorize transactions in the blockchain. Every transaction input must be signed with the private key corresponding to the public key that locks the output being spent. In this section, we explore how Schnorr and ECDSA signatures are used to secure transactions in our blockchain implementation.

## Table of Contents

1. [Overview: Digital Signatures in Blockchain](#overview-digital-signatures-in-blockchain)
2. [Schnorr Signatures: Modern Bitcoin](#schnorr-signatures-modern-bitcoin)
3. [ECDSA Signatures: Legacy Support](#ecdsa-signatures-legacy-support)
4. [Transaction Signing Process](#transaction-signing-process)
5. [Transaction Verification Process](#transaction-verification-process)
6. [Signature Schemes Comparison](#signature-schemes-comparison)
7. [Usage in Transaction Inputs](#usage-in-transaction-inputs)
8. [Security Considerations](#security-considerations)

---

## Overview: Digital Signatures in Blockchain

Digital signatures serve three critical functions in blockchain systems:

1. **Authentication**: Prove that the signer owns the private key
2. **Authorization**: Authorize spending of transaction outputs
3. **Integrity**: Ensure transaction data hasn't been tampered with

### How Digital Signatures Work

1. **Signing**: Transaction data is hashed, then signed with private key
2. **Verification**: Signature is verified against public key and transaction hash
3. **Validation**: Only valid signatures allow spending of outputs

### Signature Flow in Transactions

```
Transaction Creation
    ↓
Hash Transaction Data
    ↓
Sign with Private Key → Signature
    ↓
Attach Signature to Input
    ↓
Broadcast Transaction
    ↓
Verification: Check Signature against Public Key
    ↓
If Valid: Transaction Accepted
If Invalid: Transaction Rejected
```

---

## Schnorr Signatures: Modern Bitcoin

Schnorr signatures are the modern signature scheme introduced with Bitcoin's Taproot upgrade. They provide better security properties, smaller signatures, and support for signature aggregation.

### Implementation

**Signing:**

```rust
use secp256k1::{Keypair, Message, Secp256k1, SecretKey};

pub fn schnorr_sign_digest(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let secp = Secp256k1::new();
    
    // Parse private key (32 bytes)
    let secret_key_array: [u8; 32] = private_key.try_into()
        .map_err(|_| BtcError::TransactionSignatureError(
            "Invalid private key length".to_string()
        ))?;
    let secret_key = SecretKey::from_byte_array(secret_key_array)?;
    
    // Hash the message
    let message_hash = sha256_digest(message);
    let message_hash_array: [u8; 32] = message_hash.try_into()
        .map_err(|_| BtcError::TransactionSignatureError(
            "Invalid message hash length".to_string()
        ))?;
    
    // Create keypair and sign
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let mut rng = rng();
    let signature = secp.sign_schnorr_with_rng(
        &message_hash,
        &keypair,
        &mut rng
    );
    
    Ok(signature.as_ref().to_vec())
}
```

**Key Characteristics:**
- **Key Format**: Raw 32-byte private keys (Bitcoin-native)
- **Curve**: secp256k1 (Bitcoin's standard curve)
- **Library**: `secp256k1` crate (Bitcoin-optimized)
- **Signature Size**: Fixed 64 bytes (more efficient than ECDSA)
- **Message**: Transaction hash (32 bytes)

**Verification:**

```rust
use secp256k1::{PublicKey, XOnlyPublicKey, schnorr};

pub fn schnorr_sign_verify(
    public_key: &[u8],
    signature: &[u8],
    message: &[u8]
) -> bool {
    let secp = Secp256k1::new();
    
    // Parse public key (33 bytes compressed)
    let public_key_array: [u8; 33] = match public_key.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let public_key_obj = match PublicKey::from_byte_array_compressed(public_key_array) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    
    // Convert to XOnlyPublicKey for Schnorr
    let xonly_array: [u8; 32] = match public_key_obj.serialize()[1..33].try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let xonly_public_key = match XOnlyPublicKey::from_byte_array(xonly_array) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    
    // Hash the message
    let message_hash = sha256_digest(message);
    let message_hash_array: [u8; 32] = match message_hash.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    
    // Parse signature (64 bytes)
    let signature_array: [u8; 64] = match signature.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let signature_obj = schnorr::Signature::from_byte_array(signature_array);
    
    // Verify
    secp.verify_schnorr(&signature_obj, &message_hash, &xonly_public_key).is_ok()
}
```

**Verification Process:**
1. Parse the compressed public key (33 bytes)
2. Convert to XOnlyPublicKey (32 bytes) for Schnorr verification
3. Hash the message using SHA-256
4. Parse the signature (64 bytes)
5. Verify the signature against the message hash

### Why Schnorr?

Schnorr signatures provide:

- **Better Security**: Improved security properties and linearity
- **Smaller Signatures**: Fixed 64 bytes vs. variable 70-72 bytes for ECDSA
- **Signature Aggregation**: Can combine multiple signatures efficiently
- **Bitcoin Compatibility**: Native support for Taproot (P2TR addresses)
- **Batch Verification**: Can verify multiple signatures faster

---

## ECDSA Signatures: Legacy Support

ECDSA (Elliptic Curve Digital Signature Algorithm) is the traditional signature scheme used in older Bitcoin implementations. Our codebase provides ECDSA functions for legacy compatibility and alternative signature schemes.

### Implementation

**Signing:**

```rust
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair};

pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let rng = ring::rand::SystemRandom::new();
    let key_pair = EcdsaKeyPair::from_pkcs8(
        &ECDSA_P256_SHA256_FIXED_SIGNING,
        pkcs8,
        &rng
    )?;
    let signature = key_pair.sign(&rng, message)?;
    Ok(signature.as_ref().to_vec())
}
```

**Key Characteristics:**
- **Key Format**: PKCS#8 format (variable length, standardized)
- **Curve**: P-256 (secp256r1) - different from Bitcoin's secp256k1
- **Library**: `ring` crate (BoringSSL-based)
- **Signature Size**: Variable (typically 70-72 bytes)

**Verification:**

```rust
pub fn ecdsa_p256_sha256_sign_verify(
    public_key: &[u8],
    signature: &[u8],
    message: &[u8]
) -> bool {
    let peer_public_key = ring::signature::UnparsedPublicKey::new(
        &ECDSA_P256_SHA256_FIXED,
        public_key
    );
    peer_public_key.verify(message, signature.as_ref()).is_ok()
}
```

### Current Usage

ECDSA functions are available but not currently used in the primary transaction flow. The codebase primarily uses Schnorr signatures for modern Bitcoin operations.

---

## Transaction Signing Process

Transaction signing is a multi-step process that ensures only the owner of the private key can authorize spending.

### Step-by-Step Process

**In `Transaction::sign()`:**

```rust
// From bitcoin/src/primitives/transaction.rs
async fn sign(&mut self, blockchain: &BlockchainService, private_key: &[u8]) -> Result<()> {
    let mut tx_copy = self.trimmed_copy();

    for (idx, vin) in self.vin.iter_mut().enumerate() {
        // 1. Find the previous transaction
        let prev_tx_option = blockchain.find_transaction(vin.get_txid()).await?;
        let prev_tx = match prev_tx_option {
            Some(tx) => tx,
            None => {
                return Err(BtcError::TransactionNotFoundError(
                    "(sign) Previous transaction is not correct".to_string(),
                ));
            }
        };

        // 2. Prepare transaction copy for signing
        tx_copy.vin[idx].signature = vec![];
        tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
        tx_copy.id = tx_copy.hash()?;
        tx_copy.vin[idx].pub_key = vec![];

        // 3. Sign the transaction hash
        let signature = schnorr_sign_digest(private_key, tx_copy.get_id())?;
        vin.signature = signature;
    }
    Ok(())
}
```

### Detailed Process

1. **Create Trimmed Copy**: Remove signatures and public keys from inputs
2. **For Each Input**:
   - Find the previous transaction that created the output
   - Set the public key hash in the trimmed copy
   - Calculate the transaction hash
   - Clear the public key from the trimmed copy
   - Sign the transaction hash with the private key
   - Attach the signature to the input

### Why This Process?

- **Security**: Only the owner can sign (private key required)
- **Integrity**: Transaction hash ensures data hasn't changed
- **Authorization**: Signature proves authorization to spend
- **Non-repudiation**: Cannot deny signing the transaction

### Example Usage

```rust
// Create a transaction
let mut tx = Transaction::new_utxo_transaction(
    &blockchain,
    &wallet_address,
    &recipient_address,
    10,
)?;

// Sign the transaction
tx.sign(&blockchain, &private_key).await?;

// Transaction is now signed and ready to broadcast
```

---

## Transaction Verification Process

Transaction verification ensures that signatures are valid and that the signer authorized the transaction.

### Step-by-Step Process

**In `Transaction::verify()`:**

```rust
// From bitcoin/src/primitives/transaction.rs
pub async fn verify(&self, blockchain: &BlockchainService) -> Result<bool> {
    // Coinbase transactions don't need verification
    if self.is_coinbase() {
        return Ok(true);
    }
    
    let mut trimmed_self_copy = self.trimmed_copy();
    
    for (idx, vin) in self.vin.iter().enumerate() {
        // 1. Find the previous transaction
        let current_vin_tx_option = blockchain.find_transaction(vin.get_txid()).await?;
        let current_vin_tx = match current_vin_tx_option {
            Some(tx) => tx,
            None => {
                return Err(BtcError::TransactionNotFoundError(
                    "(verify) Previous transaction is not correct".to_string(),
                ));
            }
        };

        // 2. Prepare transaction copy for verification
        trimmed_self_copy.vin[idx].signature = vec![];
        trimmed_self_copy.vin[idx].pub_key = current_vin_tx.vout[vin.vout].pub_key_hash.clone();
        trimmed_self_copy.id = trimmed_self_copy.hash()?;
        trimmed_self_copy.vin[idx].pub_key = vec![];

        // 3. Verify the signature
        if !schnorr_sign_verify(
            vin.get_pub_key(),
            vin.get_signature(),
            trimmed_self_copy.get_id()
        ) {
            return Ok(false); // Invalid signature
        }
    }
    
    Ok(true) // All signatures valid
}
```

### Detailed Process

1. **Check Coinbase**: Coinbase transactions don't need verification
2. **Create Trimmed Copy**: Remove signatures and public keys
3. **For Each Input**:
   - Find the previous transaction
   - Set the public key hash in the trimmed copy
   - Calculate the transaction hash
   - Clear the public key from the trimmed copy
   - Verify the signature against the public key and hash
   - If invalid, return false
4. **All Valid**: Return true if all signatures are valid

### Why This Process?

- **Security**: Ensures only valid signatures are accepted
- **Authorization**: Verifies the signer authorized the transaction
- **Integrity**: Confirms transaction data hasn't been tampered with
- **Prevention**: Prevents unauthorized spending

### Example Usage

```rust
// Verify a transaction
let is_valid = tx.verify(&blockchain).await?;

if is_valid {
    // Transaction is valid, can be added to mempool
    blockchain.add_to_mempool(tx)?;
} else {
    // Transaction is invalid, reject it
    return Err(BtcError::InvalidTransaction);
}
```

---

## Signature Schemes Comparison

| Aspect | ECDSA | Schnorr |
|--------|-------|---------|
| **Signature Size** | Variable (70-72 bytes) | Fixed (64 bytes) |
| **Security Properties** | Well-established | Better (linearity, aggregation) |
| **Bitcoin Compatibility** | Traditional | Modern (Taproot) |
| **Library** | `ring` (BoringSSL) | `secp256k1` (Bitcoin-optimized) |
| **Key Format** | PKCS#8 (variable) | Raw 32-byte (Bitcoin-native) |
| **Curve** | P-256 (secp256r1) | secp256k1 (Bitcoin standard) |
| **Current Usage** | Legacy/Alternative | Primary (P2TR transactions) |
| **Signature Aggregation** | No | Yes |
| **Batch Verification** | Limited | Efficient |

### Why Both?

Our implementation supports both schemes to:

- **Provide Flexibility**: Support different signature requirements
- **Enable Migration**: Allow gradual transition from ECDSA to Schnorr
- **Maintain Compatibility**: Support legacy systems if needed
- **Enable Testing**: Compare different signature schemes

---

## Usage in Transaction Inputs

Transaction inputs contain signatures that authorize spending of outputs.

### Input Structure

```rust
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,        // Previous transaction ID
    vout: usize,         // Output index in previous transaction
    signature: Vec<u8>,   // Schnorr signature (64 bytes)
    pub_key: Vec<u8>,    // Public key (33 bytes compressed)
}
```

### Signature in Inputs

- **Purpose**: Authorize spending of the referenced output
- **Size**: 64 bytes (Schnorr) or 70-72 bytes (ECDSA)
- **Content**: Signature of transaction hash
- **Validation**: Verified against public key

### Public Key in Inputs

- **Purpose**: Used for signature verification
- **Size**: 33 bytes (compressed secp256k1)
- **Source**: From the output being spent
- **Temporary**: Cleared after verification

### Example Transaction Input

```rust
let input = TXInput {
    txid: previous_tx_id.clone(),      // Reference to previous transaction
    vout: 0,                           // First output of previous transaction
    signature: signature_bytes,         // 64-byte Schnorr signature
    pub_key: public_key_bytes,         // 33-byte compressed public key
};
```

---

## Security Considerations

### 1. Private Key Protection

Private keys must never be exposed:
- **No Logging**: Never log private keys
- **Secure Storage**: Encrypt private keys at rest
- **Memory Safety**: Rust's memory safety helps prevent leaks

### 2. Signature Replay Prevention

Signatures are tied to specific transactions:
- **Transaction Hash**: Each transaction has unique hash
- **Non-reusable**: Signatures can't be reused for other transactions
- **Context-Specific**: Signatures include transaction context

### 3. Input Validation

All inputs are validated:
- **Previous Transaction**: Must exist and be valid
- **Output Index**: Must be valid output index
- **Signature**: Must be valid Schnorr signature
- **Public Key**: Must match output's public key hash

### 4. Error Handling

All cryptographic operations return `Result` types:
- **Explicit Errors**: Forces error handling
- **No Panics**: Prevents unexpected crashes
- **Recoverability**: Allows graceful error handling

---

## Summary

Digital signatures are essential for blockchain security:

1. **Schnorr Signatures**: Modern Bitcoin signatures (primary, 64 bytes)
2. **ECDSA Signatures**: Legacy support (70-72 bytes)
3. **Transaction Signing**: Multi-step process ensuring authorization
4. **Transaction Verification**: Ensures signatures are valid
5. **Security**: Private keys protected, signatures validated

**Key Takeaways:**

- Schnorr signatures provide better security and efficiency
- Transaction signing requires private key ownership
- Verification ensures transaction integrity
- Both schemes supported for flexibility

**Next Steps:**

- Continue to [Key Pair Generation](03-Key-Pair-Generation.md) to learn about key generation
- Review [Hash Functions](01-Hash-Functions.md) to understand hash operations
- Explore [Address Encoding](04-Address-Encoding.md) to see how addresses are created
- Check [Security and Performance](05-Security-and-Performance.md) for security best practices

---

## Navigation

- **[← Previous: Hash Functions](01-Hash-Functions.md)** - SHA-256 hashing
- **[Next: Key Pair Generation →](03-Key-Pair-Generation.md)** - Key generation and derivation
- **[Cryptography Index](README.md)** - Complete guide overview
- **[Hash Functions](01-Hash-Functions.md)** - SHA-256 hashing details
- **[Address Encoding](04-Address-Encoding.md)** - Base58 encoding
- **[Security and Performance](05-Security-and-Performance.md)** - Security best practices

**Related Guides:**
- **[Rust Language Guide](../../rust/README.md)** - Rust language features
- **[Web API Architecture](../web/README.md)** - Cryptographic operations in APIs

---

<div align="center">

**📚 [← Previous: Hash Functions](01-Hash-Functions.md)** | **Digital Signatures** | **[Next: Key Pair Generation →](03-Key-Pair-Generation.md)** 📚

</div>

---

*This section covers digital signatures used in our blockchain implementation. Continue to [Key Pair Generation](03-Key-Pair-Generation.md) to learn about key generation and derivation.*
