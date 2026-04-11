use crate::types::LogicGrid;
use axum::Json;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    pub connection_token: Uuid,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct Room {
    pub id: [u8; 5],
    pub name: String,
    pub grid: Json<LogicGrid>,
}

#[derive(sqlx::FromRow)]
struct Participant {
    room_id: [u8; 5],
    user_id: Uuid,
    joined_at: DateTime<Utc>,
    is_host: bool,
}

#[derive(sqlx::FromRow)]
pub struct GameState {
    room_id: Uuid,
}

#[derive(sqlx::FromRow)]
pub struct Clue {}

#[derive(sqlx::FromRow)]
pub struct LogicGrid {}

#[derive(sqlx::FromRow)]
pub struct GridCell {}

#[derive(sqlx::FromRow)]
pub struct Action {}
