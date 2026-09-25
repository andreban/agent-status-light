mod cli;
mod error;
mod hook;
mod model;
mod sink;

use std::io::{self, IsTerminal};
use std::process;

use clap::Parser;

use cli::{Cli, Command};
use error::Result;
use model::{AgentKind, StatusPayload};
use sink::{StatusSink, StdoutSink};

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

    let mut sink = StdoutSink::new();
    sink.send(&payload)?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
