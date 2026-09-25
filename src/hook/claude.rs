use serde_json::Value;

use super::hook_error::HookError;
use crate::model::{AgentKind, AgentState, DEFAULT_SESSION_ID, StatusPayload};

pub const EVENT_USER_PROMPT_SUBMIT: &str = "UserPromptSubmit";
pub const EVENT_PRE_TOOL_USE: &str = "PreToolUse";
pub const EVENT_POST_TOOL_USE: &str = "PostToolUse";
pub const EVENT_POST_TOOL_USE_FAILURE: &str = "PostToolUseFailure";
pub const EVENT_NOTIFICATION: &str = "Notification";
pub const EVENT_STOP: &str = "Stop";
pub const EVENT_SESSION_END: &str = "SessionEnd";

pub const NOTIFICATION_PERMISSION_PROMPT: &str = "permission_prompt";
pub const NOTIFICATION_IDLE_PROMPT: &str = "idle_prompt";

/// Parses an Anthropic Claude Code hook payload into a normalized StatusPayload.
pub fn parse_claude(
    value: &Value,
    event_override: Option<&str>,
) -> Result<StatusPayload, HookError> {
    let session_id = value
        .get("session_id")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_SESSION_ID);

    let event = event_override
        .or_else(|| value.get("hook_event_name").and_then(|v| v.as_str()))
        .or_else(|| value.get("event").and_then(|v| v.as_str()));

    let notification_type = value.get("notification_type").and_then(|v| v.as_str());

    let state = match event {
        Some(EVENT_USER_PROMPT_SUBMIT) | Some(EVENT_POST_TOOL_USE) => AgentState::Thinking,
        Some(EVENT_PRE_TOOL_USE) => AgentState::Executing,
        Some(EVENT_NOTIFICATION) => match notification_type {
            Some(NOTIFICATION_PERMISSION_PROMPT) => AgentState::Waiting,
            Some(NOTIFICATION_IDLE_PROMPT) => AgentState::Done,
            _ => AgentState::Waiting,
        },
        Some(EVENT_STOP) => AgentState::Done,
        Some(EVENT_SESSION_END) => AgentState::Off,
        Some(EVENT_POST_TOOL_USE_FAILURE) => AgentState::Error,
        Some(unknown) => return Err(HookError::UnknownEvent(unknown.to_string())),
        None => {
            return Err(HookError::UnknownEvent(
                "No hook event specified in Claude payload".to_string(),
            ));
        }
    };

    Ok(StatusPayload::new(state, AgentKind::Claude, session_id))
}

#[cfg(test)]
#[path = "claude_tests.rs"]
mod tests;
