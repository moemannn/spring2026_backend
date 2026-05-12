use std::sync::Arc;
use axum::extract::State;
use crate::entity::{messages, message_mapping};
use sea_orm::{Set, ActiveModelTrait};
use crate::AppState;
use uuid::Uuid;

pub async fn debug(
    State(state): State<Arc<AppState>>,
) -> Result<(), String> {
    let _now = chrono::Utc::now();
    let db = &state.db;

    let mapping = message_mapping::ActiveModel {
        target_id: Set(Uuid::new_v4()),
        scope_id: Set(Some(Uuid::new_v4())),
        message_type: Set(message_mapping::Type::Channel),
        ..Default::default()
    };

    let message_mapping = mapping
        .insert(db)
        .await
        .map_err(|e| e.to_string())?;

    let new_message = messages::ActiveModel {
        message_mapping_id: Set(message_mapping.id),
        content: Set("hello".to_string()),
        ..Default::default()
    };

    new_message
        .insert(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}