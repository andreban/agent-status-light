# agent-status-light

Physical status lights for AI coding agents.

## Goal

Connect AI coding agents to physical hardware status lights via USB serial to provide real-time ambient feedback on your desk while agents are working (thinking, running tools, waiting for approval, succeeding, or failing).

## Objectives

- **Universal Agent Support**: Listen to lifecycle hooks from major AI agents (Google Antigravity, Anthropic Claude Code, OpenAI Codex).
- **Rust Host Software**: Implement the host-side hook and serial transmission application in Rust.
- **Cross-Platform**: First-class support for Windows, Linux, and macOS across both `x86_64` and `arm64` architectures.
- **Multi-Device Firmware**: A modular architecture supporting firmwares for different microcontroller boards in various languages (Rust, MicroPython, C/C++, etc.).

## License

[Apache 2.0](LICENSE)