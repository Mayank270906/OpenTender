use actix_web::{web, HttpResponse};
use crate::config::Config;
use crate::models::types::*;
use crate::services::stellar;

/// Get all tenders
pub async fn get_all_tenders(config: web::Data<Config>) -> HttpResponse {
    log::info!("Fetching all tenders");
    
    match stellar::get_all_tenders(&config).await {
        Ok(tender_ids) => {
            let mut tenders = Vec::new();
            
            for id in tender_ids {
                if let Ok(tender) = stellar::get_tender(&config, id).await {
                    tenders.push(tender);
                }
            }
            
            HttpResponse::Ok().json(ApiResponse::success(tenders))
        }
        Err(e) => {
            log::error!("Failed to fetch tenders: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<Vec<Tender>>::error(e.to_string()))
        }
    }
}

/// Get tender by ID
pub async fn get_tender(
    config: web::Data<Config>,
    path: web::Path<u64>,
) -> HttpResponse {
    let tender_id = path.into_inner();
    log::info!("Fetching tender {}", tender_id);
    
    match stellar::get_tender(&config, tender_id).await {
        Ok(tender) => HttpResponse::Ok().json(ApiResponse::success(tender)),
        Err(e) => {
            log::error!("Failed to fetch tender {}: {}", tender_id, e);
            HttpResponse::NotFound()
                .json(ApiResponse::<Tender>::error(e.to_string()))
        }
    }
}

/// Create new tender
pub async fn create_tender(
    config: web::Data<Config>,
    req: web::Json<CreateTenderRequest>,
) -> HttpResponse {
    log::info!("Creating tender: {}", req.title);
    
    // Validate deadlines
    let current_time = chrono::Utc::now().timestamp() as u64;
    if req.deadline <= current_time {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<u64>::error("Deadline must be in future".to_string()));
    }
    
    if req.reveal_deadline <= req.deadline {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<u64>::error(
                "Reveal deadline must be after bidding deadline".to_string()
            ));
    }
    
    match stellar::create_tender(&config, req.into_inner()).await {
        Ok(tender_id) => {
            log::info!("Tender created with ID: {}", tender_id);
            HttpResponse::Ok().json(ApiResponse::success(tender_id))
        }
        Err(e) => {
            log::error!("Failed to create tender: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<u64>::error(e.to_string()))
        }
    }
}

/// Close tender
pub async fn close_tender(
    config: web::Data<Config>,
    path: web::Path<u64>,
    req: web::Json<CloseTenderRequest>,
) -> HttpResponse {
    let tender_id = path.into_inner();
    log::info!("Closing tender {}", tender_id);
    
    match stellar::close_tender(&config, tender_id, &req.caller).await {
        Ok(_) => {
            log::info!("Tender {} closed successfully", tender_id);
            HttpResponse::Ok().json(ApiResponse::success("Tender closed"))
        }
        Err(e) => {
            log::error!("Failed to close tender {}: {}", tender_id, e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<String>::error(e.to_string()))
        }
    }
}

/// Get winner of a tender
pub async fn get_winner(
    config: web::Data<Config>,
    path: web::Path<u64>,
) -> HttpResponse {
    let tender_id = path.into_inner();
    log::info!("Fetching winner for tender {}", tender_id);
    
    match stellar::get_winner(&config, tender_id).await {
        Ok(Some(winner)) => HttpResponse::Ok().json(ApiResponse::success(winner)),
        Ok(None) => HttpResponse::NotFound()
            .json(ApiResponse::<Winner>::error("No winner found".to_string())),
        Err(e) => {
            log::error!("Failed to fetch winner: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<Winner>::error(e.to_string()))
        }
    }
}

/// Get all bidders for a tender
pub async fn get_bidders(
    config: web::Data<Config>,
    path: web::Path<u64>,
) -> HttpResponse {
    let tender_id = path.into_inner();
    log::info!("Fetching bidders for tender {}", tender_id);
    
    match stellar::get_tender_bidders(&config, tender_id).await {
        Ok(bidders) => HttpResponse::Ok().json(ApiResponse::success(bidders)),
        Err(e) => {
            log::error!("Failed to fetch bidders: {}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::<Vec<String>>::error(e.to_string()))
        }
    }
}