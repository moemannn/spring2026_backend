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

                    // PRIMARY KEY (FIXED -> BIGINT)
                    .col(
                        big_integer(Messages::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    // RELATIONS
                    .col(
                        big_integer(Messages::MessageMappingId)
                            .not_null()
                    )

                    // CORE
                    .col(text(Messages::Content).not_null())

                    // TIMESTAMPS
                    .col(timestamp(Messages::CreatedAt).not_null())
                    .col(timestamp_null(Messages::ChangedAt))
                    .col(timestamp_null(Messages::DeletedAt))

                    // FOREIGN KEY
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_mapping")
                            .from(Messages::Table, Messages::MessageMappingId)
                            .to(MessageMapping::Table, MessageMapping::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )

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
    MessageMappingId,
    Content,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum MessageMapping {
    Table,
    Id,
}