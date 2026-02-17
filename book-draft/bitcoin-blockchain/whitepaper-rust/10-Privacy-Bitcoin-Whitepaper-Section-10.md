<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. Chapter 1: Introduction & Overview - Book introduction, project structure, technical stack
2. Chapter 1.2: Introduction to Bitcoin & Blockchain - Bitcoin and blockchain fundamentals
3. Chapter 1.3: Bitcoin Whitepaper - Bitcoin Whitepaper
4. **Chapter 1.4: Bitcoin Whitepaper In Rust** ← *You are here*
5. Chapter 2.0: Rust Blockchain Project - Blockchain Project
6. Chapter 2.1: Primitives - Core data structures
7. Chapter 2.2: Utilities - Utility functions and helpers
8. Chapter 2.3: Cryptography - Cryptographic primitives and libraries
9. Chapter 2.4: Blockchain (Technical Foundations) - Proof Of Work
10. Chapter 2.5: Storage Layer - Persistent storage implementation
11. Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5) - Proof Of Work
12. Chapter 2.7: Network Layer - Peer-to-peer networking and protocol
13. Chapter 2.8: Node Orchestration - Node context and coordination
14. Chapter 2.9: Wallet System - Wallet implementation and key management
15. Chapter 3: Web API Architecture - REST API implementation
16. Chapter 4: Desktop Admin Interface - Iced framework architecture
17. Chapter 5: Wallet User Interface - Wallet UI implementation
18. Chapter 6: Embedded Database & Persistence - SQLCipher integration
19. Chapter 7: Web Admin Interface - React/TypeScript web UI

### Part II: Deployment & Operations

20. Chapter 8: Docker Compose Deployment - Docker Compose guide
21. Chapter 9: Kubernetes Deployment - Kubernetes production guide
22. Chapter 10: Rust Language Guide - Rust programming language reference

</details>

</div>

---
## 10. Privacy (Bitcoin Whitepaper Section 10)

The whitepaper’s privacy section is about **pseudonymity**: transactions are public, but identities are not directly attached to keys. In other words, a pubkey/pubkey-hash on-chain is a cryptographic identifier; it does not contain a name, account id, or real-world metadata.

In practice, a key becomes “an identity” only when some external system links it to a person or entity (for example: an exchange deposit address tied to KYC, a merchant invoice that embeds an address, or logs that associate network activity with an address). The whitepaper’s implementation-facing recommendation is simple: use a new key pair per transaction to reduce linkability. ([Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf))

In this section, we connect that recommendation to wallet code: what leaks on-chain by default, what a “new address” really means at the `TxOut` level, and what a Rust wallet typically needs to store to reliably track and spend funds.

### 10.0 What leaks on-chain (and what we can and cannot change)

The blockchain is a public graph of outputs and spends. From an implementer’s perspective, privacy is mostly a **wallet problem**: we decide how we generate outputs, how we construct spends, and what metadata we keep off-chain.

The two biggest default privacy leaks are:

- **Address reuse**: if we reuse the same pubkey hash, observers can trivially cluster outputs as “paid to the same recipient identifier.”
- **Multi-input spends**: if we spend multiple inputs in one transaction, we create strong evidence those inputs were controlled by the same wallet (or coordinated signers).

To make those statements precise, we need one key definition:

- A `TxOut` does **not** store a “Bitcoin address string.” It stores a `scriptPubKey` (locking script bytes). An “address” is a human-friendly encoding of some data extracted from a standard `scriptPubKey` template.
  - For **P2PKH**, the output commits to a 20-byte pubkey hash (not the pubkey itself): `OP_DUP OP_HASH160 <pubKeyHash20> OP_EQUALVERIFY OP_CHECKSIG`.
  - For **P2WPKH (v0)**, the output commits to the same 20-byte pubkey hash inside a witness program: `0 <pubKeyHash20>`.

From here, we can answer the practical developer questions that come up when implementing “new key per transaction”:

- **Do we keep changing the script type?** Usually no. We typically choose an output policy (for example: “we use P2WPKH for new outputs on mainnet”) and keep it consistent. Switching script types is not what the whitepaper means by “new key per transaction,” and it does not provide the same privacy benefit as issuing fresh keys.
- **So how do we get a “new address” each time?** We generate a new keypair (new public key) for each payment—typically by deriving the next key from a single wallet seed. New derived pubkey ⇒ new pubkey hash ⇒ new output identifier.
- **Do future transactions use the new public key?** Yes. When we hand out a new address, the wallet has generated a new pubkey and then built a new output `scriptPubKey` for our chosen script type. When we later spend that output, we provide the corresponding public key and a signature that satisfies the prior output’s locking script (for SegWit, this data lives in the witness).
- **How are old transactions still “tied” to the wallet after we start using new keys?** We do not replace keys; we add new derived keys over time and keep the ability to re-derive and recognize the old ones. Old outputs remain locked by their original `scriptPubKey`. The wallet remains able to spend them because it still controls (or can re-derive) the corresponding private keys, and it can scan/match `scriptPubKey`s during sync/restore.

