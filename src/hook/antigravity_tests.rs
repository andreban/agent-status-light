use std::error::Error as StdError;

use serde_json::json;

use super::*;
use crate::hook::test_fixtures::{payload, sequence, shown};

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

use AgentState::{Done, Waiting, Working};

/// Maps a captured sequence and returns what the light shows.
fn shown_for(file: &str) -> Result<Vec<AgentState>> {
    let mut states = Vec::new();
    for (event, value) in sequence("antigravity", file)? {
        if let Some(status) = parse(&value, Some(&event))? {
            states.push(status.state);
        }
    }
    Ok(shown(states))
}

#[test]
fn test_captured_turns() -> Result<()> {
    let cases = [
        // PostInvocation fires after every model call and must not flash `done` mid-turn.
        ("turn_multi_tool.jsonl", vec![Working, Done]),
        (
            "turn_ask_question.jsonl",
            vec![Working, Waiting, Working, Done],
        ),
        ("turn_parallel_edit_failing_test.jsonl", vec![Working, Done]),
        // A tool rejected before running gets no PostToolUse.
        ("turn_invalid_tool_call.jsonl", vec![Working, Done]),
        // No event fires for approval prompts or cancelled turns (#24).
        ("turn_approval_and_failed_tool.jsonl", vec![Working]),
        ("turn_cancelled.jsonl", vec![Working]),
    ];
    for (file, expected) in cases {
        assert_eq!(shown_for(file)?, expected, "{file}");
    }
    Ok(())
}

#[test]
fn test_single_payloads() -> Result<()> {
    let cases = [
        ("pre_invocation.json", EVENT_PRE_INVOCATION, Some(Working)),
        (
            "pre_tool_use_run_command.json",
            EVENT_PRE_TOOL_USE,
            Some(Working),
        ),
        (
            "pre_tool_use_ask_question.json",
            EVENT_PRE_TOOL_USE,
            Some(Waiting),
        ),
        (
            "post_tool_use_ask_question.json",
            EVENT_POST_TOOL_USE,
            Some(Working),
        ),
        ("post_invocation.json", EVENT_POST_INVOCATION, None),
        ("stop.json", EVENT_STOP, Some(Done)),
    ];
    for (file, event, expected) in cases {
        let status = parse(&payload("antigravity", file)?, Some(event))?;
        assert_eq!(status.as_ref().map(|s| s.state), expected, "{file}");
        if let Some(status) = status {
            assert_eq!(status.agent, AgentKind::Antigravity);
            assert_ne!(status.session_id, DEFAULT_SESSION_ID, "{file}");
        }
    }
    Ok(())
}

#[test]
fn test_event_is_required() {
    let value = json!({ "conversationId": "c1", "toolCall": { "name": "run_command" } });
    assert!(matches!(
        parse(&value, None),
        Err(HookError::UnknownEvent(_))
    ));
    assert!(matches!(
        parse(&value, Some("Bogus")),
        Err(HookError::UnknownEvent(_))
    ));
}

#[test]
fn test_default_session_id() -> Result<()> {
    let status = parse(&json!({}), Some(EVENT_STOP))?.ok_or("expected a status")?;
    assert_eq!(status.session_id, DEFAULT_SESSION_ID);
    Ok(())
}

#[test]
fn test_response() {
    assert_eq!(response(Some(EVENT_PRE_TOOL_USE)), PRE_TOOL_USE_RESPONSE);
    for event in [Some(EVENT_PRE_INVOCATION), Some(EVENT_STOP), None] {
        assert_eq!(response(event), EMPTY_RESPONSE, "{event:?}");
    }
}
