use axum::{routing::post, Router};
use crate::state::AppState;
// Assuming a future verdict handler
// use crate::api::handlers::verdict; 

pub fn router() -> Router<AppState> {
    Router::new()
        // .route("/:room_id/solve", post(verdict::solve))
}
