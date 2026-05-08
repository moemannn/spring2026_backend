use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Servers::Table)
                    .if_not_exists()

                    // Primary key
                    .col(pk_auto(Servers::Id))

                    // Core
                    .col(string(Servers::Name).not_null())
                    .col(text(Servers::Description).null())

                    // Timestamps
                    .col(timestamp(Servers::CreatedAt).not_null())
                    .col(timestamp_null(Servers::ChangedAt))
                    .col(timestamp_null(Servers::DeletedAt))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Servers::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Servers {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}