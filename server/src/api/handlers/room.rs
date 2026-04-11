use crate::db::{tables::Room, types::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::{self, StatusCode},
};
use rand::{RngExt, distr::Alphabetic};
use serde::{Deserialize, Serialize};
use sqlx::query;
use std::sync::Arc;

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
    Path(display_name): Path<String>,
) -> Result<Json<CreateRoomResponse>, axum::http::StatusCode> {
    const MAX_ATTEMPTS: u8 = 3;

    let query = r#"
            INSERT INTO rooms (id, display_name)
            VALUES ($1, $2)
            "#;

    for _ in 0..MAX_ATTEMPTS {
        let room_id = generate_room_code();
        let insert_result: Result<_, sqlx::Error> = sqlx::query(query)
            .bind(room_id)
            .bind(display_name)
            .execute(&state.db)
            .await;

        match insert_result {
            Ok(row) => {
                return Ok(Json(CreateRoomResponse { room_id: row.id }));
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

pub async fn delete_room(
    State(state): State<AppState>,
    Path(room_code): Path<String>,
) -> StatusCode {
    match query!("DELETE from rooms WHERE id = $1", room_code)
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
