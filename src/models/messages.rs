use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct MessageResponseMin {
    pub id: Uuid,
    pub message_type: String,
    pub content: String,
    pub created_at: String,
}
