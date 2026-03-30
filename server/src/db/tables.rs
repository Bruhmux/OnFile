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
    id: Uuid,
    join_code: [u8; 5],
    name: String,
    grid: Json<LogicGrid>,
}

#[derive(sqlx::FromRow)]
struct Participants {
    room_id: Uuid,
    user_id: Uuid,
    joined_at: DateTime<Utc>,
    is_host: bool,
}

#[derive(sqlx::FromRow)]
pub struct game_state {
    room_id: Uuid,
}

#[derive(sqlx::FromRow)]
pub struct clues {}

#[derive(sqlx::FromRow)]
pub struct logic_grid {}

#[derive(sqlx::FromRow)]
pub struct grid_cell {}

#[derive(sqlx::FromRow)]
pub struct actions {}
