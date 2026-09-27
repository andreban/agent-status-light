//! Loads the captured hook payloads in `tests/fixtures` for adapter tests.

use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use crate::model::AgentState;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

fn path(agent: &str, file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(agent)
        .join(file)
}

/// Reads a single-payload fixture (`*.json`).
pub fn payload(agent: &str, file: &str) -> Result<Value> {
    Ok(serde_json::from_str(&fs::read_to_string(path(
        agent, file,
    ))?)?)
}

/// Reads a sequence fixture (`*.jsonl`) as `(event, payload)` pairs.
pub fn sequence(agent: &str, file: &str) -> Result<Vec<(String, Value)>> {
    fs::read_to_string(path(agent, file))?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut entry: Value = serde_json::from_str(line)?;
            let event = entry["event"].as_str().ok_or("missing event")?.to_string();
            Ok((event, entry["payload"].take()))
        })
        .collect()
}

/// What the light shows over a sequence: states in order, with repeats collapsed.
pub fn shown(states: impl IntoIterator<Item = AgentState>) -> Vec<AgentState> {
    let mut shown: Vec<AgentState> = Vec::new();
    for state in states {
        if shown.last() != Some(&state) {
            shown.push(state);
        }
    }
    shown
}
