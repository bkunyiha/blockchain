<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
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
## 11. Confirmations and attacker probability (Bitcoin Whitepaper Section 11)

The paper models attacker catch-up probability (Gambler’s Ruin / Poisson approximation) and provides reference C code for `AttackerSuccessProbability(q, z)`. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

This section is less about hashing bytes and more about turning Proof-of-Work into an engineering decision: **how many confirmations do we wait for before we treat a payment as “final enough”**?

In this section, we explain what confirmations mean in a node implementation, what the attacker model is assuming, and how to translate the paper’s probability calculation into Rust code that a wallet or service can actually use.

### 11.0 What a “confirmation” means in code

In a full node, the chain tip can change due to reorgs. A transaction is “confirmed” when it is included in a block that is part of the node’s current best chain.

- A transaction has **1 confirmation** when it is in the current best tip block.
- It has **z confirmations** when it is z blocks deep from the tip (i.e., there are z-1 blocks built after the block that contains it).

The operational takeaway is that “confirmations” are a proxy for **reorg resistance**: deeper transactions are harder (but not impossible) to replace with an alternative chain.

### 11.1 The threat model (what we are bounding)

The whitepaper analyzes a classic double-spend scenario:

- The honest network has fraction **p** of hash power.
- The attacker has fraction **q** of hash power.
- A merchant waits for **z** confirmations before accepting a payment as settled.

The attacker tries to privately mine an alternative branch that excludes (or reverses) the payment. The question we want to answer is:

```text
Given q and z, what is the probability the attacker
catches up and overtakes the honest chain?
```

The paper’s conclusion is the intuition that drives production policy: as z increases (more confirmations), attacker success probability drops rapidly when q < p.

### 11.2 Rust implementation of AttackerSuccessProbability(q, z)

Below is a Rust-shaped translation of the paper’s C structure, but written in a way that is easier to reason about in a codebase (incremental Poisson term, explicit parameter meanings, and guard rails).

```rust
/// Approximate attacker success probability given
/// hashpower fraction q and z confirmations.
///
/// - `q`: attacker hashpower fraction (0.0..=1.0)
/// - `z`: confirmations waited (>= 0)
///
/// Returns a value in [0.0, 1.0].
pub fn attacker_success_probability(q: f64, z: u32) -> f64 {
    // Guard rails for arbitrary caller inputs
    if !q.is_finite() {
        return f64::NAN;
    }
    if q <= 0.0 {
        return 0.0;
    }
    if q >= 1.0 {
        return 1.0;
    }

    let p = 1.0 - q;
    if p <= q {
        return 1.0;
    }

    let lambda = (z as f64) * (q / p);
    // we subtract terms from 1.0 (same structure as the paper)
    let mut sum = 1.0;

    // Compute Poisson(k; lambda) incrementally:
    // P(0) = exp(-lambda)
    // P(k) = P(k-1) * lambda / k
    let mut poisson_k = (-lambda).exp();

    for k in 0..=z {
        // term = Poisson(k; lambda) * (1 - (q/p)^(z-k))
        let deficit = (z - k) as i32;
        let catchup_from_deficit = 1.0 - (q / p).powi(deficit);
        sum -= poisson_k * catchup_from_deficit;

        // advance poisson_k for next k (avoid divide-by-zero when k == 0)
        if k < z {
            poisson_k *= lambda / ((k + 1) as f64);
        }
    }
    sum
}
```

### 11.3 What the code is doing

- We define **p = 1 - q** as the honest fraction of hash power.
- If **q ≥ p**, the attacker is at least as strong as the network; given enough time, catching up is modeled as essentially certain → return 1.0.
- Otherwise we compute **λ = z · (q / p)**, the Poisson parameter used by the paper.
- We then sum over k = 0..z:
  - k is “how many blocks the attacker found while the honest network found z blocks” (modeled by a Poisson distribution)
  - (z - k) is the remaining deficit the attacker must close
  - (q/p)^(z-k) is the paper’s catch-up term from that deficit

The result is a single number in [0.0, 1.0] that we can compare against a risk tolerance.

### 11.4 How a Rust wallet/service uses this

Nodes validate blocks and maintain the best chain; **wallets and services choose confirmation policy**. In a Rust codebase, the typical integration points are:

- **UI/merchant policy**: “For payments above X sats, require at least z confirmations.”
- **Risk-based policy**: choose the smallest z such that attacker_success_probability(q, z) ≤ ε.

In a client implementation, this code typically lives in a **confirmation policy / settlement module**. The caller is not “the network”; it is your own wallet or service deciding when to move a payment through states like:

