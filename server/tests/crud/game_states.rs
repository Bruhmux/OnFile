use crypts_and_clues::db::tables::{GameState, GameStatus};
use sqlx::PgPool;
use sqlx::types::Json;
use serde_json::json;

#[sqlx::test]
async fn test_game_state_crud(pool: PgPool) {
    sqlx::query("INSERT INTO rooms (id, display_name) VALUES ('GS1', 'Test')").execute(&pool).await.unwrap();
    
    let solution = json!({"answer": 42});
    let state = sqlx::query_as::<_, GameState>(
        "INSERT INTO game_states (room_id, status, solution_data) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind("GS1")
    .bind(GameStatus::Open)
    .bind(Json(solution.clone()))
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(state.room_id, "GS1");
    assert_eq!(state.solution_data.0, solution);
}
