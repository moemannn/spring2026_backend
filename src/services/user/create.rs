use sea_orm::{DatabaseConnection, Set, ActiveModelTrait};
use crate::entity::users;
use crate::models::{CreateUserRequest, CreateUserResponse};

pub async fn create_user(
    db: &DatabaseConnection,
    payload: CreateUserRequest,
) -> Result<CreateUserResponse, sea_orm::DbErr> {

    let new_user = users::ActiveModel {
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        email: Set(payload.email),
        password: Set(payload.password),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    Ok(CreateUserResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    })
}