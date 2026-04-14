use crate::api::handlers::clue;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:room_id", get(clue::list))
        .route("/:room_id", post(clue::setup_room))
}
