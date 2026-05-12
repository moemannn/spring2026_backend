use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use crate::AppState;
use crate::handlers::users::*;

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users_by_page).post(create_user))
        .route("/me",
               get(get_user_by_id)
                   .post(update_user_by_id)
                   .put(update_user_by_id)
                   .delete(delete_user_by_id))
}

