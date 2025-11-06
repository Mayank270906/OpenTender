mod config;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = config::Config::from_env();

    log::info!("Starting OpenTender Backend on {}:{}", config.host, config.port);
    log::info!("Contract ID: {}", config.contract_id);
    log::info!("Network: {}", config.network);

    // Clone config for use inside closure
    let config_data = config.clone();

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(config_data.clone()))
            // Health check
            .route("/health", web::get().to(health_check))
            // Tender routes
            .service(
                web::scope("/api/tenders")
                    .route("", web::get().to(routes::tender::get_all_tenders))
                    .route("", web::post().to(routes::tender::create_tender))
                    .route("/{id}", web::get().to(routes::tender::get_tender))
                    .route("/{id}/close", web::post().to(routes::tender::close_tender))
                    .route("/{id}/winner", web::get().to(routes::tender::get_winner))
                    .route("/{id}/bidders", web::get().to(routes::tender::get_bidders))
            )
            // Bid routes
            .service(
                web::scope("/api/bids")
                    .route("/submit", web::post().to(routes::bid::submit_bid))
                    .route("/reveal", web::post().to(routes::bid::reveal_bid))
                    .route("/{tender_id}/{bidder}", web::get().to(routes::bid::get_bid))
            )
            // Encryption utility
            .service(
                web::scope("/api/crypto")
                    .route("/encrypt", web::post().to(routes::bid::encrypt_amount))
                    .route("/decrypt", web::post().to(routes::bid::decrypt_amount))
            )
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "opentender-backend",
        "version": "0.1.0"
    })))
}
