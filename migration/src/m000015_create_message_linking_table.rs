use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessageLinking::Table)
                    .if_not_exists()

                    // PRIMARY KEY
                    .col(ColumnDef::new(MessageLinking::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                    )

                    // CORE
                    .col(
                        ColumnDef::new(MessageLinking::MessageChildId)
                            .uuid()
                            .not_null(),
                    )

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageLinking::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "message_linking")]
enum MessageLinking {
    Table,
    Id,
    MessageChildId,
}