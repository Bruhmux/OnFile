use crypts_and_clues::db::tables::{Category, Clue};
use sqlx::PgPool;

#[sqlx::test]
async fn test_clue_crud(pool: PgPool) {
    sqlx::query("INSERT INTO rooms (id, display_name) VALUES ('TEST1', 'Test Room')")
        .execute(&pool)
        .await
        .unwrap();

    let clue = sqlx::query_as::<_, Clue>(
        r#"
        INSERT INTO clues (room_id, x_category, x_idx, y_category, y_idx, is_true) 
        VALUES ($1, $2, $3, $4, $5, $6) 
        RETURNING *
        "#,
    )
    .bind("TEST1")
    .bind(Category::Suspect)
    .bind(0)
    .bind(Category::Weapon)
    .bind(1)
    .bind(true)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(clue.x_idx, 0);
    assert!(clue.is_true);
}
