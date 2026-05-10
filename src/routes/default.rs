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
                .route("/users", post(add_user).get(get_users))
                .route("/users/:id", get(get_user))
                .route("/users/:id", put(edit_user))
                .route("/users/:id", delete(delete_user))

                .route("/server", post(add_server).get(get_servers))
                .route("/server/:id", get(get_server))
                .route("/server/:id", put(edit_server))
                .route("/server/:id", delete(delete_server))

                .route("/message/:id/:message_type", post(post_message).get(get_messages))
                .route("/message/:id/:message_type", put(edit_message))
                .route("/message/:id/:message_type", delete(delete_message))

                .route("/debug", get(debug))
        )
        .with_state(state)
}