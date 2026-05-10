use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .if_not_exists()

                    // Primary key
                    .col(pk_auto(Channels::Id))

                    // Core
                    .col(string(Channels::Name).not_null())

                    // Timestamps
                    .col(timestamp(Channels::CreatedAt).not_null())
                    .col(timestamp_null(Channels::ChangedAt))
                    .col(timestamp_null(Channels::DeletedAt))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Channels {
    Table,
    Id,
    Name,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}
