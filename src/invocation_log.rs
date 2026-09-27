use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::model::AgentState;

/// Diagnostic record of a single CLI invocation, appended as one JSON line to the `--log` file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvocationLog {
    /// Milliseconds since the Unix epoch when the invocation started.
    pub timestamp_ms: u64,
    /// Command-line arguments, excluding the binary name.
    pub args: Vec<String>,
    /// Raw hook payload read from stdin, if any.
    pub input: Option<String>,
    /// Resulting normalized state, if the payload was mapped successfully.
    pub state: Option<AgentState>,
    /// Whether the payload was delivered to the sink.
    pub sink_ok: bool,
    /// Error message if the invocation failed.
    pub error: Option<String>,
}

impl InvocationLog {
    /// Starts a new record for an invocation with the given arguments, timestamped now.
    pub fn new(args: Vec<String>) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
            .unwrap_or_default();
        Self {
            timestamp_ms,
            args,
            input: None,
            state: None,
            sink_ok: false,
            error: None,
        }
    }

    /// Appends this record as a single newline-terminated JSON line to the file at `path`.
    pub fn append_to(&self, path: &Path) -> io::Result<()> {
        let mut line = serde_json::to_vec(self)?;
        line.push(b'\n');
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        // A single write keeps lines intact when concurrent hook processes append to the same file.
        file.write_all(&line)
    }
}

#[cfg(test)]
#[path = "invocation_log_tests.rs"]
mod tests;
