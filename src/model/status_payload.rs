use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::agent_kind::AgentKind;
use super::agent_state::AgentState;
use crate::error::{Error, Result};

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
    pub fn new(state: AgentState, agent: AgentKind, session_id: impl Into<String>) -> Self {
        Self {
            state,
            agent,
            session_id: session_id.into(),
        }
    }

    /// Creates a new status payload using the default session ID.
    pub fn with_default_session(state: AgentState, agent: AgentKind) -> Self {
        Self::new(state, agent, DEFAULT_SESSION_ID)
    }
}

impl FromStr for StatusPayload {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(Into::into)
    }
}

impl TryFrom<&str> for StatusPayload {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        s.parse()
    }
}

impl TryFrom<&StatusPayload> for String {
    type Error = Error;

    fn try_from(payload: &StatusPayload) -> Result<Self> {
        serde_json::to_string(payload).map_err(Into::into)
    }
}

impl TryFrom<StatusPayload> for String {
    type Error = Error;

    fn try_from(payload: StatusPayload) -> Result<Self> {
        Self::try_from(&payload)
    }
}

#[cfg(test)]
#[path = "status_payload_tests.rs"]
mod tests;
