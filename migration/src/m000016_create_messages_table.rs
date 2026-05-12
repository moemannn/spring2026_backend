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

                    // PRIMARY KEY
                    .col(
                        ColumnDef::new(Messages::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )

                    // CORE
                    .col(text(Messages::Content).not_null())

                    // FOREIGN/RELATIONS KEYS
                    .col(
                        ColumnDef::new(Messages::MessageMappingId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_mapping")
                            .from(Messages::Table, Messages::MessageMappingId)
                            .to(MessageMapping::Table, MessageMapping::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )

                    .col(
                        ColumnDef::new(Messages::MessageLinkingId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_linking")
                            .from(Messages::Table, Messages::MessageLinkingId)
                            .to(MessageLinking::Table, MessageLinking::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )

                    .col(
                        ColumnDef::new(Messages::MessageOwnerId)
                            .uuid()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_owner")
                            .from(Messages::Table, Messages::MessageOwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )

                    // TIMESTAMPS
                    .col(timestamp(Messages::CreatedAt)
                        .not_null()
                        .default(Expr::cust("CURRENT_TIMESTAMP"))
                    )
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
    MessageMappingId,
    MessageLinkingId,
    MessageOwnerId,
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

#[derive(DeriveIden)]
enum MessageLinking{
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users{
    Table,
    Id,
}