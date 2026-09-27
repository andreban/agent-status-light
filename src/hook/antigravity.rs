use serde_json::Value;

use super::HookError;
use crate::model::{AgentKind, AgentState, DEFAULT_SESSION_ID, StatusPayload};

pub const EVENT_PRE_INVOCATION: &str = "PreInvocation";
pub const EVENT_PRE_TOOL_USE: &str = "PreToolUse";
pub const EVENT_POST_TOOL_USE: &str = "PostToolUse";
pub const EVENT_POST_INVOCATION: &str = "PostInvocation";
pub const EVENT_STOP: &str = "Stop";

/// Tool Antigravity uses to ask the user a question.
pub const TOOL_ASK_QUESTION: &str = "ask_question";

/// Reply to a `PreToolUse` hook. It does not skip the user's approval prompt.
pub const PRE_TOOL_USE_RESPONSE: &str = r#"{"decision": "allow"}"#;

/// Reply to Antigravity's other hook events.
pub const EMPTY_RESPONSE: &str = "{}";

/// Parses a Google Antigravity hook payload. Antigravity payloads don't name their event, so it
/// comes from `--event` in the hook config. Returns `None` for events that don't change the light.
pub fn parse(value: &Value, event: Option<&str>) -> Result<Option<StatusPayload>, HookError> {
    let state = match event {
        Some(EVENT_PRE_INVOCATION | EVENT_POST_TOOL_USE) => AgentState::Working,
        Some(EVENT_PRE_TOOL_USE) => {
            let tool = value
                .get("toolCall")
                .and_then(|call| call.get("name"))
                .and_then(Value::as_str);
            if tool == Some(TOOL_ASK_QUESTION) {
                AgentState::Waiting
            } else {
                AgentState::Working
            }
        }
        Some(EVENT_STOP) => AgentState::Done,
        // Fires after every model call, not at the end of the turn.
        Some(EVENT_POST_INVOCATION) => return Ok(None),
        Some(unknown) => return Err(HookError::UnknownEvent(unknown.to_string())),
        None => {
            return Err(HookError::UnknownEvent(
                "no --event given for Antigravity hook".to_string(),
            ));
        }
    };
    let session_id = value
        .get("conversationId")
        .and_then(Value::as_str)
        .unwrap_or(DEFAULT_SESSION_ID);
    Ok(Some(StatusPayload::new(
        state,
        AgentKind::Antigravity,
        session_id,
    )))
}

/// Returns the reply Antigravity expects on stdout for `event`.
pub fn response(event: Option<&str>) -> &'static str {
    if event == Some(EVENT_PRE_TOOL_USE) {
        PRE_TOOL_USE_RESPONSE
    } else {
        EMPTY_RESPONSE
    }
}

#[cfg(test)]
#[path = "antigravity_tests.rs"]
mod tests;
