use crate::api::handlers::room;
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn make() -> Router<AppState> {
    Router::new()
        .route("/", get(room::list))
        .route("/", post(room::create))
        .route("/{id}/join", post(room::join))
        .route("/{room_code}/files", post(room::init_files))
        .route("/{room_code}", delete(room::delete_room))
}
