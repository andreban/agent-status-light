#![deny(unsafe_code)]

#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod model;

use model::{AgentKind, AgentState, StatusPayload};

fn main() {
    let payload = StatusPayload::with_default_session(AgentState::Off, AgentKind::Cli);
    let _ = payload;
    println!("agent-status-light");
}
