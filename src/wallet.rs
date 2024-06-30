// All the code for the wallet functionality will be in this file

extern crate secp256k1;
extern crate sha2;
extern crate rand;

use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use secp256k1::rand::rngs::OsRng;
use secp256k1::ecdsa::Signature;
use secp256k1::hashes::{sha256, Hash};
use sha2::{Sha256, Digest};

use crate::transaction::Transaction;


use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, to_vec};

#[derive(Parser, Debug)]
/// Wallet Operations
pub struct WalletCmd {
    name: String,
    #[clap(subcommand)]
    subcmd: WalletSubCommand,
}

#[derive(Debug, Parser)]
pub enum WalletSubCommand {
    /// Create a new wallet
    Create,
    /// Get the balance of a wallet
    Balance { address: String },
    /// Send coins from one wallet to another
    Send { from: String, to: String, amount: u64 },
    /// List all wallets
    List,
}

// implement the commands

pub fn create_wallet(name: &str)
{
    let wallet = Wallet::new(name.to_string());
    println!("Creating wallet with name: {}", name);
}

pub fn get_balance(address: &str) {
    println!("Getting balance for address: {}", address);
}

pub fn send(from: &str, to: &str, amount: u64)
{
    // test data
    let unspent_list = vec!["txid1", "txid2", "txid3"];
    // create

    let transaction = Transaction::new();

    println!("Sending {} coins from {} to {}", amount, from, to);
}

pub fn list_wallets() {
    println!("Listing all wallets");
}

// implement the function that is called from main.rs

pub fn handle_wallet_cmd(wallet_cmd: WalletCmd) {
    match wallet_cmd.subcmd {
        WalletSubCommand::Create => create_wallet(&wallet_cmd.name),
        WalletSubCommand::Balance { address } => get_balance(&address),
        WalletSubCommand::Send { from, to, amount } => send(&from, &to, amount),
        WalletSubCommand::List => list_wallets(),
    }
}

#[derive(Debug)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new(name: String) -> Wallet {
        let path = format!("{}.json", name);
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        let wallet = Wallet {
            private_key: hex::encode(secret_key.as_ref() as &[u8]),
            public_key: hex::encode(public_key.serialize()),
        };
        std::fs::write(path, to_vec(&wallet).unwrap()).unwrap();

        wallet

        // save the wallet to a file

    }

    pub fn sign(&self, message: &str) -> String {
        let secp = Secp256k1::new();
        let message = Message::from_slice(&Sha256::digest(message.as_bytes())).unwrap();
        let secret_key = SecretKey::from_slice(&hex::decode(&self.private_key).unwrap()).unwrap();
        let signature = secp.sign_ecdsa(&message, &secret_key);
        hex::encode(signature.serialize_compact())
    }

    pub fn address(&self) -> String {
        get_address(&self.public_key)
    }
}

pub fn verify_signature(public_key: &str, message: &str, signature: &str) -> bool {
    let secp = Secp256k1::new();
    let message = Message::from_slice(&Sha256::digest(message.as_bytes())).unwrap();
    let public_key = PublicKey::from_slice(&hex::decode(public_key).unwrap()).unwrap();
    let signature = Signature::from_compact(&hex::decode(signature).unwrap()).unwrap();
    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}



// Wallet to bytes
impl Serialize for Wallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let wallet_str = to_string(self).unwrap();
        serializer.serialize_str(&wallet_str)
    }
}



pub fn get_address(public_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    let result = hasher.finalize();
    let result_str = hex::encode(result);
    result_str
}