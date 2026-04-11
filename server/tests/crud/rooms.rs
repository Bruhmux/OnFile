use crypts_and_clues::db::tables::Room;
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_room(pool: PgPool) {
    let id = "ROOM1".to_string();
    let name = "Test Room".to_string();
    let room = sqlx::query_as::<_, Room>(
        "INSERT INTO rooms (id, display_name) VALUES ($1, $2) RETURNING *"
    )
    .bind(&id)
    .bind(&name)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(room.id, id);
    assert_eq!(room.display_name, name);
}

#[sqlx::test]
async fn test_get_rooms_ordered(pool: PgPool) {
    sqlx::query("INSERT INTO rooms (id, display_name) VALUES ('R1', 'First'), ('R2', 'Second')")
        .execute(&pool)
        .await
        .unwrap();

    let rooms = sqlx::query_as::<_, Room>("SELECT * FROM rooms ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(rooms.len(), 2);
    assert_eq!(rooms[0].id, "R2");
}
