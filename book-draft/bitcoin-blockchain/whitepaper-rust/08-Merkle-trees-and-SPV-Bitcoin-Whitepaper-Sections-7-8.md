<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
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
## 8. Merkle trees and SPV (Bitcoin Whitepaper Sections 7–22)

The paper uses Merkle trees so only the root is included in the block hash, and SPV verifies inclusion via a Merkle branch plus block headers. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Why this matters for implementation: Merkle trees are the bridge between “a block contains transactions” and “the header commits to them”. Once we have txids, a Merkle root is how we compress “all transactions” into one 32-byte commitment.

### 8.1 Merkle root (pairwise hash)

In this section, we compute the Merkle root the same way Bitcoin does: hash pairs upward (duplicating the last item at a level if there is an odd count) until one 32-byte root remains.

```rust
pub fn merkle_root(mut leaves: Vec<[u8; 32]>) -> [u8; 32] {
    if leaves.is_empty() {
        // Teaching code: empty tree => all-zero root
        // Real Bitcoin blocks always have >= 1 (coinbase)
        return [0u8; 32];
    }
    while leaves.len() > 1 {
        if leaves.len() % 2 == 1 {
            // Duplicate last node if odd count
            leaves.push(*leaves.last().unwrap());
        }
        // Next level has N/2 nodes (hash pairs upward)
        let mut next = Vec::with_capacity(leaves.len() / 2);
        for pair in leaves.chunks(2) {
            let mut b = Vec::with_capacity(64);
            b.extend_from_slice(&pair[0]);
            b.extend_from_slice(&pair[1]);
            // Merkle parent = sha256d(left || right)
            next.push(sha256d(&b));
        }
        leaves = next;
    }
    leaves[0]
}
```

**What this code is doing**
- Takes a list of leaf hashes (in Bitcoin, typically the txids of the block’s transactions).
- Hashes pairs upward until one hash remains: the Merkle root.
- If there is an odd number of nodes at a level, Bitcoin duplicates the last node before hashing pairs.
ie
- Input: all leaves (txids) in the block: Vec<[u8; 32]>
- Operation: builds the entire Merkle tree level-by-level (pair everything, hash upward, repeat)
- Output: the single root: BlockHeader.merkle_root
- Use case: what a miner/full node does when constructing/validating a full block (it has all txids).

Why “the next level has half as many nodes” (illustration):

```text
Each parent hash is computed from exactly 2 child hashes:
parent = H(left || right)

Pairing N hashes two-by-two produces N/2 parents.

Example with an even count (N = 4):
level 0:  A    B    C    D
          |____|    |____|
level 1:   H(AB)     H(CD)          => 2 parents = 4/2

Example with an odd count (N = 5):
level 0:  A    B    C    D    E
                              |
                            duplicate last -> E'
level 0': A    B    C    D    E    E'
          |____|    |____|    |____|
level 1:   H(AB)     H(CD)     H(EE)        => 3 parents = 6/2
```

**How it connects**

- The Merkle root is stored in `BlockHeader.merkle_root`.
- Any change to the transaction list changes the root, which changes the header hash, which invalidates PoW.

### 8.2 SPV (Simplified Payment Verification): headers + Merkle branches

In this section, we connect Section 7’s “keep commitments in headers” idea to Section 8’s SPV idea: a **lightweight client** eg a wallet client for eaxmple, can verify inclusion with (1) the header chain and (2) a Merkle branch.

SPV (Simplified Payment Verification) is the whitepaper’s answer to a practical engineering problem: **many users want to verify that a payment is “in the blockchain” without running a full node**.

To understand SPV, we separate what the blockchain provides into two layers:
- **Headers (global history + proof-of-work)**:
  - headers link by `prev_hash`, and each header is protected by PoW
  - headers are small, so a lightweight client can store the whole header chain
- **Block bodies (full transaction data)**:
  - full nodes store and validate all transactions, update the UTXO set, and enforce “not already spent”

#### 8.2.1 Why SPV is needed (and what it guarantees)

SPV is useful when we want mobile/embedded wallets that can:

- verify “this txid is included in a block header on the best chain” with small data (header chain + Merkle branch)
- avoid downloading and validating every transaction

But SPV does **not** provide full-node guarantees:

- it proves **inclusion in a PoW-backed chain**, not that the transaction is valid under all consensus rules
- it cannot independently enforce “not already spent” the way a full node does (that requires the UTXO set and full validation)

#### 8.2.2 Relationship to `HeaderChain` (from Section 7)

In Section 7 we introduced `HeaderChain` as the node’s **header index**: a fast way to store and query block headers and their linkage/work metadata. SPV depends on this idea directly.

Concretely, the “SPV client keeps block headers” statement means:

