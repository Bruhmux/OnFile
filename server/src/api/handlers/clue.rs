use crate::{
    db::tables::{Category, Clue},
    error::AppError,
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

pub async fn list(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Response, AppError> {
    let clues: Vec<Clue> = sqlx::query_as::<_, Clue>("SELECT * FROM clues WHERE room_id = $1")
        .bind(&room_id)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(clues).into_response())
}

pub async fn insert_clue(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    room_id: &str,
    x_cat: Category,
    x_idx: i32,
    y_cat: Category,
    y_idx: i32,
    is_true: bool,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO clues (id, room_id, x_category, x_idx, y_category, y_idx, is_true)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(id)
    .bind(room_id)
    .bind(x_cat)
    .bind(x_idx)
    .bind(y_cat)
    .bind(y_idx)
    .bind(is_true)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
