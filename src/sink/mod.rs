mod serial;
mod sink_error;
mod sink_type;
mod stdout;

pub use serial::{DEFAULT_BAUD, SerialSink};
pub use sink_error::SinkError;
pub use sink_type::{DEFAULT_SINK_STR, SinkType};
pub use stdout::StdoutSink;

use crate::model::StatusPayload;

/// Default transport operation timeout in milliseconds.
pub const DEFAULT_TIMEOUT_MS: u64 = 500;

/// Status sink abstraction for transmitting agent state updates.
pub trait StatusSink {
    /// Sends a status payload to the target device or endpoint.
    fn send(&mut self, payload: &StatusPayload) -> Result<(), SinkError>;
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
