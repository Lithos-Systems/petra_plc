use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlcError {
    #[error("Signal not found: {0}")]
    SignalNotFound(String),
    
    #[error("Invalid signal type: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Block execution error: {0}")]
    ExecutionError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, PlcError>;
