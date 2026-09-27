use std::time::Duration;

use super::*;
use crate::model::{AgentKind, AgentState, StatusPayload};

#[test]
fn test_send_to_missing_port_is_connection_error() {
    let payload = StatusPayload::with_default_session(AgentState::Working, AgentKind::Cli);
    let result = send(
        "/dev/nonexistent_test_port_12345",
        DEFAULT_BAUD,
        Duration::from_millis(100),
        &payload,
    );
    assert!(matches!(result, Err(SinkError::Connection(_))));
}

#[test]
fn test_retry_returns_first_success() {
    let mut calls = 0;
    let result: core::result::Result<u32, &str> = retry(4, Duration::ZERO, || {
        calls += 1;
        if calls < 3 { Err("busy") } else { Ok(calls) }
    });
    assert_eq!(result, Ok(3));
    assert_eq!(calls, 3);
}

#[test]
fn test_retry_gives_up_after_attempts() {
    let mut calls = 0;
    let result: core::result::Result<(), &str> = retry(4, Duration::ZERO, || {
        calls += 1;
        Err("busy")
    });
    assert_eq!(result, Err("busy"));
    assert_eq!(calls, 4);
}

#[test]
fn test_retry_does_not_retry_success() {
    let mut calls = 0;
    let result: core::result::Result<(), &str> = retry(4, Duration::ZERO, || {
        calls += 1;
        Ok(())
    });
    assert_eq!(result, Ok(()));
    assert_eq!(calls, 1);
}
