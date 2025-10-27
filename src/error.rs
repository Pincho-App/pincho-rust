use thiserror::Error;

/// Result type for WirePusher operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the WirePusher SDK
#[derive(Error, Debug)]
pub enum Error {
    /// Authentication failed (401, 403)
    #[error("Authentication failed: {message} (HTTP {status_code})")]
    Authentication {
        /// Error message from the API
        message: String,
        /// HTTP status code
        status_code: u16,
    },

    /// Validation error (400, 404)
    #[error("Validation error: {message} (HTTP {status_code})")]
    Validation {
        /// Error message from the API
        message: String,
        /// HTTP status code
        status_code: u16,
    },

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded: {message} (HTTP {status_code})")]
    RateLimit {
        /// Error message from the API
        message: String,
        /// HTTP status code
        status_code: u16,
    },

    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error
    #[error("API error: {message} (HTTP {status_code})")]
    Api {
        /// Error message from the API
        message: String,
        /// HTTP status code
        status_code: u16,
    },

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Builder validation error
    #[error("Builder validation error: {0}")]
    BuilderValidation(String),
}

impl Error {
    /// Returns the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Authentication { status_code, .. }
            | Error::Validation { status_code, .. }
            | Error::RateLimit { status_code, .. }
            | Error::Api { status_code, .. } => Some(*status_code),
            _ => None,
        }
    }

    /// Returns true if this is an authentication error
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, Error::Authentication { .. })
    }

    /// Returns true if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Error::Validation { .. })
    }

    /// Returns true if this is a rate limit error
    pub fn is_rate_limit_error(&self) -> bool {
        matches!(self, Error::RateLimit { .. })
    }
}
