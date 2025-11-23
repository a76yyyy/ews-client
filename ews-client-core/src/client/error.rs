use thiserror::Error;

/// EWS client error types
#[derive(Debug, Error)]
pub enum EwsError {
    /// HTTP transport error (network, connection, etc.)
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// EWS protocol error (SOAP parsing, XML issues, etc.)
    #[error("EWS protocol error: {0}")]
    Protocol(#[from] ews::Error),

    /// Authentication failure (401, invalid credentials, etc.)
    #[error("Authentication failed")]
    Authentication,

    /// EWS response contained an error code
    #[error("Response error: {0:?}")]
    ResponseError(ews::response::ResponseError),

    /// Error processing response data (validation, unexpected format, etc.)
    #[error("Processing error: {message}")]
    Processing { message: String },

    /// Missing required ID in response from Exchange
    #[error("Missing ID in response")]
    MissingIdInResponse,

    /// Response contained unexpected number of messages
    #[error("Unexpected response message count: expected {expected}, got {actual}")]
    UnexpectedResponseMessageCount { expected: usize, actual: usize },

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<ews::response::ResponseError> for EwsError {
    fn from(err: ews::response::ResponseError) -> Self {
        Self::ResponseError(err)
    }
}
