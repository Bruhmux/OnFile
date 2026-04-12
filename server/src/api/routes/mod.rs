use crate::state::AppState;
use axum::Router;

pub mod clue;
pub mod discovery;
pub mod room;
pub mod verdict;

pub fn make_routes() -> Router<AppState> {
    Router::new()
        .nest("/rooms", room::router())
        .nest("/clues", clue::router())
        .nest("/discoveries", discovery::router())
        .nest("/verdicts", verdict::router())
}
