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
## 2. Transactions = chain of signatures (Bitcoin Whitepaper Section 2)

The paper defines an electronic coin as “a chain of digital signatures” and describes each transfer as “signing a hash of the previous transaction and the public key of the next owner”. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

We can model that directly, but we have to be precise about *what is being hashed* and *what is being verified*.

In practice, implementing transactions means building three consensus-critical layers that must all line up:
- the **bytes** we produce, 
- the **hashes** we compute from them, and 
- the **authorization checks** we run against prior outputs.
If any of these layers differ between nodes, we don’t just disagree on “formatting” — we disagree on txids, Merkle roots, signature validity, and ultimately which blocks are valid.

In this section we’ll build three layers:

- **Hashing primitive (`sha256d`)**:
  - **What it does**: turns an arbitrary byte string into a stable 32-byte digest (`[u8; 32]`).
  - **Where it’s used**: txids, Merkle nodes/roots, and block hashes all ultimately depend on this (directly or indirectly).
  - **Why it matters**: if two nodes hash different bytes (or hash differently), they will disagree on identifiers and therefore on consensus-critical commitments.
- **Ownership conditions (scripts/signatures)**:
  - **What it does**: encodes “who is allowed to spend this output?” (`script_pubkey`) and “proof that we are allowed to spend it” (`script_sig` / witness) as consensus-serialized byte programs.
  - **Where it’s used**: validation ties each `TxIn` to the referenced `TxOut` by checking unlocking data against the locking condition.
  - **Why it matters**: Bitcoin is not account-based; authorization lives at the input/output boundary, so getting this representation wrong breaks spend validation.
- **Canonical serialization (the bytes we hash/sign)**:
  - **What it does**: defines the exact byte layout for `Transaction` and `BlockHeader` (ordering, endianness, and length-prefixing with CompactSize).
  - **Where it’s used**: txids are hashes of serialized transactions; signatures are computed over a specific serialized form; Merkle roots and PoW commit to hashes derived from these bytes.
  - **Why it matters**: consensus is “agree on bytes”. Deterministic serialization is what makes independent Rust implementations interoperate.

### 6 Hashing primitive (Bitcoin uses double-SHA256)

The paper explicitly calls out SHA-256 for proof-of-work (“hashed, such as with SHA-256”). The deployed Bitcoin protocol commonly uses **double-SHA256** (apply SHA-256 twice) for txids, block hashes, and Merkle nodes. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

Why we need this layer (and why it’s worth understanding):

- **Stable identifiers**: txids and block hashes are not “names” stored in a database; they are computed from bytes. If our hashing is wrong, our node will compute different IDs than the network.
- **Commitments**: Merkle roots and PoW are commitments built on these hashes. One wrong hash means we disagree on what a block commits to and whether its proof-of-work is valid.

```rust
use sha2::{Digest, Sha256};

pub fn sha256d(data: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(data);
    let second = Sha256::digest(&first);
    second.into()
}
```

**What this code is doing**

- `Sha256::digest(data)` computes a 32-byte SHA-256 digest of `data`.
- `sha256d` applies SHA-256 twice (“double-SHA256”) and returns the final 32 bytes as `[u8; 32]`.

**Why it matters (connection to the rest of the system)**

- `txid = sha256d(serialize_tx(...))`
- `merkle_node = sha256d(left || right)`
- `block_hash = sha256d(serialize_header_80B(...))`

If your hashing differs from your peers, you will disagree on txids, Merkle roots, and PoW validity.

**Library note (`sha2`)**

- The `sha2` crate implements SHA-2 (SHA-256/SHA-512).
- The `Digest` trait provides the convenience `digest(...)` API that returns a fixed-size output.

### 7 Script and signatures (real-world representation of “chain of signatures”)

The paper phrases **ownership** as “**signing a hash of the previous transaction** and the **public key of the next owner**”. In Bitcoin as implemented, that turns into:

Why we need this layer (and why it’s worth understanding):

- **Authorization is local to each input**: a `TxIn` is valid only relative to the specific `TxOut` it references (its lock). This is why spends require prevout lookup + script checking, not just “check a signature somewhere”.
- **Rust implementation**: we must treat scripts and signatures as **raw, consensus bytes** (e.g. `Vec<u8>`), not as “structured objects” we freely reformat. Nodes validate by hashing and executing **exact serialized bytes**; if we normalize, pretty-print, re-encode, or otherwise change even one byte (different DER form, different push opcode, different script encoding), we can compute a different sighash/txid and disagree with peers. Our rule is: parse bytes, validate against consensus rules, and when we serialize we produce the **canonical encoding** deterministically.

