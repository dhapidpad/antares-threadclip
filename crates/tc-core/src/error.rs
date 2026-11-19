use thiserror::Error;

#[derive(Error, Debug)]
pub enum TcError {
    #[error("Token expired: {0}")]
    TokenExpired(String),

    #[error("Rate limited: retry after {0} seconds")]
    RateLimited(u64),

    #[error("Database error: {0}")]
    Db(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid timeline: {0}")]
    InvalidTimeline(String),

    #[error("Render failed: {0}")]
    RenderError(String),

    #[error("TTS error: {0}")]
    TtsError(String),

    #[error("Asset error: {0}")]
    AssetError(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("General error: {0}")]
    General(String),
}

pub type Result<T> = std::result::Result<T, TcError>;
