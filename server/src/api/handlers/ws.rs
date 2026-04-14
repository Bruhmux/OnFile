use crate::{state::AppState, types::AppError};
use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct WsParams {
    pub token: Uuid,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    Query(params): Query<WsParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Verify token and get player name
    let user_info = sqlx::query!(
        r#"
        SELECT u.id, u.display_name
        FROM users u
        JOIN room_participants rp ON rp.user_id = u.id
        WHERE u.connection_token = $1 AND rp.room_id = $2
        "#,
        params.token,
        room_id
    )
    .fetch_optional(&state.db)
    .await?;

    match user_info {
        Some(user) => Ok(ws.on_upgrade(move |socket| {
            handle_socket(socket, state, room_id, user.id, user.display_name)
        })),
        None => Err(AppError::Forbidden(
            "Invalid token for this room".to_string(),
        )),
    }
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    room_id: String,
    user_id: Uuid,
    display_name: String,
) {
    // 2. Setup broadcast channel for the room
    let tx = state
        .channels
        .entry(room_id.clone())
        .or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        })
        .clone();

    let mut rx = tx.subscribe();

    // 3. Presence: Notify others
    let _ = tx.send(format!("{} joined the room", display_name));

    // 4. Split socket for bidirectional communication
    let (mut sink, mut stream) = socket.split();

    // Outgoing: Broadcast messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Incoming: Process client messages
    let tx_clone = tx.clone();
    let name_clone = display_name.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            if let Message::Text(text) = msg {
                // Broadcast the message to everyone in the room
                let _ = tx_clone.send(format!("{}: {}", name_clone, text));
            }
        }
    });

    // Wait for either task to end (disconnect or error)
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // 5. Cleanup: Notify others
    let _ = tx.send(format!("{} left the room", display_name));
}
