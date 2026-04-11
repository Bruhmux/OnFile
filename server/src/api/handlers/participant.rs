use axum::extract::State;

use crate::state::AppState;

pub async fn add_participant(State(state): State<AppState>) -> RetType {
    todo!();
}
