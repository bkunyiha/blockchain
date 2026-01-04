<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. [Chapter 2.0: Rust Blockchain Project](../Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../crypto/README.md) - Cryptographic primitives and libraries
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
Given q and z, what is the probability the attacker eventually catches up and overtakes the honest chain?
```

The paper’s conclusion is the intuition that drives production policy: as z increases (more confirmations), attacker success probability drops rapidly when q < p.

### 11.2 Rust implementation of AttackerSuccessProbability(q, z)

Below is a Rust-shaped translation of the paper’s C structure, but written in a way that is easier to reason about in a codebase (incremental Poisson term, explicit parameter meanings, and guard rails).

```rust
/// Approximate probability that an attacker with hashpower fraction `q` catches up
/// after the merchant waits for `z` confirmations.
///
/// - `q`: attacker fraction of hashpower (0.0..=1.0)
/// - `z`: confirmations waited (>= 0)
///
/// Returns a value in [0.0, 1.0].
pub fn attacker_success_probability(q: f64, z: u32) -> f64 {
    // Guard rails for callers (wallets/services) that may pass arbitrary inputs.
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
    let mut sum = 1.0; // we subtract terms from 1.0 (same structure as the paper)

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
    /// This is NOT derived from the chain; it is a wallet/service configuration input
    /// that encodes “how strong of an attacker are we willing to model?”
    /// Examples: 0.1 (10%), 0.3 (30%). As `assumed_q` increases, we require more confirmations.
    pub assumed_q: f64,

    /// Acceptable risk bound (epsilon).
    ///
    /// We treat a payment as "settled" once:
    /// `attacker_success_probability(assumed_q, z) <= epsilon`.
    /// Smaller epsilon => stricter policy (more confirmations).
    /// Example: 1e-6 means “at most one-in-a-million modeled success probability.”
    pub epsilon: f64,

    /// Maximum confirmations we are willing to search up to when computing a required `z`.
    ///
    /// This is a practical bound for UI/policy code (avoid unbounded loops). If no `z <= max_z`
    /// satisfies the risk threshold, we return `None` and the caller can choose a fallback policy.
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
/// Pick the smallest confirmation count such that attack probability <= epsilon.
pub fn min_confirmations_for_risk(q: f64, epsilon: f64, max_z: u32) -> Option<u32> {
    // Validate inputs: both `q` and `epsilon` are probabilities.
    // `(0.0..=1.0).contains(&x)` is true iff `0.0 <= x <= 1.0`.
    if !(0.0..=1.0).contains(&q) || !(0.0..=1.0).contains(&epsilon) {
        return None;
    }
    // Search from 0 confirmations upward and return the *first* `z` that meets our risk bound.
    for z in 0..=max_z {
        // If the attacker success probability at depth `z` is <= epsilon, we accept `z`.
        if attacker_success_probability(q, z) <= epsilon {
            return Some(z);
        }
    }
    // If no `z <= max_z` is good enough, return None so the caller can pick a stricter policy.
    None
}
```

Implementation note: this probability is not consensus-critical; it belongs in wallet/service policy code, not block validation. That means we can evolve the implementation (better numerics, caching, lookup tables) without affecting consensus.

Takeaway: confirmations are a probabilistic security margin; the “z blocks deep” intuition is backed by an explicit model.

---

<div align="center">

**[← Privacy (Bitcoin Whitepaper Section 10)](10-Privacy-Bitcoin-Whitepaper-Section-10.md)** | Confirmations and attacker probability (Bitcoin Whitepaper Section 11) | **[Conclusion (Bitcoin Whitepaper Section 12) →](12-Conclusion-Bitcoin-Whitepaper-Section-12.md)**

</div>
