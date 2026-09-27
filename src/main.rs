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

fn hook_response_json(payload: &StatusPayload) -> &'static str {
    if payload.state == AgentState::Executing || payload.state == AgentState::Waiting {
        "{\"decision\": \"allow\"}"
    } else {
        "{}"
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

    dispatch_sink(cli, &payload)?;
    log.sink_ok = true;

    // If executed as a hook with a non-stdout sink, output valid JSON to satisfy
    // agent hook runner contracts (e.g. Google Antigravity PreToolUse requires {"decision": "allow"}).
    if cli.command.is_none() && cli.sink != SinkType::Stdout {
        println!("{}", hook_response_json(&payload));
    }
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

    if let Err(err) = result {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
