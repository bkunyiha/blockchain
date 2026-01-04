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
## 4. Proof-of-work (Bitcoin Whitepaper Section 4)

The paper describes PoW as “scanning for a value that when hashed … begins with a **number of zero bits**” and implementing it by “incrementing a `nonce` in the block”. In deployed Bitcoin this is expressed as a **target threshold**: a block is valid if `block_hash <= target(bits)`. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

This section is where the whitepaper’s “**hard to find, easy to verify**” becomes a strict predicate. **Mining** is just **repeated hashing**; **verification** is one hash + one comparison.
- *One hash*: take the block header bytes (80 bytes in Bitcoin), run the PoW hash function (conceptually “double‑SHA256” in Bitcoin) to get a 32‑byte hash.
- *One comparison*: check whether that hash is ≤ the target threshold derived from the difficulty (nBits).

### 4.1 `nBits` (difficulty_bits) → target: expand compact encoding and check `block_hash <= target`
For more on `nBits`, see: **[nBits → target: why exponent is byte-length](nBits-Target-Expansion.md)**.

- **Why it’s “hard to find, easy to verify”**:
  - mining: vary the header (usually just `nonce`) and hash repeatedly until `meets_target(...)` is true
  - verification: compute one hash and do one comparison
- **Probability intuition**: if SHA-256 outputs are uniform, then \(P[\text{hash} \le \text{target}] \approx \text{target}/2^{256}\). Halving the target roughly doubles the expected work.

```rust
pub fn bits_to_target(difficulty_bits: u32) -> [u8; 32] {
    // Bitcoin "compact" target format (difficulty_bits(nBits)): [exponent (1 byte)] [mantissa (3 bytes)]
    // Interpreted as: target = mantissa * 256^(exponent-3)

    // Shifting right by 24 extracts the *top byte* (the exponent), which is why we do `>> 24`.
    // i.e. it discards the 24 least-significant bits and leaves the 8 exponent bits.
    let exp = (difficulty_bits >> 24) as u8; // exponent (8 bits / 1 byte)

    // 0x00FF_FFFF in binary is top 8 bits: 00000000 and bottom 24 bits: 11111111 11111111 11111111
    // So difficulty_bits & 0x00FF_FFFF zeros out the top 8 exponent bits and keeps only the lowest 24 bits.
    let mant = difficulty_bits & 0x00FF_FFFF; // mantissa (low 24 bits / 3 bytes)

    // Allocate a 32-byte (256-bit) buffer for the expanded target threshold.
    // We start with all zeros and then place the mantissa bytes at the position implied by `exp`.
    let mut target = [0u8; 32];

    // If the mantissa is 0, the compact encoding describes a target of 0.
    // That would make PoW impossible to satisfy (only hash==0 would pass), so in practice
    // this is either malformed input or an invalid header value.
    if mant == 0 {
        // If mantissa is zero, then target = 0 * 256^(...) = 0. (In practice, invalid/unusable for PoW.)
        return target;
    }

    // If `exp <= 3`, and target = mantissa * 256^(exp-3), then expanded target is <= 3 bytes wide, 
    //  so we must shift the mantissa
    // *right* to shrink it down to `exp` bytes (instead of shifting left to grow it).
    //  For code details, see `Worked byte example (how the mantissa bytes get placed)` below.
    if exp <= 3 {
        // If exponent is small, the mantissa is shifted right.
        let shift_bytes = (3 - exp) as u32;
        let m = (mant >> (8 * shift_bytes)) as u32;
        target[28..32].copy_from_slice(&m.to_be_bytes());
        return target;
    }

    // `start` is a byte index into the 32‑byte big-endian target array.
    // Intuition: `exp` says the expanded target should be `exp` bytes wide, 
    //  so it begins at offset: start = 32 - exp
    // Example: if exp=29, start=3, so the expanded number occupies 
    //  target[3..32] and target[0..3] are leading zeros.
    // `saturating_sub` is used to avoid underflow if an invalid header provides exp > 32.
    //  For code details, see `Worked byte example (how the mantissa bytes get placed)` below.
    let start = 32usize.saturating_sub(exp as usize); // start = 32 - exp
    if start + 3 <= 32 {
        // Write 3 mantissa bytes
        // For a more detailed explanation, 
        // write the first mantissa byte (the most-significant of the 3) into target[start]
        target[start] = ((mant >> 16) & 0xFF) as u8;

        // Write the next mantissa byte into target[start+1]
        target[start + 1] = ((mant >> 8) & 0xFF) as u8;
        
        // Write the last mantissa byte into target[start+2]
        target[start + 2] = (mant & 0xFF) as u8;
    }
    target
}

pub fn meets_target(block_hash: [u8; 32], target_be: [u8; 32]) -> bool {
    // Bitcoin's displayed hashes are typically reversed, but for numeric compare
    // treat both as big-endian integers.
    let mut h = block_hash;
    h.reverse(); // convert from little-endian digest bytes to big-endian compare
    h <= target_be
}
```

