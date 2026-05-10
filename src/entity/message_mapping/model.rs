use sea_orm::entity::prelude::*;
use super::r#type::Type;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "message_mapping")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub target_id: Option<i64>,
    pub scope_id: Option<i64>,
    pub message_type: Type,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}