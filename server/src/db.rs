use crate::types::LogicGrid;
use dotenv::dotenv;
use sqlx::{PgPool, Result, postgres::PgPoolOptions};
use std::env;
use uuid::Uuid;

pub async fn init_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().unwrap();

    let db_url = env::var("DATABASE_URL").unwrap(); // "postgres://user:password@localhost:port/db"

    PgPoolOptions::new()
        .max_connections(15)
        .connect(&db_url)
        .await
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    pub connection_token: Uuid,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct Room {
    id: Uuid,
    join_code: [u8; 5],
    name: String,
    grid: LogicGrid,
}
