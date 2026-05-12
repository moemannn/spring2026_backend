use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder, UpdateResult};
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, DbErr, EntityTrait, QueryFilter};
use sea_orm::prelude::Expr;
use crate::entity::users;
use crate::models::{UserRequest, UserResponse, UserResponseMin};
use crate::error::AppError;
use crate::services::helpers::ensure_affected;

use crate::entity::users::{Entity as User, Column};

pub async fn create_user(
    db: &DatabaseConnection,
    payload: UserRequest,
) -> Result<UserResponse, AppError> {

    let new_user = users::ActiveModel {
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        email: Set(payload.email),
        password: Set(payload.password),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    Ok(UserResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        password: user.password,
    })
}

pub async fn update_user(
    db: &DatabaseConnection,
    payload: UserRequest,
) -> Result<UserResponse, AppError> {
    Ok(UserResponse {
        id: 1,
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    })
}

pub async fn delete_user(
    db: &DatabaseConnection,
    user_id: i32,
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
    user_id: i32,
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
        password: user.password,
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
            password: u.password,
        })
        .collect();

    Ok(result)
}