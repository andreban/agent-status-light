use std::env;
use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use std::process;

use serde_json::{Value, json};

use super::*;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

fn temp_log_path(name: &str) -> PathBuf {
    env::temp_dir().join(format!("asl-{name}-{}.jsonl", process::id()))
}

#[test]
fn test_new_starts_empty_with_timestamp() {
    let entry = InvocationLog::new(vec!["--event".to_string(), "Stop".to_string()]);
    assert!(entry.timestamp_ms > 0);
    assert_eq!(entry.args, ["--event", "Stop"]);
    assert_eq!(entry.input, None);
    assert_eq!(entry.state, None);
    assert!(!entry.sink_ok);
    assert_eq!(entry.error, None);
}

#[test]
fn test_serialization() -> Result<()> {
    let mut entry = InvocationLog::new(vec!["--agent".to_string(), "claude".to_string()]);
    entry.timestamp_ms = 1_000;
    entry.input = Some(r#"{"hook_event_name":"Stop"}"#.to_string());
    entry.state = Some(AgentState::Done);
    entry.sink_ok = true;

    let value: Value = serde_json::to_value(&entry)?;
    assert_eq!(
        value,
        json!({
            "timestamp_ms": 1_000,
            "args": ["--agent", "claude"],
            "input": "{\"hook_event_name\":\"Stop\"}",
            "state": "done",
            "sink_ok": true,
            "error": null
        })
    );
    Ok(())
}

#[test]
fn test_append_to_writes_one_line_per_entry() -> Result<()> {
    let path = temp_log_path("append");
    let _ = fs::remove_file(&path);

    let mut first = InvocationLog::new(vec![]);
    first.state = Some(AgentState::Thinking);
    first.append_to(&path)?;

    let mut second = InvocationLog::new(vec![]);
    second.error = Some("Hook error: Unknown hook event: Foo".to_string());
    second.append_to(&path)?;

    let contents = fs::read_to_string(&path)?;
    fs::remove_file(&path)?;

    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(
        serde_json::from_str::<Value>(lines[0])?["state"],
        "thinking"
    );
    assert_eq!(
        serde_json::from_str::<Value>(lines[1])?["error"],
        "Hook error: Unknown hook event: Foo"
    );
    Ok(())
}

#[test]
fn test_append_to_missing_directory_fails() {
    let path = temp_log_path("missing-dir")
        .join("nested")
        .join("log.jsonl");
    assert!(InvocationLog::new(vec![]).append_to(&path).is_err());
}
