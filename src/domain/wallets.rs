// wallets

use super::wallet::Wallet;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};

pub const DEFAULT_WALLET_FILE: &str = "wallet1.dat";

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Wallets {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
        };
        wallets.load_from_file();
        wallets
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        self.save_to_file();
        address
    }

    pub fn get_addresses(&self) -> Vec<String> {
        let mut addresses = vec![];
        for address in self.wallets.keys() {
            addresses.push(address.clone())
        }
        addresses
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        if let Some(wallet) = self.wallets.get(address) {
            Some(wallet)
        } else {
            None
        }
    }

    pub fn load_from_file(&mut self) {
        let path = current_dir().unwrap().join(self.get_wallet_file_path());
        if !path.exists() {
            return;
        }
        let mut file = File::open(path).unwrap();
        let metadata = file.metadata().expect("unable to read metadata");
        let mut buf = vec![0; metadata.len() as usize];
        let _ = file.read(&mut buf).expect("buffer overflow");
        let wallets = bincode::deserialize(&buf[..]).expect("unable to deserialize file data");
        self.wallets = wallets;
    }

    fn save_to_file(&self) {
        let path = current_dir().unwrap().join(self.get_wallet_file_path());
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)
            .expect("unable to open wallet.dat");
        let mut writer = BufWriter::new(file);
        let wallets_bytes = bincode::serialize(&self.wallets).expect("unable to serialize wallets");
        writer.write_all(wallets_bytes.as_slice()).unwrap();
        let _ = writer.flush();
    }

    pub fn get_wallet_file_path(&self) -> String {
        env::var("WALLET_FILE").unwrap_or(DEFAULT_WALLET_FILE.to_string())
    }
}

impl Default for Wallets {
    fn default() -> Self {
        Self::new()
    }
}
