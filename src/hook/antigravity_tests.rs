use std::error::Error as StdError;

use serde_json::json;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_antigravity_pre_invocation() -> Result<()> {
    let payload = json!({
        "conversationId": "test-conv-123",
        "invocationNum": 1
    });

    let status = parse_antigravity(&payload, None)?;
    assert_eq!(status.state, AgentState::Thinking);
    assert_eq!(status.agent, AgentKind::Antigravity);
    assert_eq!(status.session_id, "test-conv-123");
    Ok(())
}

#[test]
fn test_antigravity_pre_tool_use() -> Result<()> {
    let payload = json!({
        "conversationId": "test-conv-123",
        "toolCall": {
            "name": "run_command",
            "args": { "CommandLine": "cargo test" }
        }
    });

    let status = parse_antigravity(&payload, None)?;
    assert_eq!(status.state, AgentState::Executing);
    assert_eq!(status.agent, AgentKind::Antigravity);
    assert_eq!(status.session_id, "test-conv-123");
    Ok(())
}

#[test]
fn test_antigravity_pre_tool_use_ask_question() -> Result<()> {
    let payload = json!({
        "conversationId": "test-conv-123",
        "toolCall": {
            "name": "ask_question",
            "args": {}
        }
    });

    let status = parse_antigravity(&payload, None)?;
    assert_eq!(status.state, AgentState::Waiting);
    assert_eq!(status.agent, AgentKind::Antigravity);
    assert_eq!(status.session_id, "test-conv-123");
    Ok(())
}

#[test]
fn test_antigravity_post_tool_use() -> Result<()> {
    let payload = json!({
        "conversationId": "test-conv-123",
        "stepIdx": 5
    });

    let status = parse_antigravity(&payload, None)?;
    assert_eq!(status.state, AgentState::Thinking);
    assert_eq!(status.agent, AgentKind::Antigravity);

    let error_payload = json!({
        "conversationId": "test-conv-123",
        "stepIdx": 5,
        "error": "command failed"
    });

    let err_status = parse_antigravity(&error_payload, None)?;
    assert_eq!(err_status.state, AgentState::Error);
    Ok(())
}

#[test]
fn test_antigravity_post_invocation_override() -> Result<()> {
    let payload = json!({
        "conversationId": "test-conv-123",
    });

    let status = parse_antigravity(&payload, Some("PostInvocation"))?;
    assert_eq!(status.state, AgentState::Done);
    assert_eq!(status.agent, AgentKind::Antigravity);
    Ok(())
}

#[test]
fn test_antigravity_stop_success_and_error() -> Result<()> {
    let success_payload = json!({
        "conversationId": "test-conv-123",
        "terminationReason": "model_stop"
    });

    let status = parse_antigravity(&success_payload, None)?;
    assert_eq!(status.state, AgentState::Done);

    let error_payload = json!({
        "conversationId": "test-conv-123",
        "terminationReason": "error",
        "error": "command failed"
    });

    let status = parse_antigravity(&error_payload, None)?;
    assert_eq!(status.state, AgentState::Error);
    Ok(())
}

#[test]
fn test_antigravity_default_session_id() -> Result<()> {
    let payload = json!({
        "event": "PreInvocation"
    });

    let status = parse_antigravity(&payload, None)?;
    assert_eq!(status.session_id, DEFAULT_SESSION_ID);
    Ok(())
}

#[test]
fn test_antigravity_unknown_event() {
    let payload = json!({
        "conversationId": "test-conv-123",
        "event": "UnknownEvent"
    });

    assert!(parse_antigravity(&payload, None).is_err());
}