**What this code is doing**

- `difficulty_bits` (nBits) stores the PoW target compactly to keep headers small.
- `bits_to_target` expands nBits into a full 256-bit target threshold.
- `meets_target` checks whether the mined `block_hash` is numerically ≤ that target.

**Worked byte example (how the mantissa bytes get placed)**

Inside `bits_to_target`, we expand the compact `difficulty_bits(nBits)` value by extracting:

- `exp` = top byte (exponent)
- `mant` = low 24 bits (mantissa, conceptually three bytes `AA BB CC`)

#### For the case `exp <= 3` (small exponent / right-shift case)
```rust
    if exp <= 3 {
        // If exponent is small, the mantissa is shifted right.
        let shift_bytes = (3 - exp) as u32;
        let m = (mant >> (8 * shift_bytes)) as u32;
        target[28..32].copy_from_slice(&m.to_be_bytes());
        return target;
    }
```

- **Why we shift right in this case**:
  - The compact rule is `target = mantissa * 256^(exp-3)`. If `exp <= 3`, then `(exp-3)` is **0 or negative**, which means we are not “growing” the mantissa; we are keeping it the same size (`exp=3`) or **shrinking** it (`exp<3`).

```text
exp = 3: exp-3 =  0  => target = mantissa * 256^0  = mantissa       (no shift)
exp = 2: exp-3 = -1  => target = mantissa / 256    (shift right 1 byte)
exp = 1: exp-3 = -2  => target = mantissa / 256^2  (shift right 2 bytes)
```

  - Division by 256 is exactly a right shift by 8 bits (one byte), so the code implements this “divide by 256^(3-exp)” by shifting the mantissa right.

- **Byte illustration** (mantissa is always 3 bytes `AA BB CC`):

```text
mantissa = [AA BB CC]

exp = 3 => target bytes: [AA BB CC]
exp = 2 => target bytes: [AA BB]     (drop 1 byte => /256)
exp = 1 => target bytes: [AA]        (drop 2 bytes => /256^2)
```

- **Step 1 — compute shift_bytes**
```rust
let shift_bytes = (3 - exp) as u32;
```

  - The mantissa is always **3 bytes**.
  - `exp` is the desired target “byte-width” in this branch (`exp` is 1, 2, or 3).
  - So `3 - exp` is “how many bytes we need to remove from the mantissa to shrink it down to `exp` bytes”.

```text
exp = 3 => shift_bytes = 0  (remove 0 bytes, keep AA BB CC)
exp = 2 => shift_bytes = 1  (remove 1 byte, keep AA BB)
exp = 1 => shift_bytes = 2  (remove 2 bytes, keep AA)
```

- **Step 2 — shift the mantissa right by `shift_bytes` bytes**

```rust
let m = (mant >> (8 * shift_bytes)) as u32;
```

  - Shifting right by 8 bits removes 1 byte.
  - So shifting right by `8 * shift_bytes` removes exactly `shift_bytes` bytes.

