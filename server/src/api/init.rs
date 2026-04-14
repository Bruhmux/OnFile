use crate::{api::routes::make, state::AppState};
use axum::{Router, http::HeaderValue, response::IntoResponse, routing::get};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn make_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", make())
        .fallback_service(ServeDir::new("client/dist"))
        .layer(make_cors())
        .with_state(state)
}

fn make_cors() -> CorsLayer {
    CorsLayer::new().allow_origin("http://localhost:5432".parse::<HeaderValue>().unwrap())
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}
