use serde::Serialize;

/// Session ID used when the hook payload has none.
pub const DEFAULT_SESSION_ID: &str = "default";

/// Agent that sent a status update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum AgentKind {
    /// Google Antigravity.
    Antigravity,
    /// Anthropic Claude Code.
    Claude,
    /// OpenAI Codex.
    Codex,
    /// Manual `set` command.
    Cli,
}

/// State shown by the status light.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum AgentState {
    /// Agent is busy: reasoning or running tools.
    Working,
    /// Agent needs the user: a permission prompt or a question.
    Waiting,
    /// Turn finished; it's the user's turn.
    Done,
    /// Session ended (light off).
    Off,
}

/// Status update sent to the light as one JSON line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusPayload {
    pub state: AgentState,
    pub agent: AgentKind,
    pub session_id: String,
}

impl StatusPayload {
    pub fn new(state: AgentState, agent: AgentKind, session_id: impl Into<String>) -> Self {
        Self {
            state,
            agent,
            session_id: session_id.into(),
        }
    }

    /// Creates a status update with [`DEFAULT_SESSION_ID`].
    pub fn with_default_session(state: AgentState, agent: AgentKind) -> Self {
        Self::new(state, agent, DEFAULT_SESSION_ID)
    }
}

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;
