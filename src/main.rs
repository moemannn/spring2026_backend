mod db;
mod handlers;
mod routes;
mod entity;
mod models;
mod services;
mod error;
mod middleware;
mod state;
mod app;

use db::connect_db;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use state::AppState;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("info"))
        .init();

    let db = connect_db().await;

    #[cfg(feature = "seed")]
    {
        seed_users(&db).await.unwrap();
    }

    let state = Arc::new(AppState { db });

    let app = app::app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}