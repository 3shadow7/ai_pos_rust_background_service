use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
pub enum ServiceError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Device error: {0}")]
    DeviceError(String),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        ServiceError::IoError(err.to_string())
    }
}

impl From<config::ConfigError> for ServiceError {
    fn from(err: config::ConfigError) -> Self {
        ServiceError::ConfigError(err.to_string())
    }
}
