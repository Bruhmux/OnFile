use crate::{
    db::types::AppState,
    handlers::{
        clue::{CreateClueRequest, create_clue, delete_clue, get_clues},
        game_state::{create_game_state, delete_game_state, get_game_state},
        participant::{
            AddParticipantRequest, add_participant, get_participants, remove_participant,
        },
        room::{CreateRoomRequest, create_room, delete_room, get_room, update_room},
        user::{CreateUserRequest, create_user, delete_user, get_user, update_user},
    },
};
use axum::extract::{Path, State};
use rustyline::{DefaultEditor, error::ReadlineError};
use std::sync::Arc;
use tokio::{sync::watch, task::JoinHandle};

pub async fn init_repl(
    State(state): State<Arc<AppState>>,
    shutdown_tx: watch::Sender<bool>,
) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        let mut repl = DefaultEditor::new().expect("Unable to initiate REPL");
        loop {
            let readline = repl.readline(">> ");
            match readline {
                Ok(line) => {
                    let trimmed = line.trim();
                    let _ = repl.add_history_entry(trimmed);
                    let parts: Vec<&str> = trimmed.splitn(4, ' ').collect();

                    match parts.as_slice() {
                        // ── Users ────────────────────────────────────────────
                        ["create", "user", display_name] => {
                            match create_user(
                                State(state.clone()),
                                axum::Json(CreateUserRequest {
                                    display_name: display_name.to_string(),
                                }),
                            )
                            .await
                            {
                                Ok(user) => println!("Created user: {:?}", user),
                                Err(e) => eprintln!("Error creating user: {:?}", e),
                            }
                        }

                        ["get", "user", uuid] => {
                            match get_user(State(state.clone()), Path(uuid.to_string())).await {
                                Ok(user) => println!("User: {:?}", user),
                                Err(e) => eprintln!("Error fetching user: {:?}", e),
                            }
                        }

                        ["update", "user", uuid, display_name] => {
                            match update_user(
                                State(state.clone()),
                                Path(uuid.to_string()),
                                axum::Json(CreateUserRequest {
                                    display_name: display_name.to_string(),
                                }),
                            )
                            .await
                            {
                                Ok(user) => println!("Updated user: {:?}", user),
                                Err(e) => eprintln!("Error updating user: {:?}", e),
                            }
                        }

                        ["delete", "user", uuid] => {
                            match delete_user(State(state.clone()), Path(uuid.to_string())).await {
                                Ok(_) => println!("Deleted user {uuid}"),
                                Err(e) => eprintln!("Error deleting user: {:?}", e),
                            }
                        }

                        // ── Rooms ────────────────────────────────────────────
                        ["create", "room", name] => {
                            match create_room(
                                State(state.clone()),
                                axum::Json(CreateRoomRequest {
                                    name: name.to_string(),
                                }),
                            )
                            .await
                            {
                                Ok(room) => println!("Created room: {:?}", room),
                                Err(e) => eprintln!("Error creating room: {:?}", e),
                            }
                        }

                        ["get", "room", room_id] => {
                            match get_room(State(state.clone()), Path(room_id.to_string())).await {
                                Ok(room) => println!("Room: {:?}", room),
                                Err(e) => eprintln!("Error fetching room: {:?}", e),
                            }
                        }

                        ["update", "room", room_id, name] => {
                            match update_room(
                                State(state.clone()),
                                Path(room_id.to_string()),
                                axum::Json(CreateRoomRequest {
                                    name: name.to_string(),
                                }),
                            )
                            .await
                            {
                                Ok(room) => println!("Updated room: {:?}", room),
                                Err(e) => eprintln!("Error updating room: {:?}", e),
                            }
                        }

                        ["delete", "room", room_id] => {
                            match delete_room(State(state.clone()), Path(room_id.to_string())).await
                            {
                                Ok(_) => println!("Deleted room {room_id}"),
                                Err(e) => eprintln!("Error deleting room: {:?}", e),
                            }
                        }

                        // ── Participants ─────────────────────────────────────
                        ["add", "participant", room_id, user_uuid] => {
                            match add_participant(
                                State(state.clone()),
                                axum::Json(AddParticipantRequest {
                                    room_id: room_id.to_string(),
                                    user_id: user_uuid.to_string(),
                                }),
                            )
                            .await
                            {
                                Ok(p) => println!("Added participant: {:?}", p),
                                Err(e) => eprintln!("Error adding participant: {:?}", e),
                            }
                        }

                        ["get", "participants", room_id] => {
                            match get_participants(State(state.clone()), Path(room_id.to_string()))
                                .await
                            {
                                Ok(list) => println!("Participants: {:?}", list),
                                Err(e) => eprintln!("Error fetching participants: {:?}", e),
                            }
                        }

                        ["remove", "participant", room_id, user_uuid] => {
                            match remove_participant(
                                State(state.clone()),
                                Path((room_id.to_string(), user_uuid.to_string())),
                            )
                            .await
                            {
                                Ok(_) => println!("Removed participant {user_uuid} from {room_id}"),
                                Err(e) => eprintln!("Error removing participant: {:?}", e),
                            }
                        }

                        // ── Game State ───────────────────────────────────────
                        ["create", "gamestate", room_uuid] => {
                            match create_game_state(
                                State(state.clone()),
                                Path(room_uuid.to_string()),
                            )
                            .await
                            {
                                Ok(gs) => println!("Created game state: {:?}", gs),
                                Err(e) => eprintln!("Error creating game state: {:?}", e),
                            }
                        }

                        ["get", "gamestate", room_uuid] => {
                            match get_game_state(State(state.clone()), Path(room_uuid.to_string()))
                                .await
                            {
                                Ok(gs) => println!("Game state: {:?}", gs),
                                Err(e) => eprintln!("Error fetching game state: {:?}", e),
                            }
                        }

                        ["delete", "gamestate", room_uuid] => {
                            match delete_game_state(
                                State(state.clone()),
                                Path(room_uuid.to_string()),
                            )
                            .await
                            {
                                Ok(_) => println!("Deleted game state for room {room_uuid}"),
                                Err(e) => eprintln!("Error deleting game state: {:?}", e),
                            }
                        }

                        // ── Clues ────────────────────────────────────────────
                        ["create", "clue", room_uuid, text] => {
                            match create_clue(
                                State(state.clone()),
                                axum::Json(CreateClueRequest {
                                    room_id: room_uuid.to_string(),
                                    text: text.to_string(),
                                }),
                            )
                            .await
                            {
                                Ok(clue) => println!("Created clue: {:?}", clue),
                                Err(e) => eprintln!("Error creating clue: {:?}", e),
                            }
                        }

                        ["get", "clues", room_uuid] => {
                            match get_clues(State(state.clone()), Path(room_uuid.to_string())).await
                            {
                                Ok(clues) => println!("Clues: {:?}", clues),
                                Err(e) => eprintln!("Error fetching clues: {:?}", e),
                            }
                        }

                        ["delete", "clue", clue_uuid] => {
                            match delete_clue(State(state.clone()), Path(clue_uuid.to_string()))
                                .await
                            {
                                Ok(_) => println!("Deleted clue {clue_uuid}"),
                                Err(e) => eprintln!("Error deleting clue: {:?}", e),
                            }
                        }

                        // ── General ──────────────────────────────────────────
                        ["status"] | ["status", ..] => {
                            println!("Running on {}{}", state.);
                        }

                        ["help"] | ["help", ..] => print_help(),

                        ["quit"] | ["quit", ..] => {
                            println!("Shutting down...");
                            shutdown_tx.send(true).ok();
                            break;
                        }

                        [""] | [] => {}

                        unknown => {
                            println!(
                                "Unknown command: '{}'. Type 'help' for available commands.",
                                unknown.join(" ")
                            );
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }
    })
}

fn print_help() {
    println!("Available commands:");
    println!();
    println!("  Users:");
    println!("    create user <display_name>         - Create a new user");
    println!("    get user <uuid>                    - Get user by ID");
    println!("    update user <uuid> <display_name>  - Update user display name");
    println!("    delete user <uuid>                 - Delete a user");
    println!();
    println!("  Rooms:");
    println!("    create room <name>                 - Create a new room");
    println!("    get room <room_id>                 - Get room by ID");
    println!("    update room <room_id> <name>       - Update room name");
    println!("    delete room <room_id>              - Delete a room");
    println!();
    println!("  Participants:");
    println!("    add participant <room_id> <user_uuid>    - Add user to room");
    println!("    get participants <room_id>               - List participants in room");
    println!("    remove participant <room_id> <user_uuid> - Remove user from room");
    println!();
    println!("  Game State:");
    println!("    create gamestate <room_uuid>       - Create game state for room");
    println!("    get gamestate <room_uuid>          - Get game state for room");
    println!("    delete gamestate <room_uuid>       - Delete game state for room");
    println!();
    println!("  Clues:");
    println!("    create clue <room_uuid> <text>     - Create a clue in a room");
    println!("    get clues <room_uuid>              - List all clues in a room");
    println!("    delete clue <clue_uuid>            - Delete a clue by ID");
    println!();
    println!("  General:");
    println!("    status                             - Show server status");
    println!("    help                               - Show this message");
    println!("    quit                               - Shut down the server");
}
