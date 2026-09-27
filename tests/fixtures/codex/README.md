# Codex hook fixtures

Real Codex CLI (0.157.1, Windows) hook payloads captured with `ASL_LOG` (2026-09-27, `default` permission mode). The hooks were configured in `~/.codex/hooks.json`, using the same schema as Claude Code.

The payloads are scrubbed so they reveal nothing about the machine or projects they were captured on. Keys, types, event order and timing are unchanged.

- Paths are replaced with fixed placeholders (`C:\workspace\project`, `C:\path\to\transcript.jsonl`). Patch targets become `C:/workspace/project/capture.txt`.
- Patch contents, tool output, the subagent task message, prompts and final assistant messages are replaced with `<redacted>` or placeholder text.
- The test commands (`git --version`, `ping -n 3 127.0.0.1`, …) are kept as-is.

Like Claude Code, every payload names its own event in `hook_event_name`. Codex adds `turn_id` and `model`.

## Files

- `*.json`: one stdin payload per event and tool.
- `*.jsonl`: ordered sequences of `{"event", "t_ms", "payload"}` lines.
  - `turn_approvals_and_denial.jsonl`:
    - sandboxed commands (see the note below);
    - parallel commands;
    - a created and edited file (`apply_patch`, approved);
    - an approved escalated command;
    - a denied escalated command, which ends in `Interrupt`.
  - `turn_subagent.jsonl`: the main thread calls `spawn_agent` and `wait_agent`. The subagent runs a command between `SubagentStart` and `SubagentStop`. The turn ends with `Stop`.
  - `turn_failing_command_and_interrupt.jsonl`:
    - an approved command that runs and fails;
    - an approved `ping` interrupted with Esc while running, which fires `Interrupt`.

## Observations

- **`PermissionRequest` fires 0.4–0.8 s after `PreToolUse`**, for approved edits (`apply_patch`) and for escalated commands. It has no `tool_use_id`.
- **Denying a permission prompt fires `Interrupt`, and the turn is aborted.** So does Esc while a command runs. `Interrupt` carries `turn_id` but no tool details. Unlike Antigravity and Claude Code, Codex reports this, so the light can leave `waiting`/`executing` right away.
- **A command that runs and fails produces a normal `PostToolUse`.** The failure is visible only in `tool_response` output; there's no exit code field and no failure event.
- **A tool that is rejected before it runs gets no `PostToolUse`.** During capture, Codex's Windows sandbox failed to start (`helper_unknown_error`), so every sandboxed command was rejected this way; only escalated commands ran. The failed `apply_patch` delete behaved the same.
- **Tool calls issued together run their hooks concurrently.** The two `PreToolUse` hooks were 60 ms apart.
- **Subagents fire `SubagentStart` and `SubagentStop`** with `agent_id` and `agent_type`. The subagent's own tool hooks carry the same `agent_id`/`agent_type`; main-thread hooks don't.
- **Tool names seen:** `Bash`, `apply_patch`, `collaborationspawn_agent`, `collaborationwait_agent`.
- **`SessionStart` has `source: "startup"`, and `SessionEnd` has `reason: "other"`.** One `/quit` produced `SessionEnd` and another didn't, possibly because of the 1–3 s timeout on `SessionEnd` hooks.
- **Hook commands must print nothing and exit 0.** Codex parses stdout JSON as a decision and treats exit 1 as a hook failure. The capture used a `.cmd` wrapper that redirects output and always exits 0.
