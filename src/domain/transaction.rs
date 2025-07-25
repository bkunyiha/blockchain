use super::blockchain::Blockchain;
use super::utxo_set::UTXOSet;
use super::wallet::{ADDRESS_CHECK_SUM_LEN, hash_pub_key};
use super::wallets::Wallets;
use crate::util::utils::{
    base58_decode, ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify, sha256_digest,
};
use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SUBSIDY: i32 = 10;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
        TXInput {
            txid: txid.to_vec(),
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let locking_hash = hash_pub_key(self.pub_key.as_slice());
        locking_hash.eq(pub_key_hash)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    in_global_mem_pool: bool,
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn new(value: i32, address: &str) -> TXOutput {
        let mut output = TXOutput {
            value,
            in_global_mem_pool: false,
            pub_key_hash: vec![],
        };
        output.lock(address);
        output
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    // The `lock` function locks the output to the address.
    // It uses the `base58_decode` function to decode the address.
    // It uses the `ADDRESS_CHECK_SUM_LEN` constant to get the length of the address check sum.
    // It uses the `pub_key_hash` field to store the public key hash.
    // It returns the new output.
    fn lock(&mut self, address: &str) {
        let payload = base58_decode(address);
        let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN].to_vec();
        self.pub_key_hash = pub_key_hash;
    }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }

    pub fn set_in_global_mem_pool(&mut self, value: bool) {
        self.in_global_mem_pool = value;
    }

    pub fn is_in_global_mem_pool(&self) -> bool {
        self.in_global_mem_pool
    }
    pub fn not_in_global_mem_pool(&self) -> bool {
        !self.in_global_mem_pool
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    // The `new_coinbase_tx` function creates a new coinbase transaction.
    // It uses the `SUBSIDY` constant to set the value of the transaction.
    // It uses the `to` parameter to set the address of the recipient.
    // It returns the new transaction.
    pub fn new_coinbase_tx(to: &str) -> Transaction {
        let txout = TXOutput::new(SUBSIDY, to);
        let tx_input = TXInput {
            signature: Uuid::new_v4().as_bytes().to_vec(),
            ..Default::default()
        };

        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_input],
            vout: vec![txout],
        };

        tx.id = tx.hash();
        tx
    }

    ///
    /// This function constructs a new UTXO-based transaction
    /// by selecting spendable outputs and creating inputs for the transaction.
    /// It calculates the inputs required based on available outputs,
    /// manages outputs for the recipient and change, signs the transaction,
    /// and computes its ID through hashing:
    ///
    /// # Arguments
    ///
    /// * `from` - The address of the sender.
    /// * `to` - The address of the recipient.
    pub fn new_utxo_transaction(
        from: &str,
        to: &str,
        amount: i32,
        utxo_set: &UTXOSet,
    ) -> Transaction {
        let wallets = Wallets::new();
        let from_wallet = wallets.get_wallet(from).expect("unable to found wallet");
        let public_key_hash = hash_pub_key(from_wallet.get_public_key());

        let (accumulated, valid_outputs) =
            utxo_set.find_spendable_outputs(public_key_hash.as_slice(), amount);
        if accumulated < amount {
            panic!("Error: Not enough funds")
        }

        let mut inputs = vec![];
        for (txid_hex, outs) in valid_outputs {
            let txid = HEXLOWER.decode(txid_hex.as_bytes()).unwrap();
            for out in outs {
                let input = TXInput {
                    txid: txid.clone(),
                    vout: out,
                    signature: vec![],
                    pub_key: from_wallet.get_public_key().to_vec(),
                };
                inputs.push(input);
            }
        }

        let mut outputs = vec![TXOutput::new(amount, to)];

        if accumulated > amount {
            outputs.push(TXOutput::new(accumulated - amount, from)) // to: Return change to the sender
        }

        // Create a new transaction with the spent inputs and unspent outputs
        let mut tx = Transaction {
            id: vec![],
            vin: inputs,
            vout: outputs,
        };
        tx.id = tx.hash();

        tx.sign(utxo_set.get_blockchain(), from_wallet.get_pkcs8());
        tx
    }

    ///
    /// `trimmed_copy` is an internal function that creates a trimmed copy of the transaction,
    /// excluding signatures, enabling signature verification without modifying
    /// the original transaction
    ///
    /// # Returns
    ///
    /// A trimmed copy of the transaction.
    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let txinput = TXInput::new(input.get_txid(), input.get_vout());
            inputs.push(txinput);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            id: self.id.clone(),
            vin: inputs,
            vout: outputs,
        }
    }

    ///
    /// The `sign` function signs the transaction inputs using the Elliptic Curve Digital Signature Algorithm
    /// (ECDSA). It retrieves previous transactions, prepares a copy for signature verification,
    /// signs inputs with the corresponding private keys, and updates the transaction with signatures:
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    /// * `pkcs8` - A reference to the private key.
    ///
    /// # Returns
    ///
    /// A signed transaction.
    fn sign(&mut self, blockchain: &Blockchain, pkcs8: &[u8]) {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.vin.iter_mut().enumerate() {
            let prev_tx_option = blockchain.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].signature = vec![];
            tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[idx].pub_key = vec![];

            let signature = ecdsa_p256_sha256_sign_digest(pkcs8, tx_copy.get_id());
            vin.signature = signature;
        }
    }

    ///
    /// This function verifies transaction signatures against corresponding public keys.
    /// It checks for Coinbase transactions, prepares a trimmed copy,
    /// validates signatures against public keys, and ensures the transaction is valid.
    ///
    /// # Arguments
    ///
    /// * `blockchain` - A reference to the blockchain.
    ///
    /// # Returns
    ///
    pub fn verify(&self, blockchain: &Blockchain) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut trimmed_self_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let current_vin_tx_option = blockchain.find_transaction(vin.get_txid());
            if current_vin_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let current_vin_tx = current_vin_tx_option.unwrap();
            trimmed_self_copy.vin[idx].signature = vec![];
            trimmed_self_copy.vin[idx].pub_key = current_vin_tx.vout[vin.vout].pub_key_hash.clone();
            trimmed_self_copy.id = trimmed_self_copy.hash();
            trimmed_self_copy.vin[idx].pub_key = vec![];

            let verify = ecdsa_p256_sha256_sign_verify(
                vin.pub_key.as_slice(),
                vin.signature.as_slice(),
                trimmed_self_copy.get_id(),
            );
            if !verify {
                return false;
            }
        }
        true
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1
            && self
                .vin
                .first()
                .iter()
                .any(|tx_in| tx_in.get_pub_key().is_empty())
    }

    ///
    /// The `hash` function generates the transaction's hash by creating a copy without the ID,
    /// serializing it, and computing its SHA-256 digest
    ///
    /// # Returns
    ///
    /// The transaction's hash.
    fn hash(&mut self) -> Vec<u8> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        sha256_digest(tx_copy.serialize().as_slice())
    }

    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    pub fn get_id_bytes(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub fn get_vin(&self) -> &[TXInput] {
        self.vin.as_slice()
    }

    pub fn get_vout(&self) -> &[TXOutput] {
        self.vout.as_slice()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn deserialize(bytes: &[u8]) -> Transaction {
        bincode::deserialize(bytes).unwrap()
    }
}
