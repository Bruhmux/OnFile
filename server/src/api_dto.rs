use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: Uuid,
    pub connection_token: Uuid,
}