- **`scriptPubKey` (locking script)**: defines who may spend the output (e.g., pay-to-pubkey-hash).
- **`scriptSig` (unlocking script)**: provides a signature and public key that satisfy the lock. The output locks to the new owner: the sender creates a scriptPubKey (e.g. P2PKH/P2WPKH) that commits to the new owner’s pubkey hash. That means only the holder of the matching private key can spend it later.

A minimal P2PKH-style `scriptPubKey` is:

```text
OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
```

**How to read this (what each opcode enforces)**

This is a small stack program that implements: **“to spend this output, prove you control the public key whose hash equals `<pubKeyHash>`, and provide a valid signature for this transaction.”**

- `OP_DUP`: **duplicate** the spender-provided `pubkey` (we need it twice: once to hash+compare, once to verify the signature)
- `OP_HASH160`: hash the top stack item using Bitcoin’s `HASH160` construction.
  - **What an opcode is**: Script is a small, consensus-defined bytecode language. Each token like `OP_HASH160` is a single-byte “instruction” with a fixed meaning in the Script interpreter.
  - **What `OP_HASH160` means**: by consensus definition, it computes `HASH160(x) = RIPEMD160(SHA256(x))` and replaces the top stack item with the 20-byte result. There is no “choosing” the hash at runtime; the opcode definition is the rule.
  - **Why Bitcoin bundles this**: it produces a shorter identifier (20 bytes) and reuses the standard pubkey-hash form used by P2PKH/P2WPKH outputs.
- `<pubKeyHash>`: a 20-byte constant embedded in the output (the intended recipient’s pubkey hash)
- `OP_EQUALVERIFY`: fail unless `HASH160(pubkey) == pubKeyHash` (then remove the compared items)
- `OP_CHECKSIG`: verify the spender-provided signature against the transaction’s signature-hash (“sighash”) using the provided `pubkey`

If we visualize the spend as providing `scriptSig: <sig> <pubkey>`, then after `OP_EQUALVERIFY` the stack still contains `<sig> <pubkey>`, and `OP_CHECKSIG` is the final authorization check.

In Rust, you can represent scripts as `Vec<u8>` and build them deterministically. Example (P2PKH scriptPubKey builder):
```rust
pub fn p2pkh_script_pubkey(pubkey_hash20: [u8; 20]) -> Vec<u8> {
    let mut s = Vec::with_capacity(25);
    s.push(0x76); // OP_DUP
    s.push(0xA9); // OP_HASH160
    s.push(0x14); // PUSH20
    s.extend_from_slice(&pubkey_hash20);
    s.push(0x88); // OP_EQUALVERIFY
    s.push(0xAC); // OP_CHECKSIG
    s
}
```

**What this code is doing**

- Builds the P2PKH locking script (25 bytes). In plain English: “only the holder of the private key for `<pubKeyHash>` may spend this output”.
- The script encodes *rules*, not identities. `TxOut.script_pubkey` is the output’s spend condition.

**How it connects**

- A spending `TxIn` must provide unlocking data that satisfies this script (for P2PKH: a DER-encoded ECDSA signature + the full public key).
- That unlocking data lives in `TxIn.script_sig` (legacy) or in the witness (SegWit).

**Library note (`secp256k1`)**

- Bitcoin uses ECDSA over the secp256k1 curve for signatures. The Rust `secp256k1` crate provides key types and signature verification primitives; Bitcoin’s Script rules decide *what* is signed and *where* signatures are placed.

### 8 Consensus serialization (why bytes matter)

To be “closer to real Bitcoin”, you must serialize transactions and headers in a **canonical, little-endian** way (consensus encoding). That is what txids and block hashes are computed over.

The key idea: consensus isn’t “agree on a struct”. Consensus is “**agree on an exact byte string**”. That’s why this section spends time on CompactSize and little-endian integer encoding.

CompactSize / varint (used for vector lengths and script lengths):
- the method `encode_compact_size(...)` shown bellow is called by the example serializer shown in the example that follows (**`serialize_tx_legacy(...)`**) to write:
  - the input count (`tx.inputs.len()`)
  - the output count (`tx.outputs.len()`)
  - each `scriptSig` length
  - each `scriptPubKey` length

