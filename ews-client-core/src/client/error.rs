use thiserror::Error;

/// EWS client error types
#[derive(Debug, Error)]
pub enum EwsError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("EWS protocol error: {0}")]
    Protocol(#[from] ews::Error),

    #[error("Authentication failed")]
    Authentication,

    #[error("Response error: {0:?}")]
    Response(ews::response::ResponseError),

    #[error("Operation error: {message}")]
    Operation { message: String },

    #[error("Missing ID in response")]
    MissingId,

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<ews::response::ResponseError> for EwsError {
    fn from(err: ews::response::ResponseError) -> Self {
        Self::Response(err)
    }
}
