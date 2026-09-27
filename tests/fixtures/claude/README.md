# Claude Code hook fixtures

Real Claude Code hook payloads captured with `ASL_LOG` (2026-09-27, in `auto` and then `default` permission mode).

The payloads are scrubbed so they reveal nothing about the machine or projects they were captured on. Keys, types, event order and timing are unchanged.

- Paths are replaced with fixed placeholders (`C:/workspace/project`, `C:/path/to/transcript.jsonl`), and file paths with `C:\workspace\project\src\fileN.ext`.
- Commands are replaced with generic equivalents (`git status`, `npm test`, `gh issue view 1`, `python example.py`).
- `prompt` and `last_assistant_message` are replaced with placeholder text.
- File contents, tool output, edit descriptions and error details are replaced with `<redacted>`. Errors that start with an exit code keep just that line (`Exit code 1`).

Unlike Antigravity, every payload names its own event in `hook_event_name`.

## Files

- `*.json`: one stdin payload per event, tool and notification type.
- `parallel_tool_calls.jsonl`: two Bash calls issued in one response, as `{"event", "t_ms", "sink_ok", "payload"}` lines.
- `permission_prompt_and_question.jsonl` (same format, `default` permission mode):
  - an approved Bash command;
  - an answered `AskUserQuestion`;
  - an approved `Read` that then failed.
- `interrupted_at_permission_prompt.jsonl`: the user pressed Esc while a permission prompt was showing.
- `denied_permission_prompt.jsonl`: the user chose **No** at a permission prompt.
- `interrupted_running_tool.jsonl`: the user pressed Esc while an approved Bash command was running (`auto` mode, so no prompt).

## Observations

- **`PermissionRequest` is the timely "waiting" signal.** It fires about 150 ms after `PreToolUse`, when the prompt appears. `Notification` with `permission_prompt` arrives about 6 s later. `AskUserQuestion` also goes through `PermissionRequest`, so one mapping covers both approvals and questions. PostToolUse fires once the user approves or answers.
- **Interrupts and denials fire no hook.** This covers Esc or **No** at a permission prompt, and Esc while a tool is running. In each case there was no PostToolUse, no `PostToolUseFailure` (not even with `is_interrupt: true`) and no `Stop`; the next event was the user's next `UserPromptSubmit`. The light stays on its last state (`waiting` or `executing`) until then. `PermissionDenied` was subscribed for a second denial and didn't fire either; it appears to cover only auto-mode denials.
- **Read-only commands (e.g. `echo`) skip `PermissionRequest`**, even in `default` mode.
- **`SessionEnd` carries a `reason`.** `prompt_input_exit` came from `/exit`. `resume` came from a fresh session that ended when the user resumed an older one, so it fired for a session that did nothing. `SessionEnd` from any session turns the light off, even if another session is busy.
- **Parallel tool calls run their hooks concurrently.** The two PreToolUse hooks started 5 ms apart. The second failed to open the serial port ("Access is denied" on Windows), so that update was dropped (`"sink_ok": false`).
- **Tool completion order isn't call order.** When the short call's PostToolUse arrived first, the light went back to `thinking` while the other tool was still running.
- **Routine command failures fire `PostToolUseFailure`.** `error` holds the exit code followed by stderr (only `Exit code 1` is kept here), and `is_interrupt` is `false`. Mapping this to the `error` state flashes red for ordinary failures such as a script typo.
- **`Notification` with `idle_prompt`** arrives about 60 s after `Stop` while the prompt sits idle ("Claude is waiting for your input"). It's redundant after `Stop`.
- **`Stop` includes `background_tasks` and `session_crons`.** A non-empty `background_tasks` may mean the session isn't really idle.
- PreToolUse/PostToolUse pairs share a `tool_use_id`, so overlapping calls can be matched up.

Not yet captured:
- subagent events.