Note: in our project codebase, `Transaction::serialize()` uses `bincode` serialization rather than Bitcoin Core-style consensus serialization in the examples bellow. The purpose of this section is to show what we would implement if/when we switch to consensus byte encoding.
```rust
pub fn encode_compact_size(n: u64, out: &mut Vec<u8>) {
    // Decoding rule (how many bytes follow the first byte):
    // 0x00..=0xFC => 0 more (value is the byte itself)
    // 0xFD => 2 more bytes (u16 LE)
    // 0xFE => 4 more bytes (u32 LE)
    // 0xFF => 8 more bytes (u64 LE)
    match n {
        // 0..=252: encode directly as a single byte
        0..=252 => out.push(n as u8),

        // 253..=0xFFFF: marker 0xFD + 2-byte little-endian u16
        253..=0xFFFF => {
            out.push(0xFD);
            out.extend_from_slice(&(n as u16).to_le_bytes());
        }

        // 0x1_0000..=0xFFFF_FFFF: marker 0xFE + 4-byte u32 LE
        0x1_0000..=0xFFFF_FFFF => {
            out.push(0xFE);
            out.extend_from_slice(&(n as u32).to_le_bytes());
        }

        // Otherwise: marker 0xFF + 8-byte u64 LE
        _ => {
            out.push(0xFF);
            out.extend_from_slice(&n.to_le_bytes());
        }
    }
}
```

**Match ranges explanation (marker bytes)**
- CompactSize is a **self-delimiting integer encoding**: the first byte either *is* the value (for small values) or it is a **marker** that tells you how many bytes follow.
- The thresholds are chosen to avoid ambiguity:

  - **What “self-delimiting” means in practice**: when decoding, you read the first byte and immediately know how many additional bytes to read (0, 2, 4, or 8). That lets the parser move through a transaction byte stream deterministically without any out-of-band length metadata.

  - **What “marker bytes” are**: a marker byte is a **reserved prefix value** that does *not* represent the number directly. Instead, it tells the decoder: “the number is stored in the next N bytes”.
    - In `CompactSize`, the marker is the *first* byte. Depending on its value, the decoder reads `0`, `2`, `4`, or `8` more bytes to get the integer.

  - **Why the thresholds avoid ambiguity**: marker bytes (`0xFD`, `0xFE`, `0xFF`) must never be confused with “a small value encoded in one byte”.
    - Example: the value **253** in decimal is `0xFD` in hex. If Bitcoin allowed “one-byte values up to 255”, then the byte `0xFD` would be ambiguous: is it the number 253, or is it the marker meaning “read the next 2 bytes”?
    - Bitcoin resolves this by defining that the one-byte form ends at `0xFC` (decimal 252). So:
      - **252** is encoded as the single byte `0xFC`
      - **253** is encoded as `0xFD 0xFD 0x00` (marker `0xFD`, then 253 as a little-endian `u16`)

    - **How to read `0xFD 0xFD 0x00`** (byte-by-byte):
      - first byte `0xFD` → “marker: read the next 2 bytes as a little-endian `u16`”
      - next two bytes are `0xFD 0x00` → that is 253 in little-endian (`0x00FD` = 253)

  - Values **0..=252** are encoded as a single byte `0x00..=0xFC` (decimal **0..=252**).
  - The byte `0xFD` (decimal **253**) is reserved as a marker meaning “the next **2 bytes** (u16 LE) contain the value”.
  - The byte `0xFE` (decimal **254**) is reserved as a marker meaning “the next **4 bytes** (u32 LE) contain the value”.
  - The byte `0xFF` (decimal **255**) is reserved as a marker meaning “the next **8 bytes** (u64 LE) contain the value”.
- The upper bounds in hex are just the maximum values of those integer widths:
  - `0xFFFF` = decimal **65,535** (max `u16`)
  - `0x1_0000` = decimal **65,536** (first value that no longer fits in `u16`)
  - `0xFFFF_FFFF` = decimal **4,294,967,295** (max `u32`)

**How `marker bytes` are used in real transaction bytes**

`CompactSize` appears anywhere the format needs a “length” or “count” that can vary:

- **Vector counts**:
  - number of inputs (`vin` count)
  - number of outputs (`vout` count)
- **Byte field lengths**:
  - `scriptSig` length (legacy)
  - `scriptPubKey` length (outputs)
  - (SegWit) witness item counts and witness item lengths

When we decode a transaction, `CompactSize` is how we know **how many things to read next**:

- If the next byte is `0x02`, that means “2” and we immediately read **2 inputs**.
- If the next byte is `0xFD`, that does *not* mean 253 inputs; it means “read the next 2 bytes to get the count”.

This is why they’re called “marker bytes”: they **mark the width** of the integer that follows, so parsing stays unambiguous and we always know where the next field begins.

**What this code is doing**

- Encodes an integer in Bitcoin’s **`CompactSize` format**.
- This is used to prefix variable-length vectors (input/output counts) and variable-length byte fields (script sizes).

**How it connects**

- Transaction serialization uses CompactSize for counts and scripts.
- Because txids and Merkle nodes hash the serialized bytes, CompactSize must be exact.

