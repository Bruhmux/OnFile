use crate::api::handlers::ws;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/{room_id}", get(ws::handler))
}
