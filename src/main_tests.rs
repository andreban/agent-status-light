use std::error::Error as StdError;

use super::*;
use crate::model::AgentState;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

fn make_test_cli(sink: SinkType, port: Option<String>) -> Cli {
    Cli {
        sink,
        port,
        baud: sink::DEFAULT_BAUD,
        url: None,
        timeout_ms: sink::DEFAULT_TIMEOUT_MS,
        agent: None,
        event: None,
        command: None,
    }
}

#[test]
fn test_dispatch_sink_stdout() -> Result<()> {
    let cli = make_test_cli(SinkType::Stdout, None);
    let payload = StatusPayload::with_default_session(AgentState::Thinking, AgentKind::Antigravity);

    assert!(dispatch_sink(&cli, &payload).is_ok());
    Ok(())
}

#[test]
fn test_dispatch_sink_http_unimplemented() -> Result<()> {
    let cli = make_test_cli(SinkType::Http, None);
    let payload = StatusPayload::with_default_session(AgentState::Executing, AgentKind::Claude);

    let result = dispatch_sink(&cli, &payload);
    assert!(result.is_err());
    let err_str = result.unwrap_err().to_string();
    assert!(err_str.contains("HTTP sink is not yet implemented"));
    Ok(())
}

#[test]
fn test_dispatch_sink_serial_missing_port() -> Result<()> {
    let cli_none = make_test_cli(SinkType::Serial, None);
    let payload = StatusPayload::with_default_session(AgentState::Waiting, AgentKind::Codex);

    let result_none = dispatch_sink(&cli_none, &payload);
    assert!(result_none.is_err());
    let err_str = result_none.unwrap_err().to_string();
    assert!(err_str.contains("Serial port path must be specified"));

    let cli_empty = make_test_cli(SinkType::Serial, Some("   ".to_string()));
    let result_empty = dispatch_sink(&cli_empty, &payload);
    assert!(result_empty.is_err());
    let err_empty_str = result_empty.unwrap_err().to_string();
    assert!(err_empty_str.contains("Serial port path must be specified"));
    Ok(())
}

#[test]
fn test_dispatch_sink_serial_invalid_port() -> Result<()> {
    let cli = make_test_cli(
        SinkType::Serial,
        Some("/dev/nonexistent_serial_port_999".to_string()),
    );
    let payload = StatusPayload::with_default_session(AgentState::Done, AgentKind::Codex);

    let result = dispatch_sink(&cli, &payload);
    assert!(result.is_err());
    let err_str = result.unwrap_err().to_string();
    assert!(err_str.contains("Sink connection failed"));
    Ok(())
}

#[test]
fn test_hook_response_json() -> Result<()> {
    let executing =
        StatusPayload::with_default_session(AgentState::Executing, AgentKind::Antigravity);
    assert_eq!(hook_response_json(&executing), "{\"decision\": \"allow\"}");

    let thinking =
        StatusPayload::with_default_session(AgentState::Thinking, AgentKind::Antigravity);
    assert_eq!(hook_response_json(&thinking), "{}");

    let done = StatusPayload::with_default_session(AgentState::Done, AgentKind::Claude);
    assert_eq!(hook_response_json(&done), "{}");

    let error = StatusPayload::with_default_session(AgentState::Error, AgentKind::Codex);
    assert_eq!(hook_response_json(&error), "{}");
    Ok(())
}
