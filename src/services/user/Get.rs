use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::entity::users;
use crate::models::UserResponseMin;

pub async fn get_user(
    db: &DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    Ok(())
}

pub async fn get_users(
    db: &DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    Ok(())
}