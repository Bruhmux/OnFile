use crypts_and_clues::db::tables::{User, Room, RoomParticipant};
use sqlx::PgPool;

#[sqlx::test]
async fn test_add_participant(pool: PgPool) {
    sqlx::query("INSERT INTO rooms (id, display_name) VALUES ('PART1', 'Test')").execute(&pool).await.unwrap();
    let user = sqlx::query_as::<_, User>("INSERT INTO users (display_name) VALUES ('U1') RETURNING *").fetch_one(&pool).await.unwrap();

    let part = sqlx::query_as::<_, RoomParticipant>(
        "INSERT INTO room_participants (room_id, user_id, is_host) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind("PART1")
    .bind(user.id)
    .bind(true)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(part.room_id, "PART1");
    assert!(part.is_host);
}
