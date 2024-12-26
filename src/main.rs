// main.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health))
            .route("/ping", web::get().to(ping))
            .route("/pong", web::get().to(pong))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn root() -> impl Responder {
    HttpResponse::Ok().body("Service is running")
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

async fn pong() -> impl Responder {
    HttpResponse::Ok().body("ping")
}