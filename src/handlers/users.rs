use axum::{
    extract::Path,
    Json,
};
use utoipa::path;

use crate::models::users::UserResponseMin;


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
        email: "".into(),
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
        (status = 200, body = UserResponseMin)
    )
)]
pub async fn add_user() -> Json<UserResponseMin> {
    Json(UserResponseMin {
        id: 0,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
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
        email: "".into(),
    })
}