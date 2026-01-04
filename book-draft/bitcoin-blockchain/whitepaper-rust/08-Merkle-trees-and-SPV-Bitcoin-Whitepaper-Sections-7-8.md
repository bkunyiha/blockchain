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
## 8. Merkle trees and SPV (Bitcoin Whitepaper Sections 7–8)

The paper uses Merkle trees so only the root is included in the block hash, and SPV verifies inclusion via a Merkle branch plus block headers. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Why this matters for implementation: Merkle trees are the bridge between “a block contains transactions” and “the header commits to them”. Once we have txids, a Merkle root is how we compress “all transactions” into one 32-byte commitment.

### 8.1 Merkle root (pairwise hash)

In this section, we compute the Merkle root the same way Bitcoin does: hash pairs upward (duplicating the last item at a level if there is an odd count) until one 32-byte root remains.

```rust
pub fn merkle_root(mut leaves: Vec<[u8; 32]>) -> [u8; 32] {
    if leaves.is_empty() {
        // Convention for this teaching code: empty tree => all-zero root.
        // (Real Bitcoin blocks always have at least 1 transaction: the coinbase.)
        return [0u8; 32];
    }
    while leaves.len() > 1 {
        if leaves.len() % 2 == 1 {
            // Bitcoin rule: if a level has an odd count, duplicate the last node
            // so we still hash pairs.
            leaves.push(*leaves.last().unwrap());
        }
        // Next level has half as many nodes as the current level. See illustration bellow.
        let mut next = Vec::with_capacity(leaves.len() / 2);
        for pair in leaves.chunks(2) {
            // Merkle parent = sha256d(left_child || right_child)
            let mut b = Vec::with_capacity(64);
            b.extend_from_slice(&pair[0]);
            b.extend_from_slice(&pair[1]);
            // sha256d = SHA256(SHA256(bytes)) (Bitcoin-style “double SHA-256”)
            next.push(sha256d(&b));
        }
        // Replace the current level with the computed parent level and repeat.
        leaves = next;
    }
    // When one hash remains, it is the Merkle root.
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

So if we start with N hashes at a level, pairing them two-by-two produces N/2 parents.

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
pub fn verify_merkle_branch(leaf: [u8; 32], branch: &[[u8; 32]], mut index: usize) -> [u8; 32] {
    // Start from the leaf (a txid) and hash upward toward the root.
    let mut h = leaf;
    for sibling in branch {
        // Each step hashes exactly two 32-byte children => 64 bytes total.
        let mut b = Vec::with_capacity(64);
        if index % 2 == 0 {
            // If our node is the LEFT child at this level, hash (h || sibling).
            b.extend_from_slice(&h);
            b.extend_from_slice(sibling);
        } else {
            // If our node is the RIGHT child, hash (sibling || h).
            b.extend_from_slice(sibling);
            b.extend_from_slice(&h);
        }
        // Parent = sha256d(left || right)
        h = sha256d(&b);
        // Move one level up in the tree:
        // integer division by 2 maps a leaf index to its parent index.
        index /= 2;
    }
    // After consuming the full branch, h is the reconstructed Merkle root.
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

**[← Reclaiming disk space (Bitcoin Whitepaper Section 7)](07-Reclaiming-disk-space-Bitcoin-Whitepaper-Section-7.md)** | Merkle trees and SPV (Bitcoin Whitepaper Sections 7–8) | **[Combining/splitting value (Bitcoin Whitepaper Section 9) →](09-Combining-splitting-value-Bitcoin-Whitepaper-Section-9.md)**

</div>
