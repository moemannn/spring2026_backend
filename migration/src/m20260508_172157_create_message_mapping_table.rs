use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessageMapping::Table)
                    .if_not_exists()

                    // PRIMARY KEY (FIXED -> BIGINT)
                    .col(
                        big_integer(MessageMapping::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    // TYPE
                    .col(string(MessageMapping::MessageType).not_null())

                    // RELATIONS
                    .col(big_integer(MessageMapping::TargetId).null())
                    .col(big_integer(MessageMapping::ScopeId).null())

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageMapping::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MessageMapping {
    Table,
    Id,
    MessageType,
    TargetId,
    ScopeId,
}