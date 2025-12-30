use axum::response::IntoResponse;

pub async fn root() -> &'static str {
    "API Running..."
}

pub async fn get_rooms() -> impl IntoResponse {
    // TODO: Get room ids, corresponding room names, and host, other users stay private
}
