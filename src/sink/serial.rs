use std::io::Write;
use std::thread;
use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, StopBits};

#[cfg(test)]
use super::DEFAULT_TIMEOUT_MS;
use super::StatusSink;
use super::sink_error::SinkError;
use crate::model::StatusPayload;

/// Default serial communication baud rate (115200 8N1).
pub const DEFAULT_BAUD: u32 = 115_200;

/// Number of attempts to open the serial port. Agents run hooks for parallel tool calls
/// concurrently, and the port can only be held by one process at a time.
pub const OPEN_ATTEMPTS: u32 = 4;

/// Delay between serial port open attempts.
pub const OPEN_RETRY_DELAY: Duration = Duration::from_millis(50);

/// Direct USB serial transport sink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialSink {
    port_name: String,
    baud_rate: u32,
    timeout: Duration,
}

impl SerialSink {
    /// Creates a serial sink with custom baud rate and timeout.
    pub fn with_options(port_name: impl Into<String>, baud_rate: u32, timeout: Duration) -> Self {
        Self {
            port_name: port_name.into(),
            baud_rate,
            timeout,
        }
    }
}

#[cfg(test)]
impl SerialSink {
    /// Creates a new serial sink with default baud rate and timeout.
    pub fn new(port_name: impl Into<String>) -> Self {
        Self::with_options(
            port_name,
            DEFAULT_BAUD,
            Duration::from_millis(DEFAULT_TIMEOUT_MS),
        )
    }

    /// Returns the configured serial port path.
    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    /// Returns the configured baud rate.
    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    /// Returns the configured timeout duration.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

/// Formats and writes a status payload as a newline-delimited JSON line to a writer.
pub fn write_payload_line<W: Write>(
    writer: &mut W,
    payload: &StatusPayload,
) -> Result<(), SinkError> {
    let mut json = serde_json::to_string(payload)?;
    json.push('\n');
    writer.write_all(json.as_bytes())?;
    writer.flush()?;
    Ok(())
}

/// Calls `op` up to `attempts` times, sleeping `delay` between failed attempts, and returns
/// the first success or the last error.
pub fn retry<T, E>(
    attempts: u32,
    delay: Duration,
    mut op: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    let mut result = op();
    for _ in 1..attempts {
        if result.is_ok() {
            break;
        }
        thread::sleep(delay);
        result = op();
    }
    result
}

impl StatusSink for SerialSink {
    fn send(&mut self, payload: &StatusPayload) -> Result<(), SinkError> {
        let builder = serialport::new(&self.port_name, self.baud_rate)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(self.timeout);
        let mut port = retry(OPEN_ATTEMPTS, OPEN_RETRY_DELAY, || builder.clone().open())?;

        // Disable DTR and RTS to prevent microcontroller reboot
        let _ = port.write_data_terminal_ready(false);
        let _ = port.write_request_to_send(false);

        write_payload_line(&mut port, payload)
    }
}

#[cfg(test)]
#[path = "serial_tests.rs"]
mod tests;
