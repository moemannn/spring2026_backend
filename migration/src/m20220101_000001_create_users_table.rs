use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()

                    // Primary key
                    .col(pk_auto(Users::Id))

                    // Core
                    .col(string(Users::FirstName).not_null())
                    .col(string(Users::LastName).not_null())
                    .col(string(Users::Email).not_null())
                    .col(string(Users::Password).not_null())

                    // Flag
                    .col(boolean(Users::Admin).not_null().default(false))

                    // Timestamps
                    .col(
                        timestamp(Users::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_null(Users::ChangedAt))
                    .col(timestamp_null(Users::DeletedAt))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Users::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    Admin,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}