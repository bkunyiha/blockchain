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
