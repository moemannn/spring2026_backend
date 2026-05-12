use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UserResponseMin {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UserRequest {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
