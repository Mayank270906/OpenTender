use crate::config::Config;
use crate::models::types::*;
use std::process::Command;
use serde_json::Value;

/// Execute soroban CLI commands to interact with the contract

pub async fn get_all_tenders(config: &Config) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    log::info!("Calling contract {} to get all tenders", config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "GBF5GDLTILW6WKTMKXWCG23BNPQRIQJ7OKEAXVPSUGFP4HIPE3CEA77M",
            "--network", &config.network,
            "--",
            "get_all_tenders",
        ])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get tenders: {}", error).into());
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    parse_u64_vec(&result)
}

pub async fn get_tender(config: &Config, tender_id: u64) -> Result<Tender, Box<dyn std::error::Error>> {
    log::info!("Getting tender {} from contract {}", tender_id, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "GBF5GDLTILW6WKTMKXWCG23BNPQRIQJ7OKEAXVPSUGFP4HIPE3CEA77M",
            "--network", &config.network,
            "--",
            "get_tender",
            "--tender_id", &tender_id.to_string(),
        ])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get tender: {}", error).into());
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    parse_tender(&result)
}

pub async fn create_tender(
    config: &Config,
    req: CreateTenderRequest,
) -> Result<u64, Box<dyn std::error::Error>> {
    log::info!("Creating tender on contract {}", config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",  // Use the admin identity
            "--network", &config.network,
            "--",
            "create_tender",
            "--creator", &req.creator,
            "--title", &req.title,
            "--description", &req.description,
            "--ipfs_hash", &req.ipfs_hash,
            "--deadline", &req.deadline.to_string(),
            "--reveal_deadline", &req.reveal_deadline.to_string(),
            "--min_bid", &req.min_bid.to_string(),
        ])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        log::error!("Failed to create tender: {}", error);
        return Err(format!("Failed to create tender: {}", error).into());
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    log::info!("Create tender result: {}", result);
    
    // Parse the tender ID from the output
    result.trim().parse::<u64>()
        .map_err(|e| format!("Failed to parse tender ID: {}", e).into())
}

pub async fn submit_bid(
    config: &Config,
    req: SubmitBidRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Submitting bid for tender {} on contract {}", req.tender_id, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",
            "--network", &config.network,
            "--",
            "submit_bid",
            "--tender_id", &req.tender_id.to_string(),
            "--bidder", &req.bidder,
            "--encrypted_amount", &req.encrypted_amount,
        ])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to submit bid: {}", error).into());
    }
    
    Ok(())
}

pub async fn reveal_bid(
    config: &Config,
    req: RevealBidRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Revealing bid for tender {} on contract {}", req.tender_id, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",
            "--network", &config.network,
            "--",
            "reveal_bid",
            "--tender_id", &req.tender_id.to_string(),
            "--bidder", &req.bidder,
            "--actual_amount", &req.actual_amount.to_string(),
            "--_decryption_key", &req.decryption_key,
        ])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to reveal bid: {}", error).into());
    }
    
    Ok(())
}

pub async fn close_tender(
    config: &Config,
    tender_id: u64,
    caller: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Closing tender {} by {} on contract {}", tender_id, caller, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",
            "--network", &config.network,
            "--",
            "close_tender",
            "--tender_id", &tender_id.to_string(),
            "--caller", caller,
        ])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to close tender: {}", error).into());
    }
    
    Ok(())
}

pub async fn get_winner(
    config: &Config,
    tender_id: u64,
) -> Result<Option<Winner>, Box<dyn std::error::Error>> {
    log::info!("Getting winner for tender {} from contract {}", tender_id, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",
            "--network", &config.network,
            "--",
            "get_winner",
            "--tender_id", &tender_id.to_string(),
        ])
        .output()?;
    
    if !output.status.success() {
        // Winner might not exist yet, that's okay
        return Ok(None);
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    if result.trim().is_empty() || result.contains("null") {
        return Ok(None);
    }
    
    let winner = parse_winner(&result)?;
    Ok(Some(winner))
}

