use sha2::{Sha256, Digest};

use bincode::serialize;
use serde::Serialize;

use crate::wallet::Wallet;


// Transactions
// Transactions are 2 types
// Coinbase transactions -- the first transaction in a block
// Regular transactions -- all other transactions
// A regular transaction must have at least one input(Unspent) and one output


pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub txid: String,
}

pub struct Input {
    pub txid: String,
    pub index: u32,
    pub amount: u64,
    pub address: String,
    pub signature: String,
}

pub struct Output {
    pub amount: u64,
    pub address: String,
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serialized = serialize(self).unwrap();
        serializer.serialize_bytes(&serialized)
    }
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            inputs: Vec::new(),
            outputs: Vec::new(),
            txid: String::new(),
        }
    }

    pub fn add_input(&mut self, txid: &str, index: u32, amount: u64, address: &str, signature: &str) {
        self.inputs.push(Input {
            txid: txid.to_string(),
            index,
            amount,
            address: address.to_string(),
            signature: signature.to_string(),
        });
    }

    pub fn add_output(&mut self, amount: u64, address: &str) {
        self.outputs.push(Output {
            amount,
            address: address.to_string(),
        });
    }

    pub fn finalize(&mut self) {
        let mut hasher = Sha256::new();
        let serialized = bincode::serialize(&self).unwrap();
        hasher.update(&serialized);
        self.txid = hex::encode(hasher.finalize());
    }
}

pub fn sign_input(tx: &Transaction, index: usize, wallet: Wallet) -> String {
    let input = &tx.inputs[index];
    let mut hasher = Sha256::new();
    hasher.update(&tx.txid);
    hasher.update(&input.index.to_be_bytes());
    hasher.update(&input.amount.to_be_bytes());
    hasher.update(input.address.as_bytes());
    let sig_hash = hasher.finalize();

    wallet.sign(&hex::encode(sig_hash))
}



