use std::vec;

use axum::{
    extract::State, http::StatusCode, routing::get, Json, Router
};

use serde::{Deserialize, Serialize};
use sqlx::Executor;

use super::AppState;

#[derive(Deserialize, Serialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}


pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/post", get(fetch_all))
}

async fn fetch_all(State(state): State<AppState>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = vec![
        Post {
            id: 1,
            title: "First Post".to_string(),
            body: "This is the first post".to_string(),
            published: true,
        },
        Post {
            id: 2,
            title: "Second Post".to_string(),
            body: "This is the second post".to_string(),
            published: false,
        }
    ];

    Ok(Json(posts))
}