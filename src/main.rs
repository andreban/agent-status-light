mod cli;
mod error;
mod hook;
mod invocation_log;
mod model;
mod sink;

use std::env;
use std::io::{self, IsTerminal};
use std::process;
use std::time::Duration;

use clap::Parser;

use cli::{Cli, Command};
use error::{Error, Result};
use invocation_log::InvocationLog;
use model::{AgentKind, AgentState, StatusPayload};
use sink::{SerialSink, SinkError, SinkType, StatusSink, StdoutSink};

fn dispatch_sink(cli: &Cli, payload: &StatusPayload) -> Result<()> {
    match cli.sink {
        SinkType::Stdout => {
            let mut sink = StdoutSink::new();
            sink.send(payload)?;
        }
        SinkType::Serial => {
            let port = match cli.port.as_deref() {
                Some(p) if !p.trim().is_empty() => p,
                _ => {
                    return Err(Error::Sink(SinkError::Connection(
                        "Serial port path must be specified via --port or ASL_PORT environment variable"
                            .to_string(),
                    )));
                }
            };
            let mut sink =
                SerialSink::with_options(port, cli.baud, Duration::from_millis(cli.timeout_ms));
            sink.send(payload)?;
        }
        SinkType::Http => {
            return Err(Error::Sink(SinkError::Other(
                "HTTP sink is not yet implemented (scheduled for PR 5)".to_string(),
            )));
        }
    }
    Ok(())
}

/// Reply Google Antigravity expects from a `PreToolUse` hook. It does not skip the user's approval prompt.
const ANTIGRAVITY_PRE_TOOL_USE_RESPONSE: &str = r#"{"decision": "allow"}"#;

/// Reply for Antigravity's other hook events.
const ANTIGRAVITY_EMPTY_RESPONSE: &str = "{}";

/// Returns the stdout reply the agent expects for this hook event, if any. Only Antigravity
/// expects one; Claude Code and Codex parse hook stdout as decisions, so they must get nothing.
fn hook_response(payload: &StatusPayload) -> Option<&'static str> {
    if payload.agent != AgentKind::Antigravity {
        return None;
    }
    // PreToolUse is the only Antigravity event mapped to `executing` or `waiting`.
    Some(match payload.state {
        AgentState::Executing | AgentState::Waiting => ANTIGRAVITY_PRE_TOOL_USE_RESPONSE,
        _ => ANTIGRAVITY_EMPTY_RESPONSE,
    })
}

/// Process exit code. Hook runs always exit 0, because agents treat a failing hook as an error
/// or a blocking decision, and a status light must never affect the agent. Manual commands
/// report failures.
fn exit_code(cli: &Cli, result: &Result<()>) -> i32 {
    match (&cli.command, result) {
        (Some(_), Err(_)) => 1,
        _ => 0,
    }
}

fn run(cli: &Cli, log: &mut InvocationLog) -> Result<()> {
    let payload = match cli.command {
        Some(Command::Set { state }) => StatusPayload::with_default_session(state, AgentKind::Cli),
        None => {
            if io::stdin().is_terminal() && cli.event.is_none() {
                println!(
                    "agent-status-light: no command or hook payload provided (use --help for usage)"
                );
                return Ok(());
            }
            let input = hook::read_stdin()?;
            let parsed = hook::parse_hook(&input, cli.agent, cli.event.as_deref());
            log.input = Some(input);
            parsed?
        }
    };
    log.state = Some(payload.state);

    // Reply before touching the sink, so a busy or missing device can't change what the agent sees.
    if cli.command.is_none()
        && cli.sink != SinkType::Stdout
        && let Some(response) = hook_response(&payload)
    {
        println!("{response}");
    }

    dispatch_sink(cli, &payload)?;
    log.sink_ok = true;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let mut log = InvocationLog::new(env::args().skip(1).collect());
    let result = run(&cli, &mut log);

    if let Some(path) = &cli.log {
        if let Err(err) = &result {
            log.error = Some(err.to_string());
        }
        // Diagnostic logging is best-effort and must never affect the hook outcome.
        let _ = log.append_to(path);
    }

    if let Err(err) = &result {
        eprintln!("Error: {err}");
    }
    process::exit(exit_code(&cli, &result));
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
