use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::entity::users;
use crate::models::UserResponseMin;

pub async fn delete_user(
    db: &DatabaseConnection,
    id: i32,
) -> Result<(), sea_orm::DbErr> {
    let now = chrono::Utc::now().naive_utc();
    let user = users::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("User not found".into()))?;

    let mut active: users::ActiveModel = user.into();

    active.deleted_at = Set(Some(now));

    active.update(db).await?;

    Ok(())
}