Minimal legacy transaction serialization (no SegWit. See the SegWit note below for how SegWit differs from legacy signature handling):

```rust
pub fn serialize_tx_legacy(tx: &Transaction) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&tx.version.to_le_bytes());
    encode_compact_size(tx.inputs.len() as u64, &mut out);
    for i in &tx.inputs {
        out.extend_from_slice(&i.previous_output.txid);
        out.extend_from_slice(&i.previous_output.vout.to_le_bytes());
        encode_compact_size(i.script_sig.len() as u64, &mut out);
        out.extend_from_slice(&i.script_sig);
        out.extend_from_slice(&i.sequence.to_le_bytes());
    }
    encode_compact_size(tx.outputs.len() as u64, &mut out);
    for o in &tx.outputs {
        out.extend_from_slice(&o.value.to_le_bytes());
        encode_compact_size(o.script_pubkey.len() as u64, &mut out);
        out.extend_from_slice(&o.script_pubkey);
    }
    out.extend_from_slice(&tx.lock_time.to_le_bytes());
    out
}
```

**What this code is doing**

- Produces the canonical “legacy” transaction byte layout (pre-SegWit).
- Notice how every variable-length list/field is length-prefixed with CompactSize:
  - number of inputs
  - each scriptSig length
  - number of outputs
  - each scriptPubKey length

**How it connects**

- These bytes feed directly into `txid_legacy`.
- They also feed into Merkle root computation (txid list) and therefore into the block header hash.

Then the txid (legacy) becomes:

```rust
pub fn txid_legacy(tx: &Transaction) -> [u8; 32] {
    sha256d(&serialize_tx_legacy(tx))
}
```

This is the “shared bytes” contract that makes two implementations agree on transaction identity.

**What this code is doing**

- Computes the transaction identifier as a double-SHA256 over the canonical serialized transaction bytes.

**Important nuance**

- The “hash bytes” are often displayed reversed in user interfaces (endianness convention). Internally, keep a consistent `[u8; 32]` representation and define explicit helpers for display vs compare.

**SegWit note (what it is, how it differs, and why it exists)**
SegWit (“Segregated Witness”) is a Bitcoin upgrade that changes how signature-related data is carried and committed. In legacy transactions, unlocking data lives in `scriptSig` and is part of the byte string that defines the transaction identifier. With SegWit spends, the unlocking data (the “witness”: signatures and related stack items) is moved into a separate witness structure.

- **When it was introduced**: SegWit was activated on Bitcoin mainnet in **2017** (BIP141, soft fork activation at block **481,824**, 2017-08-24).
- **How it differs from legacy serialization**:
  - **Legacy**: the serialized bytes include `scriptSig` inside each input (see the **`serialize_tx_legacy(...)`** example right above). The legacy **txid** is exactly what we computed in **`txid_legacy(...) = sha256d(serialize_tx_legacy(tx))`**.
  - **SegWit**: the transaction has a “base” serialization (still used for the legacy-visible **txid**) *plus* witness data. The witness-including identifier is the **wtxid**.
- **Why it was introduced**:
  - **Fix transaction malleability** for signature data: third parties could historically modify certain signature encodings and change the txid without changing the economic meaning. SegWit makes txid independent of witness data, which enables higher-level protocols (notably payment channels / Lightning).
  - **Increase effective capacity**: blocks use **weight**, and witness bytes are discounted compared to non-witness bytes, so more transactions fit without changing old nodes’ 1MB “base” view.
- **Practical advantages over legacy**:
  - stable txids for SegWit spends (malleability fix for signature data)
  - more efficient use of block space via the witness discount
  - modern script forms (e.g. native P2WPKH / `bc1...`) with fewer edge cases in practice

Beginner takeaway (so the next sections click):

- A transaction becomes “real” on the network only after we can express it as **canonical bytes**.
- From those bytes we compute an **ID** (`txid`), and later blocks commit to those IDs via the **Merkle root**.
- Inputs don’t contain coins; an input contains a **pointer** (`OutPoint`) to a prior output plus proof that we are allowed to spend it (script/signature data).
- The “ledger state” we check against is the **UTXO set**: if the referenced output is not in the UTXO set at our current tip, it is already spent (or never existed).

---

<div align="center">

**[← Introduction (Bitcoin Whitepaper Section 1)](01-Introduction-Bitcoin-Whitepaper-Section-1.md)** | Transactions = chain of signatures (Bitcoin Whitepaper Section 2) | **[Timestamp server (Bitcoin Whitepaper Section 3) →](03-Timestamp-server-block-header-chaining-Bitcoin-Whitepaper-Section-3.md)**

</div>
