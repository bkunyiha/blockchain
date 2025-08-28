// wallets

use super::wallet::Wallet;
use crate::domain::error::{BtcError, Result};
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
    pub fn new() -> Result<Wallets> {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
        };
        wallets.load_from_file()?;
        Ok(wallets)
    }

    pub fn create_wallet(&mut self) -> Result<String> {
        let wallet = Wallet::new()?;
        let address = wallet.get_address()?;
        self.wallets.insert(address.clone(), wallet);
        self.save_to_file()?;
        Ok(address)
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

    pub fn load_from_file(&mut self) -> Result<()> {
        let path = current_dir()
            .map_err(|e| BtcError::WalletsFilePathError(e.to_string()))?
            .join(self.get_wallet_file_path());
        if !path.exists() {
            return Ok(());
        }
        let mut file =
            File::open(path).map_err(|e| BtcError::WalletsFileOpenError(e.to_string()))?;
        let metadata = file
            .metadata()
            .map_err(|e| BtcError::WalletsFileMetadataError(e.to_string()))?;
        let mut buf = vec![0; metadata.len() as usize];
        let _ = file
            .read(&mut buf)
            .map_err(|e| BtcError::WalletsFileReadError(e.to_string()))?;
        let wallets = bincode::serde::decode_from_slice(&buf[..], bincode::config::standard())
            .map_err(|e| BtcError::WalletsDeserializationError(e.to_string()))?
            .0;
        self.wallets = wallets;
        Ok(())
    }

    fn save_to_file(&self) -> Result<()> {
        let path = current_dir()
            .map_err(|e| BtcError::WalletsFilePathError(e.to_string()))?
            .join(self.get_wallet_file_path());
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)
            .map_err(|e| BtcError::SavingWalletsError(e.to_string()))?;
        let mut writer = BufWriter::new(file);
        let wallets_bytes =
            bincode::serde::encode_to_vec(&self.wallets, bincode::config::standard())
                .map_err(|e| BtcError::WalletsSerializationError(e.to_string()))?;
        writer
            .write_all(wallets_bytes.as_slice())
            .map_err(|e| BtcError::SavingWalletsError(e.to_string()))?;
        writer
            .flush()
            .map_err(|e| BtcError::SavingWalletsError(e.to_string()))?;
        Ok(())
    }

    pub fn get_wallet_file_path(&self) -> String {
        env::var("WALLET_FILE").unwrap_or(DEFAULT_WALLET_FILE.to_string())
    }
}
