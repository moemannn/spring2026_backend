pub use sea_orm_migration::prelude::*;

mod m20260301_000011_create_users_table;
mod m20260301_000012_create_servers_table;
mod m20260301_000013_create_channels_table;
mod m20260301_000014_create_message_mapping_table;
mod m20260301_000015_create_message_linking_table;
mod m20260301_000016_create_messages_table;
mod m20260301_000023_create_refresh_tokens_table;
mod m20260301_000024_create_users_to_servers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260301_000011_create_users_table::Migration),
            Box::new(m20260301_000012_create_servers_table::Migration),
            Box::new(m20260301_000013_create_channels_table::Migration),
            Box::new(m20260301_000014_create_message_mapping_table::Migration),
            Box::new(m20260301_000015_create_message_linking_table::Migration),
            Box::new(m20260301_000016_create_messages_table::Migration),
            Box::new(m20260301_000023_create_refresh_tokens_table::Migration),
            Box::new(m20260301_000024_create_users_to_servers_table::Migration),
        ]
    }
}