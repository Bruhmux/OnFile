use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(sqlx::Type)]
pub enum GameStatus {
    Lobby,
    InProgress,
    Done,
}
