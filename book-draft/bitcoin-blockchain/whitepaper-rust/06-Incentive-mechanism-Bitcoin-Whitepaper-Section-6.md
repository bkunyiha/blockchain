<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
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
<!--
  Split from the Whitepaper-in-Rust unit index.
-->

## 6. Incentive mechanism (Bitcoin Whitepaper Section 6)

The whitepaper explains why miners spend resources: the first transaction in each block creates new coins (“coinbase”), and transaction fees accrue to the miner. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Implementation framing:

- **Consensus**: coinbase must be the first transaction; value creation must obey the block subsidy schedule + fees.
- **Engineering**: fee calculation depends on prevout lookup (UTXO view) and correct value conservation.

### 6.1 Fee calculation (inputs − outputs)

In the UTXO model, fees are not an explicit field. We compute:

```text
fee = sum(input_values) - sum(output_values)
```

Rust example function to calculate transaction fees (UTXO-aware):

```rust
pub fn tx_fee(tx: &Transaction, utxos: &dyn UtxoView) -> Result<i64, TxError> {
    if tx.is_coinbase() {
        return Ok(0);
    }

    let mut in_sum: i64 = 0;
    for vin in &tx.inputs {
        let prev = utxos.get(&vin.previous_output)?; // OutPoint -> TxOut
        // checked_add: returns None on overflow instead of wrapping/panicking.
        // This prevents silent money-creation bugs when summing many inputs.
        in_sum = in_sum.checked_add(prev.value).ok_or(TxError::Overflow)?;
    }

    let mut out_sum: i64 = 0;
    for vout in &tx.outputs {
        // checked_add: same idea for output sum (detect overflow explicitly).
        out_sum = out_sum.checked_add(vout.value).ok_or(TxError::Overflow)?;
    }

    // checked_sub: returns None on underflow (e.g. outputs > inputs).
    // That case corresponds to an invalid transaction (it would create value).
    in_sum.checked_sub(out_sum).ok_or(TxError::Underflow)
}
```

Where `checked_add` / `checked_sub` come from:

- They are **Rust standard library methods on integer primitives** (here: `i64`), available via the prelude (no import needed).
- Conceptually, they implement “do the arithmetic, but fail safely if it can’t be represented in this integer type”.

### 6.2 Coinbase transaction sketch

Coinbase is special:

- it has no real prevouts (it creates new value)
- it pays the miner (typically to a miner-controlled scriptPubKey)

Rust-shaped intent (simplified):

```rust
pub fn make_coinbase(height: u32, reward: i64, pay_to_script: Vec<u8>) -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn::coinbase(height)],
        outputs: vec![TxOut { value: reward, script_pubkey: pay_to_script }],
        lock_time: 0,
    }
}
```

Takeaway: incentives are “value conservation + one special mint transaction”; everything else is enforced by validation against the UTXO view.

---

<div align="center">

**[← Network operation (Bitcoin Whitepaper Section 5)](05-Network-operation-Bitcoin-Whitepaper-Section-5.md)** | Incentive mechanism (Bitcoin Whitepaper Section 6) | **[Reclaiming disk space (Bitcoin Whitepaper Section 7) →](07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)**

</div>
