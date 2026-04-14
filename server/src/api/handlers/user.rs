use crate::{db::tables::User, state::AppState, types::AppError};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    connection_token: Uuid,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Verify no duplicate names
    // TODO: SQL insertion using FromRow

    let new_user = User {
        id: Uuid::new_v4(),
        display_name: payload.display_name,
        connection_token: Uuid::new_v4(),
        connected_at: Utc::now(),
        last_heartbeat: Utc::now(),
    };

    Ok((
        StatusCode::CREATED,
        Json(CreateUserResponse {
            connection_token: new_user.connection_token,
        }),
    ))
}
