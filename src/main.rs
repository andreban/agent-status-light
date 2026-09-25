mod cli;
mod error;
mod hook;
mod model;
mod sink;

use std::io::{self, IsTerminal};
use std::process;
use std::time::Duration;

use clap::Parser;

use cli::{Cli, Command};
use error::{Error, Result};
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
    if payload.state == AgentState::Executing {
        "{\"decision\": \"allow\"}"
    } else {
        "{}"
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

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
            hook::parse_hook(&input, cli.agent, cli.event.as_deref())?
        }
    };

    dispatch_sink(&cli, &payload)?;

    // If executed as a hook with a non-stdout sink, output valid JSON to satisfy
    // agent hook runner contracts (e.g. Google Antigravity PreToolUse requires {"decision": "allow"}).
    if cli.command.is_none() && cli.sink != SinkType::Stdout {
        println!("{}", hook_response_json(&payload));
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
