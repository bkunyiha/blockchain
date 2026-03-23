# Revised Text — Chapters 11–15
## Specific Changes for Implementation

This file contains the exact text replacements for all issues found in the review. Each section shows the file path, the original text, and the revised text.

---

## Chapter 11: Storage Layer

### Issue 11.1: Add database terminology analogy

**File**: `/bitcoin-blockchain/store/README.md`

**Original** (line 81):
```
Our conventions on top of sled:

- a **blocks tree** that maps `block_hash -> serialized Block bytes`
- a stable **tip key** (`"tip_block_hash"`) that points at the canonical tip hash
- atomic updates for "insert block + move tip" (sled transactions)
```

**Revised**:
```
Our conventions on top of sled:

- a **blocks tree** that maps `block_hash -> serialized Block bytes`
- a stable **tip key** (`"tip_block_hash"`) that points at the canonical tip hash
- atomic updates for "insert block + move tip" (sled transactions)

Think of sled as a persistent in-memory data structure (like a `BTreeMap`) that writes changes to disk atomically. If the process crashes, the database either contains both the block and the updated tip, or neither — it never gets stuck in an inconsistent state.
```

---

### Issue 11.2: Fix breadcrumb chapter numbering

**File**: `/bitcoin-blockchain/store/01-Storage-Layer-Code-Walkthrough.md`

**Original** (line 550–551):
```
<div align="center">

**[← Chapter 11: Storage Layer](README.md)** | **Chapter 11.A: Code Walkthrough** | **[Next: Chapter 12 (Network Layer) →](../net/README.md)**

</div>
```

**Revised**:
```
<div align="center">

**[← Chapter 11: Storage Layer](README.md)** | **Chapter 11.A: Storage Layer — Code Walkthrough** | **[Next: Chapter 12: Network Layer →](../net/README.md)**

</div>
```

---

## Chapter 12: Network Layer

### Issue 12.1: Add TCP/JSON pipeline clarification

**File**: `/bitcoin-blockchain/net/README.md`

**Original** (lines 62–64):
```
This chapter explains the network layer as an implementer reads it in Rust: **as a pipeline of concrete methods** that transform bytes on a TCP stream into node actions (mempool admission, block download, block connection).
```

**Revised**:
```
This chapter explains the network layer as an implementer reads it in Rust: **as a pipeline of concrete methods** that transform bytes on a TCP stream into node actions (mempool admission, block download, block connection). The pipeline is simple: convert byte sequences from the TCP stream into JSON, deserialize JSON into typed Rust messages (the `Package` enum), then dispatch each message to the appropriate handler (mempool, chainstate, relay, etc.).
```

---

### Issue 12.2: Explain "gossip + fetch" pattern

**File**: `/bitcoin-blockchain/net/README.md`

**Original** (lines 99–122):
```
## Diagram: the minimal protocol loop in this implementation

```text
Peer A has object (tx or block)
  |
  | 1) announce (hash only)
  v
INV(op_type, [id]) -----> Peer B
                             |
                             | 2) request bytes
                             v
                          GETDATA -----> Peer A
                                           |
                                           | 3) send full bytes
                                           v
                                  (TX | BLOCK) -----> Peer B
                                                        |
                                                        | 4) hand to node
                                                        v
                                                 mempool / add_block
```

This loop is the core of the "gossip + fetch" strategy used throughout Bitcoin-like systems.
```

**Revised**:
```
## Diagram: the minimal protocol loop in this implementation

Bitcoin uses a **gossip and fetch** strategy to minimize bandwidth: peers announce what they have by sharing only the hash (the gossip phase), peers request the full object by hash (the fetch phase), and peers deliver the full object. This three-message pattern—announce, request, deliver—is the heartbeat of blockchain peer-to-peer communication.

```text
Peer A has object (tx or block)
  |
  | 1) announce (hash only)
  v
INV(op_type, [id]) -----> Peer B
                             |
                             | 2) request bytes
                             v
                          GETDATA -----> Peer A
                                           |
                                           | 3) send full bytes
                                           v
                                  (TX | BLOCK) -----> Peer B
                                                        |
                                                        | 4) hand to node
                                                        v
                                                 mempool / add_block
