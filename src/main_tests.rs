use std::error::Error as StdError;

use super::*;
use crate::model::AgentState;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

fn make_test_cli(sink: SinkType, port: Option<String>) -> Cli {
    Cli {
        sink,
        port,
        baud: sink::serial::DEFAULT_BAUD,
        url: None,
        timeout_ms: sink::DEFAULT_TIMEOUT_MS,
        agent: None,
        event: None,
        log: None,
        command: None,
    }
}

#[test]
fn test_dispatch_sink_stdout() -> Result<()> {
    let cli = make_test_cli(SinkType::Stdout, None);
    let payload = StatusPayload::with_default_session(AgentState::Working, AgentKind::Antigravity);

    assert!(dispatch_sink(&cli, &payload).is_ok());
    Ok(())
}

#[test]
fn test_dispatch_sink_http_unimplemented() -> Result<()> {
    let cli = make_test_cli(SinkType::Http, None);
    let payload = StatusPayload::with_default_session(AgentState::Working, AgentKind::Claude);

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
fn test_exit_code_hook_mode_never_fails() {
    let hook = make_test_cli(SinkType::Serial, None);
    let failure: crate::error::Result<()> = Err(SinkError::Other("busy".to_string()).into());
    assert_eq!(exit_code(&hook, &Ok(())), 0);
    assert_eq!(exit_code(&hook, &failure), 0);

    let mut manual = make_test_cli(SinkType::Serial, None);
    manual.command = Some(Command::Set {
        state: AgentState::Done,
    });
    assert_eq!(exit_code(&manual, &Ok(())), 0);
    assert_eq!(exit_code(&manual, &failure), 1);
}