pub async fn get_bid(
    config: &Config,
    tender_id: u64,
    bidder: &str,
) -> Result<Option<Bid>, Box<dyn std::error::Error>> {
    log::info!("Getting bid for tender {} from bidder {} on contract {}", 
        tender_id, bidder, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",
            "--network", &config.network,
            "--",
            "get_bid",
            "--tender_id", &tender_id.to_string(),
            "--bidder", bidder,
        ])
        .output()?;
    
    if !output.status.success() {
        return Ok(None);
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    if result.trim().is_empty() || result.contains("null") {
        return Ok(None);
    }
    
    let bid = parse_bid(&result)?;
    Ok(Some(bid))
}

pub async fn get_tender_bidders(
    config: &Config,
    tender_id: u64,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    log::info!("Getting bidders for tender {} from contract {}", tender_id, config.contract_id);
    
    let output = Command::new("soroban")
        .args([
            "contract", "invoke",
            "--id", &config.contract_id,
            "--source-account", "moon",
            "--network", &config.network,
            "--",
            "get_tender_bidders",
            "--tender_id", &tender_id.to_string(),
        ])
        .output()?;
    
    if !output.status.success() {
        return Ok(vec![]);
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    parse_string_vec(&result)
}

// Parsing helper functions
fn parse_u64_vec(output: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let trimmed = output.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return Ok(vec![]);
    }
    
    // Parse JSON array: [1, 2, 3]
    let json: Value = serde_json::from_str(trimmed)?;
    if let Some(arr) = json.as_array() {
        let ids: Vec<u64> = arr.iter()
            .filter_map(|v| v.as_u64())
            .collect();
        Ok(ids)
    } else {
        Ok(vec![])
    }
}

fn parse_string_vec(output: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let trimmed = output.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return Ok(vec![]);
    }
    
    let json: Value = serde_json::from_str(trimmed)?;
    if let Some(arr) = json.as_array() {
        let strings: Vec<String> = arr.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
        Ok(strings)
    } else {
        Ok(vec![])
    }
}

fn parse_tender(output: &str) -> Result<Tender, Box<dyn std::error::Error>> {
    let json: Value = serde_json::from_str(output.trim())?;
    
    Ok(Tender {
        id: json["id"].as_u64().unwrap_or(0),
        creator: json["creator"].as_str().unwrap_or("").to_string(),
        title: json["title"].as_str().unwrap_or("").to_string(),
        description: json["description"].as_str().unwrap_or("").to_string(),
        ipfs_hash: json["ipfs_hash"].as_str().unwrap_or("").to_string(),
        deadline: json["deadline"].as_u64().unwrap_or(0),
        reveal_deadline: json["reveal_deadline"].as_u64().unwrap_or(0),
        min_bid: json["min_bid"].as_str()
            .and_then(|s| s.parse::<i128>().ok())
            .or_else(|| json["min_bid"].as_i64().map(|i| i as i128))
            .unwrap_or(0),
        is_closed: json["is_closed"].as_bool().unwrap_or(false),
        created_at: json["created_at"].as_u64().unwrap_or(0),
    })
}

fn parse_winner(output: &str) -> Result<Winner, Box<dyn std::error::Error>> {
    let json: Value = serde_json::from_str(output.trim())?;
    
    Ok(Winner {
        tender_id: json["tender_id"].as_u64().unwrap_or(0),
        bidder: json["bidder"].as_str().unwrap_or("").to_string(),
        amount: json["amount"].as_str()
            .and_then(|s| s.parse::<i128>().ok())
            .or_else(|| json["amount"].as_i64().map(|i| i as i128))
            .unwrap_or(0),
        selected_at: json["selected_at"].as_u64().unwrap_or(0),
    })
}

fn parse_bid(output: &str) -> Result<Bid, Box<dyn std::error::Error>> {
    let json: Value = serde_json::from_str(output.trim())?;
    
    let revealed_amount = if json["revealed_amount"].is_null() {
        None
    } else {
        json["revealed_amount"].as_str()
            .and_then(|s| s.parse::<i128>().ok())
            .or_else(|| json["revealed_amount"].as_i64().map(|i| i as i128))
    };
    
    Ok(Bid {
        bidder: json["bidder"].as_str().unwrap_or("").to_string(),
        tender_id: json["tender_id"].as_u64().unwrap_or(0),
        encrypted_amount: json["encrypted_amount"].as_str().unwrap_or("").to_string(),
        revealed_amount,
        is_valid: json["is_valid"].as_bool().unwrap_or(false),
        timestamp: json["timestamp"].as_u64().unwrap_or(0),
    })
}