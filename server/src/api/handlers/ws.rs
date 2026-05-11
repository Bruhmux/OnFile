use crate::{
    db::tables::{Category, Clue, Discovery as DiscoveryRecord, DiscoveryCardType, Room},
    error::AppError,
    state::AppState,
    types::{
        discovery::{Discovery, File},
        evidence::{Evidence, Location, Suspect, Weapon},
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
    // FIX: This is temporary, not to be had in final version
    InitFiles {
        amount: u8,
    },
    ChooseFile {
        discovery_id: Uuid,
        file_idx: u8,
        category: Category,
    },
    PlaceClue {
        x_category: Category,
        x_idx: i32,
        y_category: Category,
        y_idx: i32,
        is_true: bool,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerResponse {
    Error(String),
    Chat {
        sender: String,
        message: String,
    },
    Joined {
        display_name: String,
    },
    Leave {
        display_name: String,
    },
    TurnStarted {
        display_name: String,
    },
    VerdictResult {
        correct: bool,
        message: String,
    },
    Removed {
        reason: String,
    },
    DrawDiscovery {
        discovery_id: Uuid,
        card: Discovery,
        files: u8,
    },
    CluePlaced {
        display_name: String,
        clue: Clue,
    },
    FileRevealed {
        file_idx: u8,
        evidence: Evidence,
    },
    // FIX: This is temporary, not to be had in final version
    FilesInitiated {
        files: Vec<File>,
    },
    PlayerStatus {
        display_name: String,
        active: bool,
    },
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
    let _ = tx.send(join_msg);

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

                                let discovery_id = Uuid::new_v4();
                                let response;
                                tokio::select! {
                                    mut decks = state.decks.lock() => {
                                        let deck = decks.entry(room_id.clone()).or_default();
                                        let card = deck.draw();

                                        let mut card_type = DiscoveryCardType::Wild;
                                        let mut cat1: Option<Category> = None;
                                        let mut cat2: Option<Category> = None;
                                        let mut files = 1;

                                        match &card {
                                            Discovery::Wild => {
                                                card_type = DiscoveryCardType::Wild;
                                                files = 1;
                                            }
                                            Discovery::Same(c) => {
                                                card_type = DiscoveryCardType::Same;
                                                cat1 = Some(*c);
                                                files = 2;
                                            }
                                            Discovery::Different(c1, c2) => {
                                                card_type = DiscoveryCardType::Different;
                                                cat1 = Some(*c1);
                                                cat2 = Some(*c2);
                                                files = 2;
                                            }
                                        }

                                        let _ = sqlx::query!(
                                            r#"
                                            INSERT INTO discoveries (id, player_id, room_id, card_type, category_1, category_2)
                                            VALUES ($1, $2, $3, $4, $5, $6)
                                            "#,
                                            discovery_id,
                                            user_id,
                                            room_id,
                                            card_type as DiscoveryCardType,
                                            cat1 as Option<Category>,
                                            cat2 as Option<Category>,
                                        )
                                        .execute(&state.db)
                                        .await;

                                        response = ServerResponse::DrawDiscovery { discovery_id, card, files }
                                    }
                                    _ = sleep(Duration::from_secs(5)) => {
                                        response = ServerResponse::Error("Could not get lock on deck".to_string())
                                    }
                                };

                                let _ =
                                    tx_clone.send(serde_json::to_string_pretty(&response).unwrap());
                            }
                            // FIX: This is temporary, not to be had in final version
                            ClientRequest::InitFiles { amount } => {
                                let files = crate::types::discovery::init_files(amount);
                                let file_data = serde_json::to_value(&files).unwrap_or_default();

                                let _ = sqlx::query("UPDATE rooms SET file_data = $1 WHERE id = $2")
                                    .bind(&file_data)
                                    .bind(&room_id)
                                    .execute(&state.db)
                                    .await;

                                let response = ServerResponse::FilesInitiated { files };
                                let _ = tx_clone.send(serde_json::to_string_pretty(&response).unwrap());
                            }
                            ClientRequest::ChooseFile {
                                discovery_id,
                                file_idx,
                                category,
                            } => {
                                let discovery = match sqlx::query_as::<_, DiscoveryRecord>(
                                    "SELECT * FROM discoveries WHERE id = $1",
                                )
                                .bind(discovery_id)
                                .fetch_one(&state.db)
                                .await
                                {
                                    Ok(d) => d,
                                    Err(e) => {
                                        let err = ServerResponse::Error(format!("Discovery not found: {}", e));
                                        let _ = tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                        continue;
                                    }
                                };

                                let room = match sqlx::query_as::<_, Room>(
                                    "SELECT * FROM rooms WHERE id = $1",
                                )
                                .bind(&room_id)
                                .fetch_one(&state.db)
                                .await
                                {
                                    Ok(r) => r,
                                    Err(e) => {
                                        let err = ServerResponse::Error(format!("Room not found: {}", e));
                                        let _ = tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                        continue;
                                    }
                                };

                                let files: Vec<File> = match room.file_data {
                                    Some(data) => match serde_json::from_value(data) {
                                        Ok(f) => f,
                                        Err(e) => {
                                            let err = ServerResponse::Error(format!("Invalid file data: {}", e));
                                            let _ = tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                            continue;
                                        }
                                    },
                                    None => {
                                        let err = ServerResponse::Error("Files not initialized".into());
                                        let _ = tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                        continue;
                                    }
                                };

                                let file = match files.get(file_idx as usize) {
                                    Some(f) => f,
                                    None => {
                                        let err = ServerResponse::Error(format!("File index {} out of range", file_idx));
                                        let _ = tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                        continue;
                                    }
                                };

                                let evidence = match category {
                                    Category::Suspect => Evidence::Suspect(file.suspect()),
                                    Category::Weapon => Evidence::Weapon(file.weapon()),
                                    Category::Location => Evidence::Location(file.location()),
                                };

                                let response = ServerResponse::FileRevealed { file_idx, evidence };
                                let _ = tx_clone.send(serde_json::to_string_pretty(&response).unwrap());
                            }
                            ClientRequest::PlaceClue {
                                x_category,
                                x_idx,
                                y_category,
                                y_idx,
                                is_true,
                            } => {
                                if !session.active {
                                    let err = ServerResponse::Error("It is not your turn".into());
                                    let _ =
                                        tx_clone.send(serde_json::to_string_pretty(&err).unwrap());
                                    continue;
                                }
                                let clue_id = Uuid::new_v4();
                                let clue = query_as::<_, Clue>(r#"
                                    INSERT INTO clues (id, room_id, x_category, x_idx, y_category, y_idx, is_true)
                                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                                    RETURNING *
                                    "#)
                                    .bind(clue_id)
                                    .bind(&room_id)
                                    .bind(x_category)
                                    .bind(x_idx)
                                    .bind(y_category)
                                    .bind(y_idx)
                                    .bind(is_true)
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
