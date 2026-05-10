mod db;
mod handlers;
mod routes;
pub mod entity;
mod models;
mod services;

use db::connect_db;
use std::sync::Arc;
use axum::Router;

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = connect_db().await;

    let state = Arc::new(AppState { db });

    let app = Router::new()
        .merge(routes::default::get_default_routes(state.clone()))
        .merge(routes::swagger::get_swagger_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}