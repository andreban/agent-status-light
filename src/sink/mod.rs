mod sink_error;
mod sink_type;
mod stdout;

pub use sink_error::SinkError;
pub use sink_type::{DEFAULT_SINK_STR, SinkType};
pub use stdout::StdoutSink;

use crate::model::StatusPayload;

/// Default serial communication baud rate (115200 8N1).
pub const DEFAULT_BAUD: u32 = 115_200;

/// Default transport operation timeout in milliseconds.
pub const DEFAULT_TIMEOUT_MS: u64 = 500;

/// Status sink abstraction for transmitting agent state updates.
pub trait StatusSink {
    /// Sends a status payload to the target device or endpoint.
    fn send(&mut self, payload: &StatusPayload) -> Result<(), SinkError>;
}
