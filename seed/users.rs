use uuid::Uuid;
use sea_orm::{ActiveModelTrait, Set};
use fake::Fake;
use crate::entity::users;

#[cfg(feature = "seed")]
pub async fn seed_users(db: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        first_name: Set(FirstName().fake().to_string()),
        last_name: Set(LastName().to_string()),
        email: Set(Email().to_string()),
        password: Set("default".to_string()),
    };

    user.insert(db).await?;

    Ok(())
}