use serde_json::Value;

use super::hook_error::HookError;
use crate::model::{AgentKind, AgentState, DEFAULT_SESSION_ID, StatusPayload};

pub const EVENT_USER_PROMPT_SUBMIT: &str = "UserPromptSubmit";
pub const EVENT_PRE_TOOL_USE: &str = "PreToolUse";
pub const EVENT_POST_TOOL_USE: &str = "PostToolUse";
pub const EVENT_POST_TOOL_USE_FAILURE: &str = "PostToolUseFailure";
pub const EVENT_PERMISSION_REQUEST: &str = "PermissionRequest";
pub const EVENT_PERMISSION_DENIED: &str = "PermissionDenied";
pub const EVENT_NOTIFICATION: &str = "Notification";
pub const EVENT_STOP: &str = "Stop";
pub const EVENT_INTERRUPT: &str = "Interrupt";
pub const EVENT_SESSION_START: &str = "SessionStart";
pub const EVENT_SESSION_END: &str = "SessionEnd";
pub const EVENT_SUBAGENT_START: &str = "SubagentStart";
pub const EVENT_SUBAGENT_STOP: &str = "SubagentStop";
pub const EVENT_PRE_COMPACT: &str = "PreCompact";
pub const EVENT_POST_COMPACT: &str = "PostCompact";

/// Parses a Claude Code or Codex hook payload; both agents use the same schema, and name the
/// event in `hook_event_name`. Returns `None` for events that don't change the light.
pub fn parse(
    value: &Value,
    agent: AgentKind,
    event_override: Option<&str>,
) -> Result<Option<StatusPayload>, HookError> {
    let event = event_override.or_else(|| value.get("hook_event_name").and_then(Value::as_str));
    let state = match event {
        // A failed command doesn't stop the agent, so it stays `working`.
        Some(
            EVENT_USER_PROMPT_SUBMIT
            | EVENT_PRE_TOOL_USE
            | EVENT_POST_TOOL_USE
            | EVENT_POST_TOOL_USE_FAILURE,
        ) => AgentState::Working,
        // Fires when a permission prompt or a question is shown.
        Some(EVENT_PERMISSION_REQUEST) => AgentState::Waiting,
        // Codex fires `Interrupt` when the user denies a prompt or presses Esc.
        Some(EVENT_STOP | EVENT_INTERRUPT) => AgentState::Done,
        Some(EVENT_SESSION_END) => AgentState::Off,
        Some(
            EVENT_NOTIFICATION
            | EVENT_PERMISSION_DENIED
            | EVENT_SESSION_START
            | EVENT_SUBAGENT_START
            | EVENT_SUBAGENT_STOP
            | EVENT_PRE_COMPACT
            | EVENT_POST_COMPACT,
        ) => return Ok(None),
        Some(unknown) => return Err(HookError::UnknownEvent(unknown.to_string())),
        None => {
            return Err(HookError::UnknownEvent(
                "no hook_event_name in payload".to_string(),
            ));
        }
    };
    let session_id = value
        .get("session_id")
        .and_then(Value::as_str)
        .unwrap_or(DEFAULT_SESSION_ID);
    Ok(Some(StatusPayload::new(state, agent, session_id)))
}

#[cfg(test)]
#[path = "claude_tests.rs"]
mod tests;
