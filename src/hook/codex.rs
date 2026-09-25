use serde_json::Value;

use super::hook_error::HookError;
use crate::model::{AgentKind, AgentState, DEFAULT_SESSION_ID, StatusPayload};

pub const EVENT_BEFORE_SUBMIT_PROMPT: &str = "beforeSubmitPrompt";
pub const EVENT_TOOL_EXECUTE_BEFORE: &str = "tool_execute_before";
pub const EVENT_STOP: &str = "stop";
pub const EVENT_TURN_END: &str = "turn_end";
pub const EVENT_PERMISSION_ASKED: &str = "permission_asked";
pub const EVENT_ERROR: &str = "error";

/// Parses an OpenAI Codex hook payload into a normalized StatusPayload.
pub fn parse_codex(
    value: &Value,
    event_override: Option<&str>,
) -> Result<StatusPayload, HookError> {
    let session_id = value
        .get("session_id")
        .or_else(|| value.get("sessionId"))
        .or_else(|| value.get("conversation_id"))
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_SESSION_ID);

    let event = event_override
        .or_else(|| value.get("event").and_then(|v| v.as_str()))
        .or_else(|| value.get("hook_name").and_then(|v| v.as_str()))
        .or_else(|| value.get("type").and_then(|v| v.as_str()))
        .or_else(|| {
            if value.get("tool_execute_before").is_some() {
                Some(EVENT_TOOL_EXECUTE_BEFORE)
            } else if value.get("beforeSubmitPrompt").is_some() {
                Some(EVENT_BEFORE_SUBMIT_PROMPT)
            } else {
                None
            }
        });

    let state = match event {
        Some(EVENT_BEFORE_SUBMIT_PROMPT) => AgentState::Thinking,
        Some(EVENT_TOOL_EXECUTE_BEFORE) => AgentState::Executing,
        Some(EVENT_PERMISSION_ASKED) => AgentState::Waiting,
        Some(EVENT_STOP) | Some(EVENT_TURN_END) => AgentState::Done,
        Some(EVENT_ERROR) => AgentState::Error,
        Some(unknown) => return Err(HookError::UnknownEvent(unknown.to_string())),
        None => {
            return Err(HookError::UnknownEvent(
                "No hook event specified in Codex payload".to_string(),
            ));
        }
    };

    Ok(StatusPayload::new(state, AgentKind::Codex, session_id))
}

#[cfg(test)]
#[path = "codex_tests.rs"]
mod tests;
