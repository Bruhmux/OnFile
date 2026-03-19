use crate::{
    db::{AppState, init_connection},
    tasks::cli::cli_loop,
    tasks::server::serve_app,
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
    let shutdown_cl = shutdown.clone();

    let pool: PgPool = init_connection()
        .await
        .expect("Failed to connect to database");
    let app_state = Arc::new(AppState { db: pool });

    //     Server Start
    let server_task = serve_app(app_state.clone());
    let cli_task = cli_loop(State(app_state.clone()));

    join!(server_task, cli_task);
}
