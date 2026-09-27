pub mod serial;

use std::io::{self, Write};

use thiserror::Error;

use crate::model::StatusPayload;

/// Default transport timeout in milliseconds.
pub const DEFAULT_TIMEOUT_MS: u64 = 500;

/// Where status updates are sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum SinkType {
    /// USB serial device.
    Serial,
    /// HTTP endpoint (not implemented yet).
    Http,
    /// Standard output, for dry runs.
    Stdout,
}

/// Errors sending a status update.
#[derive(Debug, Error)]
pub enum SinkError {
    #[error("Sink I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Sink serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Sink connection failed: {0}")]
    Connection(String),
    #[error("Sink error: {0}")]
    Other(String),
}

impl From<serialport::Error> for SinkError {
    fn from(err: serialport::Error) -> Self {
        Self::Connection(err.to_string())
    }
}

/// Writes a status update as one newline-terminated JSON line, then flushes.
pub fn write_line<W: Write>(writer: &mut W, payload: &StatusPayload) -> Result<(), SinkError> {
    let mut line = serde_json::to_vec(payload)?;
    line.push(b'\n');
    writer.write_all(&line)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
