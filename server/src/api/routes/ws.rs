use axum::{routing::get, Router};
use crate::api::handlers::ws;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/:room_id", get(ws::handler))
}
