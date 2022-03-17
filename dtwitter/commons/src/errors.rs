use std::error::Error;

pub type AppError = Box<dyn Error + Sync + Send + 'static>;
pub type AppResult<T> = Result<T, AppError>;
