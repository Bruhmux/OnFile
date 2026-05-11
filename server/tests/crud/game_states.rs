use crypts_and_clues::db::tables::{GameState, GameStatus};
use sqlx::PgPool;

#[sqlx::test]
async fn test_game_state_crud(pool: PgPool) {
    sqlx::query("INSERT INTO rooms (id, display_name) VALUES ('TS123', 'Test')")
        .execute(&pool)
        .await
        .unwrap();

    let state = sqlx::query_as::<_, GameState>(
        "INSERT INTO game_states (room_id, status, solution_file) VALUES ($1, $2, 0) RETURNING *",
    )
    .bind("TS123")
    .bind(GameStatus::Open)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(state.room_id, "TS123");
    assert_eq!(state.status, GameStatus::Open);
}
