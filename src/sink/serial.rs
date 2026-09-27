use std::thread;
use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, StopBits};

use super::{SinkError, write_line};
use crate::model::StatusPayload;

/// Default serial communication baud rate (115200 8N1).
pub const DEFAULT_BAUD: u32 = 115_200;

/// Number of attempts to open the serial port. Agents run hooks for parallel tool calls
/// concurrently, and the port can only be held by one process at a time.
pub const OPEN_ATTEMPTS: u32 = 4;

/// Delay between serial port open attempts.
pub const OPEN_RETRY_DELAY: Duration = Duration::from_millis(50);

/// Opens `port`, writes one status line and closes the port.
pub fn send(
    port: &str,
    baud: u32,
    timeout: Duration,
    payload: &StatusPayload,
) -> Result<(), SinkError> {
    let builder = serialport::new(port, baud)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::None)
        .timeout(timeout);
    let mut port = retry(OPEN_ATTEMPTS, OPEN_RETRY_DELAY, || builder.clone().open())?;

    // Keep DTR and RTS low: boards with a USB-UART bridge reset when they toggle.
    let _ = port.write_data_terminal_ready(false);
    let _ = port.write_request_to_send(false);

    write_line(&mut port, payload)
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

#[cfg(test)]
#[path = "serial_tests.rs"]
mod tests;
