use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[sqlx(type_name = "discovery_card_type", rename_all = "lowercase")]
pub enum DiscoveryCardType {
    Wild,
    Same,
    Different,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "game_status", rename_all = "snake_case")]
pub enum GameStatus {
    Open,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "category", rename_all = "lowercase")]
pub enum Category {
    Suspect,
    Weapon,
    Location,
}

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
    pub file_data: Option<serde_json::Value>,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct RoomParticipant {
    pub id: Uuid,
    pub room_id: String,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
    pub is_host: bool,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub room_id: String,
    pub status: GameStatus,
    pub current_turn_user: Option<Uuid>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub solution_data: serde_json::Value,
    pub files_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Clue {
    pub id: Uuid,
    pub room_id: String,
    pub x_category: Category,
    pub x_idx: i32,
    pub y_category: Category,
    pub y_idx: i32,
    pub is_true: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Discovery {
    pub id: Uuid,
    pub player_id: Uuid,
    pub room_id: String,
    pub card_type: DiscoveryCardType,
    pub category_1: Category,
    pub category_2: Category,
}
