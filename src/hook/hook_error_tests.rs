use std::error::Error as StdError;
use std::io;

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_hook_error_display() -> Result<()> {
    let empty_err = HookError::EmptyPayload;
    assert_eq!(empty_err.to_string(), "Hook payload is empty");

    assert_eq!(
        HookError::MissingAgent.to_string(),
        "--agent is required for hook events"
    );
    assert_eq!(
        HookError::UnknownAgent.to_string(),
        "Agent does not send hook events"
    );

    let event_err = HookError::UnknownEvent("custom_event".to_string());
    assert_eq!(event_err.to_string(), "Unknown hook event: custom_event");

    let field_err = HookError::MissingField("conversationId".to_string());
    assert_eq!(
        field_err.to_string(),
        "Missing required field in hook payload: conversationId"
    );
    Ok(())
}

#[test]
fn test_hook_error_from_io() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::UnexpectedEof, "pipe closed");
    let hook_err: HookError = io_err.into();
    assert!(matches!(hook_err, HookError::Io(_)));
    assert!(
        hook_err
            .to_string()
            .starts_with("I/O error reading hook input:")
    );
    Ok(())
}

#[test]
fn test_hook_error_from_json() -> Result<()> {
    let json_err = serde_json::from_str::<serde_json::Value>("not valid json").unwrap_err();
    let hook_err: HookError = json_err.into();
    assert!(matches!(hook_err, HookError::Json(_)));
    assert!(
        hook_err
            .to_string()
            .starts_with("JSON error parsing hook payload:")
    );
    Ok(())
}
