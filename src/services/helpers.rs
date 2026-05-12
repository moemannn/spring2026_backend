use sea_orm::UpdateResult;

use crate::error::AppError;

pub fn ensure_affected(result: UpdateResult) -> Result<(), AppError> {
    if result.rows_affected == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}