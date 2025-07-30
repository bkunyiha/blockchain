use blockchain::{
    Blockchain, BtcError, CENTERAL_NODE, ConnectNode, GLOBAL_CONFIG, Result, Server, Transaction,
    UTXOSet, Wallets, convert_address, hash_pub_key, send_tx, validate_address,
};
use data_encoding::HEXLOWER;
use log::LevelFilter;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::RwLock;

const MINE_TRUE: usize = 1;

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
    Send {
        #[structopt(name = "from", help = "Source wallet address")]
        from: String,
        #[structopt(name = "to", help = "Destination wallet address")]
        to: String,
        #[structopt(name = "amount", help = "Amount to send")]
        amount: i32,
        #[structopt(name = "mine", help = "Mine immediately on the same node")]
        mine: usize,
    },
    #[structopt(name = "printchain", about = "Print blockchain all block")]
    Printchain,
    #[structopt(name = "reindexutxo", about = "rebuild UTXO index set")]
    Reindexutxo,
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
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let opt = Opt::from_args();
    match opt.command {
        Command::Createwallet => {
            let mut wallets = Wallets::new().unwrap();
            let address = wallets.create_wallet().unwrap();
            println!("Your new address: {}", address)
        }
        Command::ListAddresses => {
            let wallets = Wallets::new().unwrap();
            for address in wallets.get_addresses() {
                println!("{}", address)
            }
        }
        Command::Send {
            from,
            to,
            amount,
            mine,
        } => {
            if !validate_address(from.as_str()).unwrap() {
                panic!("ERROR: Sender address is not valid")
            }
            if !validate_address(to.as_str()).unwrap() {
                panic!("ERROR: Recipient address is not valid")
            }
            let blockchain = Blockchain::open_blockchain().unwrap();
            let utxo_set = UTXOSet::new(blockchain.clone());

            let transaction =
                Transaction::new_utxo_transaction(from.as_str(), to.as_str(), amount, &utxo_set)
                    .unwrap();

            // If the node a mining node, it mines the block.
            if mine == MINE_TRUE {
                // If the node is a mining node, it creates a new coinbase transaction.
                // The `new_coinbase_tx` function creates a new coinbase transaction.
                // It uses the `from` parameter to set the address of the sender.
                // It returns the new transaction.
                let coinbase_tx = Transaction::new_coinbase_tx(from.as_str()).unwrap();

                let block = blockchain.mine_block(&[transaction, coinbase_tx]).unwrap();

                utxo_set.update(&block).unwrap();
            } else {
                send_tx(&CENTERAL_NODE, &transaction).await;
            }
            println!("Success!")
        }
        Command::Printchain => {
            let mut block_iterator = Blockchain::open_blockchain().unwrap().iterator().unwrap();
            loop {
                let option = block_iterator.next();
                if option.is_none() {
                    break;
                }
                let block = option.unwrap();
                println!("Pre block hash: {}", block.get_pre_block_hash());
                println!("Cur block hash: {}", block.get_hash());
                println!("Cur block Timestamp: {}", block.get_timestamp());
                for tx in block.get_transactions() {
                    let cur_txid_hex = HEXLOWER.encode(tx.get_id());
                    println!("- Transaction txid_hex: {}", cur_txid_hex);

                    if !tx.is_coinbase() {
                        for input in tx.get_vin() {
                            let txid_hex = HEXLOWER.encode(input.get_txid());
                            let pub_key_hash = hash_pub_key(input.get_pub_key());
                            let address = convert_address(pub_key_hash.as_slice()).unwrap();
                            println!(
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
                        println!("-- Output value = {}, to = {}", output.get_value(), address,)
                    }
                }
                println!()
            }
        }
        Command::Reindexutxo => {
            let blockchain = Blockchain::open_blockchain().unwrap();
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex().unwrap();
            let count = utxo_set.count_transactions().unwrap();
            println!("Done! There are {} transactions in the UTXO set.", count);
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
                println!("Mining is on. Address to receive rewards: {}", wlt_addr);
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
                                println!(
                                    "Seed Node, Creating BlockChain With Address: {}",
                                    wlt_addr
                                );
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
                            println!("Blockchain error: {}", e);
                            Err(e)
                        }
                    }
                }
            };

            let blockchain = blockchain_result.unwrap();
            let sockert_addr = GLOBAL_CONFIG.get_node_addr();
            println!("Starting node at address: {}", sockert_addr);
            println!("Will try connect to nodes: {:?}", connect_nodes);
            let connect_nodes: HashSet<ConnectNode> = connect_nodes.into_iter().collect();
            Server::new(Arc::new(RwLock::new(blockchain)))
                .run(&sockert_addr, connect_nodes)
                .await;
        }
    }
}
