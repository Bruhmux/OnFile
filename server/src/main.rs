use axum::extract::State;
use crypts_and_clues::{
    cli::repl::init_repl, config::init_config, db::init_connection, state::AppState,
    tasks::server::assemble_app,
};
use std::sync::Arc;
use tokio::{join, sync::watch};

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
    let app_server = assemble_app(app_state.clone(), shutdown_rx);
    let cli_loop = init_repl(State(app_state.clone()), shutdown_tx);

    join!(app_server, cli_loop);
}
