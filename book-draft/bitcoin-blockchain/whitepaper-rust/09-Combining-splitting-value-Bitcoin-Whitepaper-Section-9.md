<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
## 9. Combining/splitting value (Bitcoin Whitepaper Section 9)

The paper notes transactions may contain “multiple inputs and outputs” and usually have “one for the payment and one returning the change”. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

This is one of the most “implementation-shaped” statements in the paper. It is describing what a transaction *actually looks like* in a UTXO system:

- We do not have account balances we can “debit” directly.
- Instead, we spend **specific previous outputs** (UTXOs) and create **new outputs**.

### What the whitepaper requires:

In this section, the paper is establishing two requirements we must reflect in our Rust data model and validation:

- **Combining**: a payment can consume multiple earlier outputs (many inputs) to reach the amount we want to pay.
- **Splitting**: a payment can create multiple new outputs (many outputs), typically:
  - one output to the recipient (the payment), and
  - one output back to ourselves (change).

This is not an optional wallet convenience—this is the core “coin selection + change” behavior implied by the UTXO model.

### How this connects to validation and fees

This section also hints at two consensus-critical realities that show up immediately in code:

- **Value conservation**: total input value must be greater than or equal to total output value; the difference is the **fee** (see Section 6).

```text
sum(inputs)  >= sum(outputs)
fee = sum(inputs) - sum(outputs)
```
- **“Not already spent”**: each input points to a specific `OutPoint(txid, vout)` which must exist in the current UTXO set at our chosen chain tip.

Rust object model implication:

- `Transaction.inputs: Vec<TxIn>` (combine)
- `Transaction.outputs: Vec<TxOut>` (split + change)

### Rust mental model (what we are constructing)

At the “business object” level, our wallet is building:

```text
inputs:  [OutPoint -> TxIn]  (spend these UTXOs)
outputs: [TxOut, TxOut, ...] (create new UTXOs: payment + optional change)
```

### Worked example (numbers)

Suppose we want to pay **7_000 sats** and we select two UTXOs we control:

- input #0: 5_000 sats
- input #1: 4_000 sats

If the fee is **300 sats**, then:

```text
sum(inputs)  = 9_000
sum(outputs) = 7_000 (pay) + 1_700 (change)
fee          = 9_000 - 8_700 = 300
```

Rust example (combine UTXOs + make change):

```rust
pub fn make_payment(
    selected_inputs: Vec<OutPoint>,
    pay_value: i64,
    pay_script: Vec<u8>,
    change_value: i64,
    change_script: Vec<u8>,
) -> Transaction {
    let mut outputs = vec![TxOut { value: pay_value, script_pubkey: pay_script }];
    if change_value > 0 {
        outputs.push(TxOut { value: change_value, script_pubkey: change_script });
    }

    Transaction {
        version: 1,
        inputs: selected_inputs.into_iter().map(TxIn::from_outpoint).collect(),
        outputs,
        lock_time: 0,
    }
}
```

---

<div align="center">

**[← Merkle trees and SPV (Bitcoin Whitepaper Sections 7–8)](08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)** | Combining/splitting value (Bitcoin Whitepaper Section 9) | **[Privacy (Bitcoin Whitepaper Section 10) →](10-Privacy-Bitcoin-Whitepaper-Section-10.md)**

</div>
