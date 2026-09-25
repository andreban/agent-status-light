use std::io;

use thiserror::Error;

/// Errors that can occur when parsing or routing agent hook events.
#[derive(Debug, Error)]
pub enum HookError {
    /// Standard I/O failure while reading hook input.
    #[error("I/O error reading hook input: {0}")]
    Io(#[from] io::Error),

    /// JSON syntax or deserialization error.
    #[error("JSON error parsing hook payload: {0}")]
    Json(#[from] serde_json::Error),

    /// Hook payload is empty.
    #[error("Hook payload is empty")]
    EmptyPayload,

    /// Agent runtime could not be detected from payload.
    #[error("Could not determine agent type from payload")]
    UnknownAgent,

    /// Unrecognized hook event name.
    #[error("Unknown hook event: {0}")]
    UnknownEvent(String),

    /// Missing required field in hook payload.
    #[error("Missing required field in hook payload: {0}")]
    MissingField(String),
}

#[cfg(test)]
#[path = "hook_error_tests.rs"]
mod tests;
