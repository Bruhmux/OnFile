use crate::{db::tables, state::AppState, types::AppError};
use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
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
        None => Err(AppError::Http(
            StatusCode::FORBIDDEN,
            "Invalid token for this room".to_string(),
        )),
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum ClientRequest {
    PlaceClue {
        item1: String,
        item2: String,
        is_true: bool,
    },
    Chat(String),
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    room_id: String,
    user_id: Uuid,
    display_name: String,
) {
    let tx = state
        .channels
        .entry(room_id.clone())
        .or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        })
        .clone();

    let mut rx = tx.subscribe();

    let _ = tx.send(format!("{} joined the room", display_name));

    let (mut sink, mut stream) = socket.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sink.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Process incoming messages
    let tx_clone = tx.clone();
    let name_clone = display_name.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<ClientRequest>(&text) {
                    Ok(req) => match req {
                        ClientRequest::PlaceClue {
                            item1,
                            item2,
                            is_true,
                        } => {
                            // TODO: Resolve identifiers and save to DB
                            let broadcast_msg = format!(
                                "{}: Placed clue {} vs {} = {}",
                                name_clone, item1, item2, is_true
                            );
                            let _ = tx_clone.send(broadcast_msg);
                        }
                        ClientRequest::Chat(msg) => {
                            let _ = tx_clone.send(format!("{}: {}", name_clone, msg));
                        }
                    },
                    Err(e) => {
                        let _ = tx_clone.send(format!("Error parsing message: {}", e));
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let _ = tx.send(format!("{} left the room", display_name));
}
