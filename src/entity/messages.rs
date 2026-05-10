use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub message_mapping_id: i64,

    #[sea_orm(column_type = "Text")]
    pub content: String,

    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub changed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[sea_orm(column_type = "TimestampWithTimeZone")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}