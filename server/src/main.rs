use axum::{Router, http::HeaderValue, routing::get};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{db::init_connection, types::AppState};

mod db;
mod routes;
mod types;
mod user;

#[tokio::main]
async fn main() {
    let db = init_connection().await.unwrap();
    let state = Arc::new(AppState { db });

    let cors =
        CorsLayer::new().allow_origin("http://localhost:5432".parse::<HeaderValue>().unwrap());
    let client_static_path = "client/dist";

    let api_routes = Router::new()
        .route("/rooms", get(routes::get_rooms))
        .route("/hello", get(routes::root));

    let app = Router::new()
        .nest("/api", api_routes)
        // Serve bun built static files
        .fallback_service(ServeDir::new(client_static_path))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app).await.expect("Server Error");
}