```

This loop is the core of the gossip and fetch strategy used throughout Bitcoin-like systems.
```

---

### Issue 12.3: Clarify appendix file naming

**File**: `/bitcoin-blockchain/net/README.md`

**Original** (lines 140–143):
```
An additional technical appendix explains the transport trade-offs and an actionable migration plan:

- **[Appendix: `std::net::TcpStream` vs `tokio::net::TcpStream`](02-Std-vs-Tokio-TcpStream.md)**
```

**Revised**:
```
An additional technical section explains the transport trade-offs and migration considerations:

- **[Chapter 12.B: `std::net::TcpStream` vs `tokio::net::TcpStream`](02-Std-vs-Tokio-TcpStream.md)** — Trade-offs between standard library and Tokio TCP streams, with a migration path for upgrading from blocking I/O to async.
```

---

## Chapter 13: Node Orchestration

### Issue 13.1: Reference stale mining protection

**File**: `/bitcoin-blockchain/node/README.md`

**Original** (lines 85–91):
```
At runtime, several subsystems must work together:

- **Network** receives bytes and produces typed messages (`Package`)
- **NodeContext** decides what subsystem should handle the message (mempool, chainstate, mining)
- **Chainstate** persists blocks and updates the canonical tip + UTXO
- **Mempool** stores unconfirmed transactions and marks outputs as "in mempool"
- **Mining** turns mempool contents into a block and relays the new tip
- **Peers** is the node's peer set used for relay
```

**Revised**:
```
At runtime, several subsystems must work together:

- **Network** receives bytes and produces typed messages (`Package`)
- **NodeContext** decides what subsystem should handle the message (mempool, chainstate, mining)
- **Chainstate** persists blocks and updates the canonical tip + UTXO
- **Mempool** stores unconfirmed transactions and marks outputs as "in mempool"
- **Mining** turns mempool contents into a block and relays the new tip (with stale mining protection — see Chapter 11 for details)
- **Peers** is the node's peer set used for relay
```

---

### Issue 13.2: Move checkpoint before bottom navigation

**File**: `/bitcoin-blockchain/node/README.md`

**Original**: Checkpoint appears at lines 143–144, after the bottom breadcrumbs.

**Revised**: Move the checkpoint block from after line 145 to immediately before the `<div align="center">` at line 136. Final structure:

```
In the next chapter, we build the wallet system — key generation, address derivation, and transaction signing — that gives users the ability to hold and spend cryptocurrency.

---

> **Checkpoint:** This is a major milestone — the node can now run as a standalone process. Start it with `cargo run -p bitcoin` and you should see log output showing the node binding to a port and waiting for peer connections. If you have two terminals, start a second instance pointed at the first and verify that they exchange version handshakes and synchronize chain state.

---

<div align="center">
```

---

## Chapter 14: Wallet System

### Issue 14.1: Add note about custom address format

**File**: `/bitcoin-blockchain/wallet/README.md`

**Original** (lines 109–118):
```
## Diagram: address payload structure used here

This wallet implementation uses a payload that matches the "classic" Base58Check idea (version + data + checksum), but note the version byte and hashing choices are specific to our implementation.

```text
payload bytes:
  [ version: 1 byte ] [ pub_key_hash: N bytes ] [ checksum: 4 bytes ]

encoded as:
  Base58(payload)
```
```

**Revised**:
```
## Diagram: address payload structure used here

This wallet implementation uses a payload that matches the "classic" Base58Check idea (version + data + checksum), but note the version byte and hashing choices are specific to our implementation. This is intentional for teaching purposes; Bitcoin's actual address format is slightly different.

```text
payload bytes:
  [ version: 1 byte ] [ pub_key_hash: N bytes ] [ checksum: 4 bytes ]

encoded as:
  Base58(payload)
```

Note: This is a teaching implementation optimized for clarity. Bitcoin uses a different address version byte and hashing convention. For a complete understanding of Bitcoin's address format, see BIP-141 (Segregated Witness) and the further reading section below.
```

---

## Chapter 15: Web API Architecture

### Issue 15.1: Fix breadcrumb chapter references (README.md)

**File**: `/bitcoin-blockchain/web/README.md`

