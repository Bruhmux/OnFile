use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn remove_player(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    Query(player_id): Query<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let _ = state.channels.lock().await.remove(&player_id.to_string());
    Err::<StatusCode, AppError>(AppError::Http(
        StatusCode::NOT_IMPLEMENTED,
        "Not implemented".into(),
    ))
}
