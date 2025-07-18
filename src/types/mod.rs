use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub type BlockHash = [u8; 32];
pub type NodeId = [u8; 32];
pub type ProofHash = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub zk_proof: ZKProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_number: u64,
    pub parent_hash: BlockHash,
    pub timestamp: DateTime<Utc>,
    pub merkle_root: BlockHash,
    pub validator: NodeId,
    pub difficulty: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: [u8; 32],
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKProof {
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub proof_type: ProofType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofType {
    Groth16,
    Plonk,
    Nova,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    pub current_block: u64,
    pub validators: HashMap<NodeId, ValidatorInfo>,
    pub total_stake: u64,
    pub epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub stake: u64,
    pub is_active: bool,
    pub last_block_time: DateTime<Utc>,
    pub performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    NewBlock(Block),
    BlockVote(BlockVote),
    ConsensusState(ConsensusState),
    ZKProofRequest(ProofRequest),
    ZKProofResponse(ProofResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockVote {
    pub block_hash: BlockHash,
    pub validator: NodeId,
    pub vote: VoteType,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Approve,
    Reject,
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRequest {
    pub block_number: u64,
    pub request_id: [u8; 32],
    pub requester: NodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResponse {
    pub request_id: [u8; 32],
    pub proof: ZKProof,
    pub responder: NodeId,
}

impl Block {
    pub fn hash(&self) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(&bincode::serialize(&self.header).unwrap());
        hasher.update(&bincode::serialize(&self.transactions).unwrap());
        hasher.finalize().into()
    }
    
    pub fn verify_zk_proof(&self) -> bool {
        // TODO: Implement ZK proof verification
        true
    }
}

impl BlockHeader {
    pub fn hash(&self) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(&bincode::serialize(self).unwrap());
        hasher.finalize().into()
    }
} 