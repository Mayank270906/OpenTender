use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tender {
    pub id: u64,
    pub creator: String,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
    pub deadline: u64,
    pub reveal_deadline: u64,
    pub min_bid: i128,
    pub is_closed: bool,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenderRequest {
    pub creator: String,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
    pub deadline: u64,
    pub reveal_deadline: u64,
    pub min_bid: i128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    pub bidder: String,
    pub tender_id: u64,
    pub encrypted_amount: String,
    pub revealed_amount: Option<i128>,
    pub is_valid: bool,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitBidRequest {
    pub tender_id: u64,
    pub bidder: String,
    pub encrypted_amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevealBidRequest {
    pub tender_id: u64,
    pub bidder: String,
    pub actual_amount: i128,
    pub decryption_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Winner {
    pub tender_id: u64,
    pub bidder: String,
    pub amount: i128,
    pub selected_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseTenderRequest {
    pub tender_id: u64,
    pub caller: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub amount: i128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    pub encrypted_amount: String,
    pub decryption_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub encrypted_amount: String,
    pub decryption_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub amount: i128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}