use axum::{
    extract::Path,
    Json,
};
use utoipa::path;

use crate::models::servers::ServerResponseMin;


/// GET all servers
#[utoipa::path(
    get,
    path = "/api/servers",
    responses(
        (status = 200, body = [ServerResponseMin])
    )
)]
pub async fn get_servers() -> Json<Vec<ServerResponseMin>> {
    Json(vec![])
}


/// GET single server by ID
#[utoipa::path(
    get,
    path = "/api/servers/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = ServerResponseMin)
    )
)]
pub async fn get_server(Path(id): Path<i32>) -> Json<ServerResponseMin> {
    Json(ServerResponseMin {
        id,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
}


/// DELETE server
#[utoipa::path(
    delete,
    path = "/api/servers/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200)
    )
)]
pub async fn delete_server(Path(id): Path<i32>) -> Json<()> {
    let _ = id;
    Json(())
}


/// CREATE server
#[utoipa::path(
    post,
    path = "/api/servers",
    responses(
        (status = 200, body = ServerResponseMin)
    )
)]
pub async fn add_server() -> Json<ServerResponseMin> {
    Json(ServerResponseMin {
        id: 0,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
}


/// UPDATE server
#[utoipa::path(
    put,
    path = "/api/servers/{id}",
    params(
        ("id" = i32, Path)
    ),
    responses(
        (status = 200, body = ServerResponseMin)
    )
)]
pub async fn edit_server(Path(id): Path<i32>) -> Json<ServerResponseMin> {
    Json(ServerResponseMin {
        id,
        first_name: "".into(),
        last_name: "".into(),
        email: "".into(),
    })
}