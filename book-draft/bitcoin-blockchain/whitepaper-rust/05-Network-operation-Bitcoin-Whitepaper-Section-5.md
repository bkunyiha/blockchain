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
## 5. Network operation (Bitcoin Whitepaper Section 5)

The paper’s([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf)) network loop is explicit and short (6 steps): 
- broadcast txs, 
- collect into blocks, 
- PoW, 
- broadcast blocks, 
- validate, 
- extend on accepted blocks.

In practice, Section 5 becomes two core pipelines in our node:

- **Transaction pipeline** (mempool admission)
- **Block pipeline** (block acceptance + chainstate update)

Network highlevel intuition:

- The network is mostly **gossip**: peers forward transactions and blocks to each other.
- Our node keeps two key “views” of the world:
  - **mempool**: “valid transactions we have seen that are not confirmed yet”
  - **chainstate (UTXO set at our best tip)**: “the spendable outputs according to the best chain we currently believe”
- Forks happen naturally (two miners can extend the same tip). A **reorg** is just updating our best tip and recomputing the UTXO set by undoing/redoing block state transitions.

### 5.1 Transaction pipeline (mempool admission)

What we must decide for each incoming transaction:

- **Is it well-formed?** (non-empty inputs/outputs, sizes within limits)
- **Is it authorized?** (signatures satisfy the locking scripts)
- **Is it valid economically?** (sum(inputs) ≥ sum(outputs), no negative/overflow)
- **Is it spendable right now?** (“not already spent” against your current UTXO view)

In Rust, we can model that as a pure function returning a typed error:

```rust
pub fn validate_tx(/* tx, utxo_view, policy */) -> Result<(), TxError> {
    // 1) structure
    // 2) lookup prevouts from UTXO (OutPoint -> TxOut)
    // 3) verify scripts/signatures using those prevouts as context
    // 4) value conservation + fees
    // 5) mark conflicts (double-spends) at the mempool layer
    Ok(())
}
```

Note on “policy” vs “consensus” (helpful for new readers):

- **Consensus rules** are mandatory: if we get them wrong, we accept invalid blocks or reject valid ones (we fork ourselves off the network).
- **Policy rules** are local preferences (mempool rules): they decide what we relay/mine, but they do not change what blocks are *valid*.

### 5.2 Block pipeline (the whitepaper’s “Step 5” in code)

Before we talk about “accepting a block”, we need a key storage mental model that might not be obvious:

- **The blockchain data (blocks/transactions) is stored as an append-only history** (often in a “block store”).
- **The UTXO set is stored separately as a fast key-value database** (“chainstate”), because validating spends requires lots of random lookups by `OutPoint(txid, vout)`.

So when we say “a block is accepted” we really mean two related things happen:

- **History**: we store/track the block as part of the chain’s history (header + transactions).
- **State**: we apply the block’s transactions as an atomic update to the UTXO database:
  - remove spent outpoints (inputs)
  - insert newly created outpoints (outputs)

This separation is why nodes can even support pruning: an implementation may delete old block bodies from disk (history) while still staying fully validating for new blocks using the current **UTXO database** (state) plus the **header chain** for PoW and linkage.

The whitepaper’s Step 5 is the consensus-critical sentence:
“Nodes accept the block only if all transactions in it are valid and not already spent.” ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Concretely, our block acceptance path needs to:

- **Validate the header**:
  - links to a known `prev_hash` (or is a candidate for later)
  - meets PoW target (`difficulty_bits → target`, `block_hash <= target`)
- **Validate the transaction set**:
  - Merkle root matches the tx list (commitment is consistent)
  - first tx is coinbase; coinbase amount obeys rules (simplified if needed)
  - every other tx:
    - spends only existing UTXOs at the parent tip
    - does not double-spend within the same block
    - authorizes each spend (script/sig verifies against referenced `TxOut.script_pubkey`)
