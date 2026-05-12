use axum::{
    extract::Path,
    Json,
};
use uuid::Uuid;
use crate::models::messages::MessageResponseMin;


/// GET all messages
#[utoipa::path(
    get,
    path = "/api/message/{id}/{message_type}",
    params(
        ("id" = Uuid, Path),
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
        ("id" = Uuid, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn delete_message(Path(id): Path<Uuid>) -> Json<()> {
    let _ = id;
    Json(())
}


/// Send message
#[utoipa::path(
    post,
    path = "/api/message/{id}/{message_type}",
    params(
        ("id" = Uuid, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn post_message() -> Json<MessageResponseMin> {
    Json(MessageResponseMin {
        id: Uuid::new_v4(),
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
        ("id" = Uuid, Path),
        ("message_type" = i32, Path)
    ),
    responses(
        (status = 200, body = [MessageResponseMin])
    )
)]
pub async fn edit_message(Path(id): Path<Uuid>) -> Json<MessageResponseMin> {
    Json(MessageResponseMin {
        id,
        message_type: "".into(),
        content: "".into(),
        created_at: "".into(),
    })
}