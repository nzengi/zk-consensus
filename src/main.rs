use clap::Parser;
use tracing::{info, error};
use tracing_subscriber;
use std::sync::Arc;

mod consensus;
mod zk_proof;
mod network;
mod storage;
mod types;

use consensus::ConsensusEngine;
use zk_proof::ZKProofGenerator;
use network::NetworkManager;
use storage::StorageManager;

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
    
    info!("Starting ZK Consensus Node");
    info!("Mode: {}", args.mode);
    info!("Port: {}", args.port);
    
    // Initialize components
    let storage = StorageManager::new("zk_consensus.db")?;
    let zk_generator = ZKProofGenerator::new()?;
    let mut consensus = ConsensusEngine::new(zk_generator, storage)?;
    let mut network = NetworkManager::new(args.port, args.bootstrap, &mut consensus)?;
    
    info!("All components initialized successfully");
    
    // Start network
    network.start().await?;
    
    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");
    
    Ok(())
}
