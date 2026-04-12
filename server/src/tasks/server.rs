use crate::{
    api::{handlers::room::create_room, init::make_app},
    cli::options::Args,
    routes::checkhealth,
    state::AppState,
};
use axum::{
    Router,
    http::HeaderValue,
    routing::{get, post},
};
use std::net::SocketAddr;
use tokio::{spawn, sync::watch, task::JoinHandle};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

pub async fn create_app(
    state: AppState,
    launch_opts: Args,
    mut shutdown_rx: watch::Receiver<bool>,
) -> JoinHandle<()> {
    spawn(async move {
        let origin = format!("http://{}:{}", launch_opts.addr, launch_opts.port);
        let cors =
            CorsLayer::new().allow_origin("http://localhost:5432".parse::<HeaderValue>().unwrap());

        let app: Router = Router::new()
            .fallback_service(ServeDir::new("client/dist"))
            .layer(cors)
            .route("/", get(checkhealth))
            .route("/room/create", post(create_room))
            .with_state(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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
