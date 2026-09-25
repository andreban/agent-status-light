use std::error::Error as StdError;

use serde_json::json;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_claude_user_prompt_submit_and_post_tool_use() -> Result<()> {
    let payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "UserPromptSubmit"
    });

    let status = parse_claude(&payload, None)?;
    assert_eq!(status.state, AgentState::Thinking);
    assert_eq!(status.agent, AgentKind::Claude);
    assert_eq!(status.session_id, "claude-session-1");

    let payload2 = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "PostToolUse"
    });

    let status2 = parse_claude(&payload2, None)?;
    assert_eq!(status2.state, AgentState::Thinking);
    Ok(())
}

#[test]
fn test_claude_pre_tool_use() -> Result<()> {
    let payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "PreToolUse"
    });

    let status = parse_claude(&payload, None)?;
    assert_eq!(status.state, AgentState::Executing);
    assert_eq!(status.agent, AgentKind::Claude);
    Ok(())
}

#[test]
fn test_claude_notifications() -> Result<()> {
    let perm_payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "Notification",
        "notification_type": "permission_prompt"
    });

    let status = parse_claude(&perm_payload, None)?;
    assert_eq!(status.state, AgentState::Waiting);

    let idle_payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "Notification",
        "notification_type": "idle_prompt"
    });

    let status = parse_claude(&idle_payload, None)?;
    assert_eq!(status.state, AgentState::Done);
    Ok(())
}

#[test]
fn test_claude_stop_and_session_end() -> Result<()> {
    let stop_payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "Stop"
    });

    let status = parse_claude(&stop_payload, None)?;
    assert_eq!(status.state, AgentState::Done);

    let end_payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "SessionEnd"
    });

    let status = parse_claude(&end_payload, None)?;
    assert_eq!(status.state, AgentState::Off);
    Ok(())
}

#[test]
fn test_claude_post_tool_use_failure() -> Result<()> {
    let payload = json!({
        "session_id": "claude-session-1",
        "hook_event_name": "PostToolUseFailure"
    });

    let status = parse_claude(&payload, None)?;
    assert_eq!(status.state, AgentState::Error);
    Ok(())
}

#[test]
fn test_claude_default_session_id() -> Result<()> {
    let payload = json!({
        "hook_event_name": "Stop"
    });

    let status = parse_claude(&payload, None)?;
    assert_eq!(status.session_id, DEFAULT_SESSION_ID);
    Ok(())
}

#[test]
fn test_claude_unknown_event() {
    let payload = json!({
        "hook_event_name": "NonExistentEvent"
    });

    assert!(parse_claude(&payload, None).is_err());
}