### 10.1 “New key per transaction” as an API design

In our wallet layer, we can make “address rotation” the default by making the caller ask for scripts, not keys. That keeps privacy decisions in one place and makes it harder to accidentally reuse addresses across the application.

```rust
pub trait AddressBook {
    /// Returns a brand-new *receive* `scriptPubKey` for an external payment.
    /// Wallet policy: do not reuse this script for multiple payers/invoices.
    fn fresh_receive_script(&mut self) -> Vec<u8>;

    /// Returns a brand-new *change* `scriptPubKey` for our own “change” output.
    /// Wallet policy: never send change back to a reused receive script.
    fn fresh_change_script(&mut self) -> Vec<u8>;
}
```

In this chapter, `AddressBook` is the minimal “wallet address management” surface:

- **What it does**: it issues fresh `scriptPubKey` bytes for two purposes: receiving funds (external) and making change (internal).
- **Issuance vs recognition**: it does not only issue new scripts; it must also recognize previously issued scripts (or re-derive them from the seed + indexes) so we can detect “this UTXO is ours” during sync/restore.
- **What it stores**: at minimum, it needs key derivation material (a seed/master key), derivation state (`receive_index` / `change_index`), and a lookup map from generated script/address → derivation metadata.
- **Why `&mut self`?** Both methods advance counters/state. If these functions were pure, we would risk re-issuing the same script again (address reuse).

Where do we store this state? `AddressBook` is the API; a real wallet backs it with persistent storage so it survives restarts. A common design is a key-value database (SQL also works). Typical records include:

- `last_receive_index` / `last_change_index`
- `scriptPubKey` (or its address encoding) → `{ kind: Receive|Change, index: u32, label/invoice_id, first_seen_height }`

What the methods do in a real wallet:

- **`fresh_receive_script`**:
  - used when the wallet needs to present a “receive address” to the outside world (UI/API): e.g., create a new invoice, display a QR code, or return an address for a payer to send funds to
  - should derive the next “external/receive” key, build a standard `scriptPubKey` (e.g., P2WPKH), persist the updated index, and record it for later scanning
- **`fresh_change_script`**:
  - used by the **spender’s wallet** (the party constructing/signing the transaction) when it needs to create a change output
  - should use a *separate change address pool* (a separate counter/index sequence) from the receive address pool:
    - **receive addresses** are shown to other people/services to pay us
    - **change addresses** are used only by us for “return to self” outputs
    Keeping them separate prevents accidental reuse (e.g., sending change back to an address that was previously shared on an invoice).
  - should be **fresh** (new) each time:
    - if we reuse the same change script/address repeatedly, observers can spot “this output looks like change” and then cluster many transactions as belonging to the same wallet
    - fresh change scripts reduce that linkability, even though change detection heuristics may still exist

Operational note — how wallets “discover” their own UTXOs (the gap limit):

When we sync (or restore from a seed), we need to figure out which on-chain outputs belong to us. We do that by regenerating the same receive/change scripts we previously handed out and searching for matches.

Given our seed + derivation rules, we can generate candidate receive scripts:
  - index 0 → receive script 0
  - index 1 → receive script 1
  - index 2 → receive script 2
  - … and so on
  The wallet then looks for UTXOs whose `scriptPubKey` matches any of these scripts (“ours”).

We cannot scan infinitely many future addresses, so wallets use a gap limit rule:
  - keep deriving/checking scripts in order until we have seen **N consecutive unused** scripts
  - then stop, assuming we have not “jumped ahead” past the last-used address

Concrete example (gap limit = 3):

- **What are these indexes?** They are *wallet derivation indexes* (address counters), not blockchain indexes. Think “the 0th receive address we generated from our seed,” “the 1st receive address,” etc.
- **What does “used” mean?** “Used” means the wallet finds at least one matching on-chain output/UTXO paying to the script derived at that index.

- Suppose we check receive indexes in order: 0, 1, 2, 3, 4, 5, …
- We find that index 0 and 1 have been used (they received coins), but index 2, 3, 4 are unused.
- After seeing **3 unused in a row** (2, 3, 4), we stop scanning at 4.

- **Why stop at 3?** The number 3 here is just the example gap limit \(N\). The rule is: stop after \(N\) consecutive unused derived scripts so scanning is bounded. Real wallets typically choose a larger \(N\); the exact value is a wallet policy/implementation detail.

