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

                    // PRIMARY KEY
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )
                    // CORE
                    .col(string(Users::FirstName).not_null())
                    .col(string(Users::LastName).not_null())
                    .col(string(Users::Email)
                        .not_null()
                        .unique_key()
                    )
                    .col(string(Users::Password).not_null())

                    // FLAG
                    .col(boolean(Users::Admin).not_null().default(false))

                    // TIMESTAMPS
                    .col(timestamp(Users::CreatedAt)
                        .not_null()
                        .default(Expr::cust("CURRENT_TIMESTAMP"))
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