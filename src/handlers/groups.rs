use axum::{
    extract::Path,
    Json,
};
use utoipa::path;

use crate::models::groups::GroupResponseMin;


/// GET all groups
#[utoipa::path(
    get,
    path = "/api/groups",
    responses(
        (status = 200, body = [GroupResponseMin])
    )
)]
pub async fn get_groups() -> Json<Vec<GroupResponseMin>> {
    Json(vec![])
}


/// GET single group by ID
#[utoipa::path(
    get,
    path = "/api/groups/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = GroupResponseMin)
    )
)]
pub async fn get_group(Path(id): Path<i32>) -> Json<GroupResponseMin> {
    Json(GroupResponseMin {
        id,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
}


/// DELETE group
#[utoipa::path(
    delete,
    path = "/api/groups/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200)
    )
)]
pub async fn delete_group(Path(id): Path<i32>) -> Json<()> {
    let _ = id;
    Json(())
}


/// CREATE group
#[utoipa::path(
    post,
    path = "/api/groups",
    responses(
        (status = 200, body = GroupResponseMin)
    )
)]
pub async fn add_group() -> Json<GroupResponseMin> {
    Json(GroupResponseMin {
        id: 0,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
}


/// UPDATE group
#[utoipa::path(
    put,
    path = "/api/groups/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = GroupResponseMin)
    )
)]
pub async fn edit_group(Path(id): Path<i32>) -> Json<GroupResponseMin> {
    Json(GroupResponseMin {
        id,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
}