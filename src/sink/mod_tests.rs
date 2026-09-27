use std::error::Error as StdError;

use super::*;
use crate::model::{AgentKind, AgentState};

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_write_line() -> Result<()> {
    let payload = StatusPayload::new(AgentState::Waiting, AgentKind::Claude, "s-1");
    let mut buffer = Vec::new();
    write_line(&mut buffer, &payload)?;
    assert_eq!(
        String::from_utf8(buffer)?,
        "{\"state\":\"waiting\",\"agent\":\"claude\",\"session_id\":\"s-1\"}\n"
    );
    Ok(())
}
