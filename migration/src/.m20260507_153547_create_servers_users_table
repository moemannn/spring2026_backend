use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServersUsers::Table)
                    .if_not_exists()

                    // Foreign key
                    .col(big_integer(ServersUsers::UserId).not_null())
                    .col(big_integer(ServersUsers::GroupId).not_null())

                    // Primary key
                    .primary_key(
                        Index::create()
                            .col(ServersUsers::UserId)
                            .col(ServersUsers::GroupId),
                    )

                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_Servers_users_user_id")
                    .from(ServersUsers::Table, ServersUsers::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_Servers_users_group_id")
                    .from(ServersUsers::Table, ServersUsers::GroupId)
                    .to(Servers::Table, Servers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ServersUsers::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ServersUsers {
    Table,
    UserId,
    GroupId,
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