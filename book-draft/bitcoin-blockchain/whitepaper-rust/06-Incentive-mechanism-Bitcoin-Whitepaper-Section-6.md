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
pub fn make_coinbase(
    height: u32,
    reward: i64,
    pay_to_script: Vec<u8>,
) -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn::coinbase(height)],
        outputs: vec![TxOut {
            value: reward,
            script_pubkey: pay_to_script,
        }],
        lock_time: 0,
    }
}
```

Takeaway: incentives are “value conservation + one special mint transaction”; everything else is enforced by validation against the UTXO view.

---

<div align="center">

**[← Network operation (Bitcoin Whitepaper Section 5)](05-Network-operation-Bitcoin-Whitepaper-Section-5.md)** | Incentive mechanism (Bitcoin Whitepaper Section 6) | **[Reclaiming disk space (Bitcoin Whitepaper Section 7) →](07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)**

</div>
