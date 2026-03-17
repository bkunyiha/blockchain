<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../util/README.md">Chapter 7: Utilities</a>
8. <a href="../crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../store/README.md">Chapter 11: Storage Layer</a>
12. <a href="README.md">Chapter 12: Network Layer</a>
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

# Network Layer — Code Walkthrough (Network Message Pipeline)

**Part I: Foundations & Core Implementation** | **Chapter 12.A: Network Layer — Code Walkthrough**

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

```text
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

```text
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

### Code Listing 2.21A-0.1 — Network globals + message model (part 1) (`bitcoin/src/node/server.rs`)

```rust
use crate::{BlockInTransit, MemoryPool, Nodes};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
// ... (more imports)

pub const NODE_VERSION: usize = 1;

// Central node for peer discovery (bootstrap rendezvous point)
pub static CENTERAL_NODE: Lazy<SocketAddr> = Lazy::new(|| {
    let addr = env::var(“CENTERAL_NODE”)
        .unwrap_or_else(|_| “127.0.0.1:2001”.to_string());
    addr.parse().expect(“invalid CENTERAL_NODE address”)
});

pub const TRANSACTION_THRESHOLD: usize = 3;
pub static GLOBAL_NODES: Lazy<Nodes> = Lazy::new(|| {
    let nodes = Nodes::new();
    nodes.add_node(*CENTERAL_NODE).expect(“Node add error”);
    nodes
});
pub static GLOBAL_MEMORY_POOL: Lazy<MemoryPool> = Lazy::new(MemoryPool::new);
pub static GLOBAL_BLOCKS_IN_TRANSIT: Lazy<BlockInTransit> =
    Lazy::new(BlockInTransit::new);
pub const TCP_WRITE_TIMEOUT: u64 = 1000;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ConnectNode {
    Local,
    Remote(SocketAddr),
}

impl ConnectNode {
    pub fn is_remote(&self) -> bool { matches!(self, ConnectNode::Remote(_)) }
    pub fn get_addr(&self) -> SocketAddr {
        match self {
            ConnectNode::Remote(addr) => *addr,
            ConnectNode::Local => *CENTERAL_NODE,
        }
    }
}

impl FromStr for ConnectNode {
    type Err = std::net::AddrParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == “local” { Ok(ConnectNode::Local) }
        else { Ok(ConnectNode::Remote(s.parse()?)) }
    }
}

// Discriminator: tells receiver what type (Tx or Block) the hash refers to.
#[derive(Debug, Serialize, Deserialize)]
pub enum OpType { Tx, Block }

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType { Error, Success, Info, Warning, Ack }
```

The message type enums define the wire-level vocabulary for discriminating between objects and status categories. Next we define the envelope types themselves:

### Code Listing 2.21A-0.2 — Message variants and admin queries (part 2)

```rust
// Admin operations (not part of P2P protocol)
#[derive(Debug, Serialize, Deserialize)]
pub enum AdminNodeQueryType {
    GetBalance { wlt_address: String },
    GetAllTransactions,
    GetBlockHeight,
    MineEmptyBlock,
    ReindexUtxo,
}

// Wire envelope: every JSON message is one Package
#[derive(Debug, Serialize, Deserialize)]
pub enum Package {
    Block { addr_from: SocketAddr, block: Vec<u8> },
    GetBlocks { addr_from: SocketAddr },
    GetData { addr_from: SocketAddr, op_type: OpType, id: Vec<u8> },
    Inv { addr_from: SocketAddr, op_type: OpType, items: Vec<Vec<u8>> },
    Tx { addr_from: SocketAddr, transaction: Vec<u8> },
    SendBitCoin {
        addr_from: SocketAddr,
        wlt_frm_addr: String,
        wlt_to_addr: String,
        amount: i32,
    },
    KnownNodes { addr_from: SocketAddr, nodes: Vec<SocketAddr> },
    Version { addr_from: SocketAddr, version: usize, best_height: usize },
    Message {
        addr_from: SocketAddr,
        message_type: MessageType,
        message: String,
    },
    AdminNodeQuery { addr_from: SocketAddr, query_type: AdminNodeQueryType },
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

### Code Listing 2.21A-1.1 — TCP server setup (part 1) (`bitcoin/src/node/server.rs`)

```rust
#[derive(Debug, Clone)]
pub struct Server {
    node_context: NodeContext,
}

impl Server {
    pub fn new(blockchain: NodeContext) -> Server {
        Server { node_context: blockchain }
    }

