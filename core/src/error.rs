use thiserror::Error;

/// Core framework errors
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Module error: {0}")]
    Module(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for core operations
pub type CoreResult<T> = Result<T, CoreError>;
