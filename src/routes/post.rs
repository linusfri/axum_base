use axum::{
    http::StatusCode, routing::get, Json, Router
};
use crate::models::post::Post;


pub fn post_routes() -> Router {
    Router::new()
        .route("/", get(fetch_all))
}

async fn fetch_all() -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = Post::fetch_all();

    Ok(Json(posts))
}