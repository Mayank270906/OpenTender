use actix_web::{web, HttpResponse};
use crate::config::Config;
use crate::models::types::*;
use crate::services::{encryption, stellar};

/// Submit a bid
pub async fn submit_bid(
    config: web::Data<Config>,
    req: web::Json<SubmitBidRequest>,
) -> HttpResponse {
    log::info!("Submitting bid for tender {}", req.tender_id);
    
    match stellar::submit_bid(&config, req.into_inner()).await {
        Ok(_) => {
            log::info!("Bid submitted successfully");
            HttpResponse::Ok().json(ApiResponse::success("Bid submitted"))
        }
        Err(e) => {
            log::error!("Failed to submit bid: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<String>::error(e.to_string()))
        }
    }
}

/// Reveal a bid
pub async fn reveal_bid(
    config: web::Data<Config>,
    req: web::Json<RevealBidRequest>,
) -> HttpResponse {
    log::info!("Revealing bid for tender {}", req.tender_id);
    
    match stellar::reveal_bid(&config, req.into_inner()).await {
        Ok(_) => {
            log::info!("Bid revealed successfully");
            HttpResponse::Ok().json(ApiResponse::success("Bid revealed"))
        }
        Err(e) => {
            log::error!("Failed to reveal bid: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<String>::error(e.to_string()))
        }
    }
}

/// Get bid details
pub async fn get_bid(
    config: web::Data<Config>,
    path: web::Path<(u64, String)>,
) -> HttpResponse {
    let (tender_id, bidder) = path.into_inner();
    log::info!("Fetching bid for tender {} from bidder {}", tender_id, bidder);
    
    match stellar::get_bid(&config, tender_id, &bidder).await {
        Ok(Some(bid)) => HttpResponse::Ok().json(ApiResponse::success(bid)),
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<Bid>::error("Bid not found".to_string())),
        Err(e) => {
            log::error!("Failed to fetch bid: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<Bid>::error(e.to_string()))
        }
    }
}

/// Encrypt amount (helper endpoint)
pub async fn encrypt_amount(
    config: web::Data<Config>,
    req: web::Json<EncryptRequest>,
) -> HttpResponse {
    log::info!("Encrypting amount");
    
    match encryption::encrypt_amount(req.amount, &config.encryption_key) {
        Ok((encrypted, key)) => {
            HttpResponse::Ok().json(ApiResponse::success(EncryptResponse {
                encrypted_amount: encrypted,
                decryption_key: key,
            }))
        }
        Err(e) => {
            log::error!("Failed to encrypt: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<EncryptResponse>::error(e.to_string()))
        }
    }
}

/// Decrypt amount (helper endpoint)
pub async fn decrypt_amount(
    config: web::Data<Config>,
    req: web::Json<DecryptRequest>,
) -> HttpResponse {
    log::info!("Decrypting amount");
    
    match encryption::decrypt_amount(
        &req.encrypted_amount,
        &req.decryption_key,
        &config.encryption_key,
    ) {
        Ok(amount) => {
            HttpResponse::Ok().json(ApiResponse::success(DecryptResponse { amount }))
        }
        Err(e) => {
            log::error!("Failed to decrypt: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<DecryptResponse>::error(e.to_string()))
        }
    }
}