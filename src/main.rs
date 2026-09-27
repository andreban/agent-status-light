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
use error::Result;
use hook::HookError;
use invocation_log::InvocationLog;
use model::{AgentKind, StatusPayload};
use sink::{SinkError, SinkType};

fn dispatch_sink(cli: &Cli, payload: &StatusPayload) -> Result<()> {
    match cli.sink {
        SinkType::Stdout => sink::write_line(&mut io::stdout().lock(), payload)?,
        SinkType::Serial => {
            let port = cli
                .port
                .as_deref()
                .filter(|p| !p.trim().is_empty())
                .ok_or_else(|| {
                    SinkError::Connection(
                        "Serial port path must be specified via --port or ASL_PORT environment variable"
                            .to_string(),
                    )
                })?;
            let timeout = Duration::from_millis(cli.timeout_ms);
            sink::serial::send(port, cli.baud, timeout, payload)?;
        }
        SinkType::Http => {
            return Err(SinkError::Other(
                "HTTP sink is not yet implemented (scheduled for PR 5)".to_string(),
            )
            .into());
        }
    }
    Ok(())
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
            // Reply first, so nothing that fails later can change what the agent sees.
            if cli.sink != SinkType::Stdout
                && let Some(response) = hook::response(cli.agent, cli.event.as_deref())
            {
                println!("{response}");
            }
            let agent = cli.agent.ok_or(HookError::MissingAgent)?;
            let input = hook::read_stdin()?;
            let parsed = hook::parse_hook(&input, agent, cli.event.as_deref());
            log.input = Some(input);
            match parsed? {
                Some(payload) => payload,
                // The event doesn't change the light.
                None => return Ok(()),
            }
        }
    };
    log.state = Some(payload.state);

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