- The client maintains a `HeaderChain`-like structure containing headers and enough metadata to:
  - follow `prev_hash` links (chain connectivity), and
  - choose the best chain by work (the whitepaper’s “longest chain” / most-work rule).

This is the same reason full nodes keep a header index instead of “just reading block files”: we need fast answers to:

- “Is this header part of the best chain?”
- “What is the best tip?”
- “What is the accumulated work behind this tip?”

Where SPV differs from a full node (and why the UTXO database matters):

- A full node has **`HeaderChain` + `UtxoStore`**:
  - `HeaderChain` tells us what the best chain is (PoW-backed history).
  - `UtxoStore` lets us validate spends (“not already spent”) and scripts for every transaction.
- An SPV client has **`HeaderChain` only**:
  - it can verify that a header is PoW-valid and on the best chain,
  - but it cannot independently enforce “not already spent” without maintaining the full UTXO set.


#### 8.2.3 What data an SPV proof contains (at a high level)

To verify inclusion of a specific transaction in a specific block header, an SPV client needs:

- the **block header** (from its header chain),
- the transaction’s **txid** (the Merkle leaf),
- the **Merkle branch** (the sibling hashes on the path to the root),
- the leaf **index** (so we concatenate left/right in the correct order at each level).

SPV relies on the fact that a block header commits to the block’s transaction list via **`merkle_root`**:

- If we know a header is on the best (most-work) chain, then the header’s `merkle_root` is a commitment to a specific set of txids.
- A **Merkle branch** is a short proof that a particular txid is one of those committed txids.

#### 8.2.4 Relationship to pruning (why this matters for storage)

Section 7 (pruning) and Section 8 (SPV) use the same underlying idea: **commitment in the header, data in the body**.

- A **pruned full node** may delete old block bodies, but it typically keeps:
  - the **header chain** (for PoW + chain selection), and
  - the **UTXO set** (for validating new blocks)
- An **SPV client** goes further and keeps:
  - the **header chain only** (no UTXO set, no full block bodies)

So pruning and SPV are related, but they are not the same system:

- **Pruning** reduces disk usage for history while keeping full validation of new blocks.
- **SPV** reduces bandwidth/disk/CPU by outsourcing full validation to full nodes and verifying only inclusion + PoW-backed headers.

#### 8.2.5 Merkle-branch verification (Rust sketch)

```rust
pub fn verify_merkle_branch(
    leaf: [u8; 32],
    branch: &[[u8; 32]],
    mut index: usize,
) -> [u8; 32] {
    let mut h = leaf;
    for sibling in branch {
        let mut b = Vec::with_capacity(64);
        if index % 2 == 0 {
            // LEFT child: hash (h || sibling)
            b.extend_from_slice(&h);
            b.extend_from_slice(sibling);
        } else {
            // RIGHT child: hash (sibling || h)
            b.extend_from_slice(sibling);
            b.extend_from_slice(&h);
        }
        h = sha256d(&b);
        // Move to parent level (index / 2)
        index /= 2;
    }
    h
}
```

Illustration (what the loop is doing, step-by-step):

```text
Assume a block has 4 txids (4 leaves):

level 0 (leaves):   A      B      C      D
                   idx0   idx1   idx2   idx3
                    |______|      |______|
level 1:              P0            P1
                      |_____________|
level 2 (root):              R

Where:
P0 = H(A || B)
P1 = H(C || D)
R  = H(P0 || P1)

Suppose we want to prove inclusion of C.
Then:
leaf   = C          // the leaf (txid) we are proving
branch = [D, P0]    // sibling at level 0 is D, sibling at level 1 is P0
index  = 2          // C is the 3rd leaf (0-based), so its leaf index is 2

Now follow the code:

Step 1 (level 0):
index = 2  => index % 2 == 0 (we are LEFT)
h = H(C || D) = P1
index /= 2 => 1

Step 2 (level 1):
index = 1  => index % 2 == 1 (we are RIGHT)
h = H(P0 || P1) = R
index /= 2 => 0

Done: reconstructed root h == R.
If h equals the header’s merkle_root, the txid is included in that block.
```

**What this code is doing**

- Recomputes the Merkle root from a leaf and its Merkle branch (the sibling hashes along the path).
- `index` determines whether the current node is left/right at each level (controls concatenation order).

**How it connects**

- An SPV client keeps block headers (prev-hash chain + PoW) and uses Merkle branches to prove a transaction is included in a particular block header, matching the whitepaper’s SPV concept. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Takeaway: SPV is “verify inclusion without validating every transaction”.

---

<div align="center">

**[← Reclaiming disk space (Bitcoin Whitepaper Section 7)](07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)** | Merkle trees and SPV (Bitcoin Whitepaper Sections 7–22) | **[Combining/splitting value (Bitcoin Whitepaper Section 9) →](09-Combining-splitting-value-Bitcoin-Whitepaper-Section-9.md)**

</div>
