#![allow(dead_code, unused)]
use crate::{
    db::{init::init_connection, types::AppState},
    tasks::{
        cli::cli_loop,
        server::{create_app, init_db},
    },
};
use axum::extract::State;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::{join, sync::broadcast};

mod db;
mod handlers;
mod routes;
mod tasks;
mod types;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (shutdown, _) = broadcast::channel::<String>(16);

    let pool: PgPool = init_connection()
        .await
        .expect("Failed to connect to database");
    let app_state = Arc::new(AppState { db: pool });

    init_db(app_state.clone());
    //     Server Start
    let app_server = create_app(app_state.clone());
    let cli_task = cli_loop(State(app_state));

    join!(app_server, cli_task);
}
