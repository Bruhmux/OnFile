use crate::{
    state::AppState,
    types::{Clue, LogicGrid},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddMarkRequest {
    pub player_id: Uuid,
    pub clue_id: Uuid,
}

pub async fn get_grid(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    // TODO: Add Player/User Auth Extraction
) -> Result<Json<LogicGrid>, StatusCode> {
    // For now, assume a dummy player_id for testing
    let dummy_player_id = Uuid::nil();

    let clues = sqlx::query_as::<_, Clue>(
        r#"
        SELECT c.* 
        FROM clues c
        JOIN discoveries d ON d.clue_id = c.id
        WHERE c.room_id = $1 AND d.player_id = $2
        "#
    )
    .bind(&room_id)
    .bind(&dummy_player_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LogicGrid::from_clues(clues)))
}

pub async fn add_mark(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(payload): Json<AddMarkRequest>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "INSERT INTO discoveries (player_id, room_id, clue_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )
    .bind(&payload.player_id)
    .bind(&room_id)
    .bind(&payload.clue_id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_mark(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(payload): Json<AddMarkRequest>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "DELETE FROM discoveries WHERE player_id = $1 AND clue_id = $2"
    )
    .bind(&payload.player_id)
    .bind(&payload.clue_id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
