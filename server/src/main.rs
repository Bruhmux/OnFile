use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod routes;
mod types;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let client_static_path = "client/dist";

    // NOTE: Add api routes here
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/rooms", get(routes::get_rooms))
        // Serve bun built static files
        .nest_service("/", ServeDir::new(client_static_path))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app).await.expect("Server Error");
}
