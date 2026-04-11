use crate::{
    api::handlers::portfolio::{get_portfolio, get_portfolios},
    state::AppState,
};
use axum::{Router, extract::State, http::HeaderValue, response::IntoResponse, routing::get};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn make_app(State(state): State<AppState>) -> Router {
    Router::new()
        .nest("/api", make_api())
        .fallback_service(ServeDir::new("client/dist"))
        .layer(make_cors())
        .with_state(state)
}

fn make_cors() -> CorsLayer {
    CorsLayer::new().allow_origin("http://localhost:5432".parse::<HeaderValue>().unwrap())
}

fn make_api() -> Router<AppState> {
    Router::new()
        .route("/posts", get(hello))
        .route("/portfolio", get(get_portfolios))
        .route("/portfolio/:id", get(get_portfolio))
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}
