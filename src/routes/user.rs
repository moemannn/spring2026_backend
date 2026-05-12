use std::sync::Arc;
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::AppState;
use crate::handlers::users::*;

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users_by_page).post(create_user))
        .route("/:id",
               get(get_user_by_id)
                   .put(update_user_by_id)
                   .delete(delete_user_by_id)
        )
}