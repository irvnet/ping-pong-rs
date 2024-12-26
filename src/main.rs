// main.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Initialize logging
    env_logger::init();

    // Retrieve service details
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    info!("Starting service on {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health))
            .route("/ping", web::get().to(ping))
            .route("/pong", web::get().to(pong))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

async fn root() -> impl Responder {
    info!("/ - Service is running");
    HttpResponse::Ok().body("Service is running")
}

async fn health() -> impl Responder {
    info!("/health - service is healthy");
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

async fn ping() -> impl Responder {
    info!("/ping - calling out");
    HttpResponse::Ok().body("pong")
}

async fn pong() -> impl Responder {
    info!("/pong - called back");
    HttpResponse::Ok().body("ping")
}