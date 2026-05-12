use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UsersToServers::Table)
                    .if_not_exists()

                    // PRIMARY KEY
                    .col(ColumnDef::new(UsersToServers::Id)
                        .not_null()
                        .uuid()
                    )

                    // FOREIGN/RELEATION KEYS
                    .col(ColumnDef::new(UsersToServers::ServerId)
                        .not_null()
                        .uuid()
                    )
                    .col(ColumnDef::new(UsersToServers::UserId)
                        .not_null()
                        .uuid()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_uuid")
                            .from(UsersToServers::Table, UsersToServers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_server_uuid")
                            .from(UsersToServers::Table, UsersToServers::UserId)
                            .to(Servers::Table, Servers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UsersToServers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UsersToServers {
    Table,
    Id,
    UserId,
    ServerId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Servers {
    Table,
    Id,
}
