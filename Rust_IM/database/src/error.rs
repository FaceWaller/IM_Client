pub type MyResult<T> = std::result::Result<T, SQLiteError>;

#[derive(Debug, thiserror::Error)]
pub enum SQLiteError {
    #[error("[WCustomError] WCustomError Happened: {0}")]
    CustomError(String),

    #[error("[R2d2Error] R2d2Error Happened: {0}")]
    R2d2Error(#[from] diesel::r2d2::Error),

    #[error("[DieselError] DieselError Happened: {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("[ioError] IoError happened: {0}")]
    IoError(#[from] std::io::Error),
}
impl SQLiteError {
    pub fn new(st: &str) -> SQLiteError {
        SQLiteError::CustomError(st.to_string())
    }
}

pub fn map_sqlite_error<E: ToString>(e: E) -> SQLiteError {
    SQLiteError::CustomError(e.to_string())
}
