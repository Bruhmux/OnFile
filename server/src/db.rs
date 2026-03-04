use std::env;

use dotenv::dotenv;
use sqlx::{Result, postgres::PgPoolOptions};

pub async fn init_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().unwrap();

    let db_url = env::var("DATABASE_URL").unwrap(); // "postgres://user:password@localhost:port/db"

    PgPoolOptions::new()
        .max_connections(15)
        .connect(&db_url)
        .await
}
