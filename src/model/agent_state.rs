use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub const STATE_THINKING: &str = "thinking";
pub const STATE_EXECUTING: &str = "executing";
pub const STATE_WAITING: &str = "waiting";
pub const STATE_DONE: &str = "done";
pub const STATE_ERROR: &str = "error";
pub const STATE_OFF: &str = "off";

/// High-level lifecycle state of an AI coding agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum AgentState {
    /// Agent is reasoning or preparing a response.
    Thinking,
    /// Agent is executing tools or shell commands.
    Executing,
    /// Agent is waiting for user confirmation or input.
    Waiting,
    /// Agent completed task successfully.
    Done,
    /// Agent encountered an error or failed execution.
    Error,
    /// Agent is idle or session has terminated (LEDs off).
    Off,
}

impl AgentState {
    /// Returns the canonical string slice representation of the state.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Thinking => STATE_THINKING,
            Self::Executing => STATE_EXECUTING,
            Self::Waiting => STATE_WAITING,
            Self::Done => STATE_DONE,
            Self::Error => STATE_ERROR,
            Self::Off => STATE_OFF,
        }
    }
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for AgentState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for AgentState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STATE_THINKING => Ok(Self::Thinking),
            STATE_EXECUTING => Ok(Self::Executing),
            STATE_WAITING => Ok(Self::Waiting),
            STATE_DONE => Ok(Self::Done),
            STATE_ERROR => Ok(Self::Error),
            STATE_OFF => Ok(Self::Off),
            _ => Err(Error::InvalidAgentState(s.to_string())),
        }
    }
}

#[cfg(test)]
#[path = "agent_state_tests.rs"]
mod tests;