```text
mant = 0x00AABBCC

exp = 2 => shift_bytes = 1(ie 3-2) => mant >> 8  = 0x0000AABB  (drops CC, keeps AA BB)
exp = 1 => shift_bytes = 2(ie 3-1) => mant >> 16 = 0x000000AA  (drops BB CC, keeps AA)
```

These two steps implement the “divide by 256^(3-exp)” idea to shrink the 3-byte mantissa down to an `exp`-byte value.

#### In the normal case (`exp > 3`)
```rust
    let start = 32usize.saturating_sub(exp as usize); // start = 32 - exp
    if start + 3 <= 32 {
        // Write 3 mantissa bytes
        // For a more detailed explanation, see `Worked byte example (how the mantissa bytes get placed)` below.
        // write the first mantissa byte (the most-significant of the 3) into target[start]
        target[start] = ((mant >> 16) & 0xFF) as u8;

        // Write the next mantissa byte into target[start+1]
        target[start + 1] = ((mant >> 8) & 0xFF) as u8;

        // Write the last mantissa byte into target[start+2]
        target[start + 2] = (mant & 0xFF) as u8;
    }
```
- **Step 1 — compute `start`**: `start = 32 - exp` (byte offset where the `exp`-byte-wide number begins inside a 32-byte big-endian array).

- **Step 2 — place the 3 mantissa bytes (`AA BB CC`) into the 32-byte target**:

```text
target[start]     = AA   // ((mant >> 16) & 0xFF)
target[start + 1] = BB   // ((mant >>  8) & 0xFF)
target[start + 2] = CC   // ( mant        & 0xFF)
```

- **Step 3 — understand `& 0xFF` (masking one byte)**:
  - `0xFF` in hex is `255` in decimal, which in binary is `11111111` (8 one-bits).
  - `x & 0xFF` keeps only the **lowest 8 bits** (one byte) of `x` and clears everything above it.

  - **AA extraction**:

```text
mant       = 0x00AABBCC
mant >> 16 = 0x000000AA
(mant >> 16) & 0xFF = 0xAA   // keep only the lowest byte
```

  - **BB extraction**:

```text
mant      = 0x00AABBCC
mant >> 8 = 0x0000AABB
(mant >> 8) & 0x000000FF = 0x000000BB
=> (mant >> 8) & 0xFF = 0xBB    // keep only the lowest byte
```

  - **CC extraction**:

```text
mant = 0x00AABBCC
mant & 0x000000FF = 0x000000CC
=> mant & 0xFF = 0xCC           // keep only the lowest byte
```

**How it connects**

- The whitepaper describes “leading zero bits” as an intuition for “hard to find, easy to verify”. The target comparison is the concrete deployed mechanism. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

### 4.2 Mining loop (nonce scan)

```rust
pub fn mine(mut header: BlockHeader) -> (BlockHeader, [u8; 32]) {
    let target = bits_to_target(header.difficulty_bits);
    loop {
        let h = block_hash(&header);
        if meets_target(h, target) {
            return (header, h);
        }
        header.nonce = header.nonce.wrapping_add(1);
    }
}
```

**What this code is doing**

- Computes the target from nBits once.
- Iterates the `nonce` field and re-hashes the 80-byte header until a hash under target is found.
- Returns both the valid header and its hash (so callers can store/relay it without recomputing immediately).

**Why it works**

- Finding a valid nonce is probabilistic; verification is one hash + one compare.
- This is exactly the “scan for a value … verified by executing a single hash” property described in the whitepaper. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Takeaway: PoW is a verifiable cost that makes rewriting history expensive.

---

<div align="center">

**[← Timestamp server (Bitcoin Whitepaper Section 3)](03-Timestamp-server-block-header-chaining-Bitcoin-Whitepaper-Section-3.md)** | Proof-of-work (Bitcoin Whitepaper Section 4) | **[nBits / Target Expansion (Math + Bytes) →](nBits-Target-Expansion.md)**

</div>
