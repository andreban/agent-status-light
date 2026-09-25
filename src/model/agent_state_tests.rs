use std::error::Error as StdError;
use std::str::FromStr;

use super::*;
use crate::error::Error;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_agent_state_as_str_and_display() -> Result<()> {
    assert_eq!(AgentState::Thinking.as_str(), "thinking");
    assert_eq!(AgentState::Executing.as_str(), "executing");
    assert_eq!(AgentState::Waiting.as_str(), "waiting");
    assert_eq!(AgentState::Done.as_str(), "done");
    assert_eq!(AgentState::Error.as_str(), "error");
    assert_eq!(AgentState::Off.as_str(), "off");

    assert_eq!(AgentState::Thinking.to_string(), "thinking");
    assert_eq!(AgentState::Executing.to_string(), "executing");
    assert_eq!(AgentState::Waiting.to_string(), "waiting");
    assert_eq!(AgentState::Done.to_string(), "done");
    assert_eq!(AgentState::Error.to_string(), "error");
    assert_eq!(AgentState::Off.to_string(), "off");

    assert_eq!(AgentState::Thinking.as_ref(), "thinking");

    let s: &'static str = AgentState::Thinking.into();
    assert_eq!(s, "thinking");
    Ok(())
}

#[test]
fn test_agent_state_from_str() -> Result<()> {
    assert_eq!(AgentState::from_str("thinking")?, AgentState::Thinking);
    assert_eq!(AgentState::from_str("executing")?, AgentState::Executing);
    assert_eq!(AgentState::from_str("waiting")?, AgentState::Waiting);
    assert_eq!(AgentState::from_str("done")?, AgentState::Done);
    assert_eq!(AgentState::from_str("error")?, AgentState::Error);
    assert_eq!(AgentState::from_str("off")?, AgentState::Off);

    // TryFrom<&str>
    assert_eq!(AgentState::try_from("thinking")?, AgentState::Thinking);
    assert_eq!(AgentState::try_from("executing")?, AgentState::Executing);
    assert_eq!(AgentState::try_from("waiting")?, AgentState::Waiting);
    assert_eq!(AgentState::try_from("done")?, AgentState::Done);
    assert_eq!(AgentState::try_from("error")?, AgentState::Error);
    assert_eq!(AgentState::try_from("off")?, AgentState::Off);

    let err = AgentState::from_str("invalid").unwrap_err();
    assert!(matches!(err, Error::InvalidAgentState(_)));
    assert!(AgentState::try_from("invalid").is_err());
    Ok(())
}

#[test]
fn test_agent_state_serde_serialization() -> Result<()> {
    assert_eq!(
        serde_json::to_string(&AgentState::Thinking)?,
        "\"thinking\""
    );
    assert_eq!(
        serde_json::to_string(&AgentState::Executing)?,
        "\"executing\""
    );
    assert_eq!(serde_json::to_string(&AgentState::Waiting)?, "\"waiting\"");
    assert_eq!(serde_json::to_string(&AgentState::Done)?, "\"done\"");
    assert_eq!(serde_json::to_string(&AgentState::Error)?, "\"error\"");
    assert_eq!(serde_json::to_string(&AgentState::Off)?, "\"off\"");
    Ok(())
}

#[test]
fn test_agent_state_serde_deserialization() -> Result<()> {
    assert_eq!(
        serde_json::from_str::<AgentState>("\"thinking\"")?,
        AgentState::Thinking
    );
    assert_eq!(
        serde_json::from_str::<AgentState>("\"executing\"")?,
        AgentState::Executing
    );
    assert_eq!(
        serde_json::from_str::<AgentState>("\"waiting\"")?,
        AgentState::Waiting
    );
    assert_eq!(
        serde_json::from_str::<AgentState>("\"done\"")?,
        AgentState::Done
    );
    assert_eq!(
        serde_json::from_str::<AgentState>("\"error\"")?,
        AgentState::Error
    );
    assert_eq!(
        serde_json::from_str::<AgentState>("\"off\"")?,
        AgentState::Off
    );

    assert!(serde_json::from_str::<AgentState>("\"unknown\"").is_err());
    Ok(())
}
