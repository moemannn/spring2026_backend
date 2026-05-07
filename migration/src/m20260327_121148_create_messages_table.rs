use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()

                    // Primary key
                    .col(pk_auto(Messages::Id))

                    // Core
                    .col(big_integer(Messages::SenderId).not_null())
                    .col(text(Messages::Content).not_null())

                    // Timestamps
                    .col(timestamp(Messages::CreatedAt).not_null())
                    .col(timestamp_null(Messages::ChangedAt))
                    .col(timestamp_null(Messages::DeletedAt))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    Id,
    Content,
    SenderId,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}