use super::transaction::Transaction;
use data_encoding::HEXLOWER;
use std::collections::HashMap;
use std::sync::RwLock;

/// The `MemoryPool` struct is used to store transactions that are in the memory pool.
///
/// # Fields
///
/// `inner_tx` - A `RwLock` that holds a `HashMap` of `Transaction`s.
///
pub struct MemoryPool {
    inner_tx: RwLock<HashMap<String, Transaction>>,
}

impl MemoryPool {
    pub fn new() -> MemoryPool {
        MemoryPool {
            inner_tx: RwLock::new(HashMap::new()),
        }
    }

    pub fn contains(&self, txid_hex: &str) -> bool {
        self.inner_tx.read().unwrap().contains_key(txid_hex)
    }

    pub fn contains_transaction(&self, tx: &Transaction) -> bool {
        let txid_hex = HEXLOWER.encode(tx.get_id());
        self.contains(&txid_hex)
    }

    pub fn add(&self, tx: Transaction) {
        let txid_hex = HEXLOWER.encode(tx.get_id());
        self.inner_tx.write().unwrap().insert(txid_hex, tx);
    }

    pub fn get(&self, txid_hex: &str) -> Option<Transaction> {
        self.inner_tx.read().unwrap().get(txid_hex).cloned()
    }

    pub fn remove(&self, txid_hex: &str) {
        let mut inner = self.inner_tx.write().unwrap();
        inner.remove(txid_hex);
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        let inner = self.inner_tx.read().unwrap();
        let mut txs = vec![];
        for (_, v) in inner.iter() {
            txs.push(v.clone());
        }
        txs
    }

    pub fn len(&self) -> usize {
        self.inner_tx.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner_tx.read().unwrap().is_empty()
    }
}

/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// # Implementation
///
/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// This calls the `new` method to create a new `MemoryPool` instance.
impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

/// The `BlockInTransit` struct is used to store blocks that are in transit between nodes.
///
/// # Fields
///
/// `inner` - A `RwLock` that holds a `Vec` of `Vec<u8>`s.
///
pub struct BlockInTransit {
    inner: RwLock<Vec<Vec<u8>>>,
}

impl BlockInTransit {
    pub fn new() -> BlockInTransit {
        BlockInTransit {
            inner: RwLock::new(vec![]),
        }
    }

    pub fn add_blocks(&self, blocks: &[Vec<u8>]) {
        let mut inner = self.inner.write().unwrap();
        for hash in blocks {
            inner.push(hash.to_vec());
        }
    }

    pub fn first(&self) -> Option<Vec<u8>> {
        let inner = self.inner.read().unwrap();
        inner.first().map(|block_hash| block_hash.to_vec())
    }

    pub fn remove(&self, block_hash: &[u8]) -> Option<Vec<u8>> {
        let mut inner = self.inner.write().unwrap();
        if let Some(idx) = inner.iter().position(|x| x.eq(block_hash)) {
            inner.remove(idx);
            Some(block_hash.to_vec())
        } else {
            None
        }
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// # Implementation
///
/// The `Default` trait is implemented for the `MemoryPool` struct.
///
/// This calls the `new` method to create a new `MemoryPool` instance.
impl Default for BlockInTransit {
    fn default() -> Self {
        Self::new()
    }
}
