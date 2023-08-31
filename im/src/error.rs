use std::fmt::Debug;
pub type IMResult<T> = std::result::Result<T, IMError>;

#[derive(Debug, thiserror::Error)]
pub enum IMError {
    #[error("error happen {0}")]
    CustomError(String),
}

pub fn as_im_error<E>(e: E) -> IMError
where
    E: Debug + ToString
{
    IMError::CustomError(e.to_string())
}