```text
seen_in_mempool -> confirmed(z=1) -> confirmed(z=2) -> ... -> settled
```

#### 11.4.1 What part of wallet/client logic calls this?

Common call sites look like this:

- **On new block tip**: when your node/wallet observes a new best tip, you update your **wallet’s own persistence/index** (not Bitcoin consensus state) for each tracked transaction: recompute confirmation depth `z`, then evaluate whether it is “settled enough” for your risk tolerance.
  - In practice this is a wallet database (often a key-value DB or SQLite) that stores wallet-visible state such as:
    - txid → `{ first_seen, confirmed_height, confirmations, status }`
    - outpoints/UTXOs → `{ spendable?, spent_by_txid?, value, script_pubkey }`
    - bookkeeping metadata → `{ account, label/invoice_id }`
- **On reorg**: if the best chain(on the blockchain) reorganizes, `z` can decrease (or a tx can become unconfirmed). You recompute status using the new `z`.
- **On “accept payment” decision** (merchant/exchange): when you are about to credit a user balance or release goods, you call this policy function to decide whether current `z` meets your threshold.

Rust-shaped wiring (pseudocode):

```rust
pub struct ConfirmationPolicy {
    /// Assumed attacker hashpower fraction for our threat model.
    ///
    // / This is NOT derived from the chain; it is a wallet/service
    // configuration input
    /// that encodes “how strong of an attacker are we willing to model?”
    // / Examples: 0.1 (10%), 0.3 (30%). As `assumed_q` increases, we require
    // more confirmations.
    pub assumed_q: f64,

    /// Acceptable risk bound (epsilon).
    ///
    /// We treat a payment as "settled" once:
    /// `attacker_success_probability(assumed_q, z) <= epsilon`.
    /// Smaller epsilon => stricter policy (more confirmations).
    // / Example: 1e-20 means “at most one-in-a-million modeled success
    // probability.”
    pub epsilon: f64,

    // / Maximum confirmations we are willing to search up to when computing a
    // required `z`.
    ///
    // / This is a practical bound for UI/policy code (avoid unbounded loops).
    // If no `z <= max_z`
    // / satisfies the risk threshold, we return `None` and the caller can
    // choose a fallback policy.
    pub max_z: u32,
}

impl ConfirmationPolicy {
    /// Decide whether a transaction is "settled enough" at depth `z`.
    pub fn is_settled(&self, z: u32) -> bool {
        attacker_success_probability(self.assumed_q, z) <= self.epsilon
    }

    /// Or precompute a required confirmation count once and compare.
    pub fn required_confirmations(&self) -> Option<u32> {
        min_confirmations_for_risk(self.assumed_q, self.epsilon, self.max_z)
    }
}

pub fn on_new_best_tip(policy: &ConfirmationPolicy, tracked: &mut [TrackedTx]) {
    for tx in tracked {
        // tx.confirmations is computed from the header chain / best tip height.
        if policy.is_settled(tx.confirmations) {
            tx.status = TxStatus::Settled;
        }
    }
}
```

Implementation note for developers: `q` is an **assumption**, not something the chain tells you. In production, wallets/services treat it as configuration and typically cap it to a conservative value for high-value transfers.

Rust-shaped helper (pseudocode):

```rust
/// Pick smallest z where attack probability <= epsilon.
pub fn min_confirmations_for_risk(
    q: f64,
    epsilon: f64,
    max_z: u32,
) -> Option<u32> {
    // Validate inputs: q, epsilon in [0.0, 1.0]
    if !(0.0..=1.0).contains(&q)
        || !(0.0..=1.0).contains(&epsilon)
    {
        return None;
    }
    // Search from z=0 upward for first acceptable z
    for z in 0..=max_z {
        if attacker_success_probability(q, z) <= epsilon {
            return Some(z);
        }
    }
    // No z <= max_z meets risk threshold
    None
}
```

Implementation note: this probability is not consensus-critical; it belongs in wallet/service policy code, not block validation. That means we can evolve the implementation (better numerics, caching, lookup tables) without affecting consensus.

Takeaway: confirmations are a probabilistic security margin; the “z blocks deep” intuition is backed by an explicit model.

---

<div align="center">

**[← Privacy (Bitcoin Whitepaper Section 10)](10-Privacy-Bitcoin-Whitepaper-Section-10.md)** | Confirmations and attacker probability (Bitcoin Whitepaper Section 11) | **[Conclusion (Bitcoin Whitepaper Section 12) →](12-Conclusion-Bitcoin-Whitepaper-Section-12.md)**

</div>
