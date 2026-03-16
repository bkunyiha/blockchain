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
## nBits / Target Expansion (Math + Bytes)

This is a focused deep-dive that supports the Proof-of-Work section: how Bitcoin’s compact difficulty encoding (`nBits`, also called `difficulty_bits`) expands into the full 256-bit PoW target, and why the exponent can be read as “the expanded target’s byte-width” (in big-endian, ignoring leading `0x00` bytes).

This is not a property of SHA-256 itself; it is a property of the **compact encoding definition**.

---

## 1. What is `nBits`?

Bitcoin stores the proof-of-work threshold (“target”) in the block header as a 4-byte field commonly called:

- `nBits` (Bitcoin Core naming), or
- `difficulty_bits` (our struct field name)

It is a compact “scientific notation” style encoding:

- **exponent**: 1 byte (8 bits)
- **mantissa**: 3 bytes (24 bits)

Bit layout inside the 4-byte value:

```text
[ exponent: 8 bits ][ mantissa: 24 bits ]
```

---

## 2. The definition of the expansion

This encoding comes from the **Bitcoin protocol’s choice of a compact encoding** for the PoW target in the 4-byte header field `nBits` (`difficulty_bits`).

In this section, we interpret `nBits` as base-256 “scientific notation”:

- **`exp` (1 byte)**: tells us the expanded target’s *scale* (how many bytes wide the expanded number should be).
- **`mantissa` (3 bytes)**: the top 3 significant bytes (“digits”) of the target.

The protocol-defined expansion rule is:

```text
target = mantissa * 256^(exp-15)
```

Why `exp-3`?

- The mantissa already provides **3 bytes**.
- If the full expanded number should be **`exp` bytes wide**, we need **`exp - 3` more bytes** after the mantissa.
- Those extra bytes are zeros (they are what “scaling up” looks like in base-256), so we multiply by `256^(exp-15)`.
- In short, multiplying by 256 appends one `00` byte, so multiplying by 256^(exp-15) appends exactly exp-15 zero bytes.

Quick byte example (contrived, for intuition):

```text
exp = 5
mantissa (3 bytes) = 12 34 56

target = mantissa * 256^(5-15)
       = 12 34 56 * 256^2
       = 12 34 56 00 00   (5 bytes total)
```

---

## 3. Mathematical justification for “`exp` is the byte-width”

The expansion rule above is a **definition**. The “proof” below uses that definition plus the fact that mantissa is a normalized 3-byte number to conclude what `exp` implies about the expanded target’s size.

Step 1 — bound the mantissa (3 bytes, normalized):

```text
256^2 <= mantissa < 256^3
```

Step 2 — apply the expansion definition:

```text
256^2 * 256^(exp-15) <= target < 256^3 * 256^(exp-15)
```

Step 3 — combine exponents:

```text
256^(exp-1) <= target < 256^exp
```

Step 4 — interpret as a byte-length statement:

- `256^(exp-1)` is the smallest number that requires **exp bytes** in base-256.
- `256^exp` is the first number that would require **exp+1 bytes**.
- Therefore, `target` occupies **exactly `exp` bytes** in its minimal big-endian representation (ignoring leading `0x00` bytes).

---

## 4. Why this is equivalent to shifting

Because `256 = 2^8` and a byte is 8 bits:

```text
256^k = (2^8)^k = 2^(8k)
```

So:

```text
target = mantissa * 256^k
       = mantissa * 2^(8k)
       = mantissa << (8k)
```

This is why you will often see the conceptual form:

```text
target = mantissa << (8 * (exp - 3))
```

---

<div align="center">

**[← Proof-of-work (Bitcoin Whitepaper Section 4)](04-Proof-of-work-Bitcoin-Whitepaper-Section-4.md)** | nBits / Target Expansion (Math + Bytes) | **[Network operation (Bitcoin Whitepaper Section 5) →](05-Network-operation-Bitcoin-Whitepaper-Section-5.md)**

</div>
