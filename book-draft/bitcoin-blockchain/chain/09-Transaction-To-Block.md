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
9. <a href="README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
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

# Section 9.9: Transaction to Block (End-to-End Runtime Walkthrough)

This section is the **final code walkthrough** for the `chain/` chapter series. The goal is explicit:

> Follow the first instruction executed in `main`, and trace the call chain all the way to a block being created.

Unlike earlier sections (which focus on individual subsystems), this chapter is about **wiring and flow**: the runtime entrypoints, the message boundaries, and the concrete method-to-method path that turns “a user wants to send bitcoin” into “a miner produces a new block”.

## A practical constraint in this repo: “web server” and “miner” are separate roles

In `bitcoin/src/main.rs`, `startnode` enforces that **a node cannot run the web server and be a miner at the same time**. Mining only runs on nodes started with `is_miner=yes`, while HTTP transaction creation requires `is_web_server=yes`.

Therefore, the end-to-end story naturally involves two processes:

- **Node A (Web/API node)**: accepts an HTTP request and creates a transaction, then propagates it over P2P.
- **Node B (Miner node)**: receives the transaction over P2P, places it in the mempool, and mines a block once mining is triggered.

This is not a limitation of Bitcoin as a protocol—it is simply how this learning implementation is configured in `main`.

## Diagram: the full pipeline (two-node “tx → block”)

┌──────────────────────────────┐                         ┌──────────────────────────────┐
│ Node A: Web/API node          │                         │ Node B: Miner node            │
│ (is_web_server=yes, miner=no) │                         │ (is_miner=yes)                │
└───────────────┬──────────────┘                         └───────────────┬──────────────┘
                │ HTTP POST /api/v1/transactions                          │
                │                                                         │
                ▼                                                         │
  web::handlers::transaction::send_transaction                             │
                │                                                         │
                ▼                                                         │
          NodeContext::btc_transaction                                     │
                │                                                         │
                ▼                                                         │
          NodeContext::process_transaction (mempool accept)                │
                │                                                         │
                ├─> txmempool::add_to_memory_pool                          │
                │                                                         │
                └─> submit_transaction_for_mining (background)             │
                      │                                                   │
                      └─> net_processing::send_inv(OpType::Tx, txid) ─────►│
                                                                           │ receive INV
                                                                           ▼
                                                                 net_processing::process_stream
                                                                           │
                                                                           ├─> send_get_data(OpType::Tx, txid)
                                                                           │
                                                                           ▼
                                                                 net_processing::send_tx (full bytes)
                                                                           │
                                                                           ▼
                                                                 Package::Tx → NodeContext::process_transaction
                                                                           │
                                                                           └─> miner::process_mine_block → Block::new_block → ProofOfWork::run

## Method index (what you will see printed in this chapter)

We will walk these methods, in the order you reach them at runtime:

- `bitcoin/src/main.rs`
  - `main`
  - `process_command`
  - `start_node`
- `bitcoin/src/node/server.rs`
  - `Server::run_with_shutdown`
- `bitcoin/src/net/net_processing.rs`
  - `process_stream` (only the message arms relevant to tx/block flow)
  - `send_inv`, `send_get_data`, `send_tx`, `send_block`
- `bitcoin/src/web/server.rs` + `bitcoin/src/web/routes/api.rs` + `bitcoin/src/web/handlers/transaction.rs`
  - `WebServer::start_with_shutdown`
  - `create_api_routes` (the POST `/transactions` route)
  - `send_transaction` (HTTP handler)
- `bitcoin/src/node/context.rs`
  - `NodeContext::btc_transaction`
  - `NodeContext::process_transaction`
  - `NodeContext::submit_transaction_for_mining`
  - `NodeContext::broadcast_transaction_to_nodes`
- `bitcoin/src/node/txmempool.rs`
  - `add_to_memory_pool`
- `bitcoin/src/node/miner.rs`
  - `should_trigger_mining`
  - `prepare_mining_utxo`
  - `process_mine_block`

Throughout, we omit unrelated class/struct definitions and focus only on the methods that form the end-to-end chain.

---

## Step 1 — The process starts: `main` dispatches `startnode`

The first executed instructions in the node runtime are in `main`. From there, we parse the CLI and route into `start_node(...)`.

### Code Listing 9-9.1 — Process entrypoint (`main`)

> **Source:** `main.rs` — Source

