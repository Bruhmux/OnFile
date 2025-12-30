use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
    friends: Vec<u64>,
}

#[derive(Serialize)]
pub struct Room {
    id: u64,
    players: Vec<User>,
}
