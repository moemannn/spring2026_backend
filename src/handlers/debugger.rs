use std::sync::Arc;
use axum::extract::State;
use crate::entity::{messages, message_mapping};
use sea_orm::{Set, ActiveModelTrait};
use crate::AppState;

pub async fn debug(
    State(state): State<Arc<AppState>>,
) -> Result<(), String> {

    let db = &state.db;

    // Insert mapping first
    let mapping = message_mapping::ActiveModel {
        target_id: Set(Some(123)),
        scope_id: Set(Some(456)),
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
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    new_message
        .insert(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}