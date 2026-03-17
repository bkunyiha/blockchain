<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
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
    let mut outputs = vec![TxOut {
        value: pay_value,
        script_pubkey: pay_script,
    }];
    if change_value > 0 {
        outputs.push(TxOut {
            value: change_value,
            script_pubkey: change_script,
        });
    }

    Transaction {
        version: 1,
        inputs: selected_inputs
            .into_iter()
            .map(TxIn::from_outpoint)
            .collect(),
        outputs,
        lock_time: 0,
    }
}
```

---

<div align="center">

**[← Merkle trees and SPV (Bitcoin Whitepaper Sections 7–22)](08-Merkle-trees-and-SPV-Bitcoin-Whitepaper-Sections-7-8.md)** | Combining/splitting value (Bitcoin Whitepaper Section 9) | **[Privacy (Bitcoin Whitepaper Section 10) →](10-Privacy-Bitcoin-Whitepaper-Section-10.md)**

</div>
