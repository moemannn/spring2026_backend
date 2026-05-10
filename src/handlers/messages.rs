use axum::{
    extract::Path,
    Json,
};
use chrono::Utc;
use utoipa::path;

use crate::models::messages::MessageResponseMin;


/// GET all messages
#[utoipa::path(
    get,
    path = "/api/message/{id}/{message_type}",
    params(
        ("id" = i32, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn get_messages() -> Json<Vec<MessageResponseMin>> {
    Json(vec![])
}


/// DELETE message
#[utoipa::path(
    delete,
    path = "/api/message/{id}/{message_type}",
    params(
        ("id" = i32, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn delete_message(Path(id): Path<i32>) -> Json<()> {
    let _ = id;
    Json(())
}


/// Send message
#[utoipa::path(
    post,
    path = "/api/message/{id}/{message_type}",
    params(
        ("id" = i32, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn post_message() -> Json<MessageResponseMin> {
    Json(MessageResponseMin {
        id: 0,
        message_type: "".into(),
        content: "".into(),
        created_at: "".into(),
    })
}


/// UPDATE message
#[utoipa::path(
    put,
    path = "/api/message/{id}/{message_type}",
    params(
        ("id" = i32, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn edit_message(Path(id): Path<i32>) -> Json<MessageResponseMin> {
    Json(MessageResponseMin {
        id,
        message_type: "".into(),
        content: "".into(),
        created_at: "".into(),
    })
}