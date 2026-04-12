use crate::config::Config;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}

// Example state
//
// #[derive(Clone)]
// pub struct AppState {
//     pub db: PgPool,
//     pub config: Arc<Config>,
//     pub http_client: reqwest::Client,
//     pub jwt_secret: Arc<String>,
//     pub redis: redis::Client,
// }
//