**Original** (line 64):
```
**[← Chapter 14: Wallet](../wallet/README.md)** | **Chapter 15: Web API Architecture** | **[Chapter 16: Desktop Admin UI (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

**Revised**:
```
**[← Chapter 14: Wallet System](../wallet/README.md)** | **Chapter 15: Web API Architecture** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

---

### Issue 15.2: Clarify compression middleware in request flow

**File**: `/bitcoin-blockchain/web/01-Introduction.md`

**Original** (lines 196–199):
```
3. Compression Middleware
   - Decompresses request body if client sent compressed data (Content-Encoding)
   - Prepares to compress response (actual compression happens after handler)
```

**Revised**:
```
3. Compression Middleware
   - Decompresses incoming request body if client sent `Content-Encoding: gzip` or similar header
   - Prepares to compress outgoing response if client supports it (via `Accept-Encoding` header)
```

---

### Issue 15.3: Add context for why Utoipa matters

**File**: `/bitcoin-blockchain/web/01-Introduction.md`

**Original** (lines 177–179):
```
### 5. Automatic Documentation

OpenAPI/Swagger documentation is automatically generated from code, ensuring documentation stays synchronized with implementation. The documentation includes request/response schemas, endpoint descriptions, and interactive testing capabilities.
```

**Revised**:
```
### 5. Automatic Documentation

OpenAPI/Swagger documentation is automatically generated from code, ensuring documentation stays synchronized with implementation—when you change a handler signature, the API spec updates automatically. This is especially valuable for learning and API testing: the Swagger UI at `/api-docs` (discussed in detail in Chapter 15.9) lets you interactively test endpoints without writing curl commands. The documentation includes request/response schemas, endpoint descriptions, and example payloads.
```

---

### Issue 15.4: Fix breadcrumb in Axum.md

**File**: `/bitcoin-blockchain/web/Axum.md`

**Original** (line 64):
```
**[← Web Index](README.md)** | **Axum Framework Guide** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

**Revised**:
```
**[← Web Index](README.md)** | **Axum Framework Guide** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

---

### Issue 15.5: Fix breadcrumb in 02-Server-Setup.md

**File**: `/bitcoin-blockchain/web/02-Server-Setup.md`

**Original** (line 72):
```
**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.2: Server Setup** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

**Revised**:
```
**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.2: Server Setup** | **[Next: Chapter 16: Desktop Admin (Iced) →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

---

### Issue 15.6: Fix breadcrumb in 01-Introduction.md

**File**: `/bitcoin-blockchain/web/01-Introduction.md`

**Original** (line 72):
```
**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.1: Introduction** | **[Chapter 4: Desktop Admin UI →](../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md)**
```

**Revised**:
```
**[← Chapter 15: Web API Architecture](README.md)** | **Chapter 15.1: Introduction** | **[Next: Chapter 15.2: Server Setup →](02-Server-Setup.md)**
```

---

## Implementation Notes

**All Changes Are Non-Breaking**: Each revision maintains the same logical flow and chapter structure. No sections are added or removed; only text is clarified or corrected.

**Navigation Changes Are Safe**: The breadcrumb fixes align with the canonical chapter numbering in BOOK-CONTEXT.md (Chapters 1–24 in Part I–III structure).

**Pedagogical Improvements**: The clarity additions (Issues 12.1, 12.2, 15.3) follow the book's documented tone (senior engineer at whiteboard) and add one or two sentences of context per issue.

---

## Summary of Changes

| File | Change Type | Issue Count |
|------|-------------|-------------|
| store/README.md | Clarity | 1 |
| store/01-Storage-Layer-Code-Walkthrough.md | Formatting | 1 |
| net/README.md | Clarity + Formatting | 3 |
| node/README.md | Clarity + Structure | 2 |
| wallet/README.md | Clarity | 1 |
| web/README.md | Navigation | 1 |
| web/01-Introduction.md | Navigation + Clarity | 2 |
| web/02-Server-Setup.md | Navigation | 1 |
| web/Axum.md | Navigation | 1 |

**Total Files Modified**: 9
**Total Changes**: 14
**All changes are low-risk and editorial in nature.**

