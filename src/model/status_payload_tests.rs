use std::error::Error as StdError;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_status_payload_constructors() -> Result<()> {
    let payload = StatusPayload::new(AgentState::Thinking, AgentKind::Antigravity, "session-123");
    assert_eq!(payload.state, AgentState::Thinking);
    assert_eq!(payload.agent, AgentKind::Antigravity);
    assert_eq!(payload.session_id, "session-123");

    let default_payload =
        StatusPayload::with_default_session(AgentState::Executing, AgentKind::Claude);
    assert_eq!(default_payload.state, AgentState::Executing);
    assert_eq!(default_payload.agent, AgentKind::Claude);
    assert_eq!(default_payload.session_id, DEFAULT_SESSION_ID);
    Ok(())
}

#[test]
fn test_status_payload_serialization_matches_schema() -> Result<()> {
    let payload = StatusPayload::new(
        AgentState::Thinking,
        AgentKind::Antigravity,
        "ec33ebf9-0cba-4100-8142-c61503f6c587",
    );

    let json = String::try_from(&payload)?;
    assert_eq!(
        json,
        r#"{"state":"thinking","agent":"antigravity","session_id":"ec33ebf9-0cba-4100-8142-c61503f6c587"}"#
    );

    let json_owned = String::try_from(payload)?;
    assert_eq!(json_owned, json);
    Ok(())
}

#[test]
fn test_status_payload_deserialization() -> Result<()> {
    let json = r#"{"state":"waiting","agent":"codex","session_id":"test-session"}"#;
    let payload = StatusPayload::try_from(json)?;

    assert_eq!(payload.state, AgentState::Waiting);
    assert_eq!(payload.agent, AgentKind::Codex);
    assert_eq!(payload.session_id, "test-session");

    let parsed: StatusPayload = json.parse()?;
    assert_eq!(parsed, payload);
    Ok(())
}

#[test]
fn test_status_payload_invalid_deserialization() -> Result<()> {
    // Missing session_id
    let missing_field = r#"{"state":"thinking","agent":"antigravity"}"#;
    assert!(StatusPayload::try_from(missing_field).is_err());
    assert!(missing_field.parse::<StatusPayload>().is_err());

    // Invalid state
    let invalid_state = r#"{"state":"dancing","agent":"antigravity","session_id":"s1"}"#;
    assert!(StatusPayload::try_from(invalid_state).is_err());

    // Invalid agent
    let invalid_agent = r#"{"state":"thinking","agent":"unknown","session_id":"s1"}"#;
    assert!(StatusPayload::try_from(invalid_agent).is_err());
    Ok(())
}
