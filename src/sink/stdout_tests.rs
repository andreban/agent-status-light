use std::error::Error as StdError;

use super::*;
use crate::model::{AgentKind, AgentState, StatusPayload};

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_stdout_sink_send() -> Result<()> {
    let mut buffer = Vec::new();
    let mut sink = StdoutSink::with_writer(&mut buffer);

    let payload = StatusPayload::new(AgentState::Thinking, AgentKind::Antigravity, "session-test");
    sink.send(&payload)?;

    let output = String::from_utf8(buffer)?;
    assert_eq!(
        output,
        "{\"state\":\"thinking\",\"agent\":\"antigravity\",\"session_id\":\"session-test\"}\n"
    );
    Ok(())
}

#[test]
fn test_stdout_sink_default_and_new() {
    let _sink = StdoutSink::new();
    let _default_sink = StdoutSink::default();
}
