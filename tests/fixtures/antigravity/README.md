# Antigravity hook fixtures

Real Antigravity hook payloads captured with `ASL_LOG` (2026-09-27, Antigravity CLI, `gemini-3.8-flash-high`). The username and project paths have been replaced with placeholders.

- `*.json`: one stdin payload per event and tool. The event name is not in the payload; it comes from the hook config (`--event`).
- `*.jsonl`: ordered sequences of `{"event", "t_ms", "payload"}`, where `t_ms` is milliseconds since the first event.
  - `turn_multi_tool.jsonl`: one complete turn with four `run_command` steps, ending in `Stop`.
  - `turn_approval_and_failed_tool.jsonl`: a partial turn that contains:
    - a user approval prompt for `git branch -a`: the ~12 s gap between PreToolUse and PostToolUse, with no event in between;
    - a `view_file` that failed inside Antigravity: PreToolUse with no PostToolUse (at `t_ms` 37976).

## Observations

- `PreInvocation`/`PostInvocation` wrap every model call. `invocationNum` counts up within a turn, so `PostInvocation` is not end of turn.
- `Stop` fires once per turn with `terminationReason` (`NO_TOOL_CALL` seen), `fullyIdle` and `error` (`""` on success).
- `PostToolUse` carries the same `toolCall` as `PreToolUse`, plus `error` (`""` on success).
- No hook event fires when Antigravity asks the user to approve a tool. A `{"decision":"allow"}` response to `PreToolUse` did not skip the approval prompt.
- A tool that fails before it runs may produce no `PostToolUse`.

Not yet captured: `ask_question`, parallel tool calls, a tool that fails while running (non-empty `error`), a cancelled turn, file edits.
