use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use axum::routing::delete;
use crate::{handlers::*, AppState};

pub fn get_default_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/users", get(get_users).post(add_user))
                .route("/users/:id", get(get_user))
                .route("/users/:id", put(edit_user))
                .route("/users/:id", delete(delete_user))

                .route("/server", get(get_servers).post(add_server))
                .route("/server/:id", get(get_server))
                .route("/server/:id", put(edit_server))
                .route("/server/:id", delete(delete_server))
        )
        .with_state(state)
}