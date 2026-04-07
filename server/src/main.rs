// #![allow(dead_code, unused)]
use crate::{
    cli::{options::Args, repl::init_repl},
    db::{init::init_connection, types::AppState},
    tasks::server::{create_app, init_db},
};
use axum::extract::State;
use clap::Parser;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::{join, sync::watch};

mod cli;
mod db;
mod handlers;
mod routes;
mod tasks;
mod types;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let pool: PgPool = init_connection()
        .await
        .expect("Failed to connect to database");
    let app_state = Arc::new(AppState { db: pool });

    init_db(app_state.clone()).await;
    //     Server Start
    let app_server = create_app(app_state.clone(), args, shutdown_rx);
    let cli_task = init_repl(State(app_state), shutdown_tx);

    join!(app_server, cli_task);
}
