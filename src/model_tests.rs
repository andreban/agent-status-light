use std::error::Error as StdError;

use serde_json::json;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_wire_format() -> Result<()> {
    // The firmware parses this exact shape.
    let payload = StatusPayload::new(AgentState::Working, AgentKind::Antigravity, "s-1");
    assert_eq!(
        serde_json::to_string(&payload)?,
        r#"{"state":"working","agent":"antigravity","session_id":"s-1"}"#
    );
    Ok(())
}

#[test]
fn test_names_on_the_wire() -> Result<()> {
    let states = [
        (AgentState::Working, "working"),
        (AgentState::Waiting, "waiting"),
        (AgentState::Done, "done"),
        (AgentState::Off, "off"),
    ];
    for (state, name) in states {
        assert_eq!(serde_json::to_value(state)?, json!(name));
    }
    let agents = [
        (AgentKind::Antigravity, "antigravity"),
        (AgentKind::Claude, "claude"),
        (AgentKind::Codex, "codex"),
        (AgentKind::Cli, "cli"),
    ];
    for (agent, name) in agents {
        assert_eq!(serde_json::to_value(agent)?, json!(name));
    }
    Ok(())
}

#[test]
fn test_default_session() {
    let payload = StatusPayload::with_default_session(AgentState::Done, AgentKind::Cli);
    assert_eq!(payload.session_id, DEFAULT_SESSION_ID);
}
