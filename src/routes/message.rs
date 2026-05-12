use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::AppState;
use crate::handlers::users::*;

pub fn message_routes() -> Router<Arc<AppState>> {
    Router::new()
}