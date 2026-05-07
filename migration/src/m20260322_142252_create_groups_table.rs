use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .if_not_exists()

                    // Primary key
                    .col(pk_auto(Groups::Id))

                    // Core
                    .col(string(Groups::Name).not_null())
                    .col(text(Groups::Description).null())

                    // Timestamps
                    .col(timestamp(Groups::CreatedAt).not_null())
                    .col(timestamp_null(Groups::ChangedAt))
                    .col(timestamp_null(Groups::DeletedAt))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Groups::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Groups {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}