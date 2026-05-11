use crate::{db::tables::Room, error::AppError, state::AppState, types::evidence::Verdict};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use rand::{RngExt, distr::Alphabetic};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    pub display_name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateRoomResponse {
    pub room_id: String,
    pub user_id: Uuid,
    pub connection_token: Uuid,
}

#[derive(Serialize, Debug)]
pub struct JoinRoomResponse {
    pub user_id: Uuid,
    pub connection_token: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct InitFilesRequest {
    pub amount: u8,
}

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
        SELECT id, display_name, created_at, is_active, file_data 
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
                let connection_token = Uuid::new_v4();
                let user_row = sqlx::query!(
                    r#"
                    INSERT INTO users (display_name, connection_token, connected_at, last_heartbeat)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id
                    "#,
                    &payload.display_name,
                    connection_token,
                    Utc::now(),
                    Utc::now(),
                )
                .fetch_one(&state.db)
                .await?;

                let user_id = user_row.id;

                sqlx::query!(
                    "INSERT INTO room_participants (room_id, user_id) VALUES ($1, $2)",
                    room_id,
                    user_id,
                )
                .execute(&state.db)
                .await?;

                return Ok((
                    StatusCode::CREATED,
                    Json(CreateRoomResponse {
                        room_id,
                        user_id,
                        connection_token,
                    }),
                ));
            }
            Err(e) => {
                if let Some(db_err) = e.as_database_error()
                    && db_err.constraint() == Some("rooms_pkey")
                {
                    continue;
                }
                return Err(AppError::Database(e));
            }
        }
    }
    Err(AppError::Http(
        StatusCode::CONFLICT,
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
        return Err(AppError::Http(
            StatusCode::NOT_FOUND,
            format!("Room {} not found", room_code),
        ));
    }

    // 2. Create user
    let connection_token = Uuid::new_v4();
    let user_row = sqlx::query!(
        r#"
        INSERT INTO users (display_name, connection_token, connected_at, last_heartbeat)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        &payload.display_name,
        connection_token,
        Utc::now(),
        Utc::now(),
    )
    .fetch_one(&state.db)
    .await?;

    let user_id = user_row.id;

    // 3. Add participant
    sqlx::query!(
        "INSERT INTO room_participants (room_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        room_code,
        user_id,
    )
    .execute(&state.db)
    .await?;

    Ok(Json(JoinRoomResponse {
        user_id,
        connection_token,
    }))
}

pub async fn delete_room(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query("DELETE from rooms WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn init_files(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(payload): Json<InitFilesRequest>,
) -> Result<impl IntoResponse, AppError> {
    let files = crate::types::discovery::init_files(payload.amount);
    let file_data = serde_json::to_value(&files)
        .map_err(|e| AppError::Http(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let guilty_idx = files
        .iter()
        .position(|f| f.verdict() == Verdict::Guilty)
        .unwrap_or(0) as i32;

    sqlx::query("UPDATE rooms SET file_data = $1 WHERE id = $2")
        .bind(&file_data)
        .bind(&room_id)
        .execute(&state.db)
        .await?;

    sqlx::query(
        "INSERT INTO game_states (room_id, status, solution_file) VALUES ($1, 'open', $2) ON CONFLICT (room_id) DO UPDATE SET solution_file = $2",
    )
    .bind(&room_id)
    .bind(guilty_idx)
    .execute(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(file_data)))
}
