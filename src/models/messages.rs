use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct MessageResponseMin {
    pub id: i32,
    pub message_type: String,
    pub content: String,
    pub created_at: String,
}
