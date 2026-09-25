use std::io::{self, Write};

use super::StatusSink;
use super::sink_error::SinkError;
use crate::model::StatusPayload;

/// Sink that writes serialized newline-delimited JSON status payloads to standard output or a generic writer.
#[derive(Debug)]
pub struct StdoutSink<W = io::Stdout> {
    writer: W,
}

impl StdoutSink<io::Stdout> {
    /// Creates a new stdout sink writing to `std::io::stdout()`.
    pub fn new() -> Self {
        Self {
            writer: io::stdout(),
        }
    }
}

impl Default for StdoutSink<io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl<W: Write> StdoutSink<W> {
    /// Creates a sink writing to the provided custom writer.
    pub fn with_writer(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> StatusSink for StdoutSink<W> {
    fn send(&mut self, payload: &StatusPayload) -> Result<(), SinkError> {
        let mut json = serde_json::to_string(payload)?;
        json.push('\n');
        self.writer.write_all(json.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "stdout_tests.rs"]
mod tests;
