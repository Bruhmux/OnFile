#![allow(dead_code, unused_variables)]
use axum::extract::State;
use crypts_and_clues::{
    cli::repl::init_repl, config::init_config, db::init_connection, state::AppState,
    tasks::server::assemble_app,
};
use std::sync::Arc;
use tokio::{join, sync::watch};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_config = init_config().await;

    let app_state = AppState {
        db: init_connection(app_config.clone())
            .await
            .expect("Error initializing database connection"),

        config: app_config,
        channels: Arc::new(dashmap::DashMap::new()),
    };

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    info!("Starting tasks...");
    let app_server = assemble_app(app_state.clone(), shutdown_rx);
    info!(" >Server running...");
    let repl_loop = init_repl(State(app_state.clone()), shutdown_tx);
    info!(" >REPL running...");

    info!("Joining tasks...");

    join!(app_server, repl_loop);

    // tokio::select! {
    //     res = app_server => info!("App server task finished: {:?}", res),
    //     res = repl_loop => info!("CLI loop task finished: {:?}", res),
    // };

    info!("Main exiting...");
}
