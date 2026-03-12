use axum::{
    Router,
    http::HeaderValue,
    routing::{get, post},
};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{db::init_connection, handlers::room::create_room, routes::checkhealth};

mod db;
mod dto;
mod handlers;
mod routes;
mod types;
mod user;

pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool: PgPool = init_connection()
        .await
        .expect("Failed to connect to database");

    let app_state: Arc<AppState> = Arc::new(AppState { db: pool });

    let cors: CorsLayer =
        CorsLayer::new().allow_origin("http://localhost:5432".parse::<HeaderValue>().unwrap());

    let app: Router = Router::new()
        .fallback_service(ServeDir::new("client/dist")) // bun static file directory
        .layer(cors)
        .route("/", get(checkhealth))
        .route("/room/create", post(create_room))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000)))
        .await
        .expect("Failed to bind socket");

    axum::serve(listener, app).await.expect("Server Error");
}
