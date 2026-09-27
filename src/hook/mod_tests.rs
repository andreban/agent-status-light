use std::error::Error as StdError;

use super::*;
use crate::model::AgentState;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_parse_hook_routes_by_agent() -> Result<()> {
    let status = parse_hook(
        r#"{"conversationId":"c-1"}"#,
        AgentKind::Antigravity,
        Some("Stop"),
    )?
    .ok_or("expected a status")?;
    assert_eq!(
        (status.agent, status.state),
        (AgentKind::Antigravity, AgentState::Done)
    );
    assert_eq!(status.session_id, "c-1");

    let claude_style = r#"{"session_id":"s-1","hook_event_name":"PermissionRequest"}"#;
    for agent in [AgentKind::Claude, AgentKind::Codex] {
        let status = parse_hook(claude_style, agent, None)?.ok_or("expected a status")?;
        assert_eq!((status.agent, status.state), (agent, AgentState::Waiting));
    }
    Ok(())
}

#[test]
fn test_parse_hook_ignored_event() -> Result<()> {
    let json = r#"{"session_id":"s-1","hook_event_name":"Notification"}"#;
    assert_eq!(parse_hook(json, AgentKind::Claude, None)?, None);
    Ok(())
}

#[test]
fn test_parse_hook_errors() {
    assert!(matches!(
        parse_hook("  ", AgentKind::Claude, None),
        Err(HookError::EmptyPayload)
    ));
    assert!(matches!(
        parse_hook("{not-json}", AgentKind::Claude, None),
        Err(HookError::Json(_))
    ));
    assert!(matches!(
        parse_hook("{}", AgentKind::Cli, None),
        Err(HookError::UnknownAgent)
    ));
}

#[test]
fn test_response_only_for_antigravity() {
    assert_eq!(
        response(Some(AgentKind::Antigravity), Some("PreToolUse")),
        Some(r#"{"decision": "allow"}"#)
    );
    assert_eq!(
        response(Some(AgentKind::Antigravity), Some("Stop")),
        Some("{}")
    );
    for agent in [
        Some(AgentKind::Claude),
        Some(AgentKind::Codex),
        Some(AgentKind::Cli),
        None,
    ] {
        assert_eq!(response(agent, Some("PreToolUse")), None, "{agent:?}");
    }
}
