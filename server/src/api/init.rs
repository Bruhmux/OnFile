use crate::{api, state::AppState};
use axum::{Router, http::HeaderValue, response::IntoResponse};
use std::path::PathBuf;
use tower_http::{cors::CorsLayer, services::ServeDir};

fn client_dist_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("client/dist")
}

pub fn make_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", api::routes::route())
        .fallback_service(ServeDir::new(client_dist_path()))
        .layer(make_cors())
        .with_state(state)
}

fn make_cors() -> CorsLayer {
    CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
}

async fn check_health() -> impl IntoResponse {
    "Crypt n' Clues is running.."
}
