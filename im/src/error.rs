use std::fmt::Debug;
pub type IMResult<T> = std::result::Result<T, IMError>;

#[derive(Debug, thiserror::Error)]
pub enum IMError {
    #[error("error happen")]
    CustomError(String),
}

pub fn as_mqtt_error<E>(e: E) -> IMError
where
    E: Debug + ToString
{
    IMError::CustomError(e.to_string())
}