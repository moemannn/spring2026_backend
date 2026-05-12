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

                    // PRIMARY KEY
                    .col(
                        ColumnDef::new(Channels::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )
                    // CORE
                    .col(string(Channels::Name).not_null())

                    // FOREIGN/REATION KEYS
                    .col(ColumnDef::new(Channels::ServerId)
                        .uuid()
                        .not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channels_server")
                            .from(Channels::Table, Channels::ServerId)
                            .to(Servers::Table, Servers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )

                    // TIMESTAMPS
                    .col(timestamp(Channels::CreatedAt)
                        .not_null()
                        .default(Expr::cust("CURRENT_TIMESTAMP"))
                    )                    .col(timestamp_null(Channels::ChangedAt))
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
    ServerId,
    CreatedAt,
    ChangedAt,
    DeletedAt,
}
#[derive(DeriveIden)]
enum Servers {
    Table,
    Id,
}
