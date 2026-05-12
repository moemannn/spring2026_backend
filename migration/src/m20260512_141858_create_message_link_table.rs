use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessageLink::Table)
                    .if_not_exists()

                    // PRIMARY KEY
                    .col(
                        ColumnDef::new(MessageLink::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )

                    // RELATION KEY
                    .col(
                        ColumnDef::new(MessageLink::MessageParentId)
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(MessageLink::MessageChildId)
                            .uuid()
                            .not_null()
                    )

                    // PREVENTIVE DUP.
                    .index(
                        Index::create()
                            .name("idx_message_link_unique")
                            .col(MessageLink::MessageParentId)
                            .col(MessageLink::MessageChildId)
                            .unique(),
                    )

                    // FOREIGN KEY
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_parent_message")
                            .from(MessageLink::Table, MessageLink::MessageParentId)
                            .to(Messages::Table, Messages::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_child_message")
                            .from(MessageLink::Table, MessageLink::MessageChildId)
                            .to(Messages::Table, Messages::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageLink::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MessageLink {
    Table,
    Id,
    MessageChildId,
    MessageParentId,
}
#[derive(DeriveIden)]
enum Messages {
    Table,
    Id,

}
