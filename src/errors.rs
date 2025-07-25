#[derive(Debug)]
pub enum DSRSError {
    PromptTooLong(usize, usize),
    ApiError(String),
    NetworkError(String),
    ConfigError(String),
}

impl std::fmt::Display for DSRSError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DSRSError::PromptTooLong(len, max) => {
                write!(f, "Prompt too long: {len} chars (max: {max})")
            }
            DSRSError::ApiError(msg) => write!(f, "API error: {msg}"),
            DSRSError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            DSRSError::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for DSRSError {}
