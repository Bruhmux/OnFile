use sqlx::PgPool;

#[derive(sqlx::Type)]
pub enum GameStatus {
    Lobby,
    InProgress,
    Done,
}
