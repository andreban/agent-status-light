use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::sink_error::SinkError;

pub const SINK_SERIAL: &str = "serial";
pub const SINK_HTTP: &str = "http";
pub const SINK_STDOUT: &str = "stdout";

pub const DEFAULT_SINK_STR: &str = SINK_SERIAL;
pub const DEFAULT_SINK_TYPE: SinkType = SinkType::Serial;

/// Target transport sink for dispatching agent status updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum SinkType {
    /// Direct USB serial port connection.
    Serial,
    /// HTTP endpoint.
    Http,
    /// Standard output for dry-run or testing.
    Stdout,
}

impl SinkType {
    /// Returns the canonical string slice representation of the sink type.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Serial => SINK_SERIAL,
            Self::Http => SINK_HTTP,
            Self::Stdout => SINK_STDOUT,
        }
    }
}

impl Default for SinkType {
    fn default() -> Self {
        DEFAULT_SINK_TYPE
    }
}

impl fmt::Display for SinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for SinkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<SinkType> for &'static str {
    fn from(sink: SinkType) -> Self {
        sink.as_str()
    }
}

impl FromStr for SinkType {
    type Err = SinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SINK_SERIAL => Ok(Self::Serial),
            SINK_HTTP => Ok(Self::Http),
            SINK_STDOUT => Ok(Self::Stdout),
            _ => Err(SinkError::Other(format!("Unknown sink type: {s}"))),
        }
    }
}

impl TryFrom<&str> for SinkType {
    type Error = SinkError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[cfg(test)]
#[path = "sink_type_tests.rs"]
mod tests;
