use crate::api::handlers::game_state;
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{room_id}/grid", get(game_state::get_grid))
        .route("/{room_id}/mark", post(game_state::add_mark))
        .route("/{room_id}/mark", delete(game_state::remove_mark))
}
