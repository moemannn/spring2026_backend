pub use sea_orm_migration::prelude::*;

mod m000011_create_users_table;
mod m000012_create_servers_table;
mod m000013_create_channels_table;
mod m000014_create_message_mapping_table;
mod m000015_create_message_linking_table;
mod m000016_create_messages_table;
mod m000023_create_refresh_tokens_table;
mod m000024_create_users_to_servers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000011_create_users_table::Migration),
            Box::new(m000012_create_servers_table::Migration),
            Box::new(m000013_create_channels_table::Migration),
            Box::new(m000014_create_message_mapping_table::Migration),
            Box::new(m000015_create_message_linking_table::Migration),
            Box::new(m000016_create_messages_table::Migration),
            Box::new(m000023_create_refresh_tokens_table::Migration),
            Box::new(m000024_create_users_to_servers_table::Migration),
        ]
    }
}