use std::error::Error as StdError;

use serde_json::json;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_codex_before_submit_prompt() -> Result<()> {
    let payload = json!({
        "session_id": "codex-session-123",
        "event": "beforeSubmitPrompt"
    });

    let status = parse_codex(&payload, None)?;
    assert_eq!(status.state, AgentState::Thinking);
    assert_eq!(status.agent, AgentKind::Codex);
    assert_eq!(status.session_id, "codex-session-123");
    Ok(())
}

#[test]
fn test_codex_tool_execute_before() -> Result<()> {
    let payload = json!({
        "sessionId": "codex-session-123",
        "event": "tool_execute_before"
    });

    let status = parse_codex(&payload, None)?;
    assert_eq!(status.state, AgentState::Executing);
    assert_eq!(status.agent, AgentKind::Codex);
    Ok(())
}

#[test]
fn test_codex_permission_asked() -> Result<()> {
    let payload = json!({
        "session_id": "codex-session-123",
        "event": "permission_asked"
    });

    let status = parse_codex(&payload, None)?;
    assert_eq!(status.state, AgentState::Waiting);
    Ok(())
}

#[test]
fn test_codex_stop_and_turn_end() -> Result<()> {
    let stop_payload = json!({
        "session_id": "codex-session-123",
        "event": "stop"
    });

    let status = parse_codex(&stop_payload, None)?;
    assert_eq!(status.state, AgentState::Done);

    let turn_end_payload = json!({
        "session_id": "codex-session-123",
        "event": "turn_end"
    });

    let status2 = parse_codex(&turn_end_payload, None)?;
    assert_eq!(status2.state, AgentState::Done);
    Ok(())
}

#[test]
fn test_codex_error() -> Result<()> {
    let payload = json!({
        "session_id": "codex-session-123",
        "event": "error"
    });

    let status = parse_codex(&payload, None)?;
    assert_eq!(status.state, AgentState::Error);
    Ok(())
}

#[test]
fn test_codex_default_session_id() -> Result<()> {
    let payload = json!({
        "event": "stop"
    });

    let status = parse_codex(&payload, None)?;
    assert_eq!(status.session_id, DEFAULT_SESSION_ID);
    Ok(())
}

#[test]
fn test_codex_unknown_event() {
    let payload = json!({
        "event": "unknown_codex_event"
    });

    assert!(parse_codex(&payload, None).is_err());
}
