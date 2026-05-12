use std::sync::Arc;
use axum::Router;
use crate::AppState;

pub fn server_routes() -> Router<Arc<AppState>> {
    Router::new()
}