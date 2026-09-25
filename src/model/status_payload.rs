use serde::{Deserialize, Serialize};

use super::agent_kind::AgentKind;
use super::agent_state::AgentState;
use crate::error::Result;

/// Default session identifier used when no conversation or session ID is provided.
pub const DEFAULT_SESSION_ID: &str = "default";

/// Wire payload transmitting agent status to physical and network status sinks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusPayload {
    /// Current high-level agent lifecycle state.
    pub state: AgentState,
    /// Originating agent runtime or invocation source.
    pub agent: AgentKind,
    /// Unique session or conversation identifier.
    pub session_id: String,
}

impl StatusPayload {
    /// Creates a new status payload with the specified state, agent, and session ID.
    #[must_use]
    pub fn new(state: AgentState, agent: AgentKind, session_id: impl Into<String>) -> Self {
        Self {
            state,
            agent,
            session_id: session_id.into(),
        }
    }

    /// Creates a new status payload using the default session ID.
    #[must_use]
    pub fn with_default_session(state: AgentState, agent: AgentKind) -> Self {
        Self::new(state, agent, DEFAULT_SESSION_ID)
    }

    /// Serializes payload to a single-line JSON string without a trailing newline.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    /// Serializes payload to a newline-delimited JSON line (`{...}\n`).
    pub fn to_json_line(&self) -> Result<String> {
        let mut json = self.to_json()?;
        json.push('\n');
        Ok(json)
    }

    /// Parses a status payload from a JSON string.
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

#[cfg(test)]
#[path = "status_payload_tests.rs"]
mod tests;
