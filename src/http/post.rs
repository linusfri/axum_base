use std::vec;

use axum::{
    extract::State, http::StatusCode, routing::get, Json, Router
};

use serde::{Deserialize, Serialize};
use super::AppState;

#[derive(Deserialize, Serialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug)]
struct Number {
    sum: i64
}


pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/post", get(fetch_all))
        .route("/post/db", get(test_db))
}

async fn test_db(State(state): State<AppState>) -> Result<Json<i64>, StatusCode> {
    let res: Number = sqlx::query_as!(Number, "select 1 + 2 as sum").fetch_one(&state.db).await.unwrap();
    Ok(Json(2))
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