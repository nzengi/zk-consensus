use clap::Parser;
use tracing::{info, warn};
use tracing_subscriber;
use std::sync::Arc;
use tokio::sync::Mutex;

mod consensus;
mod zk_proof;
mod network;
mod storage;
mod types;

use consensus::ConsensusEngine;
use zk_proof::ZKProofGenerator;
use network::NetworkManager;
use storage::StorageManager;
use types::Transaction;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Node mode: validator, full_node, light_client
    #[arg(short, long, default_value = "validator")]
    mode: String,
    
    /// Network port
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    
    /// Bootstrap nodes
    #[arg(short, long)]
    bootstrap: Vec<String>,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

async fn create_test_transactions(storage: &StorageManager) -> Result<(), Box<dyn std::error::Error>> {
    info!("💰 Creating test transactions");
    
    for i in 0..5 {
        let tx = Transaction {
            id: [i; 32],
            from: [(i + 1) as u8; 32],
            to: [(i + 2) as u8; 32],
            amount: (i + 1) as u64 * 100,
            timestamp: chrono::Utc::now(),
            signature: vec![0u8; 64],
        };
        
        storage.store_transaction(&tx).await?;
        info!("📝 Created transaction #{}: {} tokens", i, tx.amount);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();
    
    info!("🚀 Starting ZK-PoV Consensus Node");
    info!("📋 Mode: {}", args.mode);
    info!("🌐 Port: {}", args.port);
    info!("🔗 Bootstrap nodes: {:?}", args.bootstrap);
    
    // Initialize components
    let storage = StorageManager::new("zk_consensus.db")?;
    let zk_generator = ZKProofGenerator::new()?;
    
    // Create test transactions
    create_test_transactions(&storage).await?;
    
    let consensus = ConsensusEngine::new(zk_generator, storage)?;
    let consensus = Arc::new(Mutex::new(consensus));
    
    info!("✅ All components initialized successfully");
    
    // Clone for network manager
    let consensus_for_network = consensus.clone();
    
    // Start consensus and network in parallel
    tokio::select! {
        result = async {
            let mut c = consensus.lock().await;
            c.start().await
        } => {
            if let Err(e) = result {
                warn!("❌ Consensus engine stopped with error: {}", e);
            }
        }
        result = async {
            // For now, just simulate network activity
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                info!("📡 Network heartbeat");
            }
            #[allow(unreachable_code)]
            Ok::<(), Box<dyn std::error::Error>>(())
        } => {
            if let Err(e) = result {
                warn!("❌ Network manager stopped with error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Received shutdown signal");
        }
    }
    
    info!("👋 Shutting down ZK-PoV Consensus Node");
    Ok(())
}
