use crate::{
    db::AppState,
    handlers::room::{CreateRoomRequest, create_room},
};
use axum::extract::State;
use std::sync::Arc;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader, Stdin},
    task::JoinHandle,
};

pub async fn cli_loop(State(state): State<Arc<AppState>>) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut reader: BufReader<Stdin> = io::BufReader::new(stdin);
        let mut line_buf = String::new();

        println!("Commands: status | quit");

        loop {
            line_buf.clear();
            match reader.read_line(&mut line_buf).await {
                Ok(0) => break,
                Ok(_) => match line_buf.trim() {
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
                    }
                    "status" => {
                        println!("Running on http://127.0.0.1:3000");
                    }
                    "" => {}
                    unknown => {
                        println!("Unknown command: {unknown}.");
                    }
                },
                Err(e) => {
                    eprintln!("CLI read error: {e}");
                    break;
                }
            }
        }
    })
}
