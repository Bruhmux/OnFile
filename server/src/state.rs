use crate::{config::Config, types::grid::Deck};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, broadcast};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub channels: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
    pub decks: Arc<Mutex<HashMap<String, Deck>>>,
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
