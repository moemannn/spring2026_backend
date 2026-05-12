use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::handlers::servers::{};

pub fn message_routes() -> Router<Arc<AppState>> {
    Router::new()
        // .route("/:page/:page_size", get(delete_message))
        // .route("/", post(post_message))
}