    #[instrument(skip(self, addrs, connect_nodes, shutdown))]
    pub async fn run_with_shutdown(
        &self,
        addrs: &SocketAddr,
        connect_nodes: HashSet<ConnectNode>,
        mut shutdown: tokio::sync::broadcast::Receiver<()>,
    ) {
        let listener = TcpListener::bind(addrs)
            .await
            .expect(“TcpListener bind error”);
        info!(“Server listening on {:?}”, listener.local_addr());

        if !addrs.eq(&CENTERAL_NODE) {
            let best_height = self
                .node_context
                .get_blockchain_height()
                .await
                .expect(“Blockchain read error”);
            send_version(&CENTERAL_NODE, best_height).await;
        } else {
            let remote_nodes: HashSet<SocketAddr> = connect_nodes
                .iter()
                .filter(|node| node.is_remote())
                .map(|node| node.get_addr())
                .collect();

            GLOBAL_NODES
                .add_nodes(remote_nodes.clone())
                .expect(“Global nodes add error”);

            for remote_node in remote_nodes {
                send_known_nodes(
                    &remote_node,
                    GLOBAL_NODES
                        .get_nodes()
                        .expect(“Global nodes get error”)
                        .iter()
                        .map(|node| node.get_addr())
                        .collect(),
                )
                .await;
            }
        }
```

After bootstrap, the server enters an event loop that accepts inbound connections and spawns one task per peer:

### Code Listing 2.21A-2 — Accept loop and per-peer spawn (part 2)

```rust
        // Serve incoming connections with graceful shutdown.
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!(“Network server shutdown signal received”);
                    break;
                }
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, _peer)) => {
                            // Each accepted stream gets handled in its own
                            // async task. This keeps the accept loop responsive
                            // even if one peer is slow.
                            let blockchain = self.node_context.clone();
                            tokio::spawn(async move {
                                // Convert tokio stream to std stream for
                                // existing processing code.
                                match stream.into_std() {
                                    Ok(std_stream) => {
                                        // Message processing uses blocking I/O.
                                        let _ =
                                            std_stream.set_nonblocking(false);
                                        if let Err(e) =
                                            net_processing::process_stream(
                                                blockchain,
                                                std_stream,
                                            )
                                            .await
                                        {
                                            error!(“Serve error: {}”, e);
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            “Failed to convert stream: {}”,
                                            e
                                        );
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            error!(“accept error: {}”, e);
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

```text
Version        -> compare height -> maybe send_get_blocks or send_version
KnownNodes     -> process_known_nodes
GetBlocks      -> get hashes -> send_inv(Block, [hashes])
Inv(Block)     -> add to in-transit -> send_get_data(Block, hash)
GetData(Block) -> get_block(id) -> send_block
Block          -> add_block, remove from mempool, maybe get_next

Inv(Tx)        -> if missing -> send_get_data(Tx, txid)
GetData(Tx)    -> mempool lookup -> send_tx
Tx             -> process_transaction (mempool + relay + maybe mine)
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
> - `NodeContext::process_transaction(...)` (defined in **[Chapter 13: Node Orchestration](../node/README.md)**)
> - `NodeContext::add_block(...)` (acceptance contract: **[Chapter 10: Block Acceptance](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)**)
> - `NodeContext::get_block_hashes(...)`, `NodeContext::get_block(...)` (defined in **[Chapter 13: Node Orchestration](../node/README.md)** and chain chapters)

### Code Listing 2.21A-6 — Dispatcher initialization (part 1) (`bitcoin/src/net/net_processing.rs`)

```rust
#[instrument(skip(node_context, stream))]
pub async fn process_stream(
    node_context: NodeContext,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    // Routes TCP stream → deserializes JSON → dispatches messages.
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(&stream);
    let pkg_reader = Deserializer::from_reader(reader).into_iter::<Package>();

    for pkg in pkg_reader {
        let pkg = pkg?;
        match pkg {
```

The dispatcher enters a match statement to handle each Package variant. Block handling comes first:

### Code Listing 2.21A-6b — Block and inventory cases (part 1b)

```rust
            Package::Block { addr_from, block } => {
                let block = Block::deserialize(block.as_slice())?;
                // Ownership boundary: network hands block to chain logic
                // for validation, reorg, and state update.
                node_context.add_block(&block).await?;
                let added_block_hash = block.get_hash_bytes();

                // Remove confirmed transactions from mempool
                for tx in block.get_transactions().await? {
                    node_context.remove_from_memory_pool(tx.clone()).await;
                }

                // UTXO updates, blocks-in-transit management
                if GLOBAL_BLOCKS_IN_TRANSIT.is_not_empty()? {
                    let block_hash = GLOBAL_BLOCKS_IN_TRANSIT.first()?;
                    send_get_data(&addr_from, OpType::Block, &block_hash).await;
                }
            }
            Package::GetBlocks { addr_from } => {
                let blocks = node_context.get_block_hashes().await?;
                send_inv(&addr_from, OpType::Block, &blocks).await;
            }
```

GetData routing branches on whether the request is for a block or transaction:

### Code Listing 2.21A-6c — GetData branching (part 1c)

```rust
            Package::GetData { addr_from, op_type, id } => match op_type {
                OpType::Block => {
                    if let Some(block) =
                        node_context.get_block(id.as_slice()).await?
                    {
                        send_block(&addr_from, &block).await;
                    }
                }
                OpType::Tx => {
                    let txid_hex = HEXLOWER.encode(id.as_slice());
                    if let Some(tx) =
                        GLOBAL_MEMORY_POOL.get(txid_hex.as_str())?
                    {
                        send_tx(&addr_from, &tx).await;
                    }
                }
            },
```

Block and transaction inventory handling follows the data routing:

### Code Listing 2.21A-7 — Dispatcher inventory and transaction cases (part 2)

```rust
            Package::Inv { addr_from, op_type, items } => match op_type {
                OpType::Block => {
                    GLOBAL_BLOCKS_IN_TRANSIT.add_blocks(items.as_slice())?;
                    let block_hash = items.first()?;
                    send_get_data(&addr_from, OpType::Block, block_hash).await;
                }
                OpType::Tx => {
                    let txid = items.first()?;
                    let txid_hex = HEXLOWER.encode(txid);
                    if !GLOBAL_MEMORY_POOL.contains(txid_hex.as_str())? {
                        send_get_data(&addr_from, OpType::Tx, txid).await;
                    }
                }
            },
            Package::Tx { addr_from, transaction } => {
                let tx = Transaction::deserialize(transaction.as_slice())?;
                match node_context.process_transaction(&addr_from, tx).await {
                    Ok(_) => {}
                    Err(
                        BtcError::TransactionAlreadyExistsInMemoryPool(txid)
                    ) => {
                        let msg = format!(
                            “Transaction {} already exists”,
                            txid
                        );
                        send_message(
                            &addr_from,
                            MessageType::Error,
                            msg,
                        )
                        .await;
                    }
                    Err(e) => Err(e)?,
                }
            }
            Package::SendBitCoin {
                addr_from,
                wlt_frm_addr,
                wlt_to_addr,
                amount,
            } => {
                let from = WalletAddress::validate(wlt_frm_addr)?;
                let to = WalletAddress::validate(wlt_to_addr)?;
                match node_context.btc_transaction(&from, &to, amount).await
                {
                    Ok(_) => {}
                    Err(BtcError::NotEnoughFunds) => {
                        let utxo_set = UTXOSet::new(
                            node_context.get_blockchain().clone(),
                        );
                        let balance =
                            utxo_set.get_balance(&from).await.unwrap_or(0);
                    }
                    Err(_) => {}
                }
            }
```

Version and peer discovery messages follow wallet operations:

### Code Listing 2.21A-8 — Dispatcher protocol and peer cases (part 3)

```rust
            Package::Version { addr_from, version, best_height } => {
                let local_height = node_context.get_blockchain_height().await?;
                if local_height < best_height {
                    send_get_blocks(&addr_from).await;
                }
                if local_height > best_height {
                    send_version(&addr_from, local_height).await;
                }
                if !GLOBAL_NODES.node_is_known(&addr_from)? {
                    GLOBAL_NODES.add_node(addr_from)?;
                }
            }
            Package::KnownNodes { addr_from, nodes } => {
                process_known_nodes(
                    node_context.clone(),
                    &addr_from,
                    nodes,
                )
                .await;
            }
            Package::Message { .. } => {}
            Package::AdminNodeQuery { .. } => {}
        }
    }
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

### Code Listing 2.21A-3.1 — Inventory and data request sends (part 1) (`bitcoin/src/net/net_processing.rs`)

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

pub async fn send_inv(
    addr_to: &SocketAddr,
    op_type: OpType,
    blocks: &[Vec<u8>],
) {
    // Outbound announce — tell peer about available ids.
    // Receiver responds with GETDATA for desired ids.
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
            block: block.serialize().expect(“Block serialization error”),
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
            transaction: tx.serialize()
                .expect(“Transaction serialization error”),
        },
    )
    .await;
}
```

Block and transaction delivery is followed by peer discovery and protocol handshake messages:

### Code Listing 2.21A-3.2 — Peer discovery sends (part 2) (`bitcoin/src/net/net_processing.rs`)

```rust
pub async fn send_known_nodes(
    addr_to: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    // Send list of nodes for peer discovery.
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
```

Messages are sent separately for errors and informational communication:

### Code Listing 2.21A-3.2b — Message send helper (part 2b)

```rust
pub async fn send_message(
    addr_to: &SocketAddr,
    message_type: MessageType,
    message: String,
) {
    // Send error/warning/info messages.
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
```

The core send primitive handles all connection and serialization logic:

### Code Listing 2.21A-3.3 — Core send logic with health checking (part 3)

```rust
async fn send_data(addr_to: &SocketAddr, pkg: Package) {
    // The only “real” send primitive.
    // Every typed send_* helper above ultimately calls this function.
    info!(“send package: {:?}”, &pkg);
    let stream = TcpStream::connect(addr_to);
    if stream.is_err() {
        // If we cannot connect, treat this node as unhealthy and evict it.
        error!(“The {} is not valid”, addr_to);

        GLOBAL_NODES
            .evict_node(addr_to)
            .expect(“Node eviction error”);
        return;
    }

    let mut stream = stream.expect(“Stream connect error”);
    // Bound write timeout.
    let _ = stream.set_write_timeout(Option::from(
        Duration::from_millis(TCP_WRITE_TIMEOUT),
    ));
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

```text
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
> - `send_known_nodes(...)` (defined earlier in Listing 2.21A-3.1)
> - `send_version(...)` (defined earlier in Listing 2.21A-3.1)

### Code Listing 2.21A-16 — Peer merge and discovery fanout (part 1) (`bitcoin/src/net/net_processing.rs`)

```rust
pub async fn process_known_nodes(
    node_context: NodeContext,
    addr_from: &SocketAddr,
    nodes: Vec<SocketAddr>,
) {
    // Merge peer list and fan out messages to converge peer knowledge.

    // Find nodes we did not know before (dedupe).
    let new_nodes: HashSet<SocketAddr> = nodes
        .iter()
        .filter(|current_new_node_candidate| {
            !GLOBAL_NODES
                .node_is_known(current_new_node_candidate)
                .expect(“Node is known error”)
        })
        .cloned()
        .collect();

    info!(“new_nodes: {:?}”, new_nodes);

    // Add newly discovered nodes to our global peer set.
    GLOBAL_NODES
        .add_nodes(new_nodes.clone())
        .expect(“Global nodes add error”);

    // Snapshot the full set of known nodes (after merging).
    let all_known_nodes_addresses: Vec<SocketAddr> = GLOBAL_NODES
        .get_nodes()
        .expect(“Global nodes get error”)
        .into_iter()
        .map(|node| node.get_addr())
        .collect();

    let mut nodes_to_add: HashSet<SocketAddr> = HashSet::new();
    // Add new nodes to the nodes to add
    nodes_to_add.extend(new_nodes.clone());
    // Add sender to the nodes to add
    nodes_to_add.insert(*addr_from);

    if all_known_nodes_addresses.len() > nodes.len() {
        // If we know more peers, push fuller list back.
        nodes_to_add.iter().for_each(|node| {
            let node_addr = *node;
            let all_nodes = all_known_nodes_addresses.clone();
            tokio::spawn(async move {
                send_known_nodes(&node_addr, all_nodes).await;
            });
        });
    }
```

After merging new nodes into the peer set, we fanout version messages to converge height knowledge:

### Code Listing 2.21A-17 — Version fanout to newly discovered peers (part 2)

```rust
    // Send height hint to new nodes and sender.
    let best_height = node_context
        .get_blockchain_height()
        .await
        .expect(“Blockchain get best height error”);

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
> - `NodeContext::add_block(...)` (**defined earlier**; see **[Chapter 10: Block Acceptance](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)**)
> - Block/tx validation and fork choice (**defined earlier**; see **[Section 9.7: Consensus and Validation](../chain/07-Consensus-and-Validation.md)**)

In this book, the acceptance contract is covered in:

- **[Chapter 10: Block Acceptance (Whitepaper Step 5)](../chain/10-Whitepaper-Step-5-Block-Acceptance.md)**

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

**[← Chapter 12: Network Layer](README.md)** | **Chapter 12.A: Network Layer — Code Walkthrough** | **[Chapter 13: Node Orchestration →](../node/README.md)** 

</div>

---

