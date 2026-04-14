pub mod tables;
pub mod types;

use crate::config::Config;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::info;

pub async fn init_connection(config: Arc<Config>) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(15)
        .connect(config.db_url())
        .await;
    info!("DB connected...");
    pool
}
