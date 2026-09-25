use std::error::Error as StdError;
use std::str::FromStr;

use super::*;
use crate::error::Error;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_agent_kind_as_str_and_display() -> Result<()> {
    assert_eq!(AgentKind::Antigravity.as_str(), "antigravity");
    assert_eq!(AgentKind::Claude.as_str(), "claude");
    assert_eq!(AgentKind::Codex.as_str(), "codex");
    assert_eq!(AgentKind::Cli.as_str(), "cli");

    assert_eq!(AgentKind::Antigravity.to_string(), "antigravity");
    assert_eq!(AgentKind::Claude.to_string(), "claude");
    assert_eq!(AgentKind::Codex.to_string(), "codex");
    assert_eq!(AgentKind::Cli.to_string(), "cli");

    assert_eq!(AgentKind::Antigravity.as_ref(), "antigravity");
    Ok(())
}

#[test]
fn test_agent_kind_from_str() -> Result<()> {
    assert_eq!(AgentKind::from_str("antigravity")?, AgentKind::Antigravity);
    assert_eq!(AgentKind::from_str("claude")?, AgentKind::Claude);
    assert_eq!(AgentKind::from_str("codex")?, AgentKind::Codex);
    assert_eq!(AgentKind::from_str("cli")?, AgentKind::Cli);

    let err = AgentKind::from_str("unknown").unwrap_err();
    assert!(matches!(err, Error::InvalidAgentKind(_)));
    Ok(())
}

#[test]
fn test_agent_kind_serde_serialization() -> Result<()> {
    assert_eq!(
        serde_json::to_string(&AgentKind::Antigravity)?,
        "\"antigravity\""
    );
    assert_eq!(serde_json::to_string(&AgentKind::Claude)?, "\"claude\"");
    assert_eq!(serde_json::to_string(&AgentKind::Codex)?, "\"codex\"");
    assert_eq!(serde_json::to_string(&AgentKind::Cli)?, "\"cli\"");
    Ok(())
}

#[test]
fn test_agent_kind_serde_deserialization() -> Result<()> {
    assert_eq!(
        serde_json::from_str::<AgentKind>("\"antigravity\"")?,
        AgentKind::Antigravity
    );
    assert_eq!(
        serde_json::from_str::<AgentKind>("\"claude\"")?,
        AgentKind::Claude
    );
    assert_eq!(
        serde_json::from_str::<AgentKind>("\"codex\"")?,
        AgentKind::Codex
    );
    assert_eq!(
        serde_json::from_str::<AgentKind>("\"cli\"")?,
        AgentKind::Cli
    );

    assert!(serde_json::from_str::<AgentKind>("\"invalid\"").is_err());
    Ok(())
}
