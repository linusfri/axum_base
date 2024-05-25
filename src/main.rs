use sqlx::mysql::MySqlPoolOptions;
use axum_base::config::Config;
use axum_base::http;

#[tokio::main]
async fn main() {
    let config = Config::new();

    let db = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&config.connection_string)
        .await
        .expect("Could not connect to database.");

    let app_state = http::AppState::new(db);

    http::serve(config, app_state).await;
}