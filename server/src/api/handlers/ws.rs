use crate::{
    db::tables::{Category, Clue},
    error::AppError,
    state::AppState,
    types::{
        discovery::{Discovery, File},
        evidence::{Location, Suspect, Weapon},
    },
};
use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use std::time::Duration;
use tokio::{sync::broadcast, time::sleep};
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
pub enum ClientRequest {
    Chat(String),
    Guess {
        suspect: Suspect,
        weapon: Weapon,
        location: Location,
    },
    DrawDiscovery,
    PlaceClue {
        clue_id: Uuid,
        is_true: bool,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerResponse {
    Error(String),
    Chat { sender: String, message: String },
    Joined { display_name: String },
    Leave { display_name: String },
    TurnStarted { display_name: String },
    VerdictResult { correct: bool, message: String },
    Removed { reason: String },
    DrawDiscovery { files: u8 },
    CluePlaced { display_name: String, clue: Clue },
    PlayerStatus { display_name: String, active: bool },
}

#[derive(Debug, Serialize)]
struct ClueInfo {
    id: Uuid,
    category_1: Category,
    category_2: Category,
}

struct PlayerState {
    can_guess: bool,
    active: bool,
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    room_id: String,
    user_id: Uuid,
    display_name: String,
) {
    let mut tx = state
        .channels
        .lock()
        .await
        .entry(room_id.clone())
        .or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        })
        .clone();
    let mut rx = tx.subscribe();

    let mut session = PlayerState {
        can_guess: true,
        active: true,
    };
    let join_msg = serde_json::to_string_pretty(&ServerResponse::Joined {
        display_name: display_name.clone(),
    })
    .unwrap();
    let _ = tx.send(join_msg + " has joined the lobby.");

    let (mut sink, mut stream) = socket.split();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
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
                    Ok(req) => {
                        match req {
                            ClientRequest::Chat(msg) => {
                                let response = ServerResponse::Chat {
                                    sender: name_clone.clone(),
                                    message: msg,
                                };
                                let _ =
                                    tx_clone.send(serde_json::to_string_pretty(&response).unwrap());
                            }
                            ClientRequest::Guess {
                                suspect,
                                weapon,
                                location,
                            } => {
                                // Check turn
                                let turn_check = sqlx::query!(
                                    "SELECT current_turn_user FROM game_states WHERE room_id = $1",
                                    room_id
                                )
                                .fetch_one(&state.db)
                                .await;

                                match turn_check {
                                    Ok(rec) => {
                                        if rec.current_turn_user != Some(user_id) {
                                            let err = ServerResponse::Error("Not your turn".into());
                                            let _ = tx_clone
                                                .send(serde_json::to_string_pretty(&err).unwrap());
                                            continue;
                                        }
                                    }
                                    _ => {
                                        let err = ServerResponse::Error(
                                            "Game not started or turn not set".into(),
                                        );
                                        let _ = tx_clone
                                            .send(serde_json::to_string_pretty(&err).unwrap());
                                        continue;
                                    }
                                }
                                if !session.active {
                                    let err = ServerResponse::Error(
                                        "You have been removed from play".into(),
                                    );
                                    let _ =
                                        tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                    continue;
                                }
                                if !session.can_guess {
                                    let err = ServerResponse::Error(
                                        "You already used your verdict guess".into(),
                                    );
                                    let _ =
                                        tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                    continue;
                                }

                                let solution_check = sqlx::query!(
                                    "SELECT solution_data FROM game_states WHERE room_id = $1",
                                    room_id
                                )
                                .fetch_one(&state.db)
                                .await;

                                let mut correct = false;
                                if let Ok(rec) = solution_check
                                    && let Ok(solution) =
                                        serde_json::from_value::<File>(rec.solution_data)
                                {
                                    correct = solution.suspect() == suspect
                                        && solution.location() == location
                                        && solution.weapon() == weapon
                                }

                                session.can_guess = true;
                                if !correct {
                                    session.active = false;
                                    let response = ServerResponse::VerdictResult {
                                        correct: false,
                                        message: "Wrong verdict! You are removed from play.".into(),
                                    };
                                    let _ = tx_clone
                                        .send(serde_json::to_string_pretty(&response).unwrap());

                                    let removed = ServerResponse::Removed {
                                        reason: "Wrong verdict guess".into(),
                                    };
                                    let _ = tx_clone
                                        .send(serde_json::to_string_pretty(&removed).unwrap());
                                } else {
                                    let response = ServerResponse::VerdictResult {
                                        correct: true,
                                        message: "You found the assassin! You win!".into(),
                                    };
                                    let _ = tx_clone
                                        .send(serde_json::to_string_pretty(&response).unwrap());
                                }
                            }
                            ClientRequest::DrawDiscovery => {
                                if !session.active {
                                    let err =
                                        ServerResponse::Error("Player removed from play".into());
                                    let _ =
                                        tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                    continue;
                                }

                                let response;
                                tokio::select! {
                                    mut deck = state.decks.lock() => {
                                        match deck.get_mut(&room_id).unwrap().draw() {
                                            Discovery::Wild => {
                                                response = ServerResponse::DrawDiscovery { files: 1 }
                                            }
                                            Discovery::Same(_) => {
                                                response = ServerResponse::DrawDiscovery { files: 2 }
                                            }
                                            Discovery::Different(_, _) => {
                                                response = ServerResponse::DrawDiscovery { files: 2 }
                                            }
                                        }
                                    }
                                    _ = sleep(Duration::from_secs(5)) => {
                                        response = ServerResponse::Error("Could not get lock on deck".to_string())
                                    }
                                };

                                let _ =
                                    tx_clone.send(serde_json::to_string_pretty(&response).unwrap());
                            }
                            ClientRequest::PlaceClue { clue_id, is_true } => {
                                if !session.active {
                                    let err = ServerResponse::Error("It is not your turn".into());
                                    let _ =
                                        tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                    continue;
                                }
                                let clue = query_as::<_, Clue>(r#"
                                    INSERT INTO clues (id, room_id, x_category, x_idx, y_category, y_idx, is_true)
                                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                                    "#)
                                    .bind(clue_id)
                                    .fetch_one(&state.db)
                                    .await
                                    .unwrap();

                                let response = ServerResponse::CluePlaced {
                                    display_name: name_clone.clone(),
                                    clue,
                                };
                                let _ =
                                    tx_clone.send(serde_json::to_string_pretty(&response).unwrap());
                            }
                        }
                    }
                    Err(e) => {
                        let err = ServerResponse::Error(format!("Invalid message format: {}", e));
                        let _ = tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let leave_msg = serde_json::to_string_pretty(&ServerResponse::Leave {
        display_name: display_name.clone(),
    })
    .unwrap();
    let _ = tx.send(leave_msg);
}
