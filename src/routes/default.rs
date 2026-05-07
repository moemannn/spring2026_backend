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

                .route("/groups", get(get_groups).post(add_user))
                .route("/groups/:id", get(get_user))
                .route("/groups/:id", put(edit_user))
                .route("/groups/:id", delete(delete_user))
        )
        .with_state(state)
}