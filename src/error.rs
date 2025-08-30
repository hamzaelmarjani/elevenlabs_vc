use std::fmt;

/// All possible errors that can occur when using the ElevenLabs API
#[derive(Debug)]
pub enum ElevenLabsVCError {
    /// HTTP request failed (network issues, timeout, etc.)
    RequestError(reqwest::Error),

    /// API returned an error status code
    ApiError { status: u16, message: String },

    /// Failed to parse JSON response
    ParseError(serde_json::Error),

    /// Failed to parse JSON response
    SerializeError(serde_json::Error),

    /// Invalid API key or authentication failed
    AuthenticationError(String),

    /// Rate limit exceeded
    RateLimitError {
        retry_after: Option<u64>, // seconds
        message: String,
    },

    /// Quota exceeded (not enough credits)
    QuotaExceededError(String),

    /// Invalid input parameters
    ValidationError(String),
}

impl fmt::Display for ElevenLabsVCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElevenLabsVCError::RequestError(e) => write!(f, "Request failed: {}", e),
            ElevenLabsVCError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ElevenLabsVCError::ParseError(e) => write!(f, "Failed to parse: {}", e),
            ElevenLabsVCError::SerializeError(e) => write!(f, "Failed to serialize: {}", e),
            ElevenLabsVCError::AuthenticationError(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            ElevenLabsVCError::RateLimitError {
                retry_after,
                message,
            } => match retry_after {
                Some(seconds) => write!(
                    f,
                    "Rate limit exceeded (retry in {}s): {}",
                    seconds, message
                ),
                None => write!(f, "Rate limit exceeded: {}", message),
            },
            ElevenLabsVCError::QuotaExceededError(msg) => write!(f, "Quota exceeded: {}", msg),
            ElevenLabsVCError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ElevenLabsVCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ElevenLabsVCError::RequestError(e) => Some(e),
            ElevenLabsVCError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ElevenLabsVCError {
    fn from(error: reqwest::Error) -> Self {
        // Check if it's a specific HTTP status error
        if let Some(status) = error.status() {
            let status_code = status.as_u16();
            match status_code {
                401 => ElevenLabsVCError::AuthenticationError("Invalid API key".to_string()),
                429 => {
                    // Try to extract retry-after header if available
                    ElevenLabsVCError::RateLimitError {
                        retry_after: None, // Could be enhanced to parse Retry-After header
                        message: "Too many requests".to_string(),
                    }
                }
                402 => ElevenLabsVCError::QuotaExceededError("Insufficient credits".to_string()),
                _ => ElevenLabsVCError::ApiError {
                    status: status_code,
                    message: error.to_string(),
                },
            }
        } else {
            ElevenLabsVCError::RequestError(error)
        }
    }
}

impl From<serde_json::Error> for ElevenLabsVCError {
    fn from(error: serde_json::Error) -> Self {
        ElevenLabsVCError::ParseError(error)
    }
}
