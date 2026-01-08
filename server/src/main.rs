use axum::{Router};
use std::net::SocketAddr;
use socketioxide::SocketIo;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod routes;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();
    let cors = CorsLayer::new().allow_origin(Any);
    let client_static_path = "client/dist";

    io.ns("/", routes::socket::on_connect);

    let app = Router::new()
        .fallback_service(ServeDir::new(client_static_path))
        .layer(layer)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app).await.expect("Server Error");
    Ok(())
}
