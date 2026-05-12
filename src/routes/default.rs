use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::{
    handlers::debugger::debug,
    AppState,
    routes::{
        user::users_routes,
        admin::admin_routes,
        message::message_routes,
        server::server_routes,
    }};

pub fn get_default_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/users", users_routes())
                .nest("/admin", admin_routes())
                .nest("/servers", server_routes())
                .nest("/messages", message_routes())
        )
        .with_state(state)
}