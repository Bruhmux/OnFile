use crypts_and_clues::{
    cli::repl::init_repl, config::init_config, db::init_connection, state::AppState,
    tasks::server::assemble_app,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, watch};
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
        channels: Arc::new(Mutex::new(HashMap::new())),
        decks: Arc::new(Mutex::new(HashMap::new())),
    };

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    info!("Starting tasks...");
    let app_server = assemble_app(app_state.clone(), shutdown_rx);
    info!(" >Server running...");
    let repl_loop = init_repl(app_state.clone(), shutdown_tx);
    info!(" >REPL running...");

    info!("Joining tasks...");

    let _ = tokio::join!(biased; app_server, repl_loop);

    info!("Main exiting...");
}
