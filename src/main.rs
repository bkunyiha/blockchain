use blockchain::{
    ADDRESS_CHECK_SUM_LEN, Blockchain, CENTERAL_NODE, ConnectNode, GLOBAL_CONFIG, Server,
    Transaction, UTXOSet, Wallets, base58_decode, convert_address, hash_pub_key, send_tx,
    validate_address,
};
use data_encoding::HEXLOWER;
use log::LevelFilter;
use structopt::StructOpt;

const MINE_TRUE: usize = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "blockchain")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "createblockchain", about = "Create a new blockchain")]
    Createblockchain {
        #[structopt(name = "address", help = "The address to send genesis block reward to")]
        address: String,
    },
    #[structopt(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    #[structopt(
        name = "getbalance",
        about = "Get the wallet balance of the target address"
    )]
    GetBalance {
        #[structopt(name = "address", help = "The wallet address")]
        address: String,
    },
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
        #[structopt(name = "miner", help = "Enable mining mode and send reward to ADDRESS")]
        miner: Option<String>,
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
        Command::Createblockchain { address } => {
            let blockchain = Blockchain::create_blockchain(address.as_str());
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            println!("Done!");
        }
        Command::Createwallet => {
            let mut wallet = Wallets::new();
            let address = wallet.create_wallet();
            println!("Your new address: {}", address)
        }
        Command::GetBalance { address } => {
            let address_valid = validate_address(address.as_str());
            if !address_valid {
                panic!("ERROR: Address is not valid")
            }
            let payload = base58_decode(address.as_str());
            let pub_key_hash = &payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN];

            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            let utxos = utxo_set.find_utxo(pub_key_hash);
            let mut balance = 0;
            for utxo in utxos {
                balance += utxo.get_value();
            }
            println!("Balance of {}: {}", address, balance);
        }
        Command::ListAddresses => {
            let wallets = Wallets::new();
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
            if !validate_address(from.as_str()) {
                panic!("ERROR: Sender address is not valid")
            }
            if !validate_address(to.as_str()) {
                panic!("ERROR: Recipient address is not valid")
            }
            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain.clone());

            let transaction =
                Transaction::new_utxo_transaction(from.as_str(), to.as_str(), amount, &utxo_set);

            // If the node a mining node, it mines the block.
            if mine == MINE_TRUE {
                // If the node is a mining node, it creates a new coinbase transaction.
                // The `new_coinbase_tx` function creates a new coinbase transaction.
                // It uses the `from` parameter to set the address of the sender.
                // It returns the new transaction.
                let coinbase_tx = Transaction::new_coinbase_tx(from.as_str());

                let block = blockchain.mine_block(&[transaction, coinbase_tx]);

                utxo_set.update(&block);
            } else {
                send_tx(&CENTERAL_NODE, &transaction).await;
            }
            println!("Success!")
        }
        Command::Printchain => {
            let mut block_iterator = Blockchain::new_blockchain().iterator();
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
                            let address = convert_address(pub_key_hash.as_slice());
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
                        let address = convert_address(pub_key_hash);
                        println!("-- Output value = {}, to = {}", output.get_value(), address,)
                    }
                }
                println!()
            }
        }
        Command::Reindexutxo => {
            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            let count = utxo_set.count_transactions();
            println!("Done! There are {} transactions in the UTXO set.", count);
        }
        Command::StartNode {
            miner,
            connect_nodes,
        } => {
            if let Some(addr) = miner {
                if !validate_address(addr.as_str()) {
                    panic!("Wrong miner address!")
                }
                println!("Mining is on. Address to receive rewards: {}", addr);
                GLOBAL_CONFIG.set_mining_addr(addr.parse().unwrap());
            }

            let blockchain = Blockchain::new_blockchain();
            let sockert_addr = GLOBAL_CONFIG.get_node_addr();
            println!("Starting node at address: {}", sockert_addr);
            println!("Will try connect to nodes: {:?}", connect_nodes);
            Server::new(blockchain.clone())
                .run(&sockert_addr, connect_nodes)
                .await;
        }
    }
}
