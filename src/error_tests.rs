use std::error::Error as StdError;
use std::io;

use super::*;
use crate::hook::HookError;
use crate::sink::SinkError;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_error_display() -> Result<()> {
    let state_err = Error::InvalidAgentState("unknown_state".to_string());
    assert_eq!(state_err.to_string(), "Invalid agent state: unknown_state");

    let kind_err = Error::InvalidAgentKind("unknown_kind".to_string());
    assert_eq!(kind_err.to_string(), "Invalid agent kind: unknown_kind");
    Ok(())
}

#[test]
fn test_error_from_serde_json() -> Result<()> {
    let serde_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let err: Error = serde_err.into();
    assert!(matches!(err, Error::Serialization(_)));
    assert!(err.to_string().starts_with("Serialization error:"));
    Ok(())
}

#[test]
fn test_error_from_io() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: Error = io_err.into();
    assert!(matches!(err, Error::Io(_)));
    assert!(err.to_string().starts_with("I/O error:"));
    Ok(())
}

#[test]
fn test_error_from_sink_error() -> Result<()> {
    let sink_err = SinkError::Timeout("connection timed out".to_string());
    let err: Error = sink_err.into();
    assert!(matches!(err, Error::Sink(_)));
    assert!(err.to_string().starts_with("Sink error:"));
    Ok(())
}

#[test]
fn test_error_from_hook_error() -> Result<()> {
    let hook_err = HookError::EmptyPayload;
    let err: Error = hook_err.into();
    assert!(matches!(err, Error::Hook(_)));
    assert_eq!(err.to_string(), "Hook error: Hook payload is empty");
    Ok(())
}
