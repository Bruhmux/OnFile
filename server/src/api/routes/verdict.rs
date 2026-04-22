use crate::state::AppState;
use axum::Router;
// Assuming a future verdict handler
// use crate::api::handlers::verdict;

pub fn router() -> Router<AppState> {
    Router::new()
    // .route("/{room_id}/solve", post(verdict::solve))
}
