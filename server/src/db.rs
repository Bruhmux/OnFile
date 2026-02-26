use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::env;
use uuid::Uuid;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Result, postgres::PgPoolOptions};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub display_name: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: Uuid,
    pub connection_token: Uuid,
}

pub async fn init_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(15)
        .connect(&db_url)
        .await
}

#[derive(FromRow)]
pub struct DbUser {
    pub id: Uuid,
    pub display_name: String,
    pub connection_token: Uuid,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}
