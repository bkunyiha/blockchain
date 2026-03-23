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
