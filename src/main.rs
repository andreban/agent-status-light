mod cli;
mod error;
mod model;
mod sink;

use std::process;

use clap::Parser;

use cli::{Cli, Command};
use error::Result;
use model::{AgentKind, StatusPayload};
use sink::{StatusSink, StdoutSink};

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Set { state }) => {
            let payload = StatusPayload::with_default_session(state, AgentKind::Cli);
            let mut sink = StdoutSink::new();
            sink.send(&payload)?;
        }
        None => {
            println!("agent-status-light: no command provided (use --help for usage)");
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
