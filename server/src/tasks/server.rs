use crate::{
    cli::options::Args, db::types::AppState, handlers::room::create_room, routes::checkhealth,
};
use axum::{
    Router,
    http::HeaderValue,
    routing::{get, post},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    spawn,
    sync::{broadcast, watch},
    task::JoinHandle,
};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub async fn create_app(
    state: Arc<AppState>,
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

pub async fn init_db(state: Arc<AppState>) {
    sqlx::migrate!("./migrations")
        .run(&state.db)
        .await
        .expect("failed to run db migration");
}
