use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ServerResponseMin {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}