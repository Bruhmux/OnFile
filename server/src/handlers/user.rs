use axum::{Json, extract::State, http};
use rand::{RngExt, distr::Alphabetic};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub connection_token: Uuid,
}

pub async fn set_username(Json(payload): Json<CreateUser>) -> (http::StatusCode, Json<User>) {
    // TODO: Verify no duplicate names
    let user = User {
        username: payload.username,
    };
    (http::StatusCode::CREATED, Json(user))
}

fn generate_uuid() -> String {
    rand::rng()
        .sample_iter(&Alphabetic)
        .take(5)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, axum::http::StatusCode> {
    let uuid = generate_uuid();
    todo!()
    // let insert_result = query!();
}
