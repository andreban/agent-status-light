# Antigravity hook fixtures

Real Antigravity hook payloads captured with `ASL_LOG` (2026-09-27, Antigravity CLI, `gemini-3.8-flash-high`). The username and project paths have been replaced with placeholders.

- `*.json`: one stdin payload per event and tool. The event name is not in the payload; it comes from the hook config (`--event`).
- `*.jsonl`: ordered sequences of `{"event", "t_ms", "payload"}`, where `t_ms` is milliseconds since the first event.
  - `turn_multi_tool.jsonl`: one complete turn with four `run_command` steps, ending in `Stop`.
  - `turn_approval_and_failed_tool.jsonl`: a partial turn that contains:
    - a user approval prompt for `git branch -a`: the ~12 s gap between PreToolUse and PostToolUse, with no event in between;
    - a `view_file` that failed inside Antigravity: PreToolUse with no PostToolUse (at `t_ms` 37976).
  - `turn_cancelled.jsonl`: the end of the same turn, which the user cancelled during a model call. The last event is `PreInvocation`, and nothing follows it: no `Stop`, and no transcript entry.
  - `turn_ask_question.jsonl`: a turn where the agent asked a question with `ask_question`. PreToolUse and PostToolUse are ~10 s apart while the user answers; the turn then ends with `Stop`.

## Observations

- `PreInvocation`/`PostInvocation` wrap every model call. `invocationNum` counts up within a turn, so `PostInvocation` is not end of turn.
- `Stop` fires once per turn with `terminationReason` (`NO_TOOL_CALL` seen), `fullyIdle` and `error` (`""` on success).
- `PostToolUse` carries the same `toolCall` as `PreToolUse`, plus `error` (`""` on success).
- No hook event fires when Antigravity asks the user to approve a tool. A `{"decision":"allow"}` response to `PreToolUse` did not skip the approval prompt.
- A tool that fails before it runs may produce no `PostToolUse`.
- `ask_question` shows up as an ordinary tool call: PreToolUse fires when the question is shown and PostToolUse when it's answered. The answer isn't included in the payload.
- Cancelling a turn fires no event, so the last state stays until the next prompt.

Not yet captured: parallel tool calls, a tool that fails while running (non-empty `error`), file edits.
