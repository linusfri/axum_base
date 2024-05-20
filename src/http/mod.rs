pub mod post;

use axum::{
    routing::get,
    Json, Router,
};
use crate::config::Config;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    db: MySqlPool
}

impl AppState {
    pub fn new(db: MySqlPool) -> Self {
        Self {
            db
        }
    }
}

pub async fn serve(config: Config, state: AppState) {
    let app = api_router().with_state(state);

    let listener = tokio::net::TcpListener::bind(config.address_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<String> {
    Json(String::from("Ping"))
}

fn root_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
}

fn api_router() -> Router<AppState> {
    root_router()
        .merge(post::post_routes())
}