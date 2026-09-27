mod antigravity;
mod claude;
mod hook_error;

use std::io::{self, Read};

use serde_json::Value;

pub use hook_error::HookError;

use crate::model::{AgentKind, StatusPayload};

/// Parses a raw hook payload from `agent` into a status update. `Ok(None)` means the event
/// doesn't change the light.
pub fn parse_hook(
    json: &str,
    agent: AgentKind,
    event_override: Option<&str>,
) -> Result<Option<StatusPayload>, HookError> {
    let trimmed = json.trim();
    if trimmed.is_empty() {
        return Err(HookError::EmptyPayload);
    }
    let value: Value = serde_json::from_str(trimmed)?;
    match agent {
        AgentKind::Antigravity => antigravity::parse(&value, event_override),
        // Codex hooks use the Claude Code payload schema.
        AgentKind::Claude | AgentKind::Codex => claude::parse(&value, agent, event_override),
        AgentKind::Cli => Err(HookError::UnknownAgent),
    }
}

/// Returns the stdout reply the agent expects from a hook, if any. It depends only on the
/// command-line arguments, so it can be sent before anything else can fail. Only Antigravity
/// expects a reply; Claude Code and Codex parse hook stdout as decisions, so they get nothing.
pub fn response(agent: Option<AgentKind>, event: Option<&str>) -> Option<&'static str> {
    match agent {
        Some(AgentKind::Antigravity) => Some(antigravity::response(event)),
        _ => None,
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

#[cfg(test)]
mod test_fixtures;
