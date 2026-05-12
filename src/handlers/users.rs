use std::sync::Arc;

use axum::{
    extract::{State, Json, Path},
    http::StatusCode,
};

use crate::{
    AppState,
    error::AppError,
    models::users::{UserResponse, UserRequest},
    services::user,
};

/// GET users by page
#[utoipa::path(
    get,
    path = "/api/users/{page}/{page_size}",
    params(
        ("page" = u64, Path),
        ("page_size" = u64, Path)
    ),
    responses(
        (status = 200, body = Vec<UserResponse>)
    )
)]
pub async fn get_users_by_page(
    State(state): State<Arc<AppState>>,
    Path((page, page_size)): Path<(u64, u64)>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = user::get_users_by_page(&state.db, page, page_size).await?;
    Ok(Json(users))
}

/// GET single user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = UserResponse)
    )
)]
pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user::get_user(&state.db, id).await?;
    Ok(Json(user))
}

/// CREATE user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserRequest,
    responses(
        (status = 201, body = UserResponse)
    )
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user::create_user(&state.db, payload).await?;
    Ok(Json(user))
}

/// UPDATE user
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path)
    ),
    request_body = UserRequest,
    responses(
        (status = 200, body = UserResponse)
    )
)]
pub async fn update_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = user::update_user(&state.db, payload).await?;
    Ok(Json(user))
}

/// DELETE user
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 204)
    )
)]
pub async fn delete_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    user::delete_user(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}