use std::sync::Arc;
use axum::Router;
use crate::AppState;

pub fn message_routes() -> Router<Arc<AppState>> {
    Router::new()
}