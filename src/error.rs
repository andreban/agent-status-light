use std::io;

use thiserror::Error;

/// Core domain and application errors for agent-status-light.
#[derive(Debug, Error)]
pub enum Error {
    /// Serialization or deserialization failure.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Standard I/O failure.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Invalid or unrecognized agent state string.
    #[error("Invalid agent state: {0}")]
    InvalidAgentState(String),

    /// Invalid or unrecognized agent kind string.
    #[error("Invalid agent kind: {0}")]
    InvalidAgentKind(String),
}

/// Convenience Result alias for domain operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
#[path = "error_tests.rs"]
mod tests;
