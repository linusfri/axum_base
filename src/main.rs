use axum::{
    routing::get,
    Json, Router,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<String> {
    Json("Root endpoint!".to_string())
}