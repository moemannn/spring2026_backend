pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20260322_142252_create_groups_table;
mod m20260327_121148_create_messages_table;
mod m20260507_153547_create_groups_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20260322_142252_create_groups_table::Migration),
            Box::new(m20260327_121148_create_messages_table::Migration),
            Box::new(m20260507_153547_create_groups_users_table::Migration),
        ]
    }
}
