use crate::{db::tables::Room, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rand::{RngExt, distr::Alphabetic};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

fn generate_room_code() -> String {
    rand::rng()
        .sample_iter(&Alphabetic)
        .take(5)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Room>>, StatusCode> {
    let query = r#"
        SELECT id, display_name, created_at, is_active 
        FROM rooms 
        ORDER BY created_at DESC 
        LIMIT 30
        "#;

    let rooms = query_as::<_, Room>(query)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rooms))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<(StatusCode, Json<CreateRoomResponse>), StatusCode> {
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
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
    Err(StatusCode::CONFLICT)
}

pub async fn join(State(state): State<AppState>, Path(room_code): Path<String>) -> StatusCode {
    todo!() // TODO:
}

pub async fn delete_room(
    State(state): State<AppState>,
    Path(room_code): Path<String>,
) -> StatusCode {
    match query("DELETE from rooms WHERE id = $1")
        .bind(room_code)
        .execute(&state.db)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    pub display_name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateRoomResponse {
    pub room_id: String,
}
