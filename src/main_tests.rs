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
        log: None,
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
fn test_hook_response_only_for_antigravity() {
    let reply = Some(ANTIGRAVITY_PRE_TOOL_USE_RESPONSE);
    let antigravity = |state| StatusPayload::with_default_session(state, AgentKind::Antigravity);
    assert_eq!(hook_response(&antigravity(AgentState::Executing)), reply);
    assert_eq!(hook_response(&antigravity(AgentState::Waiting)), reply);
    let empty = Some(ANTIGRAVITY_EMPTY_RESPONSE);
    assert_eq!(hook_response(&antigravity(AgentState::Thinking)), empty);
    assert_eq!(hook_response(&antigravity(AgentState::Done)), empty);

    for agent in [AgentKind::Claude, AgentKind::Codex, AgentKind::Cli] {
        for state in [AgentState::Executing, AgentState::Waiting, AgentState::Done] {
            let payload = StatusPayload::with_default_session(state, agent);
            assert_eq!(hook_response(&payload), None, "{agent} {state}");
        }
    }
}

#[test]
fn test_exit_code_hook_mode_never_fails() {
    let hook = make_test_cli(SinkType::Serial, None);
    let failure: crate::error::Result<()> = Err(Error::Sink(SinkError::Other("busy".to_string())));
    assert_eq!(exit_code(&hook, &Ok(())), 0);
    assert_eq!(exit_code(&hook, &failure), 0);

    let mut manual = make_test_cli(SinkType::Serial, None);
    manual.command = Some(Command::Set {
        state: AgentState::Done,
    });
    assert_eq!(exit_code(&manual, &Ok(())), 0);
    assert_eq!(exit_code(&manual, &failure), 1);
}
