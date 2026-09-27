use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::model::{AgentKind, AgentState};
use crate::sink::{DEFAULT_BAUD, DEFAULT_SINK_STR, DEFAULT_TIMEOUT_MS, SinkType};

/// Command-line arguments and configuration for agent-status-light.
#[derive(Debug, Clone, Parser, PartialEq, Eq)]
#[command(
    name = "agent-status-light",
    about = "Connects AI coding agents to physical status lights",
    version
)]
pub struct Cli {
    /// Output transport sink.
    #[arg(
        long,
        env = "ASL_SINK",
        default_value = DEFAULT_SINK_STR,
        help = "Output transport sink (serial, http, stdout)"
    )]
    pub sink: SinkType,

    /// Serial port path (required when using serial sink).
    #[arg(long, env = "ASL_PORT", help = "Serial port path")]
    pub port: Option<String>,

    /// Serial communication baud rate.
    #[arg(
        long,
        env = "ASL_BAUD",
        default_value_t = DEFAULT_BAUD,
        help = "Serial baud rate"
    )]
    pub baud: u32,

    /// Target HTTP endpoint URL for http sink.
    #[arg(long, env = "ASL_URL", help = "Target HTTP endpoint URL")]
    pub url: Option<String>,

    /// Transport timeout in milliseconds.
    #[arg(
        long,
        env = "ASL_TIMEOUT_MS",
        default_value_t = DEFAULT_TIMEOUT_MS,
        help = "Transport timeout in milliseconds"
    )]
    pub timeout_ms: u64,

    /// Agent runtime kind override.
    #[arg(
        long,
        help = "Agent runtime override (antigravity, claude, codex, cli)"
    )]
    pub agent: Option<AgentKind>,

    /// Hook event name override.
    #[arg(long, help = "Hook event name override")]
    pub event: Option<String>,

    /// Diagnostic log file capturing raw hook payloads and outcomes.
    #[arg(
        long,
        env = "ASL_LOG",
        help = "Append a JSON line per invocation (raw payload, state, errors) to this file"
    )]
    pub log: Option<PathBuf>,

    /// Optional subcommand.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Available CLI subcommands.
#[derive(Debug, Clone, Subcommand, PartialEq, Eq)]
pub enum Command {
    /// Manually trigger a status light state.
    Set {
        /// Target agent lifecycle state.
        #[arg(value_enum, help = "Agent state to set")]
        state: AgentState,
    },
}

#[cfg(test)]
#[path = "cli_tests.rs"]
mod tests;
