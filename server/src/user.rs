use axum::{Json, http};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    username: String,
    // TODO: Add logic table and other notes
}

pub async fn set_username(Json(payload): Json<CreateUser>) -> (http::StatusCode, Json<User>) {
    // TODO: Verify no duplicate names
    let user = User {
        username: payload.username,
    };
    (http::StatusCode::CREATED, Json(user))
}
