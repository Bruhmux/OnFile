use crate::{
    api::{self, init::make_app},
    state::AppState,
};
use axum::{Router, http::HeaderValue};
use std::net::SocketAddr;
use tokio::{spawn, sync::watch, task::JoinHandle};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

pub async fn create_app(state: AppState, mut shutdown_rx: watch::Receiver<bool>) -> JoinHandle<()> {
    spawn(async move {
        let origin = format!(
            "http://{}:{}",
            state.config.server_host(),
            state.config.server_port()
        );
        let cors = CorsLayer::new().allow_origin(origin.parse::<HeaderValue>().unwrap());

        let addr = SocketAddr::from((
            *state.clone().config.server_host(),
            state.clone().config.server_port(),
        ));
        let app: Router = Router::new()
            .fallback_service(ServeDir::new("client/dist"))
            .layer(cors)
            .nest("/api", api::routes::route())
            .with_state(state);

        println!("Listening on {addr}");

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind socket");

        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                shutdown_rx.changed().await.ok();
            })
            .await
            .expect("Server Error");
    })
}

pub async fn run_migrations(state: &AppState) {
    sqlx::migrate!("./migrations")
        .run(&state.db)
        .await
        .expect("failed to run db migration");
}

pub async fn assemble_app(
    state: AppState,
    mut shutdown_rx: watch::Receiver<bool>,
) -> JoinHandle<()> {
    spawn(async move {
        let address = format!(
            "{}:{}",
            state.config.server_host(),
            state.config.server_port()
        );
        let socket_addr: SocketAddr = address.parse().expect("Unable to parse socket address");

        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .expect("Failed to bind to socket");
        info!("listening on http://{}", socket_addr);

        let app = make_app(state);

        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                shutdown_rx.changed().await.ok();
            })
            .await
            .expect("Server Error");
    })
}
