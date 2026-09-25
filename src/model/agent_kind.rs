use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub const AGENT_ANTIGRAVITY: &str = "antigravity";
pub const AGENT_CLAUDE: &str = "claude";
pub const AGENT_CODEX: &str = "codex";
pub const AGENT_CLI: &str = "cli";

/// Supported AI agent runtime or caller identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum AgentKind {
    /// Google Antigravity.
    Antigravity,
    /// Anthropic Claude Code.
    Claude,
    /// OpenAI Codex.
    Codex,
    /// Direct CLI invocation or manual override.
    Cli,
}

impl AgentKind {
    /// Returns the canonical string slice representation of the agent kind.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Antigravity => AGENT_ANTIGRAVITY,
            Self::Claude => AGENT_CLAUDE,
            Self::Codex => AGENT_CODEX,
            Self::Cli => AGENT_CLI,
        }
    }
}

impl fmt::Display for AgentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for AgentKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<AgentKind> for &'static str {
    fn from(kind: AgentKind) -> Self {
        kind.as_str()
    }
}

impl FromStr for AgentKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            AGENT_ANTIGRAVITY => Ok(Self::Antigravity),
            AGENT_CLAUDE => Ok(Self::Claude),
            AGENT_CODEX => Ok(Self::Codex),
            AGENT_CLI => Ok(Self::Cli),
            _ => Err(Error::InvalidAgentKind(s.to_string())),
        }
    }
}

impl TryFrom<&str> for AgentKind {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[cfg(test)]
#[path = "agent_kind_tests.rs"]
mod tests;
