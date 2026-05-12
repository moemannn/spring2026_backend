use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum Type {
    #[sea_orm(string_value = "channel")]
    Channel,

    #[sea_orm(string_value = "dm")]
    PersonalDirectMessage,

    #[sea_orm(string_value = "group")]
    GroupDirectMessage,
}