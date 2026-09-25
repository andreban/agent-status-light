# AGENTS.md — Agent & Developer Guide

## 1. Project Overview

`agent-status-light` connects AI coding agents to physical hardware status lights (e.g. RGB LEDs, NeoPixels) connected via USB serial.

When an AI agent is active (thinking, running tools, waiting for user input, succeeding, or failing), the physical light provides real-time ambient status feedback.

### Confirmed Requirements

- **Host Application**: Built in Rust. Listens to agent lifecycle hooks and communicates status over serial to a connected microcontroller.
- **Universal Agent Support**: Must support the primary agent hook systems:
  - Google Antigravity
  - Anthropic Claude Code
  - OpenAI Codex
- **Cross-Platform**: First-class support for:
  - Windows (`x86_64` and `arm64`)
  - Linux (`x86_64` and `arm64`)
  - macOS (`x86_64` and `arm64`)
- **Polyglot & Multi-Board Firmware**:
  - Firmware organization must support different microcontroller boards.
  - Implementations may be written in Rust, MicroPython, C/C++, or other languages.

---

## 2. Architecture Decisions & Plan

The project plan and architectural specification are documented in ZNotes note `75df5220-10d0-4e72-9a50-7ea1c6b86710`.

### Confirmed MVP Architecture
- **Host Application**: Single, fast, stateless Rust CLI binary (`agent-status-light`). No background daemon process.
- **Output Sinks**:
  - **Direct Serial Sink**: Connects directly to the serial port on each hook event with `dtr=false, rts=false` (prevents MCU reboot), writes the state command, and closes. Port discovery via CLI arg, env var, or USB VID/PID matching.
  - **Direct HTTP Sink**: Sends an HTTP request via `ureq` to a local network endpoint (Wi-Fi MCU or WLED).
- **Wire Protocol**: Strictly simplex (host-to-device). Unified newline-delimited JSON payload (`{"state":"...","agent":"...","session_id":"..."}\n`) shared across both Serial and HTTP. No ping/pong handshake.
- **First Firmware Target**: **Pimoroni Tiny 2040** (RP2040, native USB CDC, active-low onboard RGB LED on GP18/GP19/GP20).
- **Deferred to v2**: Background daemon mode, MQTT broker transport, and multi-session store.


---

## 3. General Development Conventions

### Test Organization
- **Sibling Test Files**: Externalize unit tests to a sibling `[file_name]_tests.rs` file using `#[path = "..."]` across all modules, keeping production code files concise and separated from test code.
  - In `src/model/event.rs`:
    ```rust
    #[cfg(test)]
    #[path = "event_tests.rs"]
    mod tests;
    ```
  - In `src/model/event_tests.rs`:
    ```rust
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    use super::*;

    #[test]
    fn test_event_normalization() -> Result<()> {
        let event = parse_event("{}")?;
        assert_eq!(event.state, AgentState::Thinking);
        Ok(())
    }
    ```

### Rust Architecture & Code Standards
- **Deny Unsafe Code**: Enforce `#![deny(unsafe_code)]`.
- **Robust Error Handling**: Clean error handling via `thiserror` or idiomatic `Result` (no unhandled `unwrap()` or `expect()` in production code paths).
- **Linter & Formatting**: Code must pass `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`.
- **Cross-Platform Handling**: Never hardcode path separators or OS-specific assumptions.
- **Strict Domain Types & Serde Over Permissive Parsers**: Use strong domain types and enums (e.g., `AgentState`, `AgentKind`, `SinkType`) with standard Serde attributes rather than accepting loose `String` fields with custom parsing routines. Avoid speculative aliases and over-tolerant deserializers.
- **Binary Module Visibility**: In binary crates (`main.rs`), declare internal modules as private (`mod <name>;`) rather than `pub mod`. Avoid premature `pub use` re-exports before they are consumed to prevent compiler `unused_imports` warnings.
- **No Horizontal ASCII Dividers**: Do not use horizontal ASCII divider comments (such as `// -------------------`) to partition code files. If a module accumulates distinct responsibilities, decompose it into smaller, domain-specific modules.
- **Modularization & Single-Type Files**: Structure code into clean, domain-specific modules grouping related types. Aim for one file per strong type (e.g. `agent_state.rs`, `agent_kind.rs`, `payload.rs`) paired with sibling tests, rather than bundling multiple distinct types into a single large file.
- **Named Constants over Magic Literals**: Avoid hardcoding inline magic strings or numbers for timeouts, baud rates, default states, or hardware IDs. Define them as documented module-level constants (e.g., `pub const DEFAULT_BAUD: u32 = 115_200;`, `pub const PIMORONI_TINY2040_VID: u16 = 0x16D0;`).

### Cargo & Dependency Management
- **Installing Dependencies via Cargo**: Always install dependencies using `cargo add <crate>` (along with any required `--features`) instead of writing them directly into `Cargo.toml`. This ensures the latest compatible versions are resolved and recorded.


### Firmware Standards
- Keep device logic decoupled so new boards and runtimes can be added easily.
- Handle serial disconnection and corrupted input safely without hanging or panicking.

### Research & Tooling Conventions
- Use the `gh` CLI to inspect, clone, or read GitHub repositories (e.g. `gh repo view`, `gh api`).

### Work Planning & PR Sizing
- **Bounded Change Size**: When planning and executing work, keep production code changes to a maximum of around ~500 lines per PR (excluding tests, configuration files, and data files).
- **Incremental Delivery**: Decompose larger features into discrete, reviewable increments that each pass tests, formatting, and linter checks independently.

### Pre-PR Verification Checklist
Before creating or submitting any pull request, you MUST execute the following and verify that all pass with zero warnings, errors, or formatting violations:
1. **Format check**: `cargo fmt --all -- --check`
2. **Clippy check**: `cargo clippy --workspace --all-targets -- -D warnings`
3. **Test suite**: `cargo test --workspace`