- **Connect the block** (state transition):
  - spend every input outpoint (remove from UTXO)
  - add every output as a new outpoint (insert into UTXO)
  - do this **atomically** per-block so partial failure can’t corrupt state

Another way to think about Step 5:

- We treat the block as a proposed batch update to the UTXO set.
- We run validation first against a read-only UTXO view (so “not already spent” is a pure check).
- Only after everything is valid do we apply the writes (spend/remove + create/insert) as one state transition.

If we want a deeper walkthrough of Step 5 (including common missing checks and a Rust-shaped validation design), see:
`book-draft/bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md`.

Rust implementation suggestion: encode those steps as explicit functions with clear boundaries:

```rust
pub fn on_receive_transaction(/* tx */) { /* validate; add to mempool; relay */ }
pub fn make_block_template(/* mempool */) { /* choose txs; compute merkle root; build header */ }
pub fn on_receive_block(/* block */) { /* validate; update best chain; relay */ }
```

### 5.3 P2P message flow and wire encoding (implementation sketch)

The whitepaper describes “broadcast” at a high level. In real Bitcoin, “broadcast” means we send and receive **typed messages** over TCP. Two practical pieces matter for implementation:

- **Message types**: `inv`, `getdata`, `tx`, `block`, `headers`, etc.
- **Wire encoding**: a fixed header + a payload (byte-for-byte)

Definitions

- **`inv`**: “Inventory: ie I have these objects.” A peer advertises txids / block hashes so other peers can request them.
- **`getdata`**: “Send me these objects.” A peer requests full data for a previously-advertised tx or block.
- **`tx`**: “Transaction: Here is a full transaction.” Payload is the consensus-serialized `Transaction`.
- **`block`**: “Here is a full block.” Payload is the `BlockHeader` + the full transaction list.
- **`headers`**: “Here are block headers only.” Used to sync the best chain tip efficiently without downloading full blocks up front.

Brief decription of the “business objects” being moved around (what they are *for*):

- **`Transaction`**: the unit of “value movement” we relay. It spends earlier outputs and creates new outputs.
  - **Used for**: updating ledger state (UTXO set) once included in a valid block.
- **`BlockHeader`**: the small, fixed-size commitment we hash for proof-of-work (`prev_hash`, `merkle_root`, `nBits`, `nonce`, …).
  - **Used for**: chain linkage and PoW verification (cheap to relay and validate).
- **`Block`**: `BlockHeader + Vec<Transaction>`.
  - **Used for**: applying a batch state transition (validate txs, then update the UTXO set atomically).
- **`InventoryItem`**: `(kind, hash)` where hash is a `txid` or block hash.
  - **Used for**: lightweight gossip. We advertise *identifiers* first (`inv`) and only download full objects when needed (`getdata`).

Relationship (big picture):

```text
inv/getdata: advertise/request ids (txid / block hash)
tx:          carries a full Transaction
headers:     carries BlockHeader(s) (cheap chain sync)
block:       carries a full Block = header + transactions
```

Flow (from “first seen on the network” to “block persisted”):

1. **We hear about a new object** via `inv`:
   - peer sends an `inv` containing `(kind=Tx, hash=txid)` or `(kind=Block, hash=block_hash)`
2. **We request the full data** with `getdata` (if we want/need it):
   - for a txid → request `tx`
   - for a block hash → request `block` (or sometimes request `headers` first)
3. **We receive and validate a `tx`**:
   - parse bytes → verify signatures/scripts against referenced prevouts → check “not already spent” against our current UTXO view
   - if valid → put it in the **mempool** and relay an `inv` to other peers
4. **We sync chain progress efficiently with `headers`**:
   - download/validate header chain (prev-hash linkage + PoW) without downloading full blocks yet
5. **We receive and validate a `block`**:
   - validate header (links + PoW target)
   - validate tx list (Merkle root matches; coinbase rules; every spend is authorized and unspent; no in-block double spends)
