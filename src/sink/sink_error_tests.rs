use std::error::Error as StdError;
use std::io;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_sink_error_display() -> Result<()> {
    let conn_err = SinkError::Connection("failed to connect".to_string());
    assert_eq!(
        conn_err.to_string(),
        "Sink connection failed: failed to connect"
    );

    let timeout_err = SinkError::Timeout("timed out after 500ms".to_string());
    assert_eq!(
        timeout_err.to_string(),
        "Sink operation timed out: timed out after 500ms"
    );

    let other_err = SinkError::Other("unexpected failure".to_string());
    assert_eq!(other_err.to_string(), "Sink error: unexpected failure");
    Ok(())
}

#[test]
fn test_sink_error_from_io() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "pipe broken");
    let sink_err: SinkError = io_err.into();
    assert!(matches!(sink_err, SinkError::Io(_)));
    assert!(sink_err.to_string().starts_with("Sink I/O error:"));
    Ok(())
}

#[test]
fn test_sink_error_from_serde_json() -> Result<()> {
    let serde_err = serde_json::from_str::<serde_json::Value>("bad json").unwrap_err();
    let sink_err: SinkError = serde_err.into();
    assert!(matches!(sink_err, SinkError::Serialization(_)));
    assert!(
        sink_err
            .to_string()
            .starts_with("Sink serialization error:")
    );
    Ok(())
}

#[test]
fn test_sink_error_from_serialport() -> Result<()> {
    let serial_err = serialport::Error::new(serialport::ErrorKind::NoDevice, "device not found");
    let sink_err: SinkError = serial_err.into();
    assert!(matches!(sink_err, SinkError::Connection(_)));
    assert_eq!(
        sink_err.to_string(),
        "Sink connection failed: device not found"
    );
    Ok(())
}
