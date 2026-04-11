use crate::db::{tables::User, types::AppState};
use axum::{Json, extract::State, http};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    display_name: String,
    room_id: Uuid,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    connection_token: Uuid,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> (Json<CreateUserResponse>, http::StatusCode) {
    // TODO: Verify no duplicate names
    // TODO: SQL insertion using FromRow

    let new_user = User {
        id: Uuid::new_v4(),
        display_name: payload.display_name,
        connection_token: Uuid::new_v4(),
        connected_at: Utc::now(),
        last_heartbeat: Utc::now(),
    };

    (
        Json(CreateUserResponse {
            connection_token: new_user.connection_token,
        }),
        http::StatusCode::CREATED,
    )
}
