use crate::{db::tables::Room, state::AppState, types::AppError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use rand::{RngExt, distr::Alphabetic};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

fn generate_room_code() -> String {
    rand::rng()
        .sample_iter(&Alphabetic)
        .take(5)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let query = r#"
        SELECT id, display_name, created_at, is_active 
        FROM rooms 
        ORDER BY created_at DESC 
        LIMIT 30
        "#;

    let rooms = query_as::<_, Room>(query).fetch_all(&state.db).await?;

    Ok(Json(rooms))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<impl IntoResponse, AppError> {
    const MAX_ATTEMPTS: u8 = 3;

    for _ in 0..MAX_ATTEMPTS {
        let room_id = generate_room_code();

        let result = sqlx::query("INSERT INTO rooms (id, display_name) VALUES ($1, $2)")
            .bind(&room_id)
            .bind(&payload.display_name)
            .execute(&state.db)
            .await;

        match result {
            Ok(_) => {
                return Ok((StatusCode::CREATED, Json(CreateRoomResponse { room_id })));
            }
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.constraint() == Some("rooms_pkey") {
                        continue;
                    }
                }
                return Err(AppError::Database(e));
            }
        }
    }
    Err(AppError::Conflict(
        "Failed to generate a unique room code".to_string(),
    ))
}

pub async fn join(
    State(state): State<AppState>,
    Path(room_code): Path<String>,
    Json(payload): Json<JoinRoomRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Verify room exists
    let room_exists = sqlx::query("SELECT 1 FROM rooms WHERE id = $1")
        .bind(&room_code)
        .fetch_optional(&state.db)
        .await?
        .is_some();

    if !room_exists {
        return Err(AppError::NotFound(format!("Room {} not found", room_code)));
    }

    // 2. Add participant (idempotent)
    sqlx::query(
        "INSERT INTO room_participants (room_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(&room_code)
    .bind(&payload.user_id)
    .execute(&state.db)
    .await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    pub user_id: Uuid,
}

pub async fn delete_room(
    State(state): State<AppState>,
    Path(room_code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    query("DELETE from rooms WHERE id = $1")
        .bind(room_code)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    pub display_name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateRoomResponse {
    pub room_id: String,
}