This is one reason an `AddressBook` needs persistent indexes/state: if we “hand out” index 50 but later only scan until we see “3 unused in a row,” we might stop early and fail to notice a payment sent to that far-ahead script.

Implementation detail (how we actually generate “fresh” keys/scripts):

- In practice, wallets do not store a pile of unrelated random keys. They use a single seed and derive many keys (HD-style), then increment an index/counter for each new receive/change address.
- Even if we do not implement full HD derivation in this book, we still want the same *shape*:
  - a persistent `receive_index` and `change_index`
  - deterministic “derive key -> build scriptPubKey”

Rust-shaped sketch (pseudocode; the derivation method is implementation-defined):

```rust
pub struct Wallet {
    pub receive_index: u32,
    pub change_index: u32,
    // seed / master key material would live here
}

impl Wallet {
    pub fn fresh_receive_script(&mut self) -> Vec<u8> {
        let i = self.receive_index;
        self.receive_index += 1;
        // 1) Derive a NEW receive public key from our wallet seed + the next receive index.
        //    (Deterministic derivation; same seed + same index => same pubkey.)
        // let pubkey = derive_receive_pubkey(i);
        //
        // 2) Convert the pubkey to the identifier that the output script commits to.
        //    For P2WPKH / P2PKH-style outputs, this is typically HASH160(pubkey) (20 bytes).
        // let pubkey_hash20 = hash160(&pubkey.serialize());
        //
        // 3) Build the locking script (`scriptPubKey`) bytes that go into TxOut.
        //    Example (P2WPKH v0): 0 <20-byte pubkey hash>
        // let script_pubkey: Vec<u8> = build_p2wpkh_scriptpubkey(pubkey_hash20);
        //
        // 4) Return the raw `scriptPubKey` bytes. A UI may additionally encode this as a
        //    human-friendly address string (e.g., bech32 `bc1...`), but TxOut stores bytes.
        //
        // NOTE: We return `vec![]` here because this chapter is illustrating architecture
        // and responsibilities; the concrete script-building functions are implementation work.
        vec![] // placeholder for `script_pubkey`
    }

    pub fn fresh_change_script(&mut self) -> Vec<u8> {
        let i = self.change_index;
        self.change_index += 1;
        // Same idea as `fresh_receive_script`, but from the CHANGE address pool:
        //
        // - Receive addresses are shared externally (invoices/QR codes).
        // - Change addresses are internal “return to self” destinations.
        //
        // 1) Derive a NEW change public key from the seed + the next change index.
        // let pubkey = derive_change_pubkey(i);
        //
        // 2) Hash/encode it into whatever your chosen script type commits to.
        // let pubkey_hash20 = hash160(&pubkey.serialize());
        //
        // 3) Build the `scriptPubKey` bytes for the change TxOut (e.g., P2WPKH).
        // let script_pubkey: Vec<u8> = build_p2wpkh_scriptpubkey(pubkey_hash20);
        //
        // 4) Return the raw `scriptPubKey` bytes used for the change output.
        //
        // NOTE: Placeholder for the same reason as above.
        vec![] // placeholder for `script_pubkey`
    }
}
```

### 10.2 Practical privacy tradeoffs in code

This section is where privacy becomes engineering: we choose policies that trade off linkability, fees, and UX. As we implement transaction construction, we should treat privacy as a first-class constraint, not an afterthought.

- When we select inputs, we balance:
  - **fee efficiency** (fewer inputs = smaller tx)
  - **privacy** (fewer mixed inputs reduces clustering)
- When we create change, we should send it to a **new change script**, not reuse the sender’s original address.

Additional implementation guidance (common wallet pitfalls):

- **Always treat change as “a new address”**: change output reuse is one of the most common linkability leaks.
- **Avoid unnecessary multi-input spends**: combining many inputs reveals common ownership; coin selection strategy is therefore part of privacy.
- **Do not log sensitive wallet mapping** in production (e.g., “address -> user id”): logs can destroy pseudonymity even if the chain data is unlinkable.

Takeaway: privacy is not a single opcode; it is a set of wallet and policy choices built on top of the public UTXO graph.

---

<div align="center">

**[← Combining/splitting value (Bitcoin Whitepaper Section 9)](09-Combining-splitting-value-Bitcoin-Whitepaper-Section-9.md)** | Privacy (Bitcoin Whitepaper Section 10) | **[Confirmations and attacker probability (Bitcoin Whitepaper Section 11) →](11-Confirmations-and-attacker-probability-Bitcoin-Whitepaper-Section-11.md)**

</div>
