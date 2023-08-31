
pub type MqttResult<T> = std::result::Result<T, MqttError>;

#[derive(Debug, thiserror::Error)]
pub enum MqttError {
    #[error("error happen")]
    CustomError(String),
}