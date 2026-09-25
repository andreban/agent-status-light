mod antigravity;
mod claude;
mod codex;
mod hook_error;

use std::io::{self, Read};

use serde_json::Value;

pub use antigravity::parse_antigravity;
pub use claude::parse_claude;
pub use codex::parse_codex;
pub use hook_error::HookError;

use crate::model::{AgentKind, StatusPayload};

/// Detects the originating agent kind from a hook payload value.
pub fn detect_agent(value: &Value) -> Result<AgentKind, HookError> {
    if value.get("conversationId").is_some() {
        return Ok(AgentKind::Antigravity);
    }

    if value.get("hook_event_name").is_some() || value.get("notification_type").is_some() {
        return Ok(AgentKind::Claude);
    }

    if value.get("beforeSubmitPrompt").is_some()
        || value.get("tool_execute_before").is_some()
        || value
            .get("event")
            .and_then(|v| v.as_str())
            .is_some_and(|e| {
                matches!(
                    e,
                    "beforeSubmitPrompt"
                        | "tool_execute_before"
                        | "permission_asked"
                        | "turn_end"
                        | "stop"
                )
            })
    {
        return Ok(AgentKind::Codex);
    }

    if value
        .get("event")
        .and_then(|v| v.as_str())
        .is_some_and(|e| {
            matches!(
                e,
                "UserPromptSubmit" | "PreToolUse" | "PostToolUse" | "SessionEnd"
            )
        })
    {
        return Ok(AgentKind::Claude);
    }

    Err(HookError::UnknownAgent)
}

/// Routes and parses raw hook JSON content into a normalized StatusPayload.
pub fn parse_hook(
    json: &str,
    agent_override: Option<AgentKind>,
    event_override: Option<&str>,
) -> Result<StatusPayload, HookError> {
    let trimmed = json.trim();
    if trimmed.is_empty() {
        return Err(HookError::EmptyPayload);
    }

    let value: Value = serde_json::from_str(trimmed)?;

    let agent = match agent_override {
        Some(a) => a,
        None => detect_agent(&value)?,
    };

    match agent {
        AgentKind::Antigravity => parse_antigravity(&value, event_override),
        AgentKind::Claude => parse_claude(&value, event_override),
        AgentKind::Codex => parse_codex(&value, event_override),
        AgentKind::Cli => Err(HookError::UnknownAgent),
    }
}

/// Reads hook JSON payload from standard input.
pub fn read_stdin() -> Result<String, HookError> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