6. **We persist and “connect” the block** (this is the moment it becomes part of our local chain view):
   - **block store (history)**: write the block/header to disk (append-only history)
   - **chainstate / UTXO DB (state)**: apply the block atomically (remove spent outpoints, insert created outpoints)
7. **We announce it onward**:
   - relay `inv(kind=Block, hash=block_hash)` so other peers can request it

Rust-shaped message objects (simplified):

```rust
pub enum InventoryItemKind {
    Tx,
    Block,
}

pub struct InventoryItem {
    pub kind: InventoryItemKind,
    pub hash: [u8; 32], // txid or block hash
}

// "inventory" payload: count + inventory vectors
pub struct InvMessage {
    pub items: Vec<InventoryItem>,
}

// "getdata" payload: same shape as inv, but interpreted as a request
pub struct GetDataMessage {
    pub items: Vec<InventoryItem>,
}

// "tx" payload: a full consensus transaction
pub struct TxMessage {
    pub tx: Transaction,
}

// "block" payload: a full consensus block
pub struct BlockMessage {
    pub block: Block,
}

// "headers" payload: headers-only synchronization
pub struct HeadersMessage {
    pub headers: Vec<BlockHeader>,
}

pub enum Message {
    Inv(InvMessage),
    GetData(GetDataMessage),
    Tx(TxMessage),
    Block(BlockMessage),
    Headers(HeadersMessage),
}
```
### Processing P2P messages: a Rust dispatcher we can build on

In a Bitcoin node, networking is not “magic broadcast” — it is a loop that receives bytes from peers, parses them into a typed `Message`, and then runs one central dispatcher that decides what to do next.

In this section, we demonstrate that dispatcher as a small Rust `match` over our `Message` enum. This is the point where we connect the wire protocol to the node’s core subsystems:

- **gossip (`inv`/`getdata`)**: we learn what a peer has (by id), decide what we are missing, request the full objects we need, and serve objects when peers request them.
- **mempool admission (`tx`)**: we decide whether an unconfirmed transaction is valid under our *current* chain tip (UTXO view). If it is valid, we store it in the mempool and relay it.
- **chain sync + chain selection (`headers`/`block`)**: we advance our view of the best chain by validating headers (linkage + PoW) and then validating full blocks; if a block is accepted, we persist it and update chainstate.

Conceptually:

- `Node` represents “everything our node knows and manages” (its **state**):
  - **mempool**: our current set of validated-but-unconfirmed transactions
  - **chainstate / UTXO database**: our current spendable-output view at the best tip (what enforces “not already spent”)
  - **block store + header index**: our persisted chain history (headers, and usually blocks)
  - **peer/request tracking** (often overlooked): what we have already requested, what we have already announced, and what each peer claims to have (to avoid redundant downloads/relays)
- `handle_message(&mut Node, Message)` is the top-level dispatcher that turns “a parsed message” into the next concrete work:
  - **state changes** (local): insert into mempool, connect a block (UTXO updates), persist headers/blocks, update per-peer request state
  - **network follow-ups** (outbound): send `getdata` after `inv`, send `inv` after accepting a tx/block, serve `tx`/`block` when peers request them

