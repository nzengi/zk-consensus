use crate::types::{
    Block, BlockHeader, BlockHash, NodeId, ConsensusState, ConsensusMessage, 
    BlockVote, VoteType, ValidatorInfo, ZKProof
};
use crate::zk_proof::ZKProofGenerator;
use crate::storage::StorageManager;
use chrono::{DateTime, Utc, Duration};
use sha2::{Sha256, Digest};
use anyhow::Result;
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use std::collections::HashMap;

pub struct ConsensusEngine {
    zk_generator: Arc<ZKProofGenerator>,
    storage: Arc<StorageManager>,
    state: Arc<RwLock<ConsensusState>>,
    node_id: NodeId,
    message_tx: mpsc::Sender<ConsensusMessage>,
    message_rx: mpsc::Receiver<ConsensusMessage>,
    block_time: Duration,
    min_validators: usize,
}

impl ConsensusEngine {
    pub fn new(
        zk_generator: ZKProofGenerator,
        storage: StorageManager,
    ) -> Result<Self> {
        info!("Initializing ZK-PoV Consensus Engine");
        
        let node_id = Self::generate_node_id();
        let (message_tx, message_rx) = mpsc::channel(1000);
        
        let state = ConsensusState {
            current_block: 0,
            validators: HashMap::new(),
            total_stake: 0,
            epoch: 0,
        };
        
        Ok(Self {
            zk_generator: Arc::new(zk_generator),
            storage: Arc::new(storage),
            state: Arc::new(RwLock::new(state)),
            node_id,
            message_tx,
            message_rx,
            block_time: Duration::seconds(12), // 12 second block time
            min_validators: 3,
        })
    }
    
    fn generate_node_id() -> NodeId {
        let mut hasher = Sha256::new();
        hasher.update(&rand::random::<[u8; 32]>());
        hasher.update(&chrono::Utc::now().timestamp().to_le_bytes());
        hasher.finalize().into()
    }
    
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting ZK-PoV Consensus Engine");
        
        // Start consensus loop
        self.consensus_loop().await?;
        
