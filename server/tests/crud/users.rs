use crypts_and_clues::db::tables::User;
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_user(pool: PgPool) {
    let name = "test_user".to_string();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (display_name) VALUES ($1) RETURNING *"
    )
    .bind(&name)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(user.display_name, name);
}

#[sqlx::test]
async fn test_update_user(pool: PgPool) {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (display_name) VALUES ($1) RETURNING *"
    )
    .bind("old_name")
    .fetch_one(&pool)
    .await
    .unwrap();

    let new_name = "new_name".to_string();
    let updated = sqlx::query_as::<_, User>(
        "UPDATE users SET display_name = $1 WHERE id = $2 RETURNING *"
    )
    .bind(&new_name)
    .bind(user.id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(updated.display_name, new_name);
}
