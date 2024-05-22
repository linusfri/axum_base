use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router
};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::http::AppState;

#[derive(Deserialize, Serialize)]
pub struct Post {
    id: u64,
    title: String,
    content: String,
    #[serde(with = "time::serde::rfc3339::option")]
    created_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct InsertPost {
    title: String,
    content: String,
}

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/posts", get(fetch_all))
        .route("/posts", post(create))
}

async fn fetch_all(State(state): State<AppState>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

async fn create(State(state): State<AppState>, Json(post): Json<InsertPost>) -> Result<StatusCode, StatusCode> {
    sqlx::query_as!(Post, "INSERT INTO posts (title, content) VALUES (?, ?)", post.title, post.content)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}