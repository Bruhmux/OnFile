use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::db::init_connection;

mod db;
mod routes;
mod types;
mod user;

#[tokio::main]
async fn main() {
    let db = init_connection(db::Branch::Test).await;
    //  TODO: utilize db to create rooms and connecto users

    let cors = CorsLayer::new().allow_origin(Any);
    let client_static_path = "client/dist";

    // NOTE: Add api routes here
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