        Ok(())
    }
    
    async fn consensus_loop(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                message = self.message_rx.recv() => {
                    if let Some(msg) = message {
                        self.handle_message(msg).await?;
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    self.tick().await?;
                }
            }
        }
    }
    
    async fn handle_message(&mut self, message: ConsensusMessage) -> Result<()> {
        match message {
            ConsensusMessage::NewBlock(block) => {
                self.handle_new_block(block).await?;
            }
            ConsensusMessage::BlockVote(vote) => {
                self.handle_block_vote(vote).await?;
            }
            ConsensusMessage::ConsensusState(state) => {
                self.handle_consensus_state(state).await?;
            }
            ConsensusMessage::ZKProofRequest(request) => {
                self.handle_proof_request(request).await?;
            }
            ConsensusMessage::ZKProofResponse(response) => {
                self.handle_proof_response(response).await?;
            }
        }
        Ok(())
    }
    
    async fn handle_new_block(&mut self, block: Block) -> Result<()> {
        debug!("Received new block {}", block.header.block_number);
        
        // Verify ZK proof
        if !self.zk_generator.verify_proof(&block.zk_proof).await? {
            warn!("Invalid ZK proof for block {}", block.header.block_number);
            return Ok(());
        }
        
        // Verify block structure
        if !self.verify_block_structure(&block).await? {
            warn!("Invalid block structure for block {}", block.header.block_number);
            return Ok(());
        }
        
        // Store block
        self.storage.store_block(&block).await?;
        
        // Vote on block
        let vote = BlockVote {
            block_hash: block.hash(),
            validator: self.node_id,
            vote: VoteType::Approve,
            timestamp: Utc::now(),
            signature: vec![], // TODO: Implement proper signing
        };
        
        // Broadcast vote
        self.broadcast_vote(vote).await?;
        
        info!("Processed new block {}", block.header.block_number);
        Ok(())
    }
    
    async fn handle_block_vote(&mut self, vote: BlockVote) -> Result<()> {
        debug!("Received vote for block {:?}", vote.block_hash);
        
        // Verify vote signature
        if !self.verify_vote_signature(&vote).await? {
            warn!("Invalid vote signature");
            return Ok(());
        }
        
        // Store vote
        self.storage.store_vote(&vote).await?;
        
        // Check if we have enough votes for finality
        self.check_block_finality(vote.block_hash).await?;
        
        Ok(())
    }
    
    async fn handle_consensus_state(&mut self, state: ConsensusState) -> Result<()> {
        debug!("Received consensus state update");
        
        let mut current_state = self.state.write().await;
        *current_state = state;
        
        Ok(())
    }
    
    async fn handle_proof_request(&mut self, request: crate::types::ProofRequest) -> Result<()> {
        debug!("Received ZK proof request for block {}", request.block_number);
        
        // Generate proof for requested block
        if let Some(block) = self.storage.get_block(request.block_number).await? {
            let proof = self.zk_generator.generate_proof(&block).await?;
            
            let response = crate::types::ProofResponse {
                request_id: request.request_id,
                proof,
                responder: self.node_id,
            };
            
            // Send response back to requester
            // TODO: Implement network response
        }
        
        Ok(())
    }
    
    async fn handle_proof_response(&mut self, response: crate::types::ProofResponse) -> Result<()> {
        debug!("Received ZK proof response");
        
        // Verify the proof
        if self.zk_generator.verify_proof(&response.proof).await? {
            info!("Verified ZK proof response");
        } else {
            warn!("Invalid ZK proof response");
        }
        
        Ok(())
    }
    
    async fn tick(&mut self) -> Result<()> {
        // Check if it's time to propose a new block
        if self.should_propose_block().await? {
            self.propose_new_block().await?;
        }
        
        // Update consensus state
        self.update_consensus_state().await?;
        
        Ok(())
    }
    
    async fn should_propose_block(&self) -> Result<bool> {
        let state = self.state.read().await;
        
        // Check if we're a validator
        if !state.validators.contains_key(&self.node_id) {
            return Ok(false);
        }
        
        // Check if enough time has passed since last block
        if let Some(last_block) = self.storage.get_latest_block().await? {
            let time_since_last = Utc::now() - last_block.header.timestamp;
            if time_since_last < self.block_time {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    async fn propose_new_block(&mut self) -> Result<()> {
        info!("Proposing new block");
        
        let state = self.state.read().await;
        let block_number = state.current_block + 1;
        
        // Get pending transactions
        let transactions = self.storage.get_pending_transactions().await?;
        
        // Create block header
        let parent_hash = if let Some(last_block) = self.storage.get_latest_block().await? {
            last_block.hash()
        } else {
            [0; 32]
        };
        
        let merkle_root = self.calculate_merkle_root(&transactions);
        
        let header = BlockHeader {
            block_number,
            parent_hash,
            timestamp: Utc::now(),
            merkle_root,
            validator: self.node_id,
            difficulty: self.calculate_difficulty().await?,
            nonce: 0,
        };
        
        // Create block
        let mut block = Block {
            header,
            transactions,
            zk_proof: ZKProof {
                proof_data: vec![],
                public_inputs: vec![],
                verification_key: vec![],
                proof_type: crate::types::ProofType::Groth16,
            },
        };
        
        // Generate ZK proof
        block.zk_proof = self.zk_generator.generate_proof(&block).await?;
        
        // Store block
        self.storage.store_block(&block).await?;
        
        // Broadcast block
        self.broadcast_block(block).await?;
        
        info!("Proposed block {}", block_number);
        Ok(())
    }
    
    async fn verify_block_structure(&self, block: &Block) -> Result<bool> {
        // Verify block number is sequential
        let state = self.state.read().await;
        if block.header.block_number != state.current_block + 1 {
            return Ok(false);
        }
        
        // Verify parent hash
        if let Some(last_block) = self.storage.get_latest_block().await? {
            if block.header.parent_hash != last_block.hash() {
                return Ok(false);
            }
        }
        
        // Verify merkle root
        let calculated_root = self.calculate_merkle_root(&block.transactions);
        if block.header.merkle_root != calculated_root {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    async fn verify_vote_signature(&self, vote: &BlockVote) -> Result<bool> {
        // TODO: Implement proper signature verification
        Ok(true)
    }
    
    async fn check_block_finality(&self, block_hash: BlockHash) -> Result<()> {
        let votes = self.storage.get_votes_for_block(block_hash).await?;
        let state = self.state.read().await;
        
        let approve_votes = votes.iter()
            .filter(|v| matches!(v.vote, VoteType::Approve))
            .count();
        
        if approve_votes >= self.min_validators {
            info!("Block {:?} reached finality with {} votes", block_hash, approve_votes);
            
            // Update consensus state
            let mut state = self.state.write().await;
            state.current_block += 1;
        }
        
        Ok(())
    }
    
    async fn calculate_difficulty(&self) -> Result<u64> {
        // Simple difficulty calculation based on block time
        // In practice, this would be more sophisticated
        Ok(1000)
    }
    
    fn calculate_merkle_root(&self, transactions: &[crate::types::Transaction]) -> BlockHash {
        if transactions.is_empty() {
            return [0; 32];
        }
        
        let mut hashes: Vec<BlockHash> = transactions.iter()
            .map(|tx| {
                let mut hasher = Sha256::new();
                hasher.update(&bincode::serialize(tx).unwrap());
                hasher.finalize().into()
            })
            .collect();
        
        // Build merkle tree
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]); // Duplicate for odd number
                }
                new_hashes.push(hasher.finalize().into());
            }
            hashes = new_hashes;
        }
        
        hashes[0]
    }
    
    async fn broadcast_block(&self, block: Block) -> Result<()> {
        // TODO: Implement network broadcasting
        debug!("Broadcasting block {}", block.header.block_number);
        Ok(())
    }
    
    async fn broadcast_vote(&self, vote: BlockVote) -> Result<()> {
        // TODO: Implement network broadcasting
        debug!("Broadcasting vote for block {:?}", vote.block_hash);
        Ok(())
    }
    
    async fn update_consensus_state(&mut self) -> Result<()> {
        // Update validator performance scores
        let mut state = self.state.write().await;
        
        for validator in state.validators.values_mut() {
            // Simple performance scoring based on recent activity
            let time_since_last = Utc::now() - validator.last_block_time;
            if time_since_last < Duration::minutes(5) {
                validator.performance_score = (validator.performance_score + 0.1).min(1.0);
            } else {
                validator.performance_score = (validator.performance_score - 0.05).max(0.0);
            }
        }
        
        Ok(())
    }
    
    pub fn get_message_sender(&self) -> mpsc::Sender<ConsensusMessage> {
        self.message_tx.clone()
    }
} 