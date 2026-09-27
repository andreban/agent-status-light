use std::error::Error as StdError;

use serde_json::json;

use super::*;
use crate::hook::test_fixtures::{payload, sequence, shown};

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

use AgentState::{Done, Off, Waiting, Working};

/// Maps a captured sequence and returns what the light shows.
fn shown_for(agent: AgentKind, dir: &str, file: &str) -> Result<Vec<AgentState>> {
    let mut states = Vec::new();
    for (_, value) in sequence(dir, file)? {
        if let Some(status) = parse(&value, agent, None)? {
            states.push(status.state);
        }
    }
    Ok(shown(states))
}

#[test]
fn test_captured_claude_turns() -> Result<()> {
    let cases = [
        // Approvals and AskUserQuestion both show `waiting` until answered; the failing Read
        // at the end stays `working`.
        (
            "permission_prompt_and_question.jsonl",
            vec![
                Done, Working, Waiting, Working, Waiting, Working, Waiting, Working,
            ],
        ),
        ("parallel_tool_calls.jsonl", vec![Working]),
        // Denials and interrupts fire nothing; the next prompt resumes `working` (#24).
        (
            "denied_permission_prompt.jsonl",
            vec![Working, Waiting, Working],
        ),
        (
            "interrupted_at_permission_prompt.jsonl",
            vec![Working, Waiting, Working],
        ),
        ("interrupted_running_tool.jsonl", vec![Working]),
    ];
    for (file, expected) in cases {
        assert_eq!(
            shown_for(AgentKind::Claude, "claude", file)?,
            expected,
            "{file}"
        );
    }
    Ok(())
}

#[test]
fn test_captured_codex_turns() -> Result<()> {
    let cases = [
        // Codex fires Interrupt when the user denies a prompt or presses Esc.
        (
            "turn_approvals_and_denial.jsonl",
            vec![
                Working, Waiting, Working, Waiting, Working, Waiting, Working, Waiting, Done,
            ],
        ),
        (
            "turn_failing_command_and_interrupt.jsonl",
            vec![Working, Waiting, Working, Waiting, Done],
        ),
        // Subagent start/stop don't change the light.
        ("turn_subagent.jsonl", vec![Working, Done]),
    ];
    for (file, expected) in cases {
        assert_eq!(
            shown_for(AgentKind::Codex, "codex", file)?,
            expected,
            "{file}"
        );
    }
    Ok(())
}

#[test]
fn test_single_payloads() -> Result<()> {
    let cases = [
        (
            AgentKind::Claude,
            "claude",
            "user_prompt_submit.json",
            Some(Working),
        ),
        (
            AgentKind::Claude,
            "claude",
            "post_tool_use_failure_bash.json",
            Some(Working),
        ),
        (
            AgentKind::Claude,
            "claude",
            "permission_request_ask_user_question.json",
            Some(Waiting),
        ),
        (
            AgentKind::Claude,
            "claude",
            "notification_permission_prompt.json",
            None,
        ),
        (
            AgentKind::Claude,
            "claude",
            "notification_idle_prompt.json",
            None,
        ),
        (AgentKind::Claude, "claude", "stop.json", Some(Done)),
        (
            AgentKind::Claude,
            "claude",
            "session_end_prompt_input_exit.json",
            Some(Off),
        ),
        (AgentKind::Codex, "codex", "session_start.json", None),
        (
            AgentKind::Codex,
            "codex",
            "permission_request_apply_patch.json",
            Some(Waiting),
        ),
        (AgentKind::Codex, "codex", "interrupt.json", Some(Done)),
        (AgentKind::Codex, "codex", "subagent_start.json", None),
        (AgentKind::Codex, "codex", "session_end.json", Some(Off)),
    ];
    for (agent, dir, file, expected) in cases {
        let status = parse(&payload(dir, file)?, agent, None)?;
        assert_eq!(status.as_ref().map(|s| s.state), expected, "{file}");
        if let Some(status) = status {
            assert_eq!(status.agent, agent, "{file}");
            assert_ne!(status.session_id, DEFAULT_SESSION_ID, "{file}");
        }
    }
    Ok(())
}

#[test]
fn test_event_override_and_errors() -> Result<()> {
    let value = json!({ "session_id": "s1" });
    let status = parse(&value, AgentKind::Claude, Some(EVENT_STOP))?.ok_or("expected a status")?;
    assert_eq!(status.state, Done);
    assert!(matches!(
        parse(&value, AgentKind::Claude, None),
        Err(HookError::UnknownEvent(_))
    ));
    assert!(matches!(
        parse(
            &json!({ "hook_event_name": "Bogus" }),
            AgentKind::Codex,
            None
        ),
        Err(HookError::UnknownEvent(_))
    ));
    Ok(())
}
