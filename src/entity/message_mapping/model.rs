use sea_orm::entity::prelude::*;
use super::r#type::Type;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "message_mapping")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub message_type: String,
    pub target_id: Uuid,
    pub scope_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
