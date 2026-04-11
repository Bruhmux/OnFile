use crypts_and_clues::db::tables::Clue;
use sqlx::PgPool;

#[sqlx::test]
async fn test_clue_crud(pool: PgPool) {
    sqlx::query("INSERT INTO rooms (id, display_name) VALUES ('CLUE1', 'Test')").execute(&pool).await.unwrap();

    let clue = sqlx::query_as::<_, Clue>(
        "INSERT INTO clues (room_id, clue_order, clue_text) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind("CLUE1")
    .bind(1)
    .bind("The suspect wore a hat")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(clue.clue_text, "The suspect wore a hat");
}
