use blockchain::{
    BtcError, ConnectNode, GLOBAL_CONFIG, Result, Server, UTXOSet, WalletService, convert_address,
    hash_pub_key, service::blockchain_service::BlockchainService, validate_address,
};
use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::str::FromStr;

use tracing::info;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    prelude::*,
};

#[derive(Debug, Clone)]
enum IsMiner {
    Yes,
    No,
}

impl FromStr for IsMiner {
    type Err = BtcError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "yes" => Ok(IsMiner::Yes),
            "no" => Ok(IsMiner::No),
            _ => Err(BtcError::InvalidValueForMiner(s.to_string())),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "blockchain")]
struct Opt {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    #[command(
        name = "getbalance",
        about = "Get the wallet balance of the target address"
    )]
    #[command(name = "listaddresses", about = "Print local wallet addres")]
    ListAddresses,
    #[command(name = "send", about = "Add new block to chain")]
    #[command(name = "printchain", about = "Print blockchain all block")]
    Printchain,
    #[command(name = "reindexutxo", about = "rebuild UTXO index set")]
    #[command(name = "startnode", about = "Start a node")]
    StartNode {
        #[arg(name = "wlt_addr", help = "Wallet Address")]
        wlt_addr: String,
        #[arg(name = "is_miner", help = "Is Node a Miner?")]
        is_miner: IsMiner,
        #[arg(name = "connect_nodes", help = "Connect to a node")]
        connect_nodes: Vec<ConnectNode>,
    },
}

/// Initialize logging with functional configuration
fn initialize_logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .init();
}

/// Create a new wallet and return the address
fn create_wallet() -> Result<String> {
    WalletService::new()
        .and_then(|mut wallets| wallets.create_wallet())
        .map(|address| {
            info!("Your new address: {}", address);
            address
        })
}

/// List all wallet addresses
fn list_addresses() -> Result<()> {
    WalletService::new().map(|wallets| {
        wallets
            .get_addresses()
            .iter()
            .for_each(|address| info!("{}", address));
    })
}

/// Format transaction input information
fn format_transaction_input(input: &blockchain::TXInput) -> String {
    let txid_hex = input.get_input_tx_id_hex();
    let pub_key_hash = hash_pub_key(input.get_pub_key());
    let address =
        convert_address(pub_key_hash.as_slice()).unwrap_or_else(|_| "Unknown".to_string());

    format!(
        "-- Input txid = {}, vout = {}, from = {}",
        txid_hex,
        input.get_vout(),
        address,
    )
}

/// Format transaction output information
fn format_transaction_output(output: &blockchain::TXOutput) -> String {
    let pub_key_hash = output.get_pub_key_hash();
    let address = convert_address(pub_key_hash).unwrap_or_else(|_| "Unknown".to_string());

    format!("-- Output value = {}, to = {}", output.get_value(), address)
}

/// Process a single transaction and log its details
fn process_transaction(tx: &blockchain::Transaction) {
    let cur_txid_hex = tx.get_tx_id_hex();
    info!("- Transaction txid_hex: {}", cur_txid_hex);

    // Process inputs if not coinbase
    if !tx.is_coinbase() {
        tx.get_vin()
            .iter()
            .map(format_transaction_input)
            .for_each(|input_info| info!("{}", input_info));
    }

    // Process outputs
    tx.get_vout()
        .iter()
        .map(format_transaction_output)
        .for_each(|output_info| info!("{}", output_info));
}

/// Process a single block and log its details
fn process_block(block: &blockchain::Block) {
    info!("Pre block hash: {}", block.get_pre_block_hash());
    info!("Cur block hash: {}", block.get_hash());
    info!("Cur block Timestamp: {}", block.get_timestamp());

    block
        .get_transactions()
        .iter()
        .for_each(process_transaction);
}

/// Print the entire blockchain using functional iteration
async fn print_blockchain() -> Result<()> {
    let blockchain = BlockchainService::default().await?;
    let mut iterator = blockchain.iterator().await.expect("Failed to get iterator");
    while let Some(block) = iterator.next() {
        process_block(&block);
    }
    Ok(())
}

/// Validate miner configuration
fn validate_miner_config(wlt_addr: &str, is_miner: &IsMiner) -> Result<()> {
    match is_miner {
        IsMiner::Yes => validate_address(wlt_addr).and_then(|is_valid| {
            if is_valid {
                info!("Mining is on. Address to receive rewards: {}", wlt_addr);
                GLOBAL_CONFIG.set_mining_addr(wlt_addr.parse().unwrap());
                Ok(())
            } else {
                Err(BtcError::InvalidValueForMiner(
                    "Wrong miner address!".to_string(),
                ))
            }
        }),
        IsMiner::No => Ok(()),
    }
}

/// Create blockchain for seed node
async fn create_seed_blockchain(wlt_addr: &str) -> Result<BlockchainService> {
    info!("Seed Node, Creating BlockChain With Address: {}", wlt_addr);
    let blockchain = BlockchainService::initialize(wlt_addr).await?;
    let utxo_set = UTXOSet::new(blockchain.clone());
    utxo_set.reindex().await?;
    Ok(blockchain)
}

/// Handle blockchain opening with fallback logic
async fn open_or_create_blockchain(
    wlt_addr: &str,
    connect_nodes: &[ConnectNode],
) -> Result<BlockchainService> {
    match BlockchainService::default().await {
        Ok(blockchain) => {
            // Reindex UTXOSet when opening existing blockchain
            let utxo_set = UTXOSet::new(blockchain.clone());
            utxo_set.reindex().await?;
            Ok(blockchain)
        }
        Err(BtcError::BlockchainNotFoundError(_)) => {
            if connect_nodes.contains(&ConnectNode::Local) {
                create_seed_blockchain(wlt_addr).await
            } else {
                BlockchainService::empty().await
            }
        }
        Err(e) => {
            info!("Blockchain error: {}", e);
            Err(e)
        }
    }
}

/// Start the node with functional configuration
async fn start_node(
    wlt_addr: String,
    is_miner: IsMiner,
    connect_nodes: Vec<ConnectNode>,
) -> Result<()> {
    // Validate miner configuration
    validate_miner_config(&wlt_addr, &is_miner)?;

    // Open or create blockchain
    let blockchain = open_or_create_blockchain(&wlt_addr, &connect_nodes).await?;

    // Get node configuration
    let socket_addr = GLOBAL_CONFIG.get_node_addr();
    info!("Starting node at address: {}", socket_addr);
    info!("Will try connect to nodes: {:?}", connect_nodes);

    // Convert connect nodes to HashSet
    let connect_nodes_set: HashSet<ConnectNode> = connect_nodes.into_iter().collect();

    // Start server
    Server::new(blockchain)
        .run(&socket_addr, connect_nodes_set)
        .await;

    Ok(())
}

/// Process commands using functional patterns
async fn process_command(command: Command) -> Result<()> {
    match command {
        Command::Createwallet => create_wallet().map(|_| ()),
        Command::ListAddresses => list_addresses(),
        Command::Printchain => print_blockchain().await,
        Command::StartNode {
            wlt_addr,
            is_miner,
            connect_nodes,
        } => start_node(wlt_addr, is_miner, connect_nodes).await,
    }
}

#[tokio::main]
#[deny(unused_must_use)]
async fn main() {
    // Initialize logging
    initialize_logging();

    // Parse command line arguments
    let opt = Opt::parse();

    // Process command with error handling
    if let Err(e) = process_command(opt.command).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
