use std::error::Error as StdError;
use std::str::FromStr;

use super::*;
use crate::error::Error;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

const ALL: [(AgentState, &str); 4] = [
    (AgentState::Working, "working"),
    (AgentState::Waiting, "waiting"),
    (AgentState::Done, "done"),
    (AgentState::Off, "off"),
];

#[test]
fn test_agent_state_as_str_and_display() {
    for (state, name) in ALL {
        assert_eq!(state.as_str(), name);
        assert_eq!(state.to_string(), name);
        assert_eq!(state.as_ref(), name);
        let s: &'static str = state.into();
        assert_eq!(s, name);
    }
}

#[test]
fn test_agent_state_from_str() -> Result<()> {
    for (state, name) in ALL {
        assert_eq!(AgentState::from_str(name)?, state);
        assert_eq!(AgentState::try_from(name)?, state);
    }
    for removed in ["thinking", "executing", "error", "invalid"] {
        let err = AgentState::from_str(removed).unwrap_err();
        assert!(matches!(err, Error::InvalidAgentState(_)), "{removed}");
    }
    Ok(())
}

#[test]
fn test_agent_state_serde() -> Result<()> {
    for (state, name) in ALL {
        let json = format!("\"{name}\"");
        assert_eq!(serde_json::to_string(&state)?, json);
        assert_eq!(serde_json::from_str::<AgentState>(&json)?, state);
    }
    assert!(serde_json::from_str::<AgentState>("\"thinking\"").is_err());
    Ok(())
}
