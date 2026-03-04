use std::sync::Arc;

use axum::{Json, extract::State, http};
use rand::{RngExt, distr::Alphabetic};

use crate::{
    AppState,
    dto::{CreateRoomRequest, CreateRoomResponse},
};

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
    let room_code = generate_room_code();

    let row = sqlx::query!(
        r#"
        INSERT INTO rooms (room_code, display_name)
        VALUES ($1, $2)
        RETURNING id, room_code
        "#,
        room_code,
        payload.display_name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateRoomResponse {
        room_id: row.id,
        room_code: row.room_code,
    }))
}
