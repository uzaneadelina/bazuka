use super::db::{KvStore, KvStoreError, StringKey};
use super::primitives::{Address, Block, Money, Transaction};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("kvstore error happened")]
    KvStoreError(#[from] KvStoreError),
}

pub trait Blockchain {
    fn get_balance(&self, addr: Address) -> Result<Money, BlockchainError>;
    fn extend(&mut self, blocks: &Vec<Block>) -> Result<(), BlockchainError>;
    fn get_height(&self) -> Result<usize, BlockchainError>;
}

pub trait Identifiable {
    fn get_key(&self) -> StringKey;
}

impl Identifiable for Address {
    fn get_key(&self) -> StringKey {
        StringKey::new(&format!("addr_{:?}", self))
    }
}

pub struct KvStoreChain<K: KvStore> {
    database: K,
}

impl<K: KvStore> KvStoreChain<K> {
    pub fn new(kv_store: K) -> KvStoreChain<K> {
        KvStoreChain::<K> { database: kv_store }
    }
    fn apply_tx(tx: &Transaction) {}
}

impl<K: KvStore> Blockchain for KvStoreChain<K> {
    fn get_balance(&self, addr: Address) -> Result<Money, BlockchainError> {
        Ok(match self.database.get(addr.get_key())? {
            Some(b) => b.as_u32()?,
            None => 0,
        })
    }
    fn extend(&mut self, _blocks: &Vec<Block>) -> Result<(), BlockchainError> {
        unimplemented!();
    }
    fn get_height(&self) -> Result<usize, BlockchainError> {
        Ok(match self.database.get(StringKey::new("height"))? {
            Some(b) => b.as_usize()?,
            None => 0,
        })
    }
}
