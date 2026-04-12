use crate::types::Category;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    pub connection_token: Uuid,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct RoomParticipant {
    pub id: Uuid,
    pub room_id: String,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
    pub is_host: bool,
}
#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "game_status", rename_all = "snake_case")]
pub enum GameStatus {
    Open,
    InProgress,
    Finished,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub room_id: String,
    pub status: GameStatus,
    pub current_turn_user: Option<Uuid>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}
