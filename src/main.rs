use blockchain::{
    Blockchain, BtcError, ConnectNode, GLOBAL_CONFIG, Result, Server, UTXOSet, Wallets,
    convert_address, hash_pub_key, validate_address,
};
use data_encoding::HEXLOWER;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::RwLock;

use tracing::info;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    prelude::*,
};

#[derive(Debug)]
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

#[derive(Debug, StructOpt)]
#[structopt(name = "blockchain")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    #[structopt(
        name = "getbalance",
        about = "Get the wallet balance of the target address"
    )]
    #[structopt(name = "listaddresses", about = "Print local wallet addres")]
    ListAddresses,
    #[structopt(name = "send", about = "Add new block to chain")]
    #[structopt(name = "printchain", about = "Print blockchain all block")]
    Printchain,
    #[structopt(name = "reindexutxo", about = "rebuild UTXO index set")]
    #[structopt(name = "startnode", about = "Start a node")]
    StartNode {
        #[structopt(name = "wlt_addr", help = "Wallet Address")]
        wlt_addr: String,
        #[structopt(name = "is_miner", help = "Is Node a Miner?")]
        is_miner: IsMiner,
        #[structopt(name = "connect_nodes", help = "Connect to a node")]
        connect_nodes: Vec<ConnectNode>,
    },
}

#[tokio::main]
#[deny(unused_must_use)]
async fn main() {
    // Build an EnvFilter that defaults to INFO if RUST_LOG is not set.
    // The `from_env_lossy()` method parses directives from `RUST_LOG`
    // but falls back gracefully if it fails.
    // This builder will first check `RUST_LOG` and fall back to `INFO` if it's not set.
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .init();

    let opt = Opt::from_args();
    match opt.command {
        Command::Createwallet => {
            let mut wallets = Wallets::new().unwrap();
            let address = wallets.create_wallet().unwrap();
            info!("Your new address: {}", address)
        }
        Command::ListAddresses => {
            let wallets = Wallets::new().unwrap();
            for address in wallets.get_addresses() {
                info!("{}", address)
            }
        }
        Command::Printchain => {
            let mut block_iterator = Blockchain::open_blockchain().unwrap().iterator().unwrap();
            loop {
                let option = block_iterator.next();
                if option.is_none() {
                    break;
                }
                let block = option.unwrap();
                info!("Pre block hash: {}", block.get_pre_block_hash());
                info!("Cur block hash: {}", block.get_hash());
                info!("Cur block Timestamp: {}", block.get_timestamp());
                for tx in block.get_transactions() {
                    let cur_txid_hex = HEXLOWER.encode(tx.get_id());
                    info!("- Transaction txid_hex: {}", cur_txid_hex);

                    if !tx.is_coinbase() {
                        for input in tx.get_vin() {
                            let txid_hex = HEXLOWER.encode(input.get_txid());
                            let pub_key_hash = hash_pub_key(input.get_pub_key());
                            let address = convert_address(pub_key_hash.as_slice()).unwrap();
                            info!(
                                "-- Input txid = {}, vout = {}, from = {}",
                                txid_hex,
                                input.get_vout(),
                                address,
                            )
                        }
                    }
                    for output in tx.get_vout() {
                        let pub_key_hash = output.get_pub_key_hash();
                        let address = convert_address(pub_key_hash).unwrap();
                        info!("-- Output value = {}, to = {}", output.get_value(), address,)
                    }
                }
            }
        }
        Command::StartNode {
            wlt_addr,
            is_miner,
            connect_nodes,
        } => {
            if let IsMiner::Yes = is_miner {
                if !validate_address(wlt_addr.as_str()).unwrap() {
                    panic!("Wrong miner address!")
                }
                info!("Mining is on. Address to receive rewards: {}", wlt_addr);
                GLOBAL_CONFIG.set_mining_addr(wlt_addr.parse().unwrap());
            }

            // Open blockchain or create a new one
            let blockchain_result: Result<Blockchain> = match Blockchain::open_blockchain() {
                Ok(blockchain) => Ok(blockchain),
                Err(e) => {
                    match e {
                        BtcError::BlockchainNotFoundError(_) => {
                            // If seed node, create a new blockchain
                            // If not seed node, open an empty blockchain
                            if connect_nodes.contains(&ConnectNode::Local) {
                                // If seed node, create a new blockchain
                                info!("Seed Node, Creating BlockChain With Address: {}", wlt_addr);
                                let blockchain = Blockchain::create_blockchain(&wlt_addr).unwrap();
                                let utxo_set = UTXOSet::new(blockchain.clone());
                                utxo_set.reindex().unwrap();
                                Ok(blockchain)
                            } else {
                                //
                                Blockchain::open_blockchain_empty()
                            }
                        }
                        e => {
                            info!("Blockchain error: {}", e);
                            Err(e)
                        }
                    }
                }
            };

            let blockchain = blockchain_result.unwrap();
            let sockert_addr = GLOBAL_CONFIG.get_node_addr();
            info!("Starting node at address: {}", sockert_addr);
            info!("Will try connect to nodes: {:?}", connect_nodes);
            let connect_nodes: HashSet<ConnectNode> = connect_nodes.into_iter().collect();
            Server::new(Arc::new(RwLock::new(blockchain)))
                .run(&sockert_addr, connect_nodes)
                .await;
        }
    }
}
