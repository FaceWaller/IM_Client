use std::fmt::Debug;
pub type DataBaseResult<T, E = DataBaseError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DataBaseError {
    #[error("error happen: {0}")]
    CustomError(String),
}

pub fn as_database_error<E>(e: E) -> DataBaseError
where
    E: Debug + ToString
{
    DataBaseError::CustomError(e.to_string())
}