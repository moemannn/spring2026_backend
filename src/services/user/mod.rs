use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder};
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryFilter};
use sea_orm::prelude::Expr;
use uuid::Uuid;
use crate::entity::users;
use crate::models::{UserRequest, UserResponse};
use crate::error::AppError;
use crate::services::helpers::ensure_affected;
use bcrypt::hash;

use crate::entity::users::{Entity as User, Column};

pub async fn create_user(
    db: &DatabaseConnection,
    payload: UserRequest,
) -> Result<UserResponse, AppError> {

    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        email: Set(payload.email),
        password: Set(hash(payload.password, 12)?),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    Ok(UserResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    })
}

pub async fn update_user(
    _db: &DatabaseConnection,
    _payload: UserRequest,
) -> Result<UserResponse, AppError> {
    Ok(UserResponse {
        id: Uuid::new_v4(),
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
    })
}

pub async fn delete_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = User::update_many()
        .col_expr(Column::DeletedAt, Expr::value(Some(Utc::now().naive_utc())))
        .filter(Column::Id.eq(user_id))
        .exec(db)
        .await?;

    ensure_affected(result)?;

    Ok(())
}

pub async fn get_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<UserResponse, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    Ok(UserResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    })
}

pub async fn get_users_by_page(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> Result<Vec<UserResponse>, AppError> {
    let paginator = users::Entity::find()
        .order_by_asc(users::Column::Id)
        .paginate(db, page_size);

    let users = paginator.fetch_page(page).await?;

    let result = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            first_name: u.first_name,
            last_name: u.last_name,
            email: u.email,
        })
        .collect();

    Ok(result)
}