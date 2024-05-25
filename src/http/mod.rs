pub mod post;
pub mod types;

use axum::{
    routing::get,
    Json, Router,
};
use tower_http::trace::TraceLayer;
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
    // Create a new router with a TraceLayer and State.
    let app = api_router()
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Subscribe to tracing events from TraceLayer
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

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