```rust
pub struct Node {
    // Conceptual node state (what this dispatcher(ie handle_message) is coordinating):
    //
    // - mempool:
    //   - what it stores: validated, unconfirmed transactions (often indexed by txid)
    //   - what it is used for: relay transactions; select transactions when we build a block template
    //
    // - chainstate / UTXO view:
    //   - what it stores: the current spendable outputs at our best tip
    //     (keyed by OutPoint(txid, vout) -> TxOut(value, script_pubkey))
    //   - what it is used for: answer “not already spent?” and provide the prevout context for script verification
    //
    // - block store (history) + header index:
    //   - what it stores: block headers and (optionally) full block bodies, indexed by block hash
    //   - what it is used for: persist accepted blocks, serve blocks to peers, and support reorg/chain traversal
}

pub fn handle_message(node: &mut Node, msg: Message) {
    match msg {
        Message::Inv(inv) => {
            // Peer advertises txids/block hashes it has.
            // We decide what we are missing, then request it with getdata.
            // node.request_missing(inv.items);
        }
        Message::GetData(req) => {
            // Peer requests full objects we advertised earlier.
            // We look up tx/block by hash and respond with tx/block.
            // node.respond_with_objects(req.items);
        }
        Message::Tx(tx_msg) => {
            // Validate tx against our current UTXO view; if valid, put in mempool and re-advertise.
            // node.accept_tx(tx_msg.tx);
        }
        Message::Headers(hdrs) => {
            // Validate/attach headers (prev-hash linkage + PoW); may trigger block downloads.
            // node.accept_headers(hdrs.headers);
        }
        Message::Block(blk_msg) => {
            // Validate block; if accepted, persist to block store and update chainstate atomically.
            // node.accept_block(blk_msg.block);
        }
    }
}
```

Encoding note: in deployed Bitcoin, vectors in P2P messages are length-prefixed with **CompactSize**, and hashes are fixed 32-byte values.
Sample payload encoding (Rust-shaped, simplified):

```rust
pub trait Encode {
    fn encode(&self, out: &mut Vec<u8>);
}

fn put_u32_le(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_bytes(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(b);
}

fn put_compact_size(out: &mut Vec<u8>, n: usize) {
    // uses the CompactSize rules explained earlier in this chapter
    out.extend_from_slice(&encode_compact_size(n as u64));
}

impl Encode for InventoryItem {
    fn encode(&self, out: &mut Vec<u8>) {
        // In Bitcoin P2P, "type" is a u32 (e.g. MSG_TX=1, MSG_BLOCK=2). We keep it symbolic here.
        let ty: u32 = match self.kind {
            InventoryItemKind::Tx => 1,
            InventoryItemKind::Block => 2,
        };
        put_u32_le(out, ty);
        put_bytes(out, &self.hash); // 32 bytes
    }
}

pub fn encode_inv(items: &[InventoryItem]) -> Vec<u8> {
    let mut out = Vec::new();
    put_compact_size(&mut out, items.len());
    for it in items {
        it.encode(&mut out);
    }
    out
}

pub fn encode_getdata(items: &[InventoryItem]) -> Vec<u8> {
    encode_inv(items) // same payload shape as inv: count + inventory vectors
}
```

### 5.4 Wire message header (why “agree on bytes” applies to networking too)

Bitcoin’s P2P messages are framed as:

- **header** (fixed size): network magic + command + payload length + checksum
- **payload** (variable): message-specific bytes

Rust encoding sketch (simplified, but byte-oriented):

```rust
pub struct MessageHeader {
    pub magic: u32,        // network identifier (mainnet/testnet)
    pub command: [u8; 12], // ASCII, NUL-padded (e.g. "inv\0\0\0\0\0\0\0\0\0")
    pub length: u32,       // payload length in bytes
    pub checksum: [u8; 4], // first 4 bytes of sha256d(payload)
}

pub fn checksum4(payload: &[u8]) -> [u8; 4] {
    let h = sha256d(payload);
    [h[0], h[1], h[2], h[3]]
}
```

One level higher: the *on-the-wire* unit is “header + payload”:

```rust
pub struct NetMessage {
    pub header: MessageHeader,
    pub payload: Message,
}
```

This is why “encoding” matters beyond hashes: if we serialize message frames differently, peers cannot parse our messages, and we won’t participate in the gossip network described by the whitepaper.

Takeaway: networking is propagation; consensus is validation + chain selection.

---

<div align="center">

**[← nBits / Target Expansion (Math + Bytes)](nBits-Target-Expansion.md)** | Network operation (Bitcoin Whitepaper Section 5) | **[Incentive mechanism (Bitcoin Whitepaper Section 6) →](06-Incentive-mechanism-Bitcoin-Whitepaper-Section-6.md)**

</div>
