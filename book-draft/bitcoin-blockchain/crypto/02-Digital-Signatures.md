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
# Digital Signatures: Transaction Authorization

Digital signatures prove ownership and authorize transactions in the blockchain. Every transaction input must be signed with the private key corresponding to the public key that locks the output being spent. In this section, we connect the whitepaper’s “ownership” model to the actual signing and verification code paths in this project.

### Signature vs Hash: identity vs fingerprint

In the previous section we learned about digital hashes, which give us a compact, one-way fingerprint of data. In this section we move one step further and introduce digital signatures, which prove *who* authorized that data. A hash tells you what bytes you have, but it says nothing about identity or intent. A signature is produced with a private key, and anyone can verify it with the public key to confirm the data was authorized by the key holder and has not been altered since.

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

A **digital signature** is a cryptographic primitive that binds a message to a keypair: the signer uses a private key to produce a signature over a message (or message digest), and anyone can use the corresponding public key to verify that (1) the signer knew the private key and (2) the message was not modified. Digital signatures provide authenticity and integrity, but they do not provide confidentiality.

Digital signatures serve three critical functions in blockchain systems:

1. **Authentication**: Prove that the signer owns the private key
2. **Authorization**: Authorize spending of transaction outputs
3. **Integrity**: Ensure transaction data hasn't been tampered with

### How Digital Signatures Work

1. **Signing**: Construct a deterministic message digest (often called a *sighash*) from the transaction data, then run a signature algorithm with the private key to produce a signature over that digest.
2. **Verification**: Signature is verified against public key and transaction hash
3. **Validation**: Only valid signatures allow spending of outputs

### Signature Flow in Transactions

```text
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

### Figure: Signing vs verification (what the network checks)

This diagram contrasts the wallet's signing flow with the network's deterministic verification flow using the same transaction hash. `schnorr_sign_digest` uses the private key to create a signature over the hash, while `schnorr_sign_verify` uses the public key to check that signature by re-deriving the expected verification equation without revealing the private key.

```text
      signer (wallet)          verifier (node)
┌────────────────────┐    ┌─────────────────────┐
│ "trimmed copy" tx  │    │ Rebuild tx copy     │
│ per-input setup    │    │ from context        │
└────────┬───────────┘    └────────┬────────────┘
         │                         │
         │ tx.hash()               │ tx.hash()
         ▼                         ▼
      txid (32)               txid (32)
         │                         │
     sign(...)                 verify(...)
         ▼                         ▼
      sig (64)              accept/reject
```

---

## Schnorr Signatures: Modern Bitcoin

Schnorr signatures are the modern signature scheme introduced with Bitcoin's Taproot upgrade. They provide better security properties, smaller signatures, and support for signature aggregation. Legacy Bitcoin used ECDSA signatures, which we cover next.

### Implementation

**Signing:**

```rust
use secp256k1::{Keypair, Message, Secp256k1, SecretKey};

pub fn schnorr_sign_digest(
    private_key: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
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

**Important note about our current implementation:**
- `schnorr_sign_digest` hashes the provided `message` internally using `sha256_digest(message)`. In the transaction code path, we pass `tx_copy.get_id()` as the message, which means we currently sign **SHA256(txid_bytes)** (a “hash of a hash”). This can be a reasonable design, but it is not the same as “sign the txid directly,” and it is worth keeping in mind if you aim for strict Bitcoin compatibility.

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
    let public_key_obj = match
        PublicKey::from_byte_array_compressed(public_key_array)
    {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Convert to XOnlyPublicKey for Schnorr
    let pk_bytes = public_key_obj.serialize();
    let xonly_array: [u8; 32] = match pk_bytes[1..33].try_into() {
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
    secp.verify_schnorr(&signature_obj, &message_hash, &xonly_public_key)
        .is_ok()
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

pub fn ecdsa_p256_sha256_sign_digest(
    pkcs8: &[u8],
    message: &[u8],
) -> Result<Vec<u8>> {
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
async fn sign(
    &mut self,
    blockchain: &BlockchainService,
    private_key: &[u8],
) -> Result<()> {
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
        let txid = vin.get_txid();
        let current_vin_tx_option = blockchain
            .find_transaction(txid)
            .await?;
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
        let pub_key_hash = current_vin_tx.vout[vin.vout]
            .pub_key_hash
            .clone();
        trimmed_self_copy.vin[idx].pub_key = pub_key_hash;
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

- Continue to Key Pair Generation to learn about key generation
- Review Hash Functions to understand hash operations
- Explore Address Encoding to see how addresses are created
- Check Security and Performance for security best practices

---

## Navigation

- **← Previous: Hash Functions** - SHA-256 hashing
- **Next section: Key Pair Generation →** - Key generation and derivation
- **Cryptography Index** - Complete guide overview
- **Hash Functions** - SHA-256 hashing details
- **Address Encoding** - Base58 encoding
- **Security and Performance** - Security best practices

**Related Guides:**
- **Rust Language Guide** - Rust language features
- **Web API Architecture** - Cryptographic operations in APIs

---

<div align="center">

**[← Previous: Hash Functions](01-Hash-Functions.md)** | **[Digital Signatures](02-Digital-Signatures.md)** | **[Next section: Key Pair Generation →](03-Key-Pair-Generation.md)**

</div>

---

*In the next part of this section, we move from “how signatures work” to “where keys come from.” Continue to Key Pair Generation to learn how we generate and derive the keys used for signing and verification.*

---

## References

In this section, we provide references for the exact standards and Bitcoin conventions behind the signature code:

- **[BIP 340: Schnorr Signatures for secp256k1](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)** (Taproot Schnorr signing and verification model)
- **[SEC 2: Recommended Elliptic Curve Domain Parameters](https://www.secg.org/sec2-v2.pdf)** (includes secp256k1 parameters used by Bitcoin)
- **[FIPS 186-5: Digital Signature Standard (DSS)](https://csrc.nist.gov/publications/detail/fips/186/5/final)** (ECDSA definition and security considerations)
