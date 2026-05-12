use std::sync::Arc;
use axum::{
    extract::Path,
    Json,
};
use axum::extract::State;
use uuid::Uuid;
use crate::AppState;
use crate::models::messages::MessageResponseMin;


pub async fn get_messages() -> Json<Vec<MessageResponseMin>> {
    Json(vec![])
}

pub async fn get_view_messages_by_page(
    State(_state): State<Arc<AppState>>,
    Path((_id, _page, _page_size)): Path<(Uuid, u64, u64)>
) -> Json<()> {
    Json(())
}
pub async fn post_message() -> Json<MessageResponseMin> {
    Json(MessageResponseMin {
        id: Uuid::new_v4(),
        message_type: "".into(),
        content: "".into(),
        created_at: "".into(),
    })
}
pub async fn edit_message(Path(id): Path<Uuid>) -> Json<MessageResponseMin> {
    Json(MessageResponseMin {
        id,
        message_type: "".into(),
        content: "".into(),
        created_at: "".into(),
    })
}