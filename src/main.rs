use axum::{
    routing::get,
    Json, Router,
};

mod models;
mod routes;
mod error;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .route("/", get(root))
        .nest("/post", routes::post::post_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<String> {
    Json("Root endpoint!".to_string())
}