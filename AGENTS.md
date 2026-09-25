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

## 2. Architecture Status

> **Notice**: The concrete system architecture, host-device communication model, serial protocol, and supported board list have **not yet been finalized**.

AI agents working in this repository must adhere to the following:
- **Do not invent or assume premature architectural patterns** (e.g. daemons, IPC mechanisms, wire protocol formats, or specific board selections) until they are explicitly decided and documented.
- Keep components modular and strictly focused on confirmed requirements.

---

## 3. General Development Conventions

### Rust Standards
- Deny unsafe code where possible (`#![deny(unsafe_code)]`).
- Clean error handling (no unhandled `unwrap()` or `expect()` in production code paths).
- Code must pass `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`.
- Cross-platform path and OS handling: never hardcode path separators or OS-specific assumptions.

### Firmware Standards
- Keep device logic decoupled so new boards and runtimes can be added easily.
- Handle serial disconnection and corrupted input safely without hanging or panicking.
