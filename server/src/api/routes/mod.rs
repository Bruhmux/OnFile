use crate::state::AppState;
use axum::Router;

pub mod clue;
pub mod discovery;
pub mod room;
pub mod verdict;
pub mod ws;

pub fn route() -> Router<AppState> {
    Router::new()
        .nest("/rooms", room::make())
        .nest("/clues", clue::router())
        .nest("/discoveries", discovery::router())
        .nest("/verdicts", verdict::router())
        .nest("/ws", ws::router())
}
