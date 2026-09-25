use std::io;

use thiserror::Error;

/// Errors that can occur when transmitting status updates to a sink.
#[derive(Debug, Error)]
pub enum SinkError {
    /// I/O error occurred during transmission.
    #[error("Sink I/O error: {0}")]
    Io(#[from] io::Error),

    /// Payload serialization error.
    #[error("Sink serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Connection to endpoint or port failed.
    #[error("Sink connection failed: {0}")]
    Connection(String),

    /// Operation timed out.
    #[error("Sink operation timed out: {0}")]
    Timeout(String),

    /// Other sink failure.
    #[error("Sink error: {0}")]
    Other(String),
}

#[cfg(test)]
#[path = "sink_error_tests.rs"]
mod tests;
