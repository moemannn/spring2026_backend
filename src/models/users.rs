use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UserResponseMin {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateUserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}