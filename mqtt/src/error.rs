use std::fmt::Debug;
pub type MqttResult<T> = std::result::Result<T, MqttError>;

#[derive(Debug, thiserror::Error)]
pub enum MqttError {
    #[error("error happen")]
    CustomError(String),
}

pub fn as_mqtt_error<E>(e: E) -> MqttError
where
    E: Debug + ToString
{
    MqttError::CustomError(e.to_string())
}