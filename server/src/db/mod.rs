pub mod helper;
pub mod tables;
pub mod types;

use crate::config::Config;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub async fn init_connection(config: Arc<Config>) -> Result<sqlx::PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(15)
        .connect(config.db_url())
        .await
}
