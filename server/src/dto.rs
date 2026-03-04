use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub display_name: String,
}

#[derive(Serialize)]
pub struct CreateRoomResponse {
    pub room_id: Uuid,
    pub room_code: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: Uuid,
    pub connection_token: Uuid,
}
