use thiserror::Error;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("[I/O error]: {0}")]
    Io(#[from] std::io::Error),

    #[error("[JSON error]: {0}")]
    Json(#[from] serde_json::Error),

    #[error("[Configuration error]: {0}")]
    Config(String),

    #[error("[Application error]: {0}")]
    App(String),

    #[error("[Storage error]: {0}")]
    Storage(String),

    #[error("[Permission denied]: {0}")]
    Permission(String),

    #[error("[WebView error]: {0}")]
    WebView(String),

    #[error("[Boot error]: {0}")]
    Boot(String),

    #[error("[Unidentified?]: {0}")]
    Other(String),
}
