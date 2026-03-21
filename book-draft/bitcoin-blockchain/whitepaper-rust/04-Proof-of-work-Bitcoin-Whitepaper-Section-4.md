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
## 4. Proof-of-work (Bitcoin Whitepaper Section 4)

The paper describes PoW as “scanning for a value that when hashed … begins with a **number of zero bits**” and implementing it by “incrementing a `nonce` in the block”. In deployed Bitcoin this is expressed as a **target threshold**: a block is valid if `block_hash <= target(bits)`. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

This section is where the whitepaper’s “**hard to find, easy to verify**” becomes a strict predicate. **Mining** is just **repeated hashing**; **verification** is one hash + one comparison.
- *One hash*: take the block header bytes (80 bytes in Bitcoin), run the PoW hash function (conceptually “double‑SHA256” in Bitcoin) to get a 32‑byte hash.
- *One comparison*: check whether that hash is ≤ the target threshold derived from the difficulty (nBits).

### 16 `nBits` (difficulty_bits) → target: expand compact encoding and check `block_hash <= target`
For more on `nBits`, see: **nBits → target: why exponent is byte-length**.

- **Why it’s “hard to find, easy to verify”**:
  - mining: vary the header (usually just `nonce`) and hash repeatedly until `meets_target(...)` is true
  - verification: compute one hash and do one comparison
- **Probability intuition**: if SHA-256 outputs are uniform, then $P[\text{hash} \le \text{target}] \approx \text{target}/2^{256}$. Halving the target roughly doubles the expected work.

```rust
/// Expand nBits (4-byte compact difficulty) to target (32-byte big-endian).
pub fn bits_to_target(difficulty_bits: u32) -> [u8; 32] {
    let exp = (difficulty_bits >> 24) as u8;
    // Exponent: bytes to shift
    let mant = difficulty_bits & 0x00FF_FFFF;
    // Mantissa: 3-byte value
    let mut target = [0u8; 32];

    if mant == 0 {
        return target;
    }

    if exp <= 3 {
        let shift_bytes = (3 - exp) as u32;
        let m = (mant >> (8 * shift_bytes)) as u32;
        target[28..32].copy_from_slice(&m.to_be_bytes());
        return target;
    }

    let start = 32usize.saturating_sub(exp as usize);
    if start + 3 <= 32 {
        target[start] = ((mant >> 16) & 0xFF) as u8;
        target[start + 1] = ((mant >> 8) & 0xFF) as u8;
        target[start + 2] = (mant & 0xFF) as u8;
    }
    target
}
```

After expanding the compact target format, the validation function checks whether a mined block hash falls below the target threshold:

```rust
pub fn meets_target(block_hash: [u8; 32], target_be: [u8; 32]) -> bool {
    let mut h = block_hash;
    h.reverse();
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
  - The compact rule is `target = mantissa * 256^(exp-15)`. If `exp <= 3`, then `(exp-15)` is **0 or negative**, which means we are not “growing” the mantissa; we are keeping it the same size (`exp=3`) or **shrinking** it (`exp<3`).

```text
exp = 3: exp-15 =  0  => target = mantissa * 256^0  = mantissa       (no shift)
exp = 2: exp-15 = -1  => target = mantissa / 256    (shift right 1 byte)
exp = 1: exp-15 = -2  => target = mantissa / 256^2  (shift right 2 bytes)
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

exp = 2 => shift_bytes = 1 => mant >> 8  = 0x0000AABB  (keeps AA BB)
exp = 1 => shift_bytes = 2 => mant >> 16 = 0x000000AA  (keeps AA)
```

These two steps implement the “divide by 256^(3-exp)” idea to shrink the 3-byte mantissa down to an `exp`-byte value.

#### In the normal case (`exp > 3`)
```rust
    let start = 32usize.saturating_sub(exp as usize); // start = 32 - exp
    if start + 3 <= 32 {
        // Write 3 mantissa bytes
        // For a more detailed explanation, see `Worked byte example (how the
        // mantissa bytes get placed)` below.
        // write the first mantissa byte (the most-significant of the 3) into
        // target[start]
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

### 17 Mining loop (nonce scan)

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

**[← Timestamp server (Bitcoin Whitepaper Section 3)](03-Timestamp-server-block-header-chaining-Bitcoin-Whitepaper-Section-3.md)** | Proof-of-work (Bitcoin Whitepaper Section 4) | **[nBits / Target Expansion (Math + Bytes) →](04A-nBits-Target-Expansion.md)**

</div>
