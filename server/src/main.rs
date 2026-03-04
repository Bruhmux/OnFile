use axum::{Router, http::HeaderValue, routing::get};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{api_dto::checkhealth, db::init_connection};

mod api_dto;
mod db;
mod types;
mod user;

struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = init_connection().await.unwrap();

    let cors =
        CorsLayer::new().allow_origin("http://localhost:5432".parse::<HeaderValue>().unwrap());

    let client_static_path = "client/dist";
    let app = Router::new()
        // Serve bun built static files
        .fallback_service(ServeDir::new(client_static_path))
        .layer(cors)
        .route("/", get(checkhealth()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app).await.expect("Server Error");
}
