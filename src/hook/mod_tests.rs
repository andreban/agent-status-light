use std::error::Error as StdError;

use serde_json::json;

use super::*;
use crate::model::AgentState;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_detect_agent() -> Result<()> {
    let agy = json!({ "conversationId": "c1", "invocationNum": 1 });
    assert_eq!(detect_agent(&agy)?, AgentKind::Antigravity);

    let claude = json!({ "session_id": "s1", "hook_event_name": "PreToolUse" });
    assert_eq!(detect_agent(&claude)?, AgentKind::Claude);

    let codex = json!({ "session_id": "s1", "beforeSubmitPrompt": {} });
    assert_eq!(detect_agent(&codex)?, AgentKind::Codex);

    let unknown = json!({ "foo": "bar" });
    assert!(matches!(
        detect_agent(&unknown),
        Err(HookError::UnknownAgent)
    ));
    Ok(())
}

#[test]
fn test_parse_hook_auto_detected() -> Result<()> {
    let agy_json = r#"{"conversationId": "c-100", "invocationNum": 2}"#;
    let status = parse_hook(agy_json, None, None)?;
    assert_eq!(status.agent, AgentKind::Antigravity);
    assert_eq!(status.state, AgentState::Thinking);
    assert_eq!(status.session_id, "c-100");

    let claude_json = r#"{"session_id": "cl-200", "hook_event_name": "PreToolUse"}"#;
    let status = parse_hook(claude_json, None, None)?;
    assert_eq!(status.agent, AgentKind::Claude);
    assert_eq!(status.state, AgentState::Executing);
    assert_eq!(status.session_id, "cl-200");

    let codex_json = r#"{"session_id": "cx-300", "event": "stop"}"#;
    let status = parse_hook(codex_json, None, None)?;
    assert_eq!(status.agent, AgentKind::Codex);
    assert_eq!(status.state, AgentState::Done);
    assert_eq!(status.session_id, "cx-300");
    Ok(())
}

#[test]
fn test_parse_hook_overrides() -> Result<()> {
    let payload = r#"{"session_id": "custom-s"}"#;
    let status = parse_hook(payload, Some(AgentKind::Claude), Some("Stop"))?;
    assert_eq!(status.agent, AgentKind::Claude);
    assert_eq!(status.state, AgentState::Done);
    assert_eq!(status.session_id, "custom-s");
    Ok(())
}

#[test]
fn test_parse_hook_empty_and_invalid() {
    assert!(matches!(
        parse_hook("", None, None),
        Err(HookError::EmptyPayload)
    ));
    assert!(matches!(
        parse_hook("   ", None, None),
        Err(HookError::EmptyPayload)
    ));
    assert!(matches!(
        parse_hook("{not-json}", None, None),
        Err(HookError::Json(_))
    ));
}
