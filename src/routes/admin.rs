use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::AppState;
use crate::handlers::users::*;

pub fn admin_routes() -> Router<Arc<AppState>> {
    Router::new()
}