use std::error::Error as StdError;
use std::time::Duration;

use super::*;
use crate::model::{AgentKind, AgentState, StatusPayload};

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_serial_sink_new() -> Result<()> {
    let sink = SerialSink::new("/dev/ttyACM0");
    assert_eq!(sink.port_name(), "/dev/ttyACM0");
    assert_eq!(sink.baud_rate(), DEFAULT_BAUD);
    assert_eq!(sink.timeout(), Duration::from_millis(DEFAULT_TIMEOUT_MS));
    Ok(())
}

#[test]
fn test_serial_sink_with_options() -> Result<()> {
    let sink = SerialSink::with_options("COM4", 9600, Duration::from_millis(1500));
    assert_eq!(sink.port_name(), "COM4");
    assert_eq!(sink.baud_rate(), 9600);
    assert_eq!(sink.timeout(), Duration::from_millis(1500));
    Ok(())
}

#[test]
fn test_write_payload_line() -> Result<()> {
    let payload = StatusPayload::new(AgentState::Executing, AgentKind::Antigravity, "session-123");
    let mut buffer = Vec::new();

    write_payload_line(&mut buffer, &payload)?;

    let output = String::from_utf8(buffer)?;
    assert!(output.ends_with('\n'));

    let deserialized: StatusPayload = serde_json::from_str(output.trim())?;
    assert_eq!(deserialized, payload);
    Ok(())
}

#[test]
fn test_write_payload_line_all_states() -> Result<()> {
    let states = [
        AgentState::Thinking,
        AgentState::Executing,
        AgentState::Waiting,
        AgentState::Done,
        AgentState::Error,
        AgentState::Off,
    ];

    for state in states {
        let payload = StatusPayload::with_default_session(state, AgentKind::Claude);
        let mut buffer = Vec::new();
        write_payload_line(&mut buffer, &payload)?;

        let line = String::from_utf8(buffer)?;
        assert!(line.ends_with('\n'));
        let parsed: StatusPayload = serde_json::from_str(line.trim())?;
        assert_eq!(parsed.state, state);
        assert_eq!(parsed.agent, AgentKind::Claude);
    }

    Ok(())
}

#[test]
fn test_serial_sink_send_nonexistent_port() -> Result<()> {
    let mut sink = SerialSink::new("/dev/nonexistent_test_port_12345");
    let payload = StatusPayload::with_default_session(AgentState::Thinking, AgentKind::Cli);

    let result = sink.send(&payload);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, SinkError::Connection(_)));
    Ok(())
}
