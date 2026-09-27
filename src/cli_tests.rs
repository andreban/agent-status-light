use std::error::Error as StdError;
use std::path::PathBuf;

use clap::Parser;

use super::*;
use crate::model::{AgentKind, AgentState};
use crate::sink::SinkType;

type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[test]
fn test_cli_default_parsing() -> Result<()> {
    let cli = Cli::try_parse_from(["agent-status-light"])?;
    assert_eq!(cli.sink, SinkType::Serial);
    assert_eq!(cli.baud, DEFAULT_BAUD);
    assert_eq!(cli.timeout_ms, DEFAULT_TIMEOUT_MS);
    assert_eq!(cli.port, None);
    assert_eq!(cli.url, None);
    assert_eq!(cli.agent, None);
    assert_eq!(cli.event, None);
    assert_eq!(cli.log, None);
    assert_eq!(cli.command, None);
    Ok(())
}

#[test]
fn test_cli_custom_flags_parsing() -> Result<()> {
    let cli = Cli::try_parse_from([
        "agent-status-light",
        "--sink",
        "stdout",
        "--port",
        "/dev/ttyUSB0",
        "--baud",
        "9600",
        "--url",
        "http://192.168.1.100/status",
        "--timeout-ms",
        "1500",
    ])?;

    assert_eq!(cli.sink, SinkType::Stdout);
    assert_eq!(cli.port, Some("/dev/ttyUSB0".to_string()));
    assert_eq!(cli.baud, 9600);
    assert_eq!(cli.url, Some("http://192.168.1.100/status".to_string()));
    assert_eq!(cli.timeout_ms, 1500);
    assert_eq!(cli.command, None);
    Ok(())
}

#[test]
fn test_cli_agent_and_event_flags() -> Result<()> {
    let cli = Cli::try_parse_from([
        "agent-status-light",
        "--agent",
        "antigravity",
        "--event",
        "PreInvocation",
    ])?;

    assert_eq!(cli.agent, Some(AgentKind::Antigravity));
    assert_eq!(cli.event, Some("PreInvocation".to_string()));
    Ok(())
}

#[test]
fn test_cli_subcommand_set() -> Result<()> {
    let cli = Cli::try_parse_from(["agent-status-light", "set", "thinking"])?;
    assert_eq!(
        cli.command,
        Some(Command::Set {
            state: AgentState::Thinking
        })
    );

    let cli = Cli::try_parse_from(["agent-status-light", "set", "executing"])?;
    assert_eq!(
        cli.command,
        Some(Command::Set {
            state: AgentState::Executing
        })
    );

    let cli = Cli::try_parse_from(["agent-status-light", "set", "done"])?;
    assert_eq!(
        cli.command,
        Some(Command::Set {
            state: AgentState::Done
        })
    );

    let cli = Cli::try_parse_from(["agent-status-light", "set", "error"])?;
    assert_eq!(
        cli.command,
        Some(Command::Set {
            state: AgentState::Error
        })
    );

    let cli = Cli::try_parse_from(["agent-status-light", "set", "off"])?;
    assert_eq!(
        cli.command,
        Some(Command::Set {
            state: AgentState::Off
        })
    );

    let cli = Cli::try_parse_from(["agent-status-light", "set", "waiting"])?;
    assert_eq!(
        cli.command,
        Some(Command::Set {
            state: AgentState::Waiting
        })
    );
    Ok(())
}

#[test]
fn test_cli_invalid_state() {
    assert!(Cli::try_parse_from(["agent-status-light", "set", "dancing"]).is_err());
}

#[test]
fn test_cli_invalid_sink() {
    assert!(Cli::try_parse_from(["agent-status-light", "--sink", "bluetooth"]).is_err());
}

#[test]
fn test_cli_log_flag_parsing() -> Result<()> {
    let cli = Cli::try_parse_from(["agent-status-light", "--log", "hooks.jsonl"])?;
    assert_eq!(cli.log, Some(PathBuf::from("hooks.jsonl")));
    Ok(())
}
