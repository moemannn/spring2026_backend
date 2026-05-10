use std::sync::Arc;
use axum::{
    extract::{State, Json},
};
use axum::extract::Path;
use axum::http::StatusCode;
use utoipa::path;

use crate::AppState;
use crate::models::users::*;
use crate::services::user::*;

/// GET all users
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, body = [UserResponseMin])
    )
)]
pub async fn get_users() -> Json<Vec<UserResponseMin>> {
    Json(vec![])
}


/// GET single user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = UserResponseMin)
    )
)]
pub async fn get_user(Path(id): Path<i32>) -> Json<UserResponseMin> {
    Json(UserResponseMin {
        id,
        first_name: "".into(),
        last_name: "".into(),
    })
}


/// DELETE user
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200)
    )
)]
pub async fn delete_user(Path(id): Path<i32>) -> Json<()> {
    let _ = id;
    Json(())
}


/// CREATE user
#[utoipa::path(
    post,
    path = "/api/users",
    responses(
        (status = 200, body = CreateUserResponse)
    )
)]
pub async fn add_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, axum::http::StatusCode> {
    let user = create_user(&state.db, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

/// UPDATE user
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = UserResponseMin)
    )
)]
pub async fn edit_user(Path(id): Path<i32>) -> Json<UserResponseMin> {
    Json(UserResponseMin {
        id,
        first_name: "".into(),
        last_name: "".into(),
    })
}