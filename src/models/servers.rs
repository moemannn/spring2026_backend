use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct ServerResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime,
}

#[derive(Deserialize, ToSchema)]
pub struct ServerRequest {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}