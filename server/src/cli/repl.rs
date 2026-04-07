use crate::{
    db::types::AppState,
    handlers::room::{CreateRoomRequest, create_room},
};
use axum::extract::State;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::sync::Arc;
use tokio::{
    sync::{broadcast, watch},
    task::JoinHandle,
};

pub async fn init_repl(
    State(state): State<Arc<AppState>>,
    shutdown_tx: watch::Sender<bool>,
) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        let mut repl = DefaultEditor::new().expect("Unable to initiate REPL");
        loop {
            let readline = repl.readline(">> ");
            match readline {
                Ok(line) => match line.trim() {
                    "create room" => {
                        let room = create_room(
                            State(state.clone()),
                            axum::Json(CreateRoomRequest {
                                display_name: "cli_test".to_string(),
                            }),
                        )
                        .await
                        .unwrap();

                        println!("{:?}", room);
                    }
                    "quit" => {
                        println!("Shutting down...");
                        shutdown_tx.send(true).ok();
                        break;
                    }
                    "status" => {
                        println!("Running on http://127.0.0.1:3000");
                    }
                    "" => {}
                    unknown => {
                        println!("Unknown command: {unknown}.");
                    }
                },
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
