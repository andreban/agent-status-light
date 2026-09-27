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

### Current Architecture
- **Host Application**: Single, fast, stateless Rust CLI binary (`agent-status-light`). No background daemon process.
- **States**: `working`, `waiting`, `done`, `off`. The event→state mapping per agent lives in `src/hook/` and is tested against captured payloads in `tests/fixtures/`.
- **Hook Contract**:
  - In hook mode the binary always exits 0 and never blocks the agent.
  - It prints nothing to stdout, except the reply Antigravity expects.
  - Every hook passes `--agent`; Antigravity also needs `--event`.
- **Serial Sink**: Opens the serial port on each hook event, writes one line and closes it. It retries briefly when the port is busy (parallel tool calls run hooks concurrently). The port comes from `--port` or `ASL_PORT`.
- **Wire Protocol**: Strictly simplex (host-to-device), newline-delimited JSON (`{"state":"...","agent":"...","session_id":"..."}\n`). No handshake.
- **Firmware**: **Pimoroni Tiny 2040** (RP2040, native USB CDC, active-low RGB LED on GP18/GP19/GP20), in MicroPython.
  - Hardware-free logic lives in `status_light.py`, which is tested on the desktop.
  - The firmware turns `working`/`waiting` into `done` after 10 minutes without messages, because some agents send no event on cancel or deny.
- **Planned**: an HTTP sink for Wi-Fi devices (#5).
- **Not planned**: serial port auto-discovery; a background daemon; MQTT; multi-session coordination. Propose these only with a concrete need.


---

## 3. General Development Conventions

### Simplicity First
- Prefer the smallest design that works. Don't add abstractions, options, traits, trait impls or config for hypothetical future needs.
- If a later feature needs something, add it then.

### Evidence Over Assumptions
- Base agent integrations on real behaviour, not guesses. Capture hook payloads with `--log`/`ASL_LOG`, save them as fixtures in `tests/fixtures/<agent>/`, and test mappings against them.
- Check agent hook formats against the agent's official documentation.
- Scrub fixtures before committing:
  - paths become placeholders;
  - commands become generic;
  - prompts, file contents and tool output are redacted.

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
- **Deny Unsafe Code**: Enforce `unsafe_code = "deny"` declaratively in `Cargo.toml` under `[lints.rust]`.
- **No Dead Code Suppression**: Do not add `#[allow(dead_code)]` workarounds across application code or modules; scope test helpers with `#[cfg(test)]`.
- **Robust Error Handling**: Clean error handling via `thiserror` or idiomatic `Result` (no unhandled `unwrap()` or `expect()` in production code paths).
- **Linter & Formatting**: Code must pass `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`.
- **Cross-Platform Handling**: Never hardcode path separators or OS-specific assumptions.
- **Strict Domain Types & Serde Over Permissive Parsers**: Use strong domain types and enums (e.g., `AgentState`, `AgentKind`, `SinkType`) with standard Serde attributes rather than accepting loose `String` fields with custom parsing routines. Avoid speculative aliases and over-tolerant deserializers.
- **Binary Module Visibility**: In binary crates (`main.rs`), declare internal modules as private (`mod <name>;`) rather than `pub mod`. Avoid premature `pub use` re-exports before they are consumed to prevent compiler `unused_imports` warnings.
- **No Horizontal ASCII Dividers**: Do not use horizontal ASCII divider comments (such as `// -------------------`) to partition code files. If a module accumulates distinct responsibilities, decompose it into smaller, domain-specific modules.
- **Modularization**: Group related code into domain-specific modules. Split a module when it grows large or mixes responsibilities, not by default; a small type can live alongside the code that uses it.
- **Named Constants for Tunables**: Define timeouts, baud rates, retry counts and similar tunables as documented module-level constants (e.g., `pub const DEFAULT_BAUD: u32 = 115_200;`). Don't duplicate names that serde or clap derives already define (e.g. enum variant spellings) as string constants.
- **Direct Imports over Fully Qualified Paths**: Import types, traits, and modules (e.g. `Display`, `FromStr`, `Error`) at the top of the file using standard `use` statements. Only use fully qualified paths when strictly necessary to resolve ambiguity or namespace collisions.
- **Conversions Only When Used**: Implement a standard trait (`From`, `TryFrom`, `FromStr`, `Display`, `AsRef`) only when code actually uses it; don't give every type a full set of conversions. Prefer serde and clap derives for string mappings. When a conversion is needed, use the standard trait rather than an ad-hoc method (e.g. `to_json()`).
- **Avoid Speculative or Noisy Attributes**: Do not add `#[must_use]` or similar speculative attributes across application code, basic constructors, or getters unless specifically mandated by API requirements or project-wide linter configurations.

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




