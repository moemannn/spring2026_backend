use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Database(String),
    Authentication(String),
    Internal(String),
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(_) => AppError::NotFound,
            _ => AppError::Database(err.to_string()),
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_: bcrypt::BcryptError) -> Self {
        AppError::Authentication("Password hashing failed".into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}