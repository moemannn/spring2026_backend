use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GroupsUsers::Table)
                    .if_not_exists()

                    // Foreign key
                    .col(big_integer(GroupsUsers::UserId).not_null())
                    .col(big_integer(GroupsUsers::GroupId).not_null())

                    // Primary key
                    .primary_key(
                        Index::create()
                            .col(GroupsUsers::UserId)
                            .col(GroupsUsers::GroupId),
                    )

                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_groups_users_user_id")
                    .from(GroupsUsers::Table, GroupsUsers::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_groups_users_group_id")
                    .from(GroupsUsers::Table, GroupsUsers::GroupId)
                    .to(Groups::Table, Groups::Id)
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
                    .table(GroupsUsers::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum GroupsUsers {
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
enum Groups {
    Table,
    Id,
}