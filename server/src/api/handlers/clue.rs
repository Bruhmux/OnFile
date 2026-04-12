use crate::{
    state::AppState,
    types::{Category, Clue},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rand::RngExt;

pub async fn list(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Json<Vec<Clue>>, StatusCode> {
    let clues = sqlx::query_as::<_, Clue>("SELECT * FROM clues WHERE room_id = $1")
        .bind(&room_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(clues))
}

pub async fn setup_room(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut rng = rand::rng();
    let guilty_suspect: i32 = rng.random_range(0..8);
    let guilty_weapon: i32 = rng.random_range(0..8);
    let guilty_location: i32 = rng.random_range(0..8);

    // 2. Generate all clues (64x3 total relations)
    // Simplify for now: Just generate the matches and a few mismatches
    // Real logic: Generate entire matrix of facts

    let mut transaction: sqlx::Transaction<'_, sqlx::Postgres> = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Clear existing
    sqlx::query("DELETE FROM clues WHERE room_id = $1")
        .bind(&room_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert Winning Match (S-W-L)
    insert_clue(
        &mut transaction,
        &room_id,
        Category::Suspect,
        guilty_suspect,
        Category::Weapon,
        guilty_weapon,
        true,
    )
    .await?;
    insert_clue(
        &mut transaction,
        &room_id,
        Category::Suspect,
        guilty_suspect,
        Category::Location,
        guilty_location,
        true,
    )
    .await?;
    insert_clue(
        &mut transaction,
        &room_id,
        Category::Weapon,
        guilty_weapon,
        Category::Location,
        guilty_location,
        true,
    )
    .await?;

    // TODO: Generate the rest of the ❌ facts

    transaction
        .commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn insert_clue(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    room_id: &str,
    x_cat: Category,
    x_idx: i32,
    y_cat: Category,
    y_idx: i32,
    is_true: bool,
) -> Result<(), StatusCode> {
    sqlx::query(
        r#"
        INSERT INTO clues (id, room_id, x_category, x_idx, y_category, y_idx, is_true)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(room_id)
    .bind(x_cat)
    .bind(x_idx)
    .bind(y_cat)
    .bind(y_idx)
    .bind(is_true)
    .execute(&mut **tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
