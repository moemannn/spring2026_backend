pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20260322_142252_create_servers_table;
mod m20260508_125128_create_channels_table;
mod m20260508_172157_create_message_mapping_table;
mod m20260508_173123_create_messages_table;
mod m20260512_113746_create_refresh_tokens_table;
mod m20260512_140039_create_users_to_servers_table;
mod m20260512_141858_create_message_link_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20260322_142252_create_servers_table::Migration),
            Box::new(m20260508_125128_create_channels_table::Migration),
            Box::new(m20260508_172157_create_message_mapping_table::Migration),
            Box::new(m20260508_173123_create_messages_table::Migration),
            Box::new(m20260512_113746_create_refresh_tokens_table::Migration),
            Box::new(m20260512_140039_create_users_to_servers_table::Migration),
            Box::new(m20260512_141858_create_message_link_table::Migration),
        ]
    }
}
