<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. <a href="../../ci/kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

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
target = mantissa * 256^(exp-3)
```

Why `exp-3`?

- The mantissa already provides **3 bytes**.
- If the full expanded number should be **`exp` bytes wide**, we need **`exp - 3` more bytes** after the mantissa.
- Those extra bytes are zeros (they are what “scaling up” looks like in base-256), so we multiply by `256^(exp-3)`.
- In short, multiplying by 256 appends one `00` byte, so multiplying by 256^(exp-3) appends exactly exp-3 zero bytes.

Quick byte example (contrived, for intuition):

```text
exp = 5
mantissa (3 bytes) = 12 34 56

target = mantissa * 256^(5-3)
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
256^2 * 256^(exp-3) <= target < 256^3 * 256^(exp-3)
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
