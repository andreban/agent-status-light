use thiserror::Error;

use crate::hook::HookError;
use crate::sink::SinkError;

/// Application errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Sink error: {0}")]
    Sink(#[from] SinkError),
    #[error("Hook error: {0}")]
    Hook(#[from] HookError),
}

pub type Result<T> = std::result::Result<T, Error>;
