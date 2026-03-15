<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. **Chapter 2.7: Network Layer** ← *You are here*
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

# Network Layer — Code Walkthrough (Network Message Pipeline)

**Part I: Core Blockchain Implementation** | **Chapter 2.7.A: Network Layer — Code Walkthrough**

This chapter is the **code-first network walkthrough**: it prints and explains the concrete methods that implement our Rust Bitcoin implementation’s P2P message pipeline.

If you want a *whitepaper-first* explanation of Section 5, read:

- **[Whitepaper §5 (Network Operation) — Rust Walkthrough](../whitepaper-rust/05-Network-operation-Bitcoin-Whitepaper-Section-5.md)**

Two promises:

1. **Repo not required**: every method we reference is printed in full, in this chapter, unless it is explicitly marked as “defined earlier” and we point you to the earlier listing.
2. **No signature-only snippets**: when we mention a function as part of the runtime, we show the whole method body.

> Whitepaper source: [Bitcoin: A Peer-to-Peer Electronic Cash System — Section 5](https://bitcoin.org/bitcoin.pdf)

---

## Context: Whitepaper §5 vs our Rust Bitcoin implementation’s network message loop

The Bitcoin Whitepaper §5 describes a six-step loop:

1. New transactions are broadcast.
2. Nodes collect transactions into a block.
3. Nodes work on proof-of-work.
4. A node that finds proof-of-work broadcasts the block.
5. Nodes accept the block only if all transactions are valid and not already spent.
6. Nodes express acceptance by working on the next block.

In our Rust Bitcoin implementation, those steps manifest as **message boundaries** and **method boundaries** in the P2P protocol implementation:

> **Methods involved**
>
> - `process_stream(...)` (the inbound router)
> - `send_inv(...)` + `send_get_data(...)` (announce + request)
> - `send_tx(...)` + `send_block(...)` (deliver full bytes)
> - `send_version(...)` + `process_known_nodes(...)` (handshake + peer discovery)

- Messages: `Inv`, `GetData`, `Tx`, `Block`, `Version`, `KnownNodes`
- Dispatcher: `process_stream(...)` reads a TCP stream and routes each inbound message
- Send primitives: `send_data(...)` and wrappers (`send_inv`, `send_get_data`, `send_tx`, `send_block`, …)

The heart of the networking code you will read is the INV/GETDATA loop:

```
announce by id (INV) → request bytes (GETDATA) → deliver bytes (TX/BLOCK)
```

---

## Diagram: the message pipeline and the ownership boundary

The network layer should be “dumb” in the right way: it is responsible for **routing**, not for “being the blockchain”.

> **Methods involved**
>
> - `Server::run_with_shutdown(...)` (accept + spawn per connection)
> - `process_stream(...)` (decode + dispatch)
> - `send_data(...)` and wrappers (`send_inv`, `send_get_data`, `send_block`, `send_tx`, `send_version`)

```
TCP stream bytes
  |
  | JSON (serde_json) decoding
  v
Package enum value
  |
  | dispatch by variant
  v
Network actions (request/relay)  +  Node actions (mempool/add_block)

Network-only:
  - send_inv / send_get_data / send_tx / send_block / send_version

Delegated to node/chain:
  - NodeContext::process_transaction(...)
  - NodeContext::add_block(...)
  - NodeContext::get_block_hashes(...) / get_block(...)
```

When you read `process_stream`, always ask: “is this the network doing routing, or the node doing consensus/state?”

---

## Method index (everything we will print in full)

**Message model (data types)**
- `ConnectNode` + `FromStr` + helper methods
- `OpType`
- `MessageType`
- `AdminNodeQueryType`
- `Package`

**Server loop**
- `Server::run_with_shutdown(...)`

**Network dispatcher**
- `process_stream(...)`

**Send primitives**
- `send_get_data(...)`
- `send_inv(...)`
- `send_block(...)`
- `send_tx(...)`
- `send_known_nodes(...)`
- `send_version(...)`
- `send_get_blocks(...)`
- `send_message(...)`
- `send_data(...)`

**Peer discovery**
- `process_known_nodes(...)`

All code listings are copied from the repository paths shown in each listing header.

---

## 1. Message model (what can be sent on the wire)

Before reading any runtime flow, we need to understand the **wire-level vocabulary**: what messages exist, and what fields they carry.

> **Methods involved**
>
> - `ConnectNode::is_remote`
> - `ConnectNode::get_addr`
> - `impl FromStr for ConnectNode`
> - `enum OpType`
> - `enum MessageType`
> - `enum AdminNodeQueryType`
> - `enum Package`

### Code Listing 2.7A-0.1 — Network globals + message model (`bitcoin/src/node/server.rs`)

```rust
use crate::net::net_processing;
use crate::net::net_processing::{send_known_nodes, send_version};
use crate::node::NodeContext;
use crate::{BlockInTransit, MemoryPool, Nodes};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing::{error, info, instrument};

// This file defines the network “envelope types” (`Package`, `OpType`, etc.) plus a few
// global singletons used across the node (peer list, mempool, blocks-in-transit).

// The node’s protocol version for the handshake (`Package::Version`).
// Peers compare versions / heights to decide whether to request blocks.
pub const NODE_VERSION: usize = 1;

// The “central node” is our Rust Bitcoin implementation’s bootstrap mechanism:
// - non-central nodes connect to it and announce themselves with `Version`
// - central node replies with `KnownNodes` so peers learn each other
//
// NOTE: The identifier is spelled `CENTERAL_NODE` in code; we keep it as-is.
pub static CENTERAL_NODE: Lazy<SocketAddr> = Lazy::new(|| {
    // Read `CENTERAL_NODE` from the environment; default to localhost:2001.
    let central_node_str =
        env::var("CENTERAL_NODE").unwrap_or_else(|_| "127.0.0.1:2001".to_string());

    // Handle empty string case (when CENTERAL_NODE is set but empty)
    if central_node_str.is_empty() {
        "127.0.0.1:2001"
            .parse()
            .expect("Failed to parse default CENTERAL_NODE address")
    } else {
        // Parse the env var into a SocketAddr ("ip:port").
        central_node_str
            .parse()
            .expect("CENTERAL_NODE environment variable is not a valid socket address")
    }
});

// When the mempool reaches this threshold, miners may start assembling a block.
// (The “why” lives in the miner chapter; here it’s just a constant used by node logic.)
pub const TRANSACTION_THRESHOLD: usize = 3;

// Global peer set (simple in our Rust Bitcoin implementation; no complex peer manager).
// It is pre-seeded with the central node so everyone has at least one rendezvous point.
pub static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();

    nodes.add_node(*CENTERAL_NODE).expect("Node add error");
    nodes
});

/// The `GLOBAL_MEMORY_POOL` is a lazy static variable that holds a `MemoryPool` instance.
/// It is used to store transactions that are in the memory pool.
pub static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> = Lazy::new(MemoryPool::new);

/// The `GLOBAL_BLOCKS_IN_TRANSIT` is a lazy static variable that holds a `BlockInTransit` instance.
/// It is used to store blocks that are in transit between nodes.
pub static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> = Lazy::new(BlockInTransit::new);

// Safety valve for outbound writes (milliseconds). If a peer is slow or dead,
// we prefer timing out and evicting rather than hanging the node.
pub const TCP_WRITE_TIMEOUT: u64 = 1000;

// A small “address selector” used by CLI/config: either "local" (central) or a specific remote.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ConnectNode {
    Local,
    Remote(SocketAddr),
}

impl ConnectNode {
    // Helper for filters: is this a real remote peer?
    pub fn is_remote(&self) -> bool {
        matches!(self, ConnectNode::Remote(_))
    }

    // Convert the selector into a concrete address.
    // `Local` means “talk to the central node address”.
    pub fn get_addr(&self) -> SocketAddr {
        match self {
            ConnectNode::Remote(addr) => *addr,
            ConnectNode::Local => *CENTERAL_NODE,
        }
    }
}

impl FromStr for ConnectNode {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        // Parse CLI/config strings:
        // - "local" -> central node
        // - otherwise parse as SocketAddr
        if s == "local" {
            Ok(ConnectNode::Local)
        } else {
            let ip_addr = s.parse()?;
            Ok(ConnectNode::Remote(ip_addr))
        }
    }
}

// Discriminator used by inventory messages and GETDATA:
// it tells the receiver what kind of object the hash refers to.
#[derive(Debug, Serialize, Deserialize)]
pub enum OpType {
    Tx,
    Block,
}

// Generic “human readable” message channel for the protocol (errors, warnings, etc.).
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Error,
    Success,
    Info,
    Warning,
    Ack,
}

// “Admin” messages are not part of Bitcoin’s P2P protocol; they exist to support
// our Rust Bitcoin implementation’s debugging / demo flows (query balance, mine empty block, etc.).
#[derive(Debug, Serialize, Deserialize)]
pub enum AdminNodeQueryType {
    GetBalance { wlt_address: String },
    GetAllTransactions,
    GetBlockHeight,
    MineEmptyBlock,
    ReindexUtxo,
}

// The network wire model: every JSON message is one `Package` value.
// The receiver deserializes a stream into a sequence of packages.
#[derive(Debug, Serialize, Deserialize)]
pub enum Package {
    Block {
        addr_from: SocketAddr,
        block: Vec<u8>,
    },
    GetBlocks {
        addr_from: SocketAddr,
    },
    GetData {
        addr_from: SocketAddr,
        op_type: OpType,
        id: Vec<u8>,
    },
    Inv {
        addr_from: SocketAddr,
        op_type: OpType,
        items: Vec<Vec<u8>>,
    },
    Tx {
        addr_from: SocketAddr,
        transaction: Vec<u8>,
    },
    SendBitCoin {
        addr_from: SocketAddr,
        wlt_frm_addr: String,
        wlt_to_addr: String,
        amount: i32,
    },
    KnownNodes {
        addr_from: SocketAddr,
        nodes: Vec<SocketAddr>,
    },
    Version {
        addr_from: SocketAddr,
        version: usize,
        best_height: usize,
    },
    Message {
        addr_from: SocketAddr,
        message_type: MessageType,
        message: String,
    },
    AdminNodeQuery {
        addr_from: SocketAddr,
        query_type: AdminNodeQueryType,
    },
}
```

---

## 2. TCP server loop (how a node accepts connections)

This is the runtime entry point for inbound peer connections. It binds a TCP listener, does initial bootstrap, and then spawns one task per accepted connection.

> **Methods involved**
>
> - `Server::run_with_shutdown(...)`
> - `net_processing::process_stream(...)` (printed in the next section)
> - `send_version(...)` (printed later)
> - `send_known_nodes(...)` (printed later)

### Code Listing 2.7A-1.1 — TCP accept loop + bootstrap (`bitcoin/src/node/server.rs`)

```rust
#[derive(Debug, Clone)]
pub struct Server {
    node_context: NodeContext,
}

impl Server {
    // Create a new TCP server wrapper around a clonable `NodeContext`.
    pub fn new(blockchain: NodeContext) -> Server {
        Server {
            node_context: blockchain,
        }
    }

    #[instrument(skip(self, addrs, connect_nodes, shutdown))]
    pub async fn run_with_shutdown(
        &self,
        addrs: &SocketAddr,
        connect_nodes: HashSet<ConnectNode>,
        mut shutdown: tokio::sync::broadcast::Receiver<()>,
    ) {
        // Bind the TCP listener. This is the node’s “inbound port” for peer connections.
        let listener = TcpListener::bind(addrs)
            .await
            .expect("TcpListener bind error");
        info!(
            "Server listening on {:?}",
            listener.local_addr().expect("TcpListener local_addr error")
        );

        // If the node is not the central node, send the version message to the central node.
        if !addrs.eq(&CENTERAL_NODE) {
            // Non-central node boot:
            // - ask our chainstate for height
            // - tell central node “I exist” + “this is my best height”
            let best_height = self
                .node_context
                .get_blockchain_height()
                .await
                .expect("Blockchain read error");
            send_version(&CENTERAL_NODE, best_height).await;
        } else {
            info!("Register with node {:?}", connect_nodes);
            // Add the connect node to the global nodes set.

            let remote_nodes: HashSet<SocketAddr> = connect_nodes
                .iter()
                .filter(|node| node.is_remote())
                .map(|node| node.get_addr())
                .collect();

            GLOBAL_NODES
                .add_nodes(remote_nodes.clone())
                .expect("Global nodes add error");

            for remote_node in remote_nodes {
                // Central node boot:
                // fan out our current peer list to each explicitly configured remote node.
                send_known_nodes(
                    &remote_node,
                    GLOBAL_NODES
                        .get_nodes()
                        .expect("Global nodes get error")
                        .iter()
                        .map(|node| node.get_addr())
                        .collect(),
                )
                .await;
            }
        }

        // Serve incoming connections with graceful shutdown.
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Network server shutdown signal received");
                    break;
                }
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, _peer)) => {
                            // Each accepted stream gets handled in its own async task.
                            // This keeps the accept loop responsive even if one peer is slow.
                            let blockchain = self.node_context.clone();
                            tokio::spawn(async move {
                                // Convert tokio stream to std stream for existing processing code
                                match stream.into_std() {
                                    Ok(std_stream) => {
                                        // The message processing code uses `std::net::TcpStream`
                                        // and blocking reads via `BufReader`, so we disable nonblocking mode.
                                        let _ = std_stream.set_nonblocking(false);
                                        if let Err(e) = net_processing::process_stream(blockchain, std_stream).await {
                                            error!("Serve error: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to convert stream: {}", e);
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            error!("accept error: {}", e);
                        }
                    }
                }
            }
        }
    }
}
```

---

## 3. Stream dispatcher: `process_stream` (the message router)

This is the heart of the network layer: it takes a TCP stream and iterates JSON-decoded `Package` values. Each package variant maps to a concrete action.

You will see calls out to node logic (mempool admission, block acceptance, admin queries). That is intentional: the network’s job is to **route messages to the right subsystem**, not to re-implement those subsystems here.

### Diagram: dispatch table (what each `Package` variant triggers)

```
Package::Version        -> height compare -> (send_get_blocks | send_version) + register peer
Package::KnownNodes     -> process_known_nodes
Package::GetBlocks      -> get_block_hashes -> send_inv(op=Block, items=[hashes])
Package::Inv(op=Block)  -> add blocks-in-transit -> send_get_data(op=Block, id=first_hash)
Package::GetData(Block) -> get_block(id) -> send_block
Package::Block          -> add_block + remove txs from mempool + (maybe) send_get_data(next block)

Package::Inv(op=Tx)     -> if missing locally -> send_get_data(op=Tx, id=txid)
Package::GetData(Tx)    -> mempool lookup -> send_tx
Package::Tx             -> process_transaction (mempool ingress + relay + maybe mine)
```

> **Methods involved**
>
> **Defined in this chapter (full listings below)**
> - `process_stream(...)`
> - `send_get_data(...)`
> - `send_inv(...)`
> - `send_block(...)`
> - `send_tx(...)`
> - `send_get_blocks(...)`
> - `send_version(...)`
> - `send_known_nodes(...)`
> - `send_message(...)`
> - `process_known_nodes(...)`
>
> **Defined earlier**
> - `NodeContext::process_transaction(...)` (defined in **[Chapter 2.8: Node Orchestration](../node/README.md)**)
> - `NodeContext::add_block(...)` (acceptance contract: **[Chapter 2.6: Block Acceptance](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)**)
> - `NodeContext::get_block_hashes(...)`, `NodeContext::get_block(...)` (defined in **[Chapter 2.8: Node Orchestration](../node/README.md)** and chain chapters)

### Code Listing 2.7A-2.1 — Full dispatcher (`bitcoin/src/net/net_processing.rs`)

```rust
// NOTE: Full method body printed verbatim.
// It is long by design: this is the “router” that makes the runtime readable without the repo open.

#[instrument(skip(node_context, stream))]
pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    // Note:
    // This function is the “network router”. It:
    // 1) reads a TCP stream,
    // 2) deserializes a sequence of JSON `Package` values,
    // 3) dispatches each package to the appropriate node/network action.

    // peer_addr is the address of the peer that is sending the request.
    let peer_addr = stream.peer_addr()?;
    // Wrap the stream in a buffered reader (more efficient than reading byte-by-byte).
    let reader = BufReader::new(&stream);

    // `serde_json::Deserializer` can stream-decode JSON values from a reader.
    // Here, each JSON object on the stream is decoded into a `Package`.
    let pkg_reader = Deserializer::from_reader(reader).into_iter::<Package>();

    // The `serve` function processes incoming network requests from a TCP stream.
    // It handles different types of packages, including blocks, transactions, and version information.
    // The function processes each package based on its type and performs the appropriate actions.
    // It also manages the block in transit set and the memory pool to ensure proper synchronization
    // and validation of the blockchain.
    // The function returns an error if the stream cannot be read or if the package cannot be deserialized.
    // It also shuts down the stream after processing the package.
    // Iterate over the deserialized packages from the stream.
    for pkg in pkg_reader {
        // If JSON parsing fails, propagate the error (the connection is not well-formed).
        let pkg = pkg?;
        info!("Receive request from {}: {:?}", peer_addr, pkg);

        match pkg {
            // When a node receives a block, it adds it to the blockchain and sends a request for the next block.
            // It deserializes the block and adds it to the blockchain.
            // If there are blocks in transit, it sends a get_data request for the next block.
            // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
            Package::Block { addr_from, block } => {
                // `Package::Block` contains serialized bytes. Convert bytes -> `Block`.
                let block =
                    Block::deserialize(block.as_slice()).expect("Block deserialization error");
                // If the block is not the best block, do nothing
                // `add_block` will not add the block if its height is less than current tip height in the block chain.
                //
                // This is the ownership boundary:
                // - network hands the candidate block to node/chain logic
                // - chain logic validates, potentially reorgs, and updates state
                node_context
                    .add_block(&block)
                    .await
                    .expect("Blockchain write error");
                let added_block_hash = block.get_hash_bytes();
                info!("Added block {:?}", added_block_hash.as_slice());

                // Remove transactions in block from memory pool functionally, since they have already been mined by other nodes
                for tx in block.get_transactions().await? {
                    node_context.remove_from_memory_pool(tx.clone()).await;
                }

                // The add_block() method already handles UTXO updates internally through the reorganization process.
                // Calling update_utxo_set() here would cause double UTXO updates, leading to multiple SUBSIDY rewards.
                // This was the root cause of the consensus mechanism allowing all nodes to keep their SUBSIDY.

                let removed_block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                    .remove(added_block_hash.as_ref())
                    .expect("Block removal error");
                if removed_block_hash.is_some() {
                    info!(
                        "Removed block {:?} FROM GLOBAL_BLOCKS_IN_TRANSIT",
                        removed_block_hash.expect("Block removal error").as_slice()
                    );
                }

                // If there are blocks in transit, it sends a get_data request for the next block.
                // It removes the block from the blocks in transit set when it is added to the blockchain when
                // it is receives Package::Inv message{OpType::Block, items: [block_hash]}
                // If there are no more blocks in transit, it reindexes the UTXO set of the blockchain.
                if GLOBAL_BLOCKS_IN_TRANSIT
                    .is_not_empty()
                    .expect("Blocks in transit error")
                {
                    // Simple “download queue”: request the next block hash we still want.
                    let block_hash = GLOBAL_BLOCKS_IN_TRANSIT
                        .first()
                        .expect("Blocks in transit error")
                        .expect("Blocks in transit error");
                    send_get_data(&addr_from, OpType::Block, &block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
            }
            // Retrieves all block hashes from the blockchain and sends an
            // inv message with a list of hashes to the requesting peer.
            Package::GetBlocks { addr_from } => {
                // Peer asks “what blocks do you have?”. We answer with an INV list of hashes.
                let blocks = node_context
                    .get_block_hashes()
                    .await
                    .expect("Blockchain read error");
                // Send an inv message with a list of hashes to the requesting peer.
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
            // Retrieves the requested block or transaction from the blockchain
            // or the global memory pool and sends it back to the requesting peer.
            Package::GetData {
                addr_from,
                op_type,
                id,
            } => match op_type {
                // When a node receives a block, it adds it to the blockchain and sends a request for the next block.
                OpType::Block => {
                    // Peer asks for a full block by hash.
                    if let Some(block) = node_context
                        .get_block(id.as_slice())
                        .await
                        .expect("Blockchain read error")
                    {
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    // Peer asks for a full transaction by txid.
                    // Mempool is keyed by hex string, so we convert bytes -> hex.
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    if let Some(tx) = GLOBAL_MEMORY_POOL
                        .get(txid_hex.as_str())
                        .expect("Memory pool get error")
                    {
                        send_tx(&addr_from, &tx).await;
                    } else {
                        info!("Received request to forward a Transaction that is not found in memory pool. 
                        Most likely it has been mined!!!: {:?}", txid_hex);
                    }
                }
            },
            // Adds the received blocks or transactions to the global blocks in transit
            // or the memory pool and requests missing blocks or transactions via get_data if necessary.
            Package::Inv {
                addr_from,
                op_type,
                items,
            } => match op_type {
                // When a node receives a block, it adds it to the blocks in transit set and sends a request for the first block.
                OpType::Block => {
                    // Peer announces “I have these block hashes”.
                    // We enqueue them, then immediately request the first block.
                    GLOBAL_BLOCKS_IN_TRANSIT
                        .add_blocks(items.as_slice())
                        .expect("Blocks in transit add error");

                    let block_hash = items.first().expect("Blocks in transit add error");
                    send_get_data(&addr_from, OpType::Block, block_hash).await;

                    //GLOBAL_BLOCKS_IN_TRANSIT.remove(block_hash.as_slice());
                }
                // When a node receives a transaction, it adds it to the memory pool and sends a request for the transaction.
                OpType::Tx => {
                    // Peer announces “I have this txid”. If we don’t, request the full tx bytes.
                    let txid = items.first().expect("Blocks in transit add error");
                    let txid_hex = HEXLOWER.encode(txid);

                    if !GLOBAL_MEMORY_POOL
                        .contains(txid_hex.as_str())
                        .expect("Memory pool contains error")
                    {
                        send_get_data(&addr_from, OpType::Tx, txid).await;
                    }
                }
            },
            // deserializes the transaction and adds it to the global memory pool.
            // If the node is a miner and the memory pool has reached a certain threshold,
            // it creates a new block containing transactions from the memory pool, mines it,
            // and broadcasts the new block to other nodes via inv.
            Package::Tx {
                addr_from,
                transaction,
            } => {
                // Full transaction bytes arrived. Deserialize and hand to node logic.
                let tx = Transaction::deserialize(transaction.as_slice())
                    .expect("Transaction deserialization error");
                // CPU intensive operation.
                // It will create a new transaction and add it to the memory pool.
                // It will also broadcast the transaction to all other nodes.
                // It will also mine a new block if the memory pool has reached a certain threshold.
                match node_context.process_transaction(&addr_from, tx).await {
                    Ok(_) => (),
                    Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid)) => {
                        // Use the generic `Message` channel to tell the sender what went wrong.
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            format!("Transaction: {} already exists", txid),
                        )
                        .await;
                    }
                    Err(e) => Err(e)?,
                }
            }

            // CPU intensive operation.
            // It will create a new transaction and add it to the memory pool.
            // It will also broadcast the transaction to all other nodes.
            // It will also mine a new block if the memory pool has reached a certain threshold.
            Package::SendBitCoin {
                addr_from,
                wlt_frm_addr,
                wlt_to_addr,
                amount,
            } => {
                // This is an application-level “send coins” command (not a Bitcoin P2P message).
                // It validates addresses, constructs a transaction, and then falls back into the normal
                // transaction pipeline (mempool + relay + maybe mine).
                let validated_wlt_frm_addr = WalletAddress::validate(wlt_frm_addr);
                let validated_wlt_to_addr = WalletAddress::validate(wlt_to_addr);

                match (validated_wlt_frm_addr, validated_wlt_to_addr) {
                    (Ok(_), Err(_)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_to: ${wlt_to_addr}".to_string(),
                        )
                        .await;
                    }
                    (Err(_), Ok(_)) => {
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                        )
                        .await;
                    }
                    (Err(_), Err(_)) => {
                        let send_message_invalid_to = send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_to: ${wlt_to_addr}".to_string(),
                        );
                        let send_message_invalid_from = send_message(
                            &addr_from,
                            MessageType::Error,
                            "Invalid addr_from: ${wlt_frm_addr}".to_string(),
                        );
                        // Run both in parallel
                        tokio::join!(send_message_invalid_to, send_message_invalid_from);
                    }
                    (Ok(from), Ok(to)) => {
                        // UTXOSet is used to compute balances / select spendable outputs.
                        let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());

                        match node_context.btc_transaction(&from, &to, amount).await {
                            Ok(_) => (),
                            Err(BtcError::TransactionAlreadyExistsInMemoryPool(txid)) => {
                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!("Transaction: {} already exists", txid),
                                )
                                .await;
                            }
                            Err(BtcError::NotEnoughFunds) => {
                                // We compute the current balance to return a useful error message.
                                // Get current balance for detailed error message
                                let current_balance =
                                    utxo_set.get_balance(&from).await.unwrap_or(0);

                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!(
                                        "Insufficient funds: cannot send {} bitcoin. Current balance: {} bitcoin",
                                        amount, current_balance
                                    ),
                                )
                                .await;

                                // Log the error for debugging
                                error!(
                                    "Transaction rejected: insufficient funds. From: {}, To: {}, Amount: {}, Balance: {}",
                                    from.as_str(),
                                    to.as_str(),
                                    amount,
                                    current_balance
                                );
                            }
                            Err(e) => {
                                send_message(
                                    &addr_from,
                                    MessageType::Error,
                                    format!("Transaction creation failed: {}", e),
                                )
                                .await;

                                error!("Transaction creation failed: {}", e);
                            }
                        }
                    }
                }
            }
            Package::Version {
                addr_from,
                version,
                best_height,
            } => {
                // Handshake/coordination message. This is where basic sync begins.
                // If the peer is ahead: request block hashes. If we’re ahead: send our version/height back.
                debug!("version = {}, best_height = {}", version, best_height);
                let local_best_height = node_context
                    .get_blockchain_height()
                    .await
                    .expect("Blockchain read error");
                if local_best_height < best_height {
                    send_get_blocks(&addr_from).await;
                }
                if local_best_height > best_height {
                    send_version(
                        &addr_from,
                        node_context
                            .get_blockchain_height()
                            .await
                            .expect("Blockchain read error"),
                    )
                    .await;
                }

                // If height is the same then get the first and last block hashes for comparison

                if !GLOBAL_NODES
                    .node_is_known(&addr_from)
                    .expect("Node is known error")
                {
                    // Remember this peer so future broadcasts can include it.
                    GLOBAL_NODES.add_node(addr_from).expect("Node add error");
                }
            }
            Package::KnownNodes { addr_from, nodes } => {
                // Peer discovery list arrived — merge and fan out.
                process_known_nodes(node_context.clone(), &addr_from, nodes).await;
            }
            Package::Message {
                addr_from,
                message_type,
                message,
            } => match message_type {
                MessageType::Error => {
                    // Logging-only channel in this implementation.
                    error!("{} sent error: {}", addr_from, message);
                }
                MessageType::Warning => {
                    warn!("{} sent warning: {}", addr_from, message);
                }
                MessageType::Info => {
                    debug!("{} sent info: {}", addr_from, message);
                }
                MessageType::Success => {
                    debug!("{} sent success: {}", addr_from, message);
                }
                MessageType::Ack => {
                    debug!("{} sent ack: {}", addr_from, message);
                }
            },
            Package::AdminNodeQuery {
                addr_from,
                query_type,
            } => match query_type {
                AdminNodeQueryType::GetBalance { wlt_address } => {
                    // Example admin query — compute and log balance for an address.
                    let address_valid = WalletAddress::validate(wlt_address)?;

                    let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());
                    let balance = utxo_set
                        .get_balance(&address_valid)
                        .await
                        .expect("UTXO set get balance error");
                    debug!("Balance of {}: {}", addr_from, balance);
                }
                AdminNodeQueryType::GetAllTransactions => {
                    // Prints a ledger-like summary of all transactions found in chainstate.
                    let transactions_summary = node_context
                        .find_all_transactions()
                        .await
                        .expect("Blockchain find all transactions error");

                    info!("═══════════════════════════════════════════════════════════════");
                    info!("                    BLOCKCHAIN TRANSACTIONS");
                    info!("═══════════════════════════════════════════════════════════════");

                    for (idx, (cur_txid_hex, tx_summary)) in transactions_summary.iter().enumerate()
                    {
                        let mut tx_summary_input = tx_summary.clone();
                        let mut tx_summary_output = tx_summary.clone();
                        let tx_summary_inputs = tx_summary_input.get_inputs();
                        let tx_summary_outputs = tx_summary_output.get_outputs();
                        info!("");
                        info!("┌─ Transaction #{}", idx + 1);
                        info!("│  ID: {}", cur_txid_hex);
                        info!(
                            "│  Type: {}",
                            if tx_summary_inputs.is_empty() {
                                "Coinbase"
                            } else {
                                "Regular"
                            }
                        );

                        if !tx_summary_inputs.is_empty() {
                            info!("│  ┌─ Inputs ({}):", tx_summary_inputs.len());
                            for (input_idx, input_summary) in tx_summary_inputs.iter().enumerate() {
                                info!(
                                    "│  │  {} └─ From: {} (txid: {}, vout: {})",
                                    if input_idx == tx_summary_inputs.len() - 1 {
                                        "└"
                                    } else {
                                        "├"
                                    },
                                    input_summary.get_wlt_addr().as_str(),
                                    input_summary.get_txid_hex(),
                                    input_summary.get_output_idx()
                                );
                            }
                        }

                        info!("│  ┌─ Outputs ({}):", tx_summary_outputs.len());
                        for (output_idx, output_summary) in tx_summary_outputs.iter().enumerate() {
                            info!(
                                "│  │  {} └─ To: {} (value: {} BTC)",
                                if output_idx == tx_summary_outputs.len() - 1 {
                                    "└"
                                } else {
                                    "├"
                                },
                                output_summary.get_wlt_addr().as_str(),
                                output_summary.get_value()
                            );
                        }
                        info!("└─────────────────────────────────────────────────────────────");
                    }

                    info!("");
                    info!("═══════════════════════════════════════════════════════════════");
                    info!("Total Transactions: {}", transactions_summary.len());
                    info!("═══════════════════════════════════════════════════════════════");
                }
                AdminNodeQueryType::GetBlockHeight => {
                    let height = node_context
                        .get_blockchain_height()
                        .await
                        .expect("Blockchain read error");
                    trace!("Block height: {}", height);
                }
                AdminNodeQueryType::MineEmptyBlock => {
                    // “mine now” command; only works if this node is configured as miner.
                    if GLOBAL_CONFIG.is_miner() {
                        // Get mining address from config
                        let mining_address =
                            GLOBAL_CONFIG.get_mining_addr().ok_or(BtcError::NotAMiner)?;
                        node_context
                            .mine_empty_block(&mining_address)
                            .await
                            .map(|_| ())?
                    } else {
                        trace!("Not a miner");
                    }
                    trace!("Mining empty block");
                }
                AdminNodeQueryType::ReindexUtxo => {
                    // Forces a full UTXO rebuild from the current chain.
                    let utxo_set = UTXOSet::new(node_context.get_blockchain().clone());
                    utxo_set.reindex().await.expect("UTXO set reindex error");
                    let count = utxo_set
                        .count_transactions()
                        .await
                        .expect("UTXO set count error");
                    trace!(
                        "Reindexed UTXO set. There are {} transactions in the UTXO set.",
                        count
                    );
                }
            },
        }
    }
    // Close the connection after we finish processing the stream.
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}
```

---

## 4. Outbound sends: typed wrappers around `send_data`

The network layer uses one core primitive (`send_data`) and a family of typed helpers that construct the correct `Package` variant.

> **Methods involved**
>
> - `send_get_data(...)`
> - `send_inv(...)`
> - `send_block(...)`
> - `send_tx(...)`
> - `send_known_nodes(...)`
> - `send_version(...)`
> - `send_get_blocks(...)`
> - `send_message(...)`
> - `send_data(...)`

### Code Listing 2.7A-3.1 — Full send primitives (`bitcoin/src/net/net_processing.rs`)

```rust
pub async fn send_get_data(addr_to: &SocketAddr, op_type: OpType, id: &[u8]) {
    // Outbound request — ask a peer to send us the bytes for a given object id.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetData {
            addr_from: node_addr,
            op_type,
            id: id.to_vec(),
        },
    )
    .await;
}

pub async fn send_inv(addr_to: &SocketAddr, op_type: OpType, blocks: &[Vec<u8>]) {
    // Outbound announce — tell a peer “I have these ids available”.
    // The receiver will typically respond with GETDATA for the ids it wants.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Inv {
            addr_from: node_addr,
            op_type,
            items: blocks.to_vec(),
        },
    )
    .await;
}

pub async fn send_block(addr_to: &SocketAddr, block: &Block) {
    // Outbound delivery — send the full serialized block bytes.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Block {
            addr_from: node_addr,
            block: block.serialize().expect("Block serialization error"),
        },
    )
    .await;
}

pub async fn send_tx(addr_to: &SocketAddr, tx: &Transaction) {
    // Outbound delivery — send the full serialized transaction bytes.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Tx {
            addr_from: node_addr,
            transaction: tx.serialize().expect("Transaction serialization error"),
        },
    )
    .await;
}

pub async fn send_known_nodes(addr_to: &SocketAddr, nodes: Vec<SocketAddr>) {
    // Peer discovery payload — send a list of nodes the receiver can try to contact.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::KnownNodes {
            addr_from: node_addr,
            nodes,
        },
    )
    .await;
}

pub async fn send_version(addr_to: &SocketAddr, height: usize) {
    // Handshake/sync hint — send our protocol version and current best height.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Version {
            addr_from: node_addr,
            version: NODE_VERSION,
            best_height: height,
        },
    )
    .await;
}

pub async fn send_get_blocks(addr_to: &SocketAddr) {
    // Request that a peer replies with block inventory (hash list).
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::GetBlocks {
            addr_from: node_addr,
        },
    )
    .await;
}

pub async fn send_message(addr_to: &SocketAddr, message_type: MessageType, message: String) {
    // Generic “string message” channel used for errors/warnings/info in our Rust Bitcoin implementation.
    let node_addr = GLOBAL_CONFIG.get_node_addr();
    send_data(
        addr_to,
        Package::Message {
            addr_from: node_addr,
            message_type,
            message,
        },
    )
    .await;
}

async fn send_data(addr_to: &SocketAddr, pkg: Package) {
    // The only “real” send primitive.
    // Every typed send_* helper above ultimately calls this function.
    info!("send package: {:?}", &pkg);
    let stream = TcpStream::connect(addr_to);
    if stream.is_err() {
        // If we cannot connect, treat this node as unhealthy and evict it.
        error!("The {} is not valid", addr_to);

        GLOBAL_NODES
            .evict_node(addr_to)
            .expect("Node eviction error");
        return;
    }

    let mut stream = stream.expect("Stream connect error");
    // Bound how long we’re willing to block on writing to this peer.
    let _ = stream.set_write_timeout(Option::from(Duration::from_millis(TCP_WRITE_TIMEOUT)));
    // Serialize one JSON value (one `Package`) onto the stream.
    let _ = serde_json::to_writer(&stream, &pkg);
    // Ensure bytes are pushed out immediately.
    let _ = stream.flush();
}
```

---

## 5. Peer discovery: `KnownNodes` + `process_known_nodes`

This implementation models a “central node” bootstrap and then exchanges known peers.

### Diagram: how nodes become “known”

```
Non-central node starts
  |
  | send Version(best_height) to central
  v
Central learns addr_from
  |
  | central sends KnownNodes(list)
  v
Non-central learns peer set
  |
  | non-central sends Version to new peers (fanout)
  v
Network converges to a shared peer list (best-effort)
```

> **Methods involved**
>
> - `process_known_nodes(...)`
> - `send_known_nodes(...)` (defined earlier in Listing 2.7A-3.1)
> - `send_version(...)` (defined earlier in Listing 2.7A-3.1)

### Code Listing 2.7A-4.1 — Process known nodes (`bitcoin/src/net/net_processing.rs`)

```rust
pub async fn process_known_nodes(
    node_context: NodeContext,
    addr_from: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    // Merge a received peer list into our global peer set, then fan out
    // a combination of (KnownNodes + Version) messages to converge peer knowledge.

    // Find nodes we did not know before (dedupe).
    let new_nodes: HashSet<SocketAddr> = nodes
        .iter()
        .filter(|current_new_node_candidate| {
            !GLOBAL_NODES
                .node_is_known(current_new_node_candidate)
                .expect("Node is known error")
        })
        .cloned()
        .collect();

    info!("new_nodes: {:?}", new_nodes);

    // Add newly discovered nodes to our global peer set.
    GLOBAL_NODES
        .add_nodes(new_nodes.clone())
        .expect("Global nodes add error");

    // Snapshot the full set of known nodes (after merging).
    let all_known_nodes_addresses: Vec<SocketAddr> = GLOBAL_NODES
        .get_nodes()
        .expect("Global nodes get error")
        .into_iter()
        .map(|node| node.get_addr())
        .collect();

    let mut nodes_to_add: HashSet<SocketAddr> = HashSet::new();
    // Add new nodes to the nodes to add
    nodes_to_add.extend(new_nodes.clone());
    // Add sender to the nodes to add
    nodes_to_add.insert(*addr_from);

    // Empty nodes sent or have sender doesn't know all nodes that i know
    if all_known_nodes_addresses.len() > nodes.len() {
        // If we know strictly more peers than the sender advertised,
        // push our fuller list back to the sender and to the newly discovered nodes.
        nodes_to_add.iter().for_each(|node| {
            let node_addr = *node;
            let all_nodes = all_known_nodes_addresses.clone();
            tokio::spawn(async move {
                send_known_nodes(&node_addr, all_nodes).await;
            });
        });
    }

    // Send Version to all new nodes plus sender.
    // This is the “height hint” used by peers to decide whether to request blocks.
    let best_height = node_context
        .get_blockchain_height()
        .await
        .expect("Blockchain get best height error");

    // Always acknowledge the sender with our current height.
    send_version(addr_from, best_height).await;

    // Also introduce ourselves to newly discovered peers.
    nodes_to_add
        .into_iter()
        .filter(|node| node.ne(addr_from))
        .for_each(|node| {
            let node_addr = node;
            let height = best_height;
            tokio::spawn(async move {
                send_version(&node_addr, height).await;
            });
        });
}
```

---

## 6. Boundary: acceptance/consensus is not “networking”

Whitepaper Step 5 (“valid and not already spent”) is **not** a networking rule; it is a **state transition rule**:

- networking can fetch and relay blocks
- chainstate must validate *before* mutating tip + UTXO

> **Methods involved**
>
> - `process_stream(...)` (hands the inbound `Package::Block` to `NodeContext::add_block(...)`)
> - `NodeContext::add_block(...)` (**defined earlier**; see **[Chapter 2.6: Block Acceptance](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)**)
> - Block/tx validation and fork choice (**defined earlier**; see **[Section 2.4.7: Consensus and Validation](../chain/07-Consensus-and-Validation.md)**)

In this book, the acceptance contract is covered in:

- **[Chapter 2.6: Block Acceptance (Whitepaper Step 5)](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)**

In this network chapter, we focus on:

- how blocks are announced and fetched (`Inv` + `GetData`)
- where inbound blocks are handed to the node (`NodeContext::add_block(...)`)

---

## Summary: what to remember

- `Server::run_with_shutdown` is the accept loop.
- `process_stream` is the router.
- `send_data` is the send primitive; everything else is a typed wrapper.
- INV/GETDATA is the “announce then fetch” pattern.
- Networking routes; chainstate decides whether data becomes state.

---

<div align="center">

**📚 [← Chapter 2.7: Network Layer](README.md)** | **Chapter 2.7.A: Network Layer — Code Walkthrough** | **[Chapter 2.8: Node Orchestration →](../node/README.md)** 📚

</div>

---