```rust
#[tokio::main]
#[deny(unused_must_use)]
async fn main() {
    // 1) Initialize the global tracing subscriber so later components can log.
    initialize_logging();

    // 2) Parse CLI args (subcommands like startnode/createwallet/etc.).
    let opt = Opt::parse();

    // 3) Dispatch the selected command.
    // For the end-to-end node runtime, we care about Command::StartNode.
    if let Err(e) = process_command(opt.command).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### Code Listing 9-9.2 — CLI dispatch (`process_command`)

> **Source:** `main.rs` — Source

```rust
async fn process_command(command: Command) -> Result<()> {
    match command {
        // ... other commands omitted (wallet creation, printing chain, etc.)
        // ...
        Command::StartNode {
            is_miner,
            is_web_server,
            connect_nodes,
            wlt_mining_addr,
        } => {
            // Validate and normalize the mining address.
            let validated_addr =
                WalletAddress::validate(wlt_mining_addr)?;

            // Enter the runtime wiring path.
            start_node(
                is_miner,
                is_web_server,
                connect_nodes,
                validated_addr
            ).await
        }
        _ => Ok(()),
    }
}
```

### Code Listing 9-9.3 — Wiring the node runtime (`start_node`)

This method constructs `NodeContext` and spawns the network server (and optionally the web server).

> **Source:** `main.rs` — Source

```rust
async fn start_node(
    is_miner: IsMiner,
    is_web_server: IsWebServer,
    connect_nodes: Vec<ConnectNode>,
    wlt_mining_addr: WalletAddress,
) -> Result<()> {
    // Configure the process-level GLOBAL_CONFIG
    validate_miner_config(
        &wlt_mining_addr,
        &is_miner,
        &is_web_server
    )?;

    // Open or create blockchain
    let blockchain = open_or_create_blockchain(
        &wlt_mining_addr,
        &connect_nodes
    ).await?;

    let node_context = NodeContext::new(blockchain);
    let socket_addr = GLOBAL_CONFIG.get_node_addr();

    // Convert CLI nodes into a set for bootstrapping
    let connect_nodes_set: HashSet<ConnectNode> =
        connect_nodes.into_iter().collect();

    // Create shutdown signal channel
    let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(1);

    // Start the P2P network server
    let network_server = Server::new(node_context.clone());
    let net_shutdown_rx = shutdown_tx.subscribe();
    let network_handle = tokio::spawn(async move {
        network_server
            .run_with_shutdown(
                &socket_addr,
                connect_nodes_set,
                net_shutdown_rx
            )
            .await;
    });

    let mut network_handle = network_handle;
    // ... (continue with server startup)
}
```

Once the network server is initialized, the function checks whether to launch an optional HTTP web server (for wallet nodes) or handle miner-only mode. Both paths wait for a shutdown signal:

```rust
    match (is_web_server, is_miner) {
        (IsWebServer::Yes, IsMiner::No) => {
            let web_server = create_web_server(node_context);
            let web_handle = tokio::spawn(async move {
                let _ = web_server.start_with_shutdown().await;
            });
            // ... (wait for Ctrl+C or task completion)
            Ok(())
        }
        (IsWebServer::Yes, _) => {
            Err(BtcError::InvalidConfiguration(
                “Web server and miner cannot be \
                 enabled at the same time”
                    .to_string(),
            ))
        }
        _ => {
            // ... (wait for Ctrl+C or task completion)
            Ok(())
        }
    }
}
```

---

## Step 2 — The P2P runtime loop: `Server::run_with_shutdown` → `process_stream`

Once `start_node` runs, the P2P server is spawned
and begins accepting TCP connections. Each accepted
stream is handed to the P2P message dispatcher
`net_processing::process_stream`.

### Code Listing 9-9.4 — P2P accept loop (`Server::run_with_shutdown`)

> **Source:** `server.rs` — Source

```rust
pub async fn run_with_shutdown(
    &self,
    addrs: &SocketAddr,
    connect_nodes: HashSet<ConnectNode>,
    mut shutdown: tokio::sync::broadcast::Receiver<()>,
) {
    // Bind the TCP listener
    let listener = TcpListener::bind(addrs)
        .await
        .expect("TcpListener bind error");

    // Bootstrap: if not central node, send version handshake
    if !addrs.eq(&CENTERAL_NODE) {
        let best_height = self
            .node_context
            .get_blockchain_height()
            .await
            .expect("Blockchain read error");
        send_version(&CENTERAL_NODE, best_height).await;
    } else {
        // Central node: collect and share known nodes
        let remote_nodes: HashSet<SocketAddr> = connect_nodes
            .iter()
            .filter(|node| node.is_remote())
            .map(|node| node.get_addr())
            .collect();
        GLOBAL_NODES.add_nodes(remote_nodes.clone())
            .expect("Global nodes add error");
        for remote_node in remote_nodes {
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
    // ... (continue to accept loop)
}
```

The main server loop accepts incoming TCP connections and spawns a task for each peer. Each task routes P2P messages through `process_stream`:

```rust
    loop {
        tokio::select! {
            _ = shutdown.recv() => break,
            accept_res = listener.accept() => {
                if let Ok((stream, _)) = accept_res {
                    let blockchain = self.node_context.clone();
                    tokio::spawn(async move {
                        if let Ok(std_stream) =
                            stream.into_std()
                        {
                            let _ =
                                net_processing::process_stream(
                                    blockchain,
                                    std_stream
                                ).await;
                        }
                    });
                }
            }
        }
    }
}
```

---

## Step 3 — The HTTP entrypoint (Node A): POST `/api/v1/transactions` → `NodeContext::btc_transaction`

The web server is only enabled for non-miner
nodes. Its purpose is to provide a client-friendly
HTTP API for creating wallets, querying balances,
and creating transactions.

### Code Listing 9-9.5 — Web server startup (`WebServer::start_with_shutdown`)

> **Source:** `server.rs` — Source

```rust
pub async fn start_with_shutdown(
    &self,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = self.create_app()?;
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        self.config.port
    ));
    let listener = tokio::net::TcpListener::bind(addr)
        .await?;
    let shutdown_signal = async {
        tokio::signal::ctrl_c().await?
    };
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<
            SocketAddr
        >()
    )
        .with_graceful_shutdown(shutdown_signal)
        .await?;
    Ok(())
}
```

### Code Listing 9-9.6 — Route mapping: POST `/api/v1/transactions`

> **Source:** `api.rs` — Source

```rust
pub fn create_api_routes() -> Router<Arc<NodeContext>> {
    Router::new()
        // ... other routes omitted ...
        .route("/transactions", post(transaction::send_transaction))
        // ... other routes omitted ...
}
```

### Code Listing 9-9.7 — HTTP handler: `send_transaction`

> **Source:** `transaction.rs` — Source

```rust
pub async fn send_transaction(
    State(node): State<Arc<NodeContext>>,
    Json(request): Json<SendTransactionRequest>,
) -> Result<Json<ApiResponse<SendBitCoinResponse>>, StatusCode> {
    let txid = node
        .btc_transaction(
            &request.from_address,
            &request.to_address,
            request.amount
        )
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let response = SendBitCoinResponse {
        txid,
        timestamp: chrono::Utc::now(),
    };
    Ok(Json(ApiResponse::success(response)))
}
```

---

## Step 4 — From “send bitcoin” to “mempool accept”: `NodeContext::btc_transaction` → `process_transaction`

Now we enter the core node orchestration API.

### Code Listing 9-9.8 — Create and submit a signed UTXO transaction (`NodeContext::btc_transaction`)

> **Source:** `context.rs` — Source

```rust
pub async fn btc_transaction(
    &self,
    wlt_frm_addr: &WalletAddress,
    wlt_to_addr: &WalletAddress,
    amount: i32,
) -> Result<String> {
    // 1) Build a UTXO view over the current chain state.
    // This is used to select spendable outputs owned by the sender.
    let utxo_set = UTXOSet::new(self.blockchain.clone());

    // 2) Construct and sign the transaction.
    // Transaction::new_utxo_transaction selects inputs, creates outputs
    // (including change),
    // and signs the inputs using sender's key.
    let tx = Transaction::new_utxo_transaction(
        wlt_frm_addr,
        wlt_to_addr,
        amount,
        &utxo_set
    ).await?;

    // 3) Submit to mempool + propagation via the shared processing entrypoint.
    let addr_from = crate::GLOBAL_CONFIG.get_node_addr();
    self.process_transaction(&addr_from, tx).await
}
```

### Code Listing 9-9.9 — Mempool acceptance entrypoint (`NodeContext::process_transaction`)

> **Source:** `context.rs` — Source

```rust
pub async fn process_transaction(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<String> {
    // 1) Reject duplicates early (important to prevent loops and redundant
    // work).
    if transaction_exists_in_pool(&utxo) {
        return Err(
            BtcError::TransactionAlreadyExistsInMemoryPool(
                utxo.get_tx_id_hex()
            )
        );
    }

    // 2) Add to the global mempool and mark outputs as “reserved” by mempool
    // policy.
    add_to_memory_pool(utxo.clone(), &self.blockchain).await?;

    // 3) Background work: propagation and (if this node is a miner) mining
    // trigger.
    // This is spawned so the caller (HTTP handler or P2P handler) returns
    // quickly.
    let context = self.clone();
    let addr_copy = *addr_from;
    let tx = utxo.clone();
    tokio::spawn(async move {
        let _ = context.submit_transaction_for_mining(&addr_copy, tx).await;
    });

    // 4) Return txid immediately (accept-to-mempool semantics).
    Ok(utxo.get_tx_id_hex())
}
```

### Code Listing 9-9.10 — Mempool insert (`txmempool::add_to_memory_pool`)

> **Source:** `txmempool.rs` — Source

```rust
pub async fn add_to_memory_pool(
    tx: Transaction,
    blockchain_service: &BlockchainService,
) -> Result<()> {
    // 1) Store the transaction in the global in-memory pool.
    GLOBAL_MEMORY_POOL.add(tx.clone()).expect("Memory pool add error");

    // 2) Mark referenced outputs as “in mempool” inside the UTXO view.
    // This is a local double-spend protection mechanism: it prevents two
    // mempool txs
    // from spending the same output concurrently.
    let utxo_set = UTXOSet::new(blockchain_service.clone());
    utxo_set.set_global_mem_pool_flag(&tx.clone(), true).await?;

    Ok(())
}
```

---

## Step 5 — P2P propagation: INV → GETDATA → TX

Transactions propagate over the P2P layer, not the HTTP layer. The web layer exists only for local clients (UIs/admin tooling).

### Code Listing 9-9.11 — Broadcast tx inventory to peers (`NodeContext::broadcast_transaction_to_nodes`)

> **Source:** `context.rs` — Source

```rust
async fn broadcast_transaction_to_nodes(&self, nodes: &[Node], txid: Vec<u8>) {
    // For each peer, spawn an async task that sends an INV message.
    // INV is a lightweight announcement: “I have txid”.
    let txid_clone = txid.clone();
    nodes.iter().for_each(|node| {
        let node_addr = node.get_addr();
        let txid = txid_clone.clone();
        tokio::spawn(async move {
            send_inv(&node_addr, OpType::Tx, &[txid]).await;
        });
    });
}
```

### Code Listing 9-9.12 — The background “after accept” hook (`submit_transaction_for_mining`)

> **Source:** `context.rs` — Source

```rust
async fn submit_transaction_for_mining(
    &self,
    addr_from: &std::net::SocketAddr,
    utxo: Transaction,
) -> Result<()> {
    // Propagate tx to peers if this is central node
    if GLOBAL_CONFIG.get_node_addr().eq(&CENTERAL_NODE) {
        let nodes = self.get_nodes_excluding_sender(addr_from).await?;
        self.broadcast_transaction_to_nodes(&nodes, utxo.get_id_bytes()).await;
    }
    // Trigger mining if threshold met
    if should_trigger_mining() {
        if let Some(mining_address) = GLOBAL_CONFIG.get_mining_addr() {
            if let Ok(txs) = prepare_mining_utxo(&mining_address) {
                if !txs.is_empty() {
                    process_mine_block(txs, &self.blockchain).await?;
                }
            }
        }
    }
    Ok(())
}
```

### Code Listing 9-9.13 — P2P send primitives (`send_inv`, `send_get_data`, `send_tx`, `send_block`)

> **Source:** `net_processing.rs` — Source

```rust
pub async fn send_get_data(addr_to: &SocketAddr, op_type: OpType, id: &[u8]) {
    send_data(addr_to, Package::GetData {
        addr_from: GLOBAL_CONFIG.get_node_addr(),
        op_type,
        id: id.to_vec(),
    }).await;
}

pub async fn send_inv(
    addr_to: &SocketAddr,
    op_type: OpType,
    blocks: &[Vec<u8>],
) {
    send_data(addr_to, Package::Inv {
        addr_from: GLOBAL_CONFIG.get_node_addr(),
        op_type,
        items: blocks.to_vec(),
    }).await;
}

pub async fn send_tx(addr_to: &SocketAddr, tx: &Transaction) {
    send_data(addr_to, Package::Tx {
        addr_from: GLOBAL_CONFIG.get_node_addr(),
        transaction: tx.serialize().expect("Serialization error"),
    }).await;
}

pub async fn send_block(addr_to: &SocketAddr, block: &Block) {
    send_data(addr_to, Package::Block {
        addr_from: GLOBAL_CONFIG.get_node_addr(),
        block: block.serialize().expect("Serialization error"),
    }).await;
}
```

### Code Listing 9-9.14 — P2P receive: how a miner gets the full tx (`process_stream`, tx-related arms)

This is the key moment where Node B (miner) learns about a transaction:

- It receives an `Inv { op_type: Tx, items: [txid] }`
- It requests the full bytes with `GetData { op_type: Tx, id: txid }`
- It receives `Tx { transaction: Vec<u8> }` and routes it into `NodeContext::process_transaction`

```rust
// Source: bitcoin/src/net/net_processing.rs (excerpt)
match pkg {
    // Peer says: “I have txid”.
    Package::Inv { addr_from, op_type: OpType::Tx, items } => {
        let txid = items.first().expect("INV items empty");
        let txid_hex = HEXLOWER.encode(txid);

        // If we do not already have it in our mempool, request the full tx
        // bytes.
        if !GLOBAL_MEMORY_POOL
            .contains(txid_hex.as_str())
            .expect("mempool contains error")
        {
            send_get_data(&addr_from, OpType::Tx, txid)
                .await;
        }
    }

    // Peer sends: full tx bytes (typically in response to GETDATA).
    Package::Tx { addr_from, transaction } => {
        let tx = Transaction::deserialize(transaction.as_slice())
            .expect("Transaction deserialization error");

        // Admit to mempool; this may also trigger mining in the background.
        let _ = node_context.process_transaction(&addr_from, tx).await;
    }

    // Peer sends: full block bytes (typically in response to GETDATA).
    Package::Block { addr_from, block } => {
        let block = Block::deserialize(block.as_slice())
            .expect("Block deserialization error");
        node_context.add_block(&block).await
            .expect("Blockchain write error");
    }

    _ => {}
}
```

---

## Step 6 — Mining: mempool → block template → proof-of-work → block broadcast

The mining module is the last step in the pipeline. It is only active when the process is started as a miner (`GLOBAL_CONFIG.is_miner()`).

### Code Listing 9-9.15 — Mining trigger and block assembly (`miner::*`)

> **Source:** `miner.rs` — Source

```rust
pub fn should_trigger_mining() -> bool {
    let pool_size = GLOBAL_MEMORY_POOL.len().expect("Memory pool length error");
    let is_miner = GLOBAL_CONFIG.is_miner();
    pool_size >= TRANSACTION_THRESHOLD && is_miner
}

pub fn prepare_mining_utxo(
    mining_address: &WalletAddress,
) -> Result<Vec<Transaction>> {
    // Pull all mempool txs and append a coinbase tx for the miner reward.
    let txs = GLOBAL_MEMORY_POOL.get_all()?;
    let coinbase_tx = Transaction::new_coinbase_tx(mining_address)?;
    let mut final_txs = txs;
    final_txs.push(coinbase_tx);
    Ok(final_txs)
}

pub async fn process_mine_block(
    txs: Vec<Transaction>,
    blockchain: &BlockchainService,
) -> Result<Block> {
    // Construct+mine a new block on the current tip.
    let new_block = blockchain.mine_block(&txs).await?;

    // Remove txs from mempool once they are mined.
    for tx in &txs {
        remove_from_memory_pool(tx.clone(), blockchain).await;
    }

    // Announce new block hash to peers (INV).
    broadcast_new_block(&new_block).await?;
    Ok(new_block)
}
```

### Where the block is actually created

The exact method call edge where a “block comes into existence” is:

miner::process_mine_block
  └─> BlockchainService::mine_block
        └─> BlockchainFileSystem::mine_block
              └─> Block::new_block
                    └─> ProofOfWork::run

You have already studied these methods in detail in **Section 9.7 (Consensus and Validation)**; this chapter’s purpose is to show **how we reach them at runtime** from `main`.

---

<div align="center">

**[← Previous: Section 9.8 (Node Orchestration)](08-Node-Orchestration.md)** | **Section 9.9: Transaction to Block (End-to-End)** | **[Next: Chapter 10 (Block Acceptance) →](10-Whitepaper-Step-5-Block-Acceptance.md)** 

</div>

