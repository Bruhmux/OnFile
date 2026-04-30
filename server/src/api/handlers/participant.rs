use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn add_participant(
    State(state): State<AppState>,
    Path(room_id): Path<Uuid>,
    Query(player_id): Query<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    Err::<StatusCode, AppError>(AppError::Http(
        StatusCode::NOT_IMPLEMENTED,
        "Not implemented".into(),
    ))
}
