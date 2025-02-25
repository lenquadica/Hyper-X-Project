mod consensus;
mod transactions;
mod governance;
mod cryptography;
mod storage;
mod p2p;
mod smart_contracts;
mod explorer;
mod wallet;
mod tokenomics;
mod crosschain;
mod dex;

use tokio::sync::mpsc;
use transactions::Transaction;
use consensus::Consensus;
use storage::Blockchain;
use p2p::P2PNode;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    println!("🚀 Hyper-X Blockchain Node Booting...");

    let (tx, mut rx) = mpsc::channel(32);
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let mut p2p_node = P2PNode::new().await.expect("Failed to create P2P node");

    // Start P2P Networking
    tokio::spawn(async move {
        p2p_node.run().await;
    });

    // Transaction processing loop
    let blockchain_clone = blockchain.clone();
    tokio::spawn(async move {
        while let Some(txn) = rx.recv().await {
            println!("Processing transaction: {:?}", txn);
            blockchain_clone.lock().unwrap().add_transaction(txn);
        }
    });

    // Start API Services
    tokio::spawn(explorer::start_explorer(blockchain.clone()));
    tokio::spawn(wallet::start_wallet_api(blockchain.clone()));

    println!("🌍 Hyper-X Node Running ✅");
}
