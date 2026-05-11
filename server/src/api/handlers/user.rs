use crate::{error::AppError, state::AppState};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::query;
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
    let sql = r#"
        INSERT into users (display_name, connection_token, connected_at, last_heartbeat)
        VALUES ($1, $2, $3, $4)
        "#;

    let connection_token = Uuid::new_v4();
    match query(sql)
        .bind(&payload.display_name)
        .bind(connection_token)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(CreateUserResponse { connection_token }),
        )),
        Err(err) => {
            if let Some(db_err) = err.as_database_error()
                && db_err.code().unwrap_or_default() == "23505"
            {
                return Err(AppError::Http(
                    StatusCode::CONFLICT,
                    "Display name already taken".to_string(),
                ));
            }
            Err(AppError::Database(err))
        }
    }
}
