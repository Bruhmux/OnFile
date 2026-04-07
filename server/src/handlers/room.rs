use crate::db::types::AppState;
use axum::{Json, extract::State, http};
use rand::{RngExt, distr::Alphabetic};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    pub display_name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateRoomResponse {
    pub room_id: Uuid,
    pub room_code: String,
}

fn generate_room_code() -> String {
    rand::rng()
        .sample_iter(&Alphabetic)
        .take(5)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub async fn create_room(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<Json<CreateRoomResponse>, axum::http::StatusCode> {
    const MAX_ATTEMPTS: u8 = 3;

    for _ in 0..MAX_ATTEMPTS {
        let room_code = generate_room_code();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO rooms (room_code, display_name)
            VALUES ($1, $2)
            RETURNING id, room_code
            "#,
            room_code,
            payload.display_name
        )
        .fetch_one(&state.db)
        .await;

        match insert_result {
            Ok(row) => {
                return Ok(Json(CreateRoomResponse {
                    room_id: row.id,
                    room_code: row.room_code,
                }));
            }
            Err(sqlx::Error::Database(db_err))
                if db_err.constraint() == Some("rooms_room_code_key") =>
            {
                continue;
            }
            Err(_) => return Err(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    Err(http::StatusCode::CONFLICT) // impressive if you made it this, go buy a lottery ticket
}
