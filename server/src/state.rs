use crate::config::Config;
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub channels: Arc<DashMap<String, broadcast::Sender<String>>>,
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
