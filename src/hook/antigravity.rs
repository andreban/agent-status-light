use serde_json::Value;

use super::hook_error::HookError;
use crate::model::{AgentKind, AgentState, DEFAULT_SESSION_ID, StatusPayload};

pub const EVENT_PRE_INVOCATION: &str = "PreInvocation";
pub const EVENT_PRE_TOOL_USE: &str = "PreToolUse";
pub const EVENT_POST_TOOL_USE: &str = "PostToolUse";
pub const EVENT_POST_INVOCATION: &str = "PostInvocation";
pub const EVENT_STOP: &str = "Stop";

/// Parses a Google Antigravity lifecycle hook payload into a normalized StatusPayload.
pub fn parse_antigravity(
    value: &Value,
    event_override: Option<&str>,
) -> Result<StatusPayload, HookError> {
    let session_id = value
        .get("conversationId")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_SESSION_ID);

    let event = event_override
        .or_else(|| value.get("event").and_then(|v| v.as_str()))
        .or_else(|| {
            if value.get("toolCall").is_some() {
                Some(EVENT_PRE_TOOL_USE)
            } else if value.get("stepIdx").is_some() {
                Some(EVENT_POST_TOOL_USE)
            } else if value.get("terminationReason").is_some() {
                Some(EVENT_STOP)
            } else if value.get("invocationNum").is_some() {
                Some(EVENT_PRE_INVOCATION)
            } else {
                None
            }
        });

    let state = match event {
        Some(EVENT_PRE_INVOCATION) => AgentState::Thinking,
        Some(EVENT_PRE_TOOL_USE) => {
            let tool_name = value
                .get("toolCall")
                .and_then(|t| t.get("name"))
                .and_then(|n| n.as_str());
            if matches!(tool_name, Some("ask_question") | Some("ask_followup"))
                || tool_name.is_some_and(|n| n.starts_with("ask_"))
            {
                AgentState::Waiting
            } else {
                AgentState::Executing
            }
        }
        Some(EVENT_POST_TOOL_USE) => {
            if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
                if !err.is_empty() {
                    AgentState::Error
                } else {
                    AgentState::Thinking
                }
            } else {
                AgentState::Thinking
            }
        }
        Some(EVENT_POST_INVOCATION) | Some(EVENT_STOP) => {
            if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
                if !err.is_empty() {
                    AgentState::Error
                } else {
                    AgentState::Done
                }
            } else {
                AgentState::Done
            }
        }
        Some(unknown) => return Err(HookError::UnknownEvent(unknown.to_string())),
        None => {
            return Err(HookError::UnknownEvent(
                "No event specified or detected".to_string(),
            ));
        }
    };

    Ok(StatusPayload::new(
        state,
        AgentKind::Antigravity,
        session_id,
    ))
}

#[cfg(test)]
#[path = "antigravity_tests.rs"]
mod